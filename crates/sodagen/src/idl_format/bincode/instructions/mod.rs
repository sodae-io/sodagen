use heck::{ToPascalCase, ToShoutySnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::idl_format::IdlCodegenModule;

mod instruction;
pub use instruction::*;

pub struct IxCodegenModule<'a> {
    pub program_name: &'a str,
    pub instructions: &'a [NamedInstruction],
}

impl<'a> IxCodegenModule<'a> {
    pub fn program_ix_enum_ident(&self) -> Ident {
        format_ident!("{}ProgramIx", self.program_name.to_pascal_case())
    }
}

impl IdlCodegenModule for IxCodegenModule<'_> {
    fn name(&self) -> &str {
        "instructions"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {};
        let has_accounts = self
            .instructions
            .iter()
            .map(|ix| ix.has_accounts())
            .any(|b| b);
        res.extend(quote! {
            use serde::{Serialize, Deserialize};
            use solana_pubkey::Pubkey;
            use solana_cpi::{invoke, invoke_signed};
        });
        if has_accounts {
            res.extend(quote! {
                use solana_instruction::{AccountMeta, Instruction};
                use solana_account_info::AccountInfo;
            });
        } else {
            res.extend(quote! {
                use solana_instruction::Instruction;
            });
        }
        let has_privileged_accounts = self
            .instructions
            .iter()
            .map(|ix| ix.has_privileged_accounts())
            .any(|b| b);
        if has_privileged_accounts {
            res.extend(quote! {
                use solana_program_error::ProgramError;
            });
        }
        res.extend(quote! {
            #[allow(unused_imports)] use crate::*;
        });

        // program ix enum
        let program_ix_enum_ident = self.program_ix_enum_ident();
        let program_ix_enum_variants = self.instructions.iter().map(enum_variant);

        res.extend(quote! {
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            pub enum #program_ix_enum_ident {
                #(#program_ix_enum_variants),*
            }
        });

        if has_accounts {
            res.extend(quote! {
                fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
                    ix: &Instruction,
                    accounts: A,
                ) -> ProgramResult {
                    let account_info: [AccountInfo<'info>; N] = accounts.into();
                    invoke(ix, &account_info)
                }
                fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
                    ix: &Instruction,
                    accounts: A,
                    seeds: &[&[&[u8]]],
                ) -> ProgramResult {
                    let account_info: [AccountInfo<'info>; N] = accounts.into();
                    invoke_signed(ix, &account_info, seeds)
                }
            });
        }

        res
    }

    fn gen_body(&self) -> TokenStream {
        let program_ix_enum_ident = self.program_ix_enum_ident();
        let program_id_ident = format_ident!(
            "{}_PROGRAM_ID",
            self.program_name.to_shouty_snake_case()
        );
        self.instructions
            .iter()
            .enumerate()
            .map(|(i, ix)| {
                NamedInstructionFull {
                    ix,
                    index: i,
                    program_ix_enum_ident: &program_ix_enum_ident,
                    program_id_ident: &program_id_ident,
                }
                .gen_tokens()
            })
            .collect()
    }
}

pub fn enum_variant(ix: &NamedInstruction) -> TokenStream {
    let variant_ident = ix.enum_variant_ident();
    let mut res = quote!(
        #variant_ident
    );
    if ix.has_ix_args() {
        let ix_args_ident = ix.ix_args_ident();
        res.extend(quote! {
            (#ix_args_ident)
        })
    }
    res
}
