use std::collections::HashSet;

use heck::{ToPascalCase, ToShoutySnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::idl_format::anchor::instructions::{
    gen_field_deserializations, gen_field_names, gen_field_serializations,
};
use crate::idl_format::anchor::typedefs::TypedefField;

#[derive(Deserialize)]
pub struct Event(pub EventType);

#[derive(Deserialize)]
pub struct EventType {
    pub name: String,
    // NB: theres also an `index` field that's ignored for now since we dk what it does:
    // https://github.com/coral-xyz/anchor/blob/8f30f00ec363b7e82aa0b3c7041e912919b33cf5/lang/attribute/event/src/lib.rs#L62C1-L64
    pub fields: Vec<TypedefField>,
}

impl EventType {
    pub fn struct_ident(&self) -> Ident {
        format_ident!("{}", self.name.to_pascal_case())
    }
}

impl Event {
    /// Like `ToTokens`, but threads the no-default type set into the inner
    /// struct's field deserializations so missing trailing fields can default.
    pub fn gen_with_no_default(
        &self,
        no_default_types: &HashSet<String>,
        struct_types: &HashSet<String>,
    ) -> TokenStream {
        let mut tokens = TokenStream::new();
        // discriminant
        let event_discm_ident = format_ident!("{}_EVENT_DISCM", self.0.name.to_shouty_snake_case());
        // pre-image: "event:{EventName}"
        let discm = <[u8; 8]>::try_from(
            &Sha256::digest(format!("event:{}", self.0.name).as_bytes()).as_slice()[..8],
        )
        .unwrap();
        let discm_tokens: TokenStream = format!("{:?}", discm).parse().unwrap();

        let struct_def = self.0.gen_with_no_default(no_default_types, struct_types);

        let struct_ident = self.0.struct_ident();
        let event_ident = format_ident!("{}Event", struct_ident);
        tokens.extend(quote! {
            pub const #event_discm_ident: [u8; 8] = #discm_tokens;

            #struct_def

            #[derive(Clone, Debug, PartialEq)]
            pub struct #event_ident(pub #struct_ident);

            impl #event_ident {
                pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
                    use std::io::Read;
                    let mut reader: &[u8] = *__buf;
                    let mut maybe_discm = [0u8; 8];
                    reader.read_exact(&mut maybe_discm)?;
                    if maybe_discm != #event_discm_ident {
                        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
                    }
                    let inner = #struct_ident::deserialize(&mut reader)?;
                    *__buf = reader;
                    Ok(Self(inner))
                }

                pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
                    writer.write_all(&#event_discm_ident)?;
                    self.0.serialize(&mut writer)
                }

                pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
                    let mut data = Vec::new();
                    self.serialize(&mut data)?;
                    Ok(data)
                }
            }
        });
        tokens
    }
}

impl ToTokens for Event {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // Default to the empty sets; real codegen routes through
        // `gen_with_no_default` with the program's actual sets.
        tokens.extend(self.gen_with_no_default(&HashSet::new(), &HashSet::new()));
    }
}

impl EventType {
    /// Like `ToTokens`, but threads the no-default + struct type sets into the
    /// field deserializations so missing trailing fields can default and nested
    /// struct fields read via their tolerant slice `deserialize`.
    pub fn gen_with_no_default(
        &self,
        no_default_types: &HashSet<String>,
        struct_types: &HashSet<String>,
    ) -> TokenStream {
        let struct_ident = self.struct_ident();
        let struct_fields = &self.fields;
        let field_names = gen_field_names(&self.fields);
        let field_deserializations = gen_field_deserializations(&self.fields, no_default_types, struct_types);
        let field_serializations = gen_field_serializations(&self.fields, quote! { self });
        quote! {
            #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct #struct_ident {
                #(pub #struct_fields),*
            }

            impl #struct_ident {
                pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
                    let mut reader: &[u8] = *__buf;
                    #(#field_deserializations)*
                    *__buf = reader;
                    Ok(Self { #(#field_names),* })
                }

                pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
                    #(#field_serializations)*
                    Ok(())
                }
            }
        }
    }
}

impl ToTokens for EventType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.gen_with_no_default(&HashSet::new(), &HashSet::new()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::idl_format::anchor::typedefs::TypedefFieldType;

    #[test]
    fn test_event_type_to_tokens_with_pub_fields() {
        // Define some fields for the EventType struct.
        let field1 = TypedefField {
            name: "field1".to_string(),
            r#type: TypedefFieldType::PrimitiveOrPubkey("u32".into()),
        };

        let field2 = TypedefField {
            name: "field2".to_string(),
            r#type: TypedefFieldType::PrimitiveOrPubkey("String".into()),
        };

        // Create an EventType with the fields.
        let event_type = EventType {
            name: "TestEvent".to_string(),
            fields: vec![field1, field2],
        };

        // Generate the tokens.
        let mut tokens = proc_macro2::TokenStream::new();
        event_type.to_tokens(&mut tokens);

        // Convert the tokens to a string for comparison.
        let generated_code = tokens.to_string();

        // Check that the generated code includes "pub" for each field.
        assert!(generated_code.contains("pub field1 : u32"));
        assert!(generated_code.contains("pub field2 : String"));

        // Check that the struct name is correct.
        assert!(generated_code.contains("pub struct TestEvent"));
    }
}
