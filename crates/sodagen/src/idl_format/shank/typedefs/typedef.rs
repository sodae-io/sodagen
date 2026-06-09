#![allow(non_camel_case_types)]

use std::collections::HashSet;
use std::str::FromStr;

use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use syn::Index;
use void::Void;

use crate::utils::{primitive_or_pubkey_to_token, string_or_struct, PUBKEY_TOKEN};

#[derive(Deserialize)]
pub struct NamedType {
    pub name: String,
    pub r#type: TypedefType,
}

/// Tolerant, slice-based field deserializations for shank structs — mirrors the
/// anchor generator: defaultable fields default at EOF, and directly-nested
/// struct fields are read via their own slice `deserialize` so tolerance
/// propagates through nesting.
pub fn gen_field_deserializations(
    fields: &[TypedefField],
    no_default_types: &HashSet<String>,
    struct_types: &HashSet<String>,
) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let field_name = format_ident!("{}", f.name.to_snake_case());
            let field_type = &f.r#type;
            let defaultable = f.r#type.is_optional()
                || (!f.r#type.references_no_default_type(no_default_types)
                    && !f.r#type.is_big_array());
            let is_nested_struct = matches!(
                &f.r#type,
                TypedefFieldType::defined(name) if struct_types.contains(name)
            );
            match (is_nested_struct, defaultable) {
                (true, true) => quote! {
                    let #field_name = if reader.is_empty() {
                        Default::default()
                    } else {
                        <#field_type>::deserialize(&mut reader)?
                    };
                },
                (true, false) => quote! {
                    let #field_name = <#field_type>::deserialize(&mut reader)?;
                },
                (false, true) => quote! {
                    let #field_name: #field_type = crate::borsh_de_or_default(&mut reader)?;
                },
                (false, false) => quote! {
                    let #field_name = <#field_type as borsh::BorshDeserialize>::deserialize_reader(&mut reader)?;
                },
            }
        })
        .collect()
}

impl NamedType {
    pub fn to_token_stream(
        &self,
        cli_args: &crate::Args,
        no_default_types: &HashSet<String>,
        struct_types: &HashSet<String>,
    ) -> TokenStream {
        let name = format_ident!("{}", self.name);
        // rust enums cannot impl Pod due to illegal bitpatterns
        let typedef_struct = match &self.r#type {
            TypedefType::r#struct(typedef_struct) => typedef_struct,
            TypedefType::r#enum(typedef_enum) => {
                let first_is_unit = typedef_enum.variants.first()
                    .map_or(false, |v| v.fields.is_none());
                if first_is_unit {
                    return quote! {
                        #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)]
                        pub enum #name {
                            #[default]
                            #typedef_enum
                        }
                    }
                } else {
                    return quote! {
                        #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)]
                        pub enum #name {
                            #typedef_enum
                        }
                    }
                }
            }
        };

        let has_big_array = typedef_struct.fields.iter().any(|f| f.r#type.is_big_array());
        let has_no_default_field = typedef_struct.fields.iter().any(|f| f.r#type.references_no_default_type(no_default_types));
        let needs_default = !has_big_array && !has_no_default_field;

        let derive = if cli_args.zero_copy.iter().any(|e| e == &self.name) {
            if needs_default {
                quote! {
                    #[repr(C)]
                    #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable, serde::Serialize, serde::Deserialize)]
                }
            } else {
                quote! {
                    #[repr(C)]
                    #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable, serde::Serialize, serde::Deserialize)]
                }
            }
        } else {
            if needs_default {
                quote! {
                    #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)]
                }
            } else {
                quote! {
                    #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)]
                }
            }
        };
        let field_names = typedef_struct
            .fields
            .iter()
            .map(|f| format_ident!("{}", f.name.to_snake_case()));
        let field_des =
            gen_field_deserializations(&typedef_struct.fields, no_default_types, struct_types);
        quote! {
            #derive
            pub struct #name {
                #typedef_struct
            }
            impl #name {
                pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
                    let mut reader: &[u8] = *__buf;
                    #(#field_des)*
                    *__buf = reader;
                    Ok(Self { #(#field_names),* })
                }
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind")]
pub enum TypedefType {
    r#struct(TypedefStruct),
    r#enum(TypedefEnum),
}

#[derive(Deserialize)]
pub struct TypedefStruct {
    pub fields: Vec<TypedefField>,
}

#[derive(Deserialize)]
pub struct TypedefField {
    pub name: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub r#type: TypedefFieldType,
}

/// All instances should be annotated with
/// deserialize_with = "string_or_struct"
#[derive(Deserialize)]
pub enum TypedefFieldType {
    // handled by string_or_struct's string
    PrimitiveOrPubkey(String),

    // rest handled by string_or_struct's struct
    defined(String),
    array(TypedefFieldArray),

    #[serde(deserialize_with = "string_or_struct")]
    option(Box<TypedefFieldType>),

    #[serde(deserialize_with = "string_or_struct")]
    vec(Box<TypedefFieldType>),
}

#[derive(Deserialize)]
pub struct TypedefFieldArray(
    #[serde(deserialize_with = "string_or_struct")] Box<TypedefFieldType>,
    u32, // borsh spec says array sizes are u32
);

/// serde newtype workaround for use in Vec<TypedefFieldType>:
/// https://github.com/serde-rs/serde/issues/723#issuecomment-871016087
#[derive(Deserialize)]
pub struct TypedefFieldTypeWrap(#[serde(deserialize_with = "string_or_struct")] TypedefFieldType);

impl FromStr for TypedefFieldType {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::PrimitiveOrPubkey(s.into()))
    }
}

impl FromStr for Box<TypedefFieldType> {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Box::new(TypedefFieldType::from_str(s)?))
    }
}

