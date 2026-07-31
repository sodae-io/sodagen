#![allow(non_camel_case_types)]

use std::collections::HashSet;
use std::{fmt, str::FromStr};

use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::{de, Deserialize, Deserializer};
use syn::Index;
use void::Void;

use crate::utils::{
    conditional_pascal_case, primitive_or_pubkey_to_token, string_or_struct, PUBKEY_TOKEN,
};

#[derive(Clone, Deserialize)]
pub struct NamedType {
    pub name: String,
    pub r#type: TypedefType,
}

impl NamedType {
    pub fn to_token_stream(&self, cli_args: &crate::Args) -> TokenStream {
        self.to_token_stream_with_no_default(cli_args, true, &HashSet::new())
    }

    pub fn to_token_stream_with_no_default(
        &self,
        cli_args: &crate::Args,
        derive_borsh: bool,
        no_default_types: &HashSet<String>,
    ) -> TokenStream {
        self.to_token_stream_inner(cli_args, derive_borsh, no_default_types)
    }

    /// Generate struct definition for accounts and events which also get manual
    /// field-by-field methods. Borsh derives are still included so these types can
    /// be used as fields in other borsh-serialized structs.
    pub fn to_token_stream_no_borsh(
        &self,
        cli_args: &crate::Args,
        no_default_types: &HashSet<String>,
    ) -> TokenStream {
        self.to_token_stream_inner(cli_args, true, no_default_types)
    }

    fn to_token_stream_inner(
        &self,
        cli_args: &crate::Args,
        derive_borsh: bool,
        no_default_types: &HashSet<String>,
    ) -> TokenStream {
        let name = format_ident!("{}", conditional_pascal_case(&self.name));
        // rust enums cannot impl Pod due to illegal bitpatterns
        let typedef_struct = match &self.r#type {
            TypedefType::r#struct(typedef_struct) => typedef_struct,
            TypedefType::r#enum(typedef_enum) => {
                let first_is_unit = typedef_enum
                    .variants
                    .first()
                    .map_or(false, |v| v.fields.is_none());
                let (derive, default_attr) = if first_is_unit {
                    let d = if derive_borsh {
                        quote! { #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)] }
                    } else {
                        quote! { #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)] }
                    };
                    (d, quote! { #[default] })
                } else {
                    let d = if derive_borsh {
                        quote! { #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)] }
                    } else {
                        quote! { #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)] }
                    };
                    (d, quote! {})
                };
                return quote! {
                    #derive
                    pub enum #name {
                        #default_attr
                        #typedef_enum
                    }
                };
            }
        };

        let has_big_array = typedef_struct
            .fields
            .iter()
            .any(|f| f.r#type.is_big_array());
        let has_no_default_field = typedef_struct
            .fields
            .iter()
            .any(|f| f.r#type.references_no_default_type(no_default_types));
        let needs_default = !has_big_array && !has_no_default_field;

        let derive = if cli_args.zero_copy.iter().any(|e| e == &self.name) {
            match (derive_borsh, needs_default) {
                (true, true) => quote! {
                    #[repr(C)]
                    #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable, serde::Serialize, serde::Deserialize)]
                },
                (true, false) => quote! {
                    #[repr(C)]
                    #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable, serde::Serialize, serde::Deserialize)]
                },
                (false, true) => quote! {
                    #[repr(C)]
                    #[derive(Clone, Debug, Default, PartialEq, Pod, Copy, Zeroable, serde::Serialize, serde::Deserialize)]
                },
                (false, false) => quote! {
                    #[repr(C)]
                    #[derive(Clone, Debug, PartialEq, Pod, Copy, Zeroable, serde::Serialize, serde::Deserialize)]
                },
            }
        } else {
            match (derive_borsh, needs_default) {
                (true, true) => quote! {
                    #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)]
                },
                (true, false) => quote! {
                    #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)]
                },
                (false, true) => quote! {
                    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
                },
                (false, false) => quote! {
                    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
                },
            }
        };
        quote! {
            #derive
            pub struct #name {
                #typedef_struct
            }
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum TypedefType {
    r#struct(TypedefStruct),
    r#enum(TypedefEnum),
}

#[derive(Clone)]
pub struct TypedefStruct {
    pub fields: Vec<TypedefField>,
}

impl<'de> Deserialize<'de> for TypedefStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawTypedefStruct {
            // Anchor omits `fields` entirely for empty structs — unit events
            // such as `PauseEvent` serialize as bare {"kind": "struct"}.
            // Without the default, V1 detection fails and the IDL falls
            // through to the old-Anchor parser, which then panics.
            #[serde(default)]
            fields: Vec<serde_json::Value>,
        }

        let raw = RawTypedefStruct::deserialize(deserializer)?;
        let fields = raw
            .fields
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if v.is_string() {
                    // Tuple struct field: just a type string like "bool"
                    let type_str = v.as_str().unwrap();
                    Ok(TypedefField {
                        name: format!("field_{}", i),
                        r#type: TypedefFieldType::PrimitiveOrPubkey(type_str.to_owned()),
                    })
                } else {
                    // Named field: {"name": "foo", "type": "u64"}
                    serde_json::from_value(v).map_err(serde::de::Error::custom)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(TypedefStruct { fields })
    }
}

#[derive(Clone, Deserialize)]
pub struct TypedefField {
    pub name: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub r#type: TypedefFieldType,
}

/// Deserializes the `defined` field which can be either:
/// - A plain string: `"defined": "Foo"` (old format)
/// - An object with name: `"defined": {"name": "Foo"}` (new V1 format)
fn deserialize_defined_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct DefinedNameVisitor;

    impl<'de> de::Visitor<'de> for DefinedNameVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or an object with a \"name\" field")
        }

        fn visit_str<E>(self, value: &str) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_map<M>(self, mut map: M) -> Result<String, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let mut name: Option<String> = None;
            while let Some(key) = map.next_key::<String>()? {
                if key == "name" {
                    name = Some(map.next_value()?);
                } else {
                    // skip unknown fields
                    let _: serde_json::Value = map.next_value()?;
                }
            }
            name.ok_or_else(|| de::Error::missing_field("name"))
        }
    }

    deserializer.deserialize_any(DefinedNameVisitor)
}

/// All instances should be annotated with
/// deserialize_with = "string_or_struct"
#[derive(Clone, Deserialize)]
pub enum TypedefFieldType {
    // handled by string_or_struct's string
    PrimitiveOrPubkey(String),

