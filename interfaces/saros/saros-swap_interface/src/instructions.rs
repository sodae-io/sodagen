use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum SarosSwapProgramIx {
    Initialize(InitializeIxArgs),
    Swap(SwapIxArgs),
    DepositAllTokenTypes(DepositAllTokenTypesIxArgs),
    WithdrawAllTokenTypes(WithdrawAllTokenTypesIxArgs),
    SwapExactOut(SwapExactOutIxArgs),
}
impl SarosSwapProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let fees = if reader.is_empty() {
                Default::default()
            } else {
                <Fees>::deserialize(&mut reader)?
            };
            let swap_curve: SwapCurve = crate::borsh_de_or_default(&mut reader)?;
            let swap_calculator: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Initialize(InitializeIxArgs {
                    fees,
                    swap_curve,
                    swap_calculator,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount_in,
                    minimum_amount_out,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_ALL_TOKEN_TYPES_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_ALL_TOKEN_TYPES_IX_DISCM.len()..];
            let pool_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositAllTokenTypes(DepositAllTokenTypesIxArgs {
                    pool_token_amount,
                    maximum_token_a_amount,
                    maximum_token_b_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_ALL_TOKEN_TYPES_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_ALL_TOKEN_TYPES_IX_DISCM.len()..];
            let pool_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawAllTokenTypes(WithdrawAllTokenTypesIxArgs {
                    pool_token_amount,
                    minimum_token_a_amount,
                    minimum_token_b_amount,
                }),
            );
        }
        if buf.starts_with(&SWAP_EXACT_OUT_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_OUT_IX_DISCM.len()..];
            let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapExactOut(SwapExactOutIxArgs {
                    amount_out,
                    maximum_amount_in,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fees, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.swap_curve, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.swap_calculator, &mut writer)?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                Ok(())
            }
            Self::DepositAllTokenTypes(args) => {
                writer.write_all(&DEPOSIT_ALL_TOKEN_TYPES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pool_token_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.maximum_token_a_amount,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.maximum_token_b_amount,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::WithdrawAllTokenTypes(args) => {
                writer.write_all(&WITHDRAW_ALL_TOKEN_TYPES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pool_token_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_token_a_amount,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_token_b_amount,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SwapExactOut(args) => {
                writer.write_all(&SWAP_EXACT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.maximum_amount_in, &mut writer)?;
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
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub authority_info: &'me AccountInfo<'info>,
    pub token_a_info: &'me AccountInfo<'info>,
    pub token_b_info: &'me AccountInfo<'info>,
    pub pool_mint_info: &'me AccountInfo<'info>,
    pub fee_account_info: &'me AccountInfo<'info>,
    pub destination_info: &'me AccountInfo<'info>,
    pub token_program_info: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub swap_info: Pubkey,
    pub authority_info: Pubkey,
    pub token_a_info: Pubkey,
    pub token_b_info: Pubkey,
    pub pool_mint_info: Pubkey,
    pub fee_account_info: Pubkey,
    pub destination_info: Pubkey,
    pub token_program_info: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            authority_info: *accounts.authority_info.key,
            token_a_info: *accounts.token_a_info.key,
            token_b_info: *accounts.token_b_info.key,
            pool_mint_info: *accounts.pool_mint_info.key,
            fee_account_info: *accounts.fee_account_info.key,
            destination_info: *accounts.destination_info.key,
            token_program_info: *accounts.token_program_info.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_mint_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_account_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_info,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            authority_info: pubkeys[1],
            token_a_info: pubkeys[2],
            token_b_info: pubkeys[3],
            pool_mint_info: pubkeys[4],
            fee_account_info: pubkeys[5],
            destination_info: pubkeys[6],
            token_program_info: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.authority_info.clone(),
            accounts.token_a_info.clone(),
            accounts.token_b_info.clone(),
            accounts.pool_mint_info.clone(),
            accounts.fee_account_info.clone(),
            accounts.destination_info.clone(),
            accounts.token_program_info.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: &arr[0],
            authority_info: &arr[1],
            token_a_info: &arr[2],
            token_b_info: &arr[3],
            pool_mint_info: &arr[4],
            fee_account_info: &arr[5],
            destination_info: &arr[6],
            token_program_info: &arr[7],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 1usize] = [0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub fees: Fees,
    pub swap_curve: SwapCurve,
    pub swap_calculator: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 1usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <Fees>::deserialize(&mut reader)?
        };
        let swap_curve: SwapCurve = crate::borsh_de_or_default(&mut reader)?;
        let swap_calculator: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeIxArgs {
                fees,
                swap_curve,
                swap_calculator,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_calculator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_ix(
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(SAROS_SWAP_PROGRAM_ID, keys, args)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(SAROS_SWAP_PROGRAM_ID, accounts, args)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(
        SAROS_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.authority_info.key, keys.authority_info),
        (*accounts.token_a_info.key, keys.token_a_info),
        (*accounts.token_b_info.key, keys.token_b_info),
        (*accounts.pool_mint_info.key, keys.pool_mint_info),
        (*accounts.fee_account_info.key, keys.fee_account_info),
        (*accounts.destination_info.key, keys.destination_info),
        (*accounts.token_program_info.key, keys.token_program_info),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.pool_mint_info,
        accounts.fee_account_info,
        accounts.destination_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.swap_info] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub authority_info: &'me AccountInfo<'info>,
    pub user_transfer_authority_info: &'me AccountInfo<'info>,
    pub source_info: &'me AccountInfo<'info>,
    pub swap_source_info: &'me AccountInfo<'info>,
    pub swap_destination_info: &'me AccountInfo<'info>,
    pub destination_info: &'me AccountInfo<'info>,
    pub pool_mint_info: &'me AccountInfo<'info>,
    pub pool_fee_account_info: &'me AccountInfo<'info>,
    pub token_program_info: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub swap_info: Pubkey,
    pub authority_info: Pubkey,
    pub user_transfer_authority_info: Pubkey,
    pub source_info: Pubkey,
    pub swap_source_info: Pubkey,
    pub swap_destination_info: Pubkey,
    pub destination_info: Pubkey,
    pub pool_mint_info: Pubkey,
    pub pool_fee_account_info: Pubkey,
    pub token_program_info: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            authority_info: *accounts.authority_info.key,
            user_transfer_authority_info: *accounts.user_transfer_authority_info.key,
            source_info: *accounts.source_info.key,
            swap_source_info: *accounts.swap_source_info.key,
            swap_destination_info: *accounts.swap_destination_info.key,
            destination_info: *accounts.destination_info.key,
            pool_mint_info: *accounts.pool_mint_info.key,
            pool_fee_account_info: *accounts.pool_fee_account_info.key,
            token_program_info: *accounts.token_program_info.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority_info,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_fee_account_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_info,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            authority_info: pubkeys[1],
            user_transfer_authority_info: pubkeys[2],
            source_info: pubkeys[3],
            swap_source_info: pubkeys[4],
            swap_destination_info: pubkeys[5],
            destination_info: pubkeys[6],
            pool_mint_info: pubkeys[7],
            pool_fee_account_info: pubkeys[8],
            token_program_info: pubkeys[9],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.authority_info.clone(),
            accounts.user_transfer_authority_info.clone(),
            accounts.source_info.clone(),
            accounts.swap_source_info.clone(),
            accounts.swap_destination_info.clone(),
            accounts.destination_info.clone(),
            accounts.pool_mint_info.clone(),
            accounts.pool_fee_account_info.clone(),
            accounts.token_program_info.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: &arr[0],
            authority_info: &arr[1],
            user_transfer_authority_info: &arr[2],
            source_info: &arr[3],
            swap_source_info: &arr[4],
            swap_destination_info: &arr[5],
            destination_info: &arr[6],
            pool_mint_info: &arr[7],
            pool_fee_account_info: &arr[8],
            token_program_info: &arr[9],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 1usize] = [1];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapIxData(pub SwapIxArgs);
impl From<SwapIxArgs> for SwapIxData {
    fn from(args: SwapIxArgs) -> Self {
        Self(args)
    }
}
impl SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 1usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                amount_in,
                minimum_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapKeys,
    args: SwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_ix(keys: SwapKeys, args: SwapIxArgs) -> std::io::Result<Instruction> {
    swap_ix_with_program_id(SAROS_SWAP_PROGRAM_ID, keys, args)
}
pub fn swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_invoke(accounts: SwapAccounts<'_, '_>, args: SwapIxArgs) -> ProgramResult {
    swap_invoke_with_program_id(SAROS_SWAP_PROGRAM_ID, accounts, args)
}
pub fn swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_invoke_signed(
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_invoke_signed_with_program_id(SAROS_SWAP_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.authority_info.key, keys.authority_info),
        (*accounts.user_transfer_authority_info.key, keys.user_transfer_authority_info),
        (*accounts.source_info.key, keys.source_info),
        (*accounts.swap_source_info.key, keys.swap_source_info),
        (*accounts.swap_destination_info.key, keys.swap_destination_info),
        (*accounts.destination_info.key, keys.destination_info),
        (*accounts.pool_mint_info.key, keys.pool_mint_info),
        (*accounts.pool_fee_account_info.key, keys.pool_fee_account_info),
        (*accounts.token_program_info.key, keys.token_program_info),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.authority_info,
        accounts.user_transfer_authority_info,
        accounts.source_info,
        accounts.swap_source_info,
        accounts.swap_destination_info,
        accounts.destination_info,
        accounts.pool_mint_info,
        accounts.pool_fee_account_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_verify_signer_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority_info] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_verify_writable_privileges(accounts)?;
    swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositAllTokenTypesAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub authority_info: &'me AccountInfo<'info>,
    pub user_transfer_authority_info: &'me AccountInfo<'info>,
    pub source_a_info: &'me AccountInfo<'info>,
    pub source_b_info: &'me AccountInfo<'info>,
    pub token_a_info: &'me AccountInfo<'info>,
    pub token_b_info: &'me AccountInfo<'info>,
    pub pool_mint_info: &'me AccountInfo<'info>,
    pub dest_info: &'me AccountInfo<'info>,
    pub token_program_info: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositAllTokenTypesKeys {
    pub swap_info: Pubkey,
    pub authority_info: Pubkey,
    pub user_transfer_authority_info: Pubkey,
    pub source_a_info: Pubkey,
    pub source_b_info: Pubkey,
    pub token_a_info: Pubkey,
    pub token_b_info: Pubkey,
    pub pool_mint_info: Pubkey,
    pub dest_info: Pubkey,
    pub token_program_info: Pubkey,
}
impl From<DepositAllTokenTypesAccounts<'_, '_>> for DepositAllTokenTypesKeys {
    fn from(accounts: DepositAllTokenTypesAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            authority_info: *accounts.authority_info.key,
            user_transfer_authority_info: *accounts.user_transfer_authority_info.key,
            source_a_info: *accounts.source_a_info.key,
            source_b_info: *accounts.source_b_info.key,
            token_a_info: *accounts.token_a_info.key,
            token_b_info: *accounts.token_b_info.key,
            pool_mint_info: *accounts.pool_mint_info.key,
            dest_info: *accounts.dest_info.key,
            token_program_info: *accounts.token_program_info.key,
        }
    }
}
impl From<DepositAllTokenTypesKeys>
for [AccountMeta; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositAllTokenTypesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority_info,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_a_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_b_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_info,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN]>
for DepositAllTokenTypesKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            authority_info: pubkeys[1],
            user_transfer_authority_info: pubkeys[2],
            source_a_info: pubkeys[3],
            source_b_info: pubkeys[4],
            token_a_info: pubkeys[5],
            token_b_info: pubkeys[6],
            pool_mint_info: pubkeys[7],
            dest_info: pubkeys[8],
            token_program_info: pubkeys[9],
        }
    }
}
impl<'info> From<DepositAllTokenTypesAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAllTokenTypesAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.authority_info.clone(),
            accounts.user_transfer_authority_info.clone(),
            accounts.source_a_info.clone(),
            accounts.source_b_info.clone(),
            accounts.token_a_info.clone(),
            accounts.token_b_info.clone(),
            accounts.pool_mint_info.clone(),
            accounts.dest_info.clone(),
            accounts.token_program_info.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN]>
for DepositAllTokenTypesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap_info: &arr[0],
            authority_info: &arr[1],
            user_transfer_authority_info: &arr[2],
            source_a_info: &arr[3],
            source_b_info: &arr[4],
            token_a_info: &arr[5],
            token_b_info: &arr[6],
            pool_mint_info: &arr[7],
            dest_info: &arr[8],
            token_program_info: &arr[9],
        }
    }
}
pub const DEPOSIT_ALL_TOKEN_TYPES_IX_DISCM: [u8; 1usize] = [2];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositAllTokenTypesIxArgs {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositAllTokenTypesIxData(pub DepositAllTokenTypesIxArgs);
impl From<DepositAllTokenTypesIxArgs> for DepositAllTokenTypesIxData {
    fn from(args: DepositAllTokenTypesIxArgs) -> Self {
        Self(args)
    }
}
impl DepositAllTokenTypesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 1usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_ALL_TOKEN_TYPES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pool_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositAllTokenTypesIxArgs {
                pool_token_amount,
                maximum_token_a_amount,
                maximum_token_b_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_ALL_TOKEN_TYPES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pool_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_token_b_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_all_token_types_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositAllTokenTypesKeys,
    args: DepositAllTokenTypesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositAllTokenTypesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_all_token_types_ix(
    keys: DepositAllTokenTypesKeys,
    args: DepositAllTokenTypesIxArgs,
) -> std::io::Result<Instruction> {
    deposit_all_token_types_ix_with_program_id(SAROS_SWAP_PROGRAM_ID, keys, args)
}
pub fn deposit_all_token_types_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositAllTokenTypesAccounts<'_, '_>,
    args: DepositAllTokenTypesIxArgs,
) -> ProgramResult {
    let keys: DepositAllTokenTypesKeys = accounts.into();
    let ix = deposit_all_token_types_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_all_token_types_invoke(
    accounts: DepositAllTokenTypesAccounts<'_, '_>,
    args: DepositAllTokenTypesIxArgs,
) -> ProgramResult {
    deposit_all_token_types_invoke_with_program_id(SAROS_SWAP_PROGRAM_ID, accounts, args)
}
pub fn deposit_all_token_types_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositAllTokenTypesAccounts<'_, '_>,
    args: DepositAllTokenTypesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositAllTokenTypesKeys = accounts.into();
    let ix = deposit_all_token_types_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_all_token_types_invoke_signed(
    accounts: DepositAllTokenTypesAccounts<'_, '_>,
    args: DepositAllTokenTypesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_all_token_types_invoke_signed_with_program_id(
        SAROS_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_all_token_types_verify_account_keys(
    accounts: DepositAllTokenTypesAccounts<'_, '_>,
    keys: DepositAllTokenTypesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.authority_info.key, keys.authority_info),
        (*accounts.user_transfer_authority_info.key, keys.user_transfer_authority_info),
        (*accounts.source_a_info.key, keys.source_a_info),
        (*accounts.source_b_info.key, keys.source_b_info),
        (*accounts.token_a_info.key, keys.token_a_info),
        (*accounts.token_b_info.key, keys.token_b_info),
        (*accounts.pool_mint_info.key, keys.pool_mint_info),
        (*accounts.dest_info.key, keys.dest_info),
        (*accounts.token_program_info.key, keys.token_program_info),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_all_token_types_verify_writable_privileges<'me, 'info>(
    accounts: DepositAllTokenTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_transfer_authority_info,
        accounts.source_a_info,
        accounts.source_b_info,
        accounts.token_a_info,
        accounts.token_b_info,
        accounts.pool_mint_info,
        accounts.dest_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_all_token_types_verify_signer_privileges<'me, 'info>(
    accounts: DepositAllTokenTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority_info] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_all_token_types_verify_account_privileges<'me, 'info>(
    accounts: DepositAllTokenTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_all_token_types_verify_writable_privileges(accounts)?;
    deposit_all_token_types_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAllTokenTypesAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub authority_info: &'me AccountInfo<'info>,
    pub user_transfer_authority_info: &'me AccountInfo<'info>,
    pub pool_mint_info: &'me AccountInfo<'info>,
    pub source_info: &'me AccountInfo<'info>,
    pub token_a_info: &'me AccountInfo<'info>,
    pub token_b_info: &'me AccountInfo<'info>,
    pub dest_token_a_info: &'me AccountInfo<'info>,
    pub dest_token_b_info: &'me AccountInfo<'info>,
    pub pool_fee_account_info: &'me AccountInfo<'info>,
    pub token_program_info: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawAllTokenTypesKeys {
    pub swap_info: Pubkey,
    pub authority_info: Pubkey,
    pub user_transfer_authority_info: Pubkey,
    pub pool_mint_info: Pubkey,
    pub source_info: Pubkey,
    pub token_a_info: Pubkey,
    pub token_b_info: Pubkey,
    pub dest_token_a_info: Pubkey,
    pub dest_token_b_info: Pubkey,
    pub pool_fee_account_info: Pubkey,
    pub token_program_info: Pubkey,
}
impl From<WithdrawAllTokenTypesAccounts<'_, '_>> for WithdrawAllTokenTypesKeys {
    fn from(accounts: WithdrawAllTokenTypesAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            authority_info: *accounts.authority_info.key,
            user_transfer_authority_info: *accounts.user_transfer_authority_info.key,
            pool_mint_info: *accounts.pool_mint_info.key,
            source_info: *accounts.source_info.key,
            token_a_info: *accounts.token_a_info.key,
            token_b_info: *accounts.token_b_info.key,
            dest_token_a_info: *accounts.dest_token_a_info.key,
            dest_token_b_info: *accounts.dest_token_b_info.key,
            pool_fee_account_info: *accounts.pool_fee_account_info.key,
            token_program_info: *accounts.token_program_info.key,
        }
    }
}
impl From<WithdrawAllTokenTypesKeys>
for [AccountMeta; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawAllTokenTypesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority_info,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_token_a_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_token_b_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_fee_account_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_info,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN]>
for WithdrawAllTokenTypesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            authority_info: pubkeys[1],
            user_transfer_authority_info: pubkeys[2],
            pool_mint_info: pubkeys[3],
            source_info: pubkeys[4],
            token_a_info: pubkeys[5],
            token_b_info: pubkeys[6],
            dest_token_a_info: pubkeys[7],
            dest_token_b_info: pubkeys[8],
            pool_fee_account_info: pubkeys[9],
            token_program_info: pubkeys[10],
        }
    }
}
impl<'info> From<WithdrawAllTokenTypesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAllTokenTypesAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.authority_info.clone(),
            accounts.user_transfer_authority_info.clone(),
            accounts.pool_mint_info.clone(),
            accounts.source_info.clone(),
            accounts.token_a_info.clone(),
            accounts.token_b_info.clone(),
            accounts.dest_token_a_info.clone(),
            accounts.dest_token_b_info.clone(),
            accounts.pool_fee_account_info.clone(),
            accounts.token_program_info.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN]>
for WithdrawAllTokenTypesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap_info: &arr[0],
            authority_info: &arr[1],
            user_transfer_authority_info: &arr[2],
            pool_mint_info: &arr[3],
            source_info: &arr[4],
            token_a_info: &arr[5],
            token_b_info: &arr[6],
            dest_token_a_info: &arr[7],
            dest_token_b_info: &arr[8],
            pool_fee_account_info: &arr[9],
            token_program_info: &arr[10],
        }
    }
}
pub const WITHDRAW_ALL_TOKEN_TYPES_IX_DISCM: [u8; 1usize] = [3];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawAllTokenTypesIxArgs {
    pub pool_token_amount: u64,
    pub minimum_token_a_amount: u64,
    pub minimum_token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawAllTokenTypesIxData(pub WithdrawAllTokenTypesIxArgs);
impl From<WithdrawAllTokenTypesIxArgs> for WithdrawAllTokenTypesIxData {
    fn from(args: WithdrawAllTokenTypesIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawAllTokenTypesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 1usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_ALL_TOKEN_TYPES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pool_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawAllTokenTypesIxArgs {
                pool_token_amount,
                minimum_token_a_amount,
                minimum_token_b_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_ALL_TOKEN_TYPES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pool_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_token_b_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_all_token_types_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawAllTokenTypesKeys,
    args: WithdrawAllTokenTypesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawAllTokenTypesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_all_token_types_ix(
    keys: WithdrawAllTokenTypesKeys,
    args: WithdrawAllTokenTypesIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_all_token_types_ix_with_program_id(SAROS_SWAP_PROGRAM_ID, keys, args)
}
pub fn withdraw_all_token_types_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAllTokenTypesAccounts<'_, '_>,
    args: WithdrawAllTokenTypesIxArgs,
) -> ProgramResult {
    let keys: WithdrawAllTokenTypesKeys = accounts.into();
    let ix = withdraw_all_token_types_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_all_token_types_invoke(
    accounts: WithdrawAllTokenTypesAccounts<'_, '_>,
    args: WithdrawAllTokenTypesIxArgs,
) -> ProgramResult {
    withdraw_all_token_types_invoke_with_program_id(
        SAROS_SWAP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_all_token_types_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAllTokenTypesAccounts<'_, '_>,
    args: WithdrawAllTokenTypesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawAllTokenTypesKeys = accounts.into();
    let ix = withdraw_all_token_types_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_all_token_types_invoke_signed(
    accounts: WithdrawAllTokenTypesAccounts<'_, '_>,
    args: WithdrawAllTokenTypesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_all_token_types_invoke_signed_with_program_id(
        SAROS_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_all_token_types_verify_account_keys(
    accounts: WithdrawAllTokenTypesAccounts<'_, '_>,
    keys: WithdrawAllTokenTypesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.authority_info.key, keys.authority_info),
        (*accounts.user_transfer_authority_info.key, keys.user_transfer_authority_info),
        (*accounts.pool_mint_info.key, keys.pool_mint_info),
        (*accounts.source_info.key, keys.source_info),
        (*accounts.token_a_info.key, keys.token_a_info),
        (*accounts.token_b_info.key, keys.token_b_info),
        (*accounts.dest_token_a_info.key, keys.dest_token_a_info),
        (*accounts.dest_token_b_info.key, keys.dest_token_b_info),
        (*accounts.pool_fee_account_info.key, keys.pool_fee_account_info),
        (*accounts.token_program_info.key, keys.token_program_info),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_all_token_types_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAllTokenTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_transfer_authority_info,
        accounts.pool_mint_info,
        accounts.source_info,
        accounts.token_a_info,
        accounts.token_b_info,
        accounts.dest_token_a_info,
        accounts.dest_token_b_info,
        accounts.pool_fee_account_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_all_token_types_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawAllTokenTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority_info] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_all_token_types_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAllTokenTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_all_token_types_verify_writable_privileges(accounts)?;
    withdraw_all_token_types_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactOutAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub authority_info: &'me AccountInfo<'info>,
    pub user_transfer_authority_info: &'me AccountInfo<'info>,
    pub source_info: &'me AccountInfo<'info>,
    pub swap_source_info: &'me AccountInfo<'info>,
    pub swap_destination_info: &'me AccountInfo<'info>,
    pub destination_info: &'me AccountInfo<'info>,
    pub pool_mint_info: &'me AccountInfo<'info>,
    pub pool_fee_account_info: &'me AccountInfo<'info>,
    pub token_program_info: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactOutKeys {
    pub swap_info: Pubkey,
    pub authority_info: Pubkey,
    pub user_transfer_authority_info: Pubkey,
    pub source_info: Pubkey,
    pub swap_source_info: Pubkey,
    pub swap_destination_info: Pubkey,
    pub destination_info: Pubkey,
    pub pool_mint_info: Pubkey,
    pub pool_fee_account_info: Pubkey,
    pub token_program_info: Pubkey,
}
impl From<SwapExactOutAccounts<'_, '_>> for SwapExactOutKeys {
    fn from(accounts: SwapExactOutAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            authority_info: *accounts.authority_info.key,
            user_transfer_authority_info: *accounts.user_transfer_authority_info.key,
            source_info: *accounts.source_info.key,
            swap_source_info: *accounts.swap_source_info.key,
            swap_destination_info: *accounts.swap_destination_info.key,
            destination_info: *accounts.destination_info.key,
            pool_mint_info: *accounts.pool_mint_info.key,
            pool_fee_account_info: *accounts.pool_fee_account_info.key,
            token_program_info: *accounts.token_program_info.key,
        }
    }
}
impl From<SwapExactOutKeys> for [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority_info,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_fee_account_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_info,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]> for SwapExactOutKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            authority_info: pubkeys[1],
            user_transfer_authority_info: pubkeys[2],
            source_info: pubkeys[3],
            swap_source_info: pubkeys[4],
            swap_destination_info: pubkeys[5],
            destination_info: pubkeys[6],
            pool_mint_info: pubkeys[7],
            pool_fee_account_info: pubkeys[8],
            token_program_info: pubkeys[9],
        }
    }
}
impl<'info> From<SwapExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.authority_info.clone(),
            accounts.user_transfer_authority_info.clone(),
            accounts.source_info.clone(),
            accounts.swap_source_info.clone(),
            accounts.swap_destination_info.clone(),
            accounts.destination_info.clone(),
            accounts.pool_mint_info.clone(),
            accounts.pool_fee_account_info.clone(),
            accounts.token_program_info.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]>
for SwapExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: &arr[0],
            authority_info: &arr[1],
            user_transfer_authority_info: &arr[2],
            source_info: &arr[3],
            swap_source_info: &arr[4],
            swap_destination_info: &arr[5],
            destination_info: &arr[6],
            pool_mint_info: &arr[7],
            pool_fee_account_info: &arr[8],
            token_program_info: &arr[9],
        }
    }
}
pub const SWAP_EXACT_OUT_IX_DISCM: [u8; 1usize] = [6];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactOutIxArgs {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactOutIxData(pub SwapExactOutIxArgs);
impl From<SwapExactOutIxArgs> for SwapExactOutIxData {
    fn from(args: SwapExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 1usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapExactOutIxArgs {
                amount_out,
                maximum_amount_in,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_amount_in, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactOutKeys,
    args: SwapExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_out_ix(
    keys: SwapExactOutKeys,
    args: SwapExactOutIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_out_ix_with_program_id(SAROS_SWAP_PROGRAM_ID, keys, args)
}
pub fn swap_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
) -> ProgramResult {
    let keys: SwapExactOutKeys = accounts.into();
    let ix = swap_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_out_invoke(
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
) -> ProgramResult {
    swap_exact_out_invoke_with_program_id(SAROS_SWAP_PROGRAM_ID, accounts, args)
}
pub fn swap_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactOutKeys = accounts.into();
    let ix = swap_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_out_invoke_signed(
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_out_invoke_signed_with_program_id(
        SAROS_SWAP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exact_out_verify_account_keys(
    accounts: SwapExactOutAccounts<'_, '_>,
    keys: SwapExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.authority_info.key, keys.authority_info),
        (*accounts.user_transfer_authority_info.key, keys.user_transfer_authority_info),
        (*accounts.source_info.key, keys.source_info),
        (*accounts.swap_source_info.key, keys.swap_source_info),
        (*accounts.swap_destination_info.key, keys.swap_destination_info),
        (*accounts.destination_info.key, keys.destination_info),
        (*accounts.pool_mint_info.key, keys.pool_mint_info),
        (*accounts.pool_fee_account_info.key, keys.pool_fee_account_info),
        (*accounts.token_program_info.key, keys.token_program_info),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.authority_info,
        accounts.user_transfer_authority_info,
        accounts.source_info,
        accounts.swap_source_info,
        accounts.swap_destination_info,
        accounts.destination_info,
        accounts.pool_mint_info,
        accounts.pool_fee_account_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority_info] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_account_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_out_verify_writable_privileges(accounts)?;
    swap_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
