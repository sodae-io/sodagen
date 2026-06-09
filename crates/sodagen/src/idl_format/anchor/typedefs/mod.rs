use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::idl_format::anchor::instructions::{gen_field_deserializations, gen_field_names};
use crate::idl_format::IdlCodegenModule;
use crate::utils::conditional_pascal_case;

mod typedef;
pub use typedef::*;

pub struct TypedefsCodegenModule<'a> {
    pub cli_args: &'a crate::Args,
    pub named_types: &'a [NamedType],
    // Computed over ALL program types (typedefs + accounts) by the parent
    // module so nested-struct dispatch and default propagation are correct.
    pub no_default_types: HashSet<String>,
    pub struct_types: HashSet<String>,
}

impl IdlCodegenModule for TypedefsCodegenModule<'_> {
    fn name(&self) -> &str {
        "typedefs"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
            #[allow(unused_imports)] use crate::*;
        };
        for a in self.named_types {
            if self.cli_args.zero_copy.iter().any(|e| e == &a.name) {
                res.extend(quote! {
                    use bytemuck::{Pod, Zeroable};
                });
                break;
            }
        }
        for t in self.named_types {
            if t.r#type.has_pubkey_field() {
                res.extend(quote! {
                    use solana_pubkey::Pubkey;
                });
                break;
            }
        }
        res
    }

    fn gen_body(&self) -> TokenStream {
        self.named_types
            .iter()
            .map(|e| {
                let def = e.to_token_stream_with_no_default(
                    self.cli_args,
                    true,
                    &self.no_default_types,
                );
                // For struct typedefs, also emit an inherent slice-based
                // `deserialize` so nested struct fields can be read tolerantly
                // (trailing-field defaulting propagates through direct nesting).
                // Enums keep the derived borsh impl only.
                match &e.r#type {
                    TypedefType::r#struct(s) => {
                        let ident = format_ident!("{}", conditional_pascal_case(&e.name));
                        let field_names = gen_field_names(&s.fields);
                        let field_des = gen_field_deserializations(
                            &s.fields,
                            &self.no_default_types,
                            &self.struct_types,
                        );
                        quote! {
                            #def
                            impl #ident {
                                pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
                                    let mut reader: &[u8] = *__buf;
                                    #(#field_des)*
                                    *__buf = reader;
                                    Ok(Self { #(#field_names),* })
                                }
                            }
                        }
                    }
                    _ => def,
                }
            })
            .collect()
    }
}
