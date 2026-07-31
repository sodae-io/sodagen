use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum JupiterRfqV2ProgramIx {
    FillExactIn(FillExactInIxArgs),
}
impl JupiterRfqV2ProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&FILL_EXACT_IN_IX_DISCM) {
            let mut reader = &buf[FILL_EXACT_IN_IX_DISCM.len()..];
            let taker_side: Side = crate::borsh_de_or_default(&mut reader)?;
            let amount_in_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <FillExactInParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::FillExactIn(FillExactInIxArgs {
                    taker_side,
                    amount_in_atoms,
                    params,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::FillExactIn(args) => {
                writer.write_all(&FILL_EXACT_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.taker_side, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_in_atoms, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
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
pub const FILL_EXACT_IN_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct FillExactInAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub fill_authority: &'me AccountInfo<'info>,
    pub user_base_token_account: &'me AccountInfo<'info>,
    pub user_quote_token_account: &'me AccountInfo<'info>,
    pub maker_base_token_account: &'me AccountInfo<'info>,
    pub maker_quote_token_account: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillExactInKeys {
    pub user: Pubkey,
    pub fill_authority: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub maker_base_token_account: Pubkey,
    pub maker_quote_token_account: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub instructions_sysvar: Pubkey,
}
impl From<FillExactInAccounts<'_, '_>> for FillExactInKeys {
    fn from(accounts: FillExactInAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            fill_authority: *accounts.fill_authority.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            maker_base_token_account: *accounts.maker_base_token_account.key,
            maker_quote_token_account: *accounts.maker_quote_token_account.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
        }
    }
}
impl From<FillExactInKeys> for [AccountMeta; FILL_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: FillExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fill_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.maker_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FILL_EXACT_IN_IX_ACCOUNTS_LEN]> for FillExactInKeys {
    fn from(pubkeys: [Pubkey; FILL_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            fill_authority: pubkeys[1],
            user_base_token_account: pubkeys[2],
            user_quote_token_account: pubkeys[3],
            maker_base_token_account: pubkeys[4],
            maker_quote_token_account: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            base_token_program: pubkeys[8],
            quote_token_program: pubkeys[9],
            instructions_sysvar: pubkeys[10],
        }
    }
}
impl<'info> From<FillExactInAccounts<'_, 'info>>
for [AccountInfo<'info>; FILL_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: FillExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.fill_authority.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.maker_base_token_account.clone(),
            accounts.maker_quote_token_account.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.instructions_sysvar.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FILL_EXACT_IN_IX_ACCOUNTS_LEN]>
for FillExactInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FILL_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            fill_authority: &arr[1],
            user_base_token_account: &arr[2],
            user_quote_token_account: &arr[3],
            maker_base_token_account: &arr[4],
            maker_quote_token_account: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            base_token_program: &arr[8],
            quote_token_program: &arr[9],
            instructions_sysvar: &arr[10],
        }
    }
}
pub const FILL_EXACT_IN_IX_DISCM: [u8; 8usize] = [222, 208, 6, 209, 154, 163, 54, 94];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FillExactInIxArgs {
    pub taker_side: Side,
    pub amount_in_atoms: u64,
    pub params: FillExactInParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillExactInIxData(pub FillExactInIxArgs);
impl From<FillExactInIxArgs> for FillExactInIxData {
    fn from(args: FillExactInIxArgs) -> Self {
        Self(args)
    }
}
impl FillExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let taker_side: Side = crate::borsh_de_or_default(&mut reader)?;
        let amount_in_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <FillExactInParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(FillExactInIxArgs {
                taker_side,
                amount_in_atoms,
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_EXACT_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.taker_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in_atoms, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fill_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: FillExactInKeys,
    args: FillExactInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FILL_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: FillExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_exact_in_ix(
    keys: FillExactInKeys,
    args: FillExactInIxArgs,
) -> std::io::Result<Instruction> {
    fill_exact_in_ix_with_program_id(JUPITER_RFQ_V2_PROGRAM_ID, keys, args)
}
pub fn fill_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FillExactInAccounts<'_, '_>,
    args: FillExactInIxArgs,
) -> ProgramResult {
    let keys: FillExactInKeys = accounts.into();
    let ix = fill_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fill_exact_in_invoke(
    accounts: FillExactInAccounts<'_, '_>,
    args: FillExactInIxArgs,
) -> ProgramResult {
    fill_exact_in_invoke_with_program_id(JUPITER_RFQ_V2_PROGRAM_ID, accounts, args)
}
pub fn fill_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FillExactInAccounts<'_, '_>,
    args: FillExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FillExactInKeys = accounts.into();
    let ix = fill_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fill_exact_in_invoke_signed(
    accounts: FillExactInAccounts<'_, '_>,
    args: FillExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fill_exact_in_invoke_signed_with_program_id(
        JUPITER_RFQ_V2_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fill_exact_in_verify_account_keys(
    accounts: FillExactInAccounts<'_, '_>,
    keys: FillExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.fill_authority.key, keys.fill_authority),
        (*accounts.user_base_token_account.key, keys.user_base_token_account),
        (*accounts.user_quote_token_account.key, keys.user_quote_token_account),
        (*accounts.maker_base_token_account.key, keys.maker_base_token_account),
        (*accounts.maker_quote_token_account.key, keys.maker_quote_token_account),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fill_exact_in_verify_writable_privileges<'me, 'info>(
    accounts: FillExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
        accounts.maker_base_token_account,
        accounts.maker_quote_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fill_exact_in_verify_signer_privileges<'me, 'info>(
    accounts: FillExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user, accounts.fill_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fill_exact_in_verify_account_privileges<'me, 'info>(
    accounts: FillExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fill_exact_in_verify_writable_privileges(accounts)?;
    fill_exact_in_verify_signer_privileges(accounts)?;
    Ok(())
}
