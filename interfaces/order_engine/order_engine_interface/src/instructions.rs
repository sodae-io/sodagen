use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum OrderEngineProgramIx {
    Fill(FillIxArgs),
}
impl OrderEngineProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&FILL_IX_DISCM) {
            let mut reader = &buf[FILL_IX_DISCM.len()..];
            let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expire_at: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Fill(FillIxArgs {
                    input_amount,
                    output_amount,
                    expire_at,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Fill(args) => {
                writer.write_all(&FILL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.input_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.output_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.expire_at, &mut writer)?;
                Ok(())
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
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
pub const FILL_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct FillAccounts<'me, 'info> {
    pub taker: &'me AccountInfo<'info>,
    pub maker: &'me AccountInfo<'info>,
    pub taker_input_mint_token_account: &'me AccountInfo<'info>,
    pub maker_input_mint_token_account: &'me AccountInfo<'info>,
    pub taker_output_mint_token_account: &'me AccountInfo<'info>,
    pub maker_output_mint_token_account: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub output_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillKeys {
    pub taker: Pubkey,
    pub maker: Pubkey,
    pub taker_input_mint_token_account: Pubkey,
    pub maker_input_mint_token_account: Pubkey,
    pub taker_output_mint_token_account: Pubkey,
    pub maker_output_mint_token_account: Pubkey,
    pub input_mint: Pubkey,
    pub input_token_program: Pubkey,
    pub output_mint: Pubkey,
    pub output_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<FillAccounts<'_, '_>> for FillKeys {
    fn from(accounts: FillAccounts) -> Self {
        Self {
            taker: *accounts.taker.key,
            maker: *accounts.maker.key,
            taker_input_mint_token_account: *accounts.taker_input_mint_token_account.key,
            maker_input_mint_token_account: *accounts.maker_input_mint_token_account.key,
            taker_output_mint_token_account: *accounts
                .taker_output_mint_token_account
                .key,
            maker_output_mint_token_account: *accounts
                .maker_output_mint_token_account
                .key,
            input_mint: *accounts.input_mint.key,
            input_token_program: *accounts.input_token_program.key,
            output_mint: *accounts.output_mint.key,
            output_token_program: *accounts.output_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<FillKeys> for [AccountMeta; FILL_IX_ACCOUNTS_LEN] {
    fn from(keys: FillKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.taker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_input_mint_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_input_mint_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.taker_output_mint_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_output_mint_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FILL_IX_ACCOUNTS_LEN]> for FillKeys {
    fn from(pubkeys: [Pubkey; FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            taker: pubkeys[0],
            maker: pubkeys[1],
            taker_input_mint_token_account: pubkeys[2],
            maker_input_mint_token_account: pubkeys[3],
            taker_output_mint_token_account: pubkeys[4],
            maker_output_mint_token_account: pubkeys[5],
            input_mint: pubkeys[6],
            input_token_program: pubkeys[7],
            output_mint: pubkeys[8],
            output_token_program: pubkeys[9],
            system_program: pubkeys[10],
        }
    }
}
impl<'info> From<FillAccounts<'_, 'info>>
for [AccountInfo<'info>; FILL_IX_ACCOUNTS_LEN] {
    fn from(accounts: FillAccounts<'_, 'info>) -> Self {
        [
            accounts.taker.clone(),
            accounts.maker.clone(),
            accounts.taker_input_mint_token_account.clone(),
            accounts.maker_input_mint_token_account.clone(),
            accounts.taker_output_mint_token_account.clone(),
            accounts.maker_output_mint_token_account.clone(),
            accounts.input_mint.clone(),
            accounts.input_token_program.clone(),
            accounts.output_mint.clone(),
            accounts.output_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FILL_IX_ACCOUNTS_LEN]>
for FillAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            taker: &arr[0],
            maker: &arr[1],
            taker_input_mint_token_account: &arr[2],
            maker_input_mint_token_account: &arr[3],
            taker_output_mint_token_account: &arr[4],
            maker_output_mint_token_account: &arr[5],
            input_mint: &arr[6],
            input_token_program: &arr[7],
            output_mint: &arr[8],
            output_token_program: &arr[9],
            system_program: &arr[10],
        }
    }
}
pub const FILL_IX_DISCM: [u8; 8usize] = [168, 96, 183, 163, 92, 10, 40, 160];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FillIxArgs {
    pub input_amount: u64,
    pub output_amount: u64,
    pub expire_at: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillIxData(pub FillIxArgs);
impl From<FillIxArgs> for FillIxData {
    fn from(args: FillIxArgs) -> Self {
        Self(args)
    }
}
impl FillIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expire_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FillIxArgs {
                input_amount,
                output_amount,
                expire_at,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.expire_at, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fill_ix_with_program_id(
    program_id: Pubkey,
    keys: FillKeys,
    args: FillIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FILL_IX_ACCOUNTS_LEN] = keys.into();
    let data: FillIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_ix(keys: FillKeys, args: FillIxArgs) -> std::io::Result<Instruction> {
    fill_ix_with_program_id(ORDER_ENGINE_PROGRAM_ID, keys, args)
}
pub fn fill_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FillAccounts<'_, '_>,
    args: FillIxArgs,
) -> ProgramResult {
    let keys: FillKeys = accounts.into();
    let ix = fill_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fill_invoke(accounts: FillAccounts<'_, '_>, args: FillIxArgs) -> ProgramResult {
    fill_invoke_with_program_id(ORDER_ENGINE_PROGRAM_ID, accounts, args)
}
pub fn fill_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FillAccounts<'_, '_>,
    args: FillIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FillKeys = accounts.into();
    let ix = fill_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fill_invoke_signed(
    accounts: FillAccounts<'_, '_>,
    args: FillIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fill_invoke_signed_with_program_id(ORDER_ENGINE_PROGRAM_ID, accounts, args, seeds)
}
pub fn fill_verify_account_keys(
    accounts: FillAccounts<'_, '_>,
    keys: FillKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.taker.key, keys.taker),
        (*accounts.maker.key, keys.maker),
        (
            *accounts.taker_input_mint_token_account.key,
            keys.taker_input_mint_token_account,
        ),
        (
            *accounts.maker_input_mint_token_account.key,
            keys.maker_input_mint_token_account,
        ),
        (
            *accounts.taker_output_mint_token_account.key,
            keys.taker_output_mint_token_account,
        ),
        (
            *accounts.maker_output_mint_token_account.key,
            keys.maker_output_mint_token_account,
        ),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.input_token_program.key, keys.input_token_program),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.output_token_program.key, keys.output_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fill_verify_writable_privileges<'me, 'info>(
    accounts: FillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.taker,
        accounts.maker,
        accounts.taker_input_mint_token_account,
        accounts.maker_input_mint_token_account,
        accounts.taker_output_mint_token_account,
        accounts.maker_output_mint_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fill_verify_signer_privileges<'me, 'info>(
    accounts: FillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.taker, accounts.maker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fill_verify_account_privileges<'me, 'info>(
    accounts: FillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fill_verify_writable_privileges(accounts)?;
    fill_verify_signer_privileges(accounts)?;
    Ok(())
}
