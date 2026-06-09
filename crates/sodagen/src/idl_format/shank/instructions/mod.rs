use std::collections::HashSet;

use heck::{ToPascalCase, ToShoutySnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::idl_format::IdlCodegenModule;

mod instruction;
pub use instruction::*;

pub struct IxCodegenModule<'a> {
    pub program_name: &'a str,
    pub instructions: &'a [NamedInstruction],
    pub no_default_types: HashSet<String>,
    pub struct_types: HashSet<String>,
}

impl IdlCodegenModule for IxCodegenModule<'_> {
    fn name(&self) -> &str {
        "instructions"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {};
        let has_args = self
            .instructions
            .iter()
            .map(|ix| ix.has_ix_args())
            .any(|b| b);
        if has_args {
            res.extend(quote! {
                use borsh::{BorshDeserialize, BorshSerialize};
            });
        }
        let has_accounts = self
            .instructions
            .iter()
            .map(|ix| ix.has_accounts())
            .any(|b| b);
        res.extend(quote! {
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
            use std::io::Read;
            #[allow(unused_imports)] use crate::*;
        });

        // program ix enum
        let program_ix_enum_ident =
            format_ident!("{}ProgramIx", self.program_name.to_pascal_case());
        let program_ix_enum_variants = self.instructions.iter().map(enum_variant);
        let serialize_variant_match_arms =
            self.instructions.iter().map(serialize_variant_match_arm);
        let deserialize_variant_match_arms =
            self.instructions.iter().map(deserialize_variant_match_arm);

        res.extend(quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub enum #program_ix_enum_ident {
                #(#program_ix_enum_variants),*
            }

            impl #program_ix_enum_ident {
                pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
                    let mut reader = buf;
                    let mut maybe_discm_buf = [0u8; 1];
                    reader.read_exact(&mut maybe_discm_buf)?;
                    let maybe_discm = maybe_discm_buf[0];
                    match maybe_discm {
                        #(#deserialize_variant_match_arms),*,
                        _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
                    }
                }

                pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
                    match self {
                        #(#serialize_variant_match_arms),*,
                    }
                }

                pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
                    let mut data = Vec::new();
                    self.serialize(&mut data)?;
                    Ok(data)
                }
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
        let program_id_ident = format_ident!(
            "{}_PROGRAM_ID",
            self.program_name.to_shouty_snake_case()
        );
        self.instructions
            .iter()
            .map(|e| e.gen_tokens(&program_id_ident, &self.no_default_types, &self.struct_types))
            .collect()
    }
}

pub fn enum_variant(ix: &NamedInstruction) -> TokenStream {
    let variant_ident = format_ident!("{}", ix.name.to_pascal_case());
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

pub fn serialize_variant_match_arm(ix: &NamedInstruction) -> TokenStream {
    let variant_ident = format_ident!("{}", ix.name.to_pascal_case());
    let discm_ident = ix.discm_ident();
    let serialize_expr = if ix.has_ix_args() {
        quote! {{
            writer.write_all(&[#discm_ident])?;
            args.serialize(&mut writer)
        }}
    } else {
        quote! { writer.write_all(&[#discm_ident]) }
    };
    let mut left_matched = quote! { Self::#variant_ident };
    if ix.has_ix_args() {
        left_matched.extend(quote! { (args) });
    }
    quote! {
        #left_matched => #serialize_expr
    }
}

pub fn deserialize_variant_match_arm(ix: &NamedInstruction) -> TokenStream {
    let variant_ident = format_ident!("{}", ix.name.to_pascal_case());
    let discm_ident = ix.discm_ident();
    let mut variant_expr = quote! {
        Self::#variant_ident
    };
    if ix.has_ix_args() {
        let ix_args_ident = ix.ix_args_ident();
        variant_expr.extend(quote! {
            (#ix_args_ident::deserialize(&mut reader)?)
        })
    }
    quote! {
        #discm_ident => Ok(#variant_expr)
    }
}
