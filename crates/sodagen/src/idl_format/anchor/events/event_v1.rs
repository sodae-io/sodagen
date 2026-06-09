use std::collections::HashSet;

use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::EventType;
use crate::idl_format::anchor::instructions::{
    gen_field_deserializations, gen_field_names, gen_field_serializations,
};

pub struct EventV1 {
    pub event_type: EventType,
    pub discriminator: [u8; 8],
}

impl EventV1 {
    pub fn to_token_stream(&self) -> TokenStream {
        self.gen_with_no_default(&HashSet::new(), &HashSet::new())
    }

    pub fn gen_with_no_default(
        &self,
        no_default_types: &HashSet<String>,
        struct_types: &HashSet<String>,
    ) -> TokenStream {
        let event_discm_ident =
            format_ident!("{}_EVENT_DISCM", self.event_type.name.to_shouty_snake_case());
        let discm = self.discriminator;
        let discm_tokens: TokenStream = format!("{:?}", discm).parse().unwrap();

        let struct_ident = self.event_type.struct_ident();
        let event_ident = format_ident!("{}Event", struct_ident);

        let struct_fields = &self.event_type.fields;
        let field_names = gen_field_names(struct_fields);
        let field_deserializations = gen_field_deserializations(struct_fields, no_default_types, struct_types);
        let field_serializations = gen_field_serializations(struct_fields, quote! { self });

        quote! {
            pub const #event_discm_ident: [u8; 8] = #discm_tokens;

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
        }
    }
}
