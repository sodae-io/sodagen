use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::idl_format::IdlCodegenModule;

mod account;
pub use account::*;

mod account_v1;
pub use account_v1::*;

pub struct AccountsCodegenModule<'a> {
    pub cli_args: &'a crate::Args,
    pub named_accounts: &'a [NamedAccount],
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
            use solana_pubkey::Pubkey;
            #[allow(unused_imports)] use crate::*;
        };
        for a in self.named_accounts {
            if self.cli_args.zero_copy.iter().any(|e| e == &a.0.name) {
                res.extend(quote! {
                    use bytemuck::{Pod, Zeroable};
                });
                break;
            }
        }
        res
    }

    fn gen_body(&self) -> TokenStream {
        self.named_accounts
            .iter()
            .map(|e| e.to_token_stream(self.cli_args, &self.no_default_types, &self.struct_types))
            .collect()
    }
}

pub struct AccountsV1CodegenModule<'a> {
    pub cli_args: &'a crate::Args,
    pub named_accounts: &'a [NamedAccountV1],
    pub no_default_types: HashSet<String>,
    pub struct_types: HashSet<String>,
}

impl IdlCodegenModule for AccountsV1CodegenModule<'_> {
    fn name(&self) -> &str {
        "accounts"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
            use solana_pubkey::Pubkey;
            #[allow(unused_imports)] use crate::*;
        };
        for a in self.named_accounts {
            if self
                .cli_args
                .zero_copy
                .iter()
                .any(|e| e == &a.named_type.name)
            {
                res.extend(quote! {
                    use bytemuck::{Pod, Zeroable};
                });
                break;
            }
        }
        res
    }

    fn gen_body(&self) -> TokenStream {
        self.named_accounts
            .iter()
            .map(|e| e.to_token_stream(self.cli_args, &self.no_default_types, &self.struct_types))
            .collect()
    }
}
