use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::idl_format::IdlCodegenModule;

use super::typedefs::NamedType;

pub struct AccountsCodegenModule<'a> {
    pub cli_args: &'a crate::Args,
    pub named_types: &'a [NamedType],
    pub no_default_types: HashSet<String>,
    pub struct_types: HashSet<String>,
}

impl IdlCodegenModule for AccountsCodegenModule<'_> {
    fn name(&self) -> &str {
        "accounts"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
        };
        for a in self.named_types {
            if self.cli_args.zero_copy.iter().any(|e| e == &a.name) {
                res.extend(quote! {
                    use bytemuck::{Pod, Zeroable};
                });
                break;
            }
        }
        let mut has_pubkey = false;
        let mut has_defined = false;
        for a in self.named_types {
            if a.r#type.has_pubkey_field() && !has_pubkey {
                has_pubkey = true;
                res.extend(quote! {
                    use solana_pubkey::Pubkey;
                });
            }
            if a.r#type.has_defined_field() && !has_defined {
                has_defined = true;
                res.extend(quote! {
                    #[allow(unused_imports)] use crate::*;
                })
            }
            if has_defined && has_pubkey {
                break;
            }
        }
        res
    }

    fn gen_body(&self) -> TokenStream {
        self.named_types
            .iter()
            .map(|e| e.to_token_stream(self.cli_args, &self.no_default_types, &self.struct_types))
            .collect()
    }
}