#[derive(Deserialize)]
pub struct TypedefEnum {
    pub variants: Vec<EnumVariant>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum EnumVariantFields {
    Struct(Vec<TypedefField>),
    Tuple(Vec<TypedefFieldTypeWrap>),
}

impl EnumVariantFields {
    pub fn has_pubkey(&self) -> bool {
        match self {
            Self::Struct(v) => v.iter().any(|f| f.r#type.is_or_has_pubkey()),
            Self::Tuple(v) => v.iter().any(|f| f.0.is_or_has_pubkey()),
        }
    }

    pub fn has_defined(&self) -> bool {
        match self {
            Self::Struct(v) => v.iter().any(|f| f.r#type.is_or_has_defined()),
            Self::Tuple(v) => v.iter().any(|f| f.0.is_or_has_defined()),
        }
    }
}

#[derive(Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<EnumVariantFields>,
}

impl ToTokens for TypedefStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let typedef_fields = self.fields.iter().map(|f| {
            let big_array_attr = if f.r#type.is_big_array() {
                quote! { #[serde(with = "crate::big_array_serde")] }
            } else {
                quote! {}
            };
            quote! { #big_array_attr pub #f }
        });
        tokens.extend(quote! {
            #(#typedef_fields),*
        })
    }
}

impl ToTokens for TypedefField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name.to_snake_case());
        let ty = &self.r#type;
        tokens.extend(quote! {
            #name: #ty
        })
    }
}

impl ToTokens for TypedefFieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty: TokenStream = match self {
            Self::PrimitiveOrPubkey(s) => primitive_or_pubkey_to_token(s).parse().unwrap(),
            Self::defined(s) => s.parse().unwrap(),
            Self::array(a) => a.to_token_stream(),
            Self::vec(v) => quote! {
                Vec<#v>
            },
            Self::option(o) => quote! {
                Option<#o>
            },
        };
        tokens.extend(ty);
    }
}

impl ToTokens for TypedefFieldArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.0;
        let n = Index::from(self.1 as usize);
        tokens.extend(quote! {
            [#ty; #n]
        })
    }
}

impl ToTokens for TypedefEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variants = &self.variants;
        tokens.extend(quote! {
            #(#variants),*
        })
    }
}

