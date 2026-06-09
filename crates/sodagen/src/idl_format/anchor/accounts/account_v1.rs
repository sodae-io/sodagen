use std::collections::HashSet;

use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::idl_format::anchor::instructions::{
    gen_field_deserializations, gen_field_names, gen_field_serializations,
};
use crate::idl_format::anchor::typedefs::{NamedType, TypedefType};
use crate::utils::conditional_pascal_case;

pub struct NamedAccountV1 {
    pub named_type: NamedType,
    pub discriminator: [u8; 8],
}

impl NamedAccountV1 {
    pub fn to_token_stream(&self, cli_args: &crate::Args, no_default_types: &HashSet<String>, struct_types: &HashSet<String>) -> TokenStream {
        let name = &self.named_type.name;
        let account_discm_ident = format_ident!("{}_ACCOUNT_DISCM", name.to_shouty_snake_case());
        let discm = self.discriminator;
        let discm_tokens: TokenStream = format!("{:?}", discm).parse().unwrap();

        let struct_ident = format_ident!("{}", conditional_pascal_case(name));
        let account_ident = format_ident!("{}Account", conditional_pascal_case(name));

        // For structs, generate field-by-field methods; for enums, fall back to derive
        let (struct_def, inner_impl) = match &self.named_type.r#type {
            TypedefType::r#struct(s) => {
                let struct_def = self.named_type.to_token_stream_no_borsh(cli_args, no_default_types);
                let field_names = gen_field_names(&s.fields);
                let field_deserializations = gen_field_deserializations(&s.fields, no_default_types, struct_types);
                let field_serializations = gen_field_serializations(&s.fields, quote! { self });
                let inner_impl = quote! {
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
                };
                (struct_def, inner_impl)
            }
            _ => {
                // Enums keep borsh derive
                let struct_def = self.named_type.to_token_stream(cli_args);
                (struct_def, TokenStream::new())
            }
        };

        quote! {
            pub const #account_discm_ident: [u8; 8] = #discm_tokens;

            #struct_def
            #inner_impl

            #[derive(Clone, Debug, PartialEq)]
            pub struct #account_ident(pub #struct_ident);

            impl #account_ident {
                pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
                    use std::io::Read;
                    let mut reader = buf;
                    let mut maybe_discm = [0u8; 8];
                    reader.read_exact(&mut maybe_discm)?;
                    if maybe_discm != #account_discm_ident {
                        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
                    }
                    Ok(Self(#struct_ident::deserialize(&mut reader)?))
                }

                pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
                    writer.write_all(&#account_discm_ident)?;
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