    // rest handled by string_or_struct's struct
    #[serde(deserialize_with = "deserialize_defined_name")]
    defined(String),
    array(TypedefFieldArray),

    #[serde(deserialize_with = "string_or_struct")]
    option(Box<TypedefFieldType>),

    #[serde(deserialize_with = "string_or_struct")]
    vec(Box<TypedefFieldType>),
}

#[derive(Clone, Deserialize)]
pub struct TypedefFieldArray(
    #[serde(deserialize_with = "string_or_struct")] Box<TypedefFieldType>,
    u32, // borsh spec says array sizes are u32
);

/// serde newtype workaround for use in Vec<TypedefFieldType>:
/// https://github.com/serde-rs/serde/issues/723#issuecomment-871016087
#[derive(Clone, Deserialize)]
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

#[derive(Clone, Deserialize)]
pub struct TypedefEnum {
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Deserialize)]
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

#[derive(Clone, Deserialize)]
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
        let snake = self.name.to_snake_case();
        // Preserve leading underscores that heck strips
        let name = if self.name.starts_with('_') && !snake.starts_with('_') {
            format_ident!("_{}", snake)
        } else {
            format_ident!("{}", snake)
        };
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

    /// Check if this field type references a defined type that doesn't implement Default.
    /// Option<T> and Vec<T> always implement Default regardless of T, so they don't propagate.
    pub fn references_no_default_type(&self, no_default_types: &HashSet<String>) -> bool {
        match self {
            Self::defined(name) => no_default_types.contains(name),
            Self::array(a) => a.0.references_no_default_type(no_default_types),
            // Option and Vec always impl Default (None / empty), so don't propagate
            _ => false,
        }
    }
}

/// Compute the set of type names that cannot derive Default.
/// This includes types with big arrays (>32 elements) and types that
/// transitively contain such types through defined-type fields.
pub fn compute_no_default_types(types: &[NamedType]) -> HashSet<String> {
    let mut no_default: HashSet<String> = HashSet::new();

    // First pass: types with direct big array fields
    for t in types {
        if let TypedefType::r#struct(s) = &t.r#type {
            if s.fields.iter().any(|f| f.r#type.is_big_array()) {
                no_default.insert(t.name.clone());
            }
        }
    }

    // Enums derive Default only when their FIRST variant is a unit variant
    // (it receives `#[default]`). Any other shape — a non-unit first variant,
    // or an empty enum — cannot derive Default. This must match the derive gate
    // in `to_token_stream_inner` (`first_is_unit`), otherwise a field of such an
    // enum would be wrongly treated as defaultable in tolerant deserialization.
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
                if s.fields
                    .iter()
                    .any(|f| f.r#type.references_no_default_type(&no_default))
                {
                    no_default.insert(t.name.clone());
                    changed = true;
                }
            }
        }
    }

    no_default
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