// TODO: handle complex enum structs
impl ToTokens for EnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let v = format_ident!("{}", self.name.to_pascal_case());
        let maybe_inner_fields = self
            .fields
            .as_ref()
            .map_or(quote! {}, |fields| match fields {
                EnumVariantFields::Struct(v) => {
                    let typedef_fields = v.iter();
                    quote! {
                        { #(#typedef_fields),* }
                    }
                }
                EnumVariantFields::Tuple(v) => {
                    let unnamed_fields = v.iter().map(|wrap| &wrap.0);
                    quote! {
                        ( #(#unnamed_fields),* )
                    }
                }
            });
        tokens.extend(quote! {
            #v #maybe_inner_fields
        });
    }
}

impl TypedefType {
    pub fn has_pubkey_field(&self) -> bool {
        match self {
            Self::r#enum(e) => e.variants.iter().any(|e| e.has_pubkey()),
            Self::r#struct(s) => s.fields.iter().any(|f| f.r#type.is_or_has_pubkey()),
        }
    }

    pub fn has_defined_field(&self) -> bool {
        match self {
            Self::r#enum(e) => e.variants.iter().any(|e| e.has_defined()),
            Self::r#struct(s) => s.fields.iter().any(|f| f.r#type.is_or_has_defined()),
        }
    }
}

impl TypedefFieldType {
    pub fn is_or_has_pubkey(&self) -> bool {
        match self {
            Self::PrimitiveOrPubkey(s) => primitive_or_pubkey_to_token(s) == PUBKEY_TOKEN,
            Self::array(a) => a.0.is_or_has_pubkey(),
            Self::option(o) => o.is_or_has_pubkey(),
            Self::vec(v) => v.is_or_has_pubkey(),
            Self::defined(_) => false,
        }
    }

    pub fn is_or_has_defined(&self) -> bool {
        match self {
            Self::PrimitiveOrPubkey(_) => false,
            Self::array(a) => a.0.is_or_has_defined(),
            Self::option(o) => o.is_or_has_defined(),
            Self::vec(v) => v.is_or_has_defined(),
            Self::defined(_) => true,
        }
    }

    pub fn is_big_array(&self) -> bool {
        match self {
            Self::array(a) => a.1 > 32,
            _ => false,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Self::option(_) => true,
            Self::defined(name) => name.starts_with("Option"),
            _ => false,
        }
    }

    pub fn references_no_default_type(&self, no_default_types: &HashSet<String>) -> bool {
        match self {
            Self::defined(name) => no_default_types.contains(name),
            Self::array(a) => a.0.references_no_default_type(no_default_types),
            Self::option(o) => o.references_no_default_type(no_default_types),
            Self::vec(v) => v.references_no_default_type(no_default_types),
            Self::PrimitiveOrPubkey(_) => false,
        }
    }
}

impl EnumVariant {
    pub fn has_pubkey(&self) -> bool {
        match &self.fields {
            None => false,
            Some(fields) => fields.has_pubkey(),
        }
    }

    pub fn has_defined(&self) -> bool {
        match &self.fields {
            None => false,
            Some(fields) => fields.has_defined(),
        }
    }
}

pub fn compute_no_default_types(types: &[&NamedType]) -> HashSet<String> {
    let mut no_default: HashSet<String> = HashSet::new();

    // Structs with big array fields
    for t in types {
        if let TypedefType::r#struct(s) = &t.r#type {
            if s.fields.iter().any(|f| f.r#type.is_big_array()) {
                no_default.insert(t.name.clone());
            }
        }
    }

    // Enums derive Default only when their first variant is a unit variant
    // (it gets #[default]); any other shape cannot. Must match the derive gate
    // in to_token_stream so tolerant field reads don't emit Default::default()
    // for a non-Default enum field.
    for t in types {
        if let TypedefType::r#enum(e) = &t.r#type {
            let first_is_unit = e.variants.first().map_or(false, |v| v.fields.is_none());
            if !first_is_unit {
                no_default.insert(t.name.clone());
            }
        }
    }

    // Propagate: types containing fields of no-default types
    let mut changed = true;
    while changed {
        changed = false;
        for t in types {
            if no_default.contains(&t.name) {
                continue;
            }
            if let TypedefType::r#struct(s) = &t.r#type {
                if s.fields.iter().any(|f| f.r#type.references_no_default_type(&no_default)) {
                    no_default.insert(t.name.clone());
                    changed = true;
                }
            }
        }
    }

    no_default
}
