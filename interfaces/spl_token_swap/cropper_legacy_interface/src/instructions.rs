use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum CropperLegacyProgramIx {
    Initialize(InitializeIxArgs),
    Swap(SwapIxArgs),
    DepositAllTokenTypes(DepositAllTokenTypesIxArgs),
    WithdrawAllTokenTypes(WithdrawAllTokenTypesIxArgs),
    DepositSingleTokenTypeExactAmountIn(DepositSingleTokenTypeExactAmountInIxArgs),
    WithdrawSingleTokenTypeExactAmountOut(WithdrawSingleTokenTypeExactAmountOutIxArgs),
}
impl CropperLegacyProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let fees = if reader.is_empty() {
                Default::default()
            } else {
                <Fees>::deserialize(&mut reader)?
            };
            let swap_curve = if reader.is_empty() {
                Default::default()
            } else {
                <SwapCurve>::deserialize(&mut reader)?
            };
            return Ok(
                Self::Initialize(InitializeIxArgs {
                    fees,
                    swap_curve,
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
        if buf.starts_with(&DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_DISCM
                .len()..];
            let source_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_pool_token_amount: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::DepositSingleTokenTypeExactAmountIn(DepositSingleTokenTypeExactAmountInIxArgs {
                    source_token_amount,
                    minimum_pool_token_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_DISCM
                .len()..];
            let destination_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_pool_token_amount: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::WithdrawSingleTokenTypeExactAmountOut(WithdrawSingleTokenTypeExactAmountOutIxArgs {
                    destination_token_amount,
                    maximum_pool_token_amount,
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
            Self::DepositSingleTokenTypeExactAmountIn(args) => {
                writer.write_all(&DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.source_token_amount,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_pool_token_amount,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::WithdrawSingleTokenTypeExactAmountOut(args) => {
                writer.write_all(&WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.destination_token_amount,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.maximum_pool_token_amount,
                    &mut writer,
                )?;
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
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_a: &'me AccountInfo<'info>,
    pub token_b: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub fee: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub pool: Pubkey,
    pub fee: Pubkey,
    pub destination: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            token_a: *accounts.token_a.key,
            token_b: *accounts.token_b.key,
            pool: *accounts.pool.key,
            fee: *accounts.fee.key,
            destination: *accounts.destination.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap: pubkeys[0],
            authority: pubkeys[1],
            token_a: pubkeys[2],
            token_b: pubkeys[3],
            pool: pubkeys[4],
            fee: pubkeys[5],
            destination: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.token_a.clone(),
            accounts.token_b.clone(),
            accounts.pool.clone(),
            accounts.fee.clone(),
            accounts.destination.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap: &arr[0],
            authority: &arr[1],
            token_a: &arr[2],
            token_b: &arr[3],
            pool: &arr[4],
            fee: &arr[5],
            destination: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 1usize] = [0];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub fees: Fees,
    pub swap_curve: SwapCurve,
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
        let swap_curve = if reader.is_empty() {
            Default::default()
        } else {
            <SwapCurve>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeIxArgs {
                fees,
                swap_curve,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_curve, &mut writer)?;
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
    initialize_ix_with_program_id(CROPPER_LEGACY_PROGRAM_ID, keys, args)
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
    initialize_invoke_with_program_id(CROPPER_LEGACY_PROGRAM_ID, accounts, args)
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
        CROPPER_LEGACY_PROGRAM_ID,
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
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_a.key, keys.token_a),
        (*accounts.token_b.key, keys.token_b),
        (*accounts.pool.key, keys.pool),
        (*accounts.fee.key, keys.fee),
        (*accounts.destination.key, keys.destination),
        (*accounts.token_program.key, keys.token_program),
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
    for should_be_writable in [accounts.swap, accounts.pool, accounts.destination] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.swap] {
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
pub const SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source: &'me AccountInfo<'info>,
    pub swap_source: &'me AccountInfo<'info>,
    pub swap_destination: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub pool_fee: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub token_program_0: &'me AccountInfo<'info>,
    pub token_program_1: &'me AccountInfo<'info>,
    pub token_program_2: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub token_program_0: Pubkey,
    pub token_program_1: Pubkey,
    pub token_program_2: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source: *accounts.source.key,
            swap_source: *accounts.swap_source.key,
            swap_destination: *accounts.swap_destination.key,
            destination: *accounts.destination.key,
            pool_mint: *accounts.pool_mint.key,
            pool_fee: *accounts.pool_fee.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            token_program_0: *accounts.token_program_0.key,
            token_program_1: *accounts.token_program_1.key,
            token_program_2: *accounts.token_program_2.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap: pubkeys[0],
            authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            source: pubkeys[3],
            swap_source: pubkeys[4],
            swap_destination: pubkeys[5],
            destination: pubkeys[6],
            pool_mint: pubkeys[7],
            pool_fee: pubkeys[8],
            source_mint: pubkeys[9],
            destination_mint: pubkeys[10],
            token_program_0: pubkeys[11],
            token_program_1: pubkeys[12],
            token_program_2: pubkeys[13],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source.clone(),
            accounts.swap_source.clone(),
            accounts.swap_destination.clone(),
            accounts.destination.clone(),
            accounts.pool_mint.clone(),
            accounts.pool_fee.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.token_program_0.clone(),
            accounts.token_program_1.clone(),
            accounts.token_program_2.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap: &arr[0],
            authority: &arr[1],
            user_transfer_authority: &arr[2],
            source: &arr[3],
            swap_source: &arr[4],
            swap_destination: &arr[5],
            destination: &arr[6],
            pool_mint: &arr[7],
            pool_fee: &arr[8],
            source_mint: &arr[9],
            destination_mint: &arr[10],
            token_program_0: &arr[11],
            token_program_1: &arr[12],
            token_program_2: &arr[13],
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
    swap_ix_with_program_id(CROPPER_LEGACY_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(CROPPER_LEGACY_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(CROPPER_LEGACY_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.source.key, keys.source),
        (*accounts.swap_source.key, keys.swap_source),
        (*accounts.swap_destination.key, keys.swap_destination),
        (*accounts.destination.key, keys.destination),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.pool_fee.key, keys.pool_fee),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.token_program_0.key, keys.token_program_0),
        (*accounts.token_program_1.key, keys.token_program_1),
        (*accounts.token_program_2.key, keys.token_program_2),
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
        accounts.source,
        accounts.swap_source,
        accounts.swap_destination,
        accounts.destination,
        accounts.pool_mint,
        accounts.pool_fee,
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
    for should_be_signer in [accounts.user_transfer_authority] {
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
pub const DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DepositAllTokenTypesAccounts<'me, 'info> {
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub deposit_token_a: &'me AccountInfo<'info>,
    pub deposit_token_b: &'me AccountInfo<'info>,
    pub swap_token_a: &'me AccountInfo<'info>,
    pub swap_token_b: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositAllTokenTypesKeys {
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub deposit_token_a: Pubkey,
    pub deposit_token_b: Pubkey,
    pub swap_token_a: Pubkey,
    pub swap_token_b: Pubkey,
    pub pool_mint: Pubkey,
    pub destination: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositAllTokenTypesAccounts<'_, '_>> for DepositAllTokenTypesKeys {
    fn from(accounts: DepositAllTokenTypesAccounts) -> Self {
        Self {
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            deposit_token_a: *accounts.deposit_token_a.key,
            deposit_token_b: *accounts.deposit_token_b.key,
            swap_token_a: *accounts.swap_token_a.key,
            swap_token_b: *accounts.swap_token_b.key,
            pool_mint: *accounts.pool_mint.key,
            destination: *accounts.destination.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositAllTokenTypesKeys>
for [AccountMeta; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositAllTokenTypesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
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
            swap: pubkeys[0],
            authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            deposit_token_a: pubkeys[3],
            deposit_token_b: pubkeys[4],
            swap_token_a: pubkeys[5],
            swap_token_b: pubkeys[6],
            pool_mint: pubkeys[7],
            destination: pubkeys[8],
            token_a_mint: pubkeys[9],
            token_b_mint: pubkeys[10],
            token_a_program: pubkeys[11],
            token_b_program: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<DepositAllTokenTypesAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAllTokenTypesAccounts<'_, 'info>) -> Self {
        [
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.deposit_token_a.clone(),
            accounts.deposit_token_b.clone(),
            accounts.swap_token_a.clone(),
            accounts.swap_token_b.clone(),
            accounts.pool_mint.clone(),
            accounts.destination.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN]>
for DepositAllTokenTypesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap: &arr[0],
            authority: &arr[1],
            user_transfer_authority: &arr[2],
            deposit_token_a: &arr[3],
            deposit_token_b: &arr[4],
            swap_token_a: &arr[5],
            swap_token_b: &arr[6],
            pool_mint: &arr[7],
            destination: &arr[8],
            token_a_mint: &arr[9],
            token_b_mint: &arr[10],
            token_a_program: &arr[11],
            token_b_program: &arr[12],
            token_program: &arr[13],
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
    deposit_all_token_types_ix_with_program_id(CROPPER_LEGACY_PROGRAM_ID, keys, args)
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
    deposit_all_token_types_invoke_with_program_id(
        CROPPER_LEGACY_PROGRAM_ID,
        accounts,
        args,
    )
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
        CROPPER_LEGACY_PROGRAM_ID,
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
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.deposit_token_a.key, keys.deposit_token_a),
        (*accounts.deposit_token_b.key, keys.deposit_token_b),
        (*accounts.swap_token_a.key, keys.swap_token_a),
        (*accounts.swap_token_b.key, keys.swap_token_b),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.destination.key, keys.destination),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.deposit_token_a,
        accounts.deposit_token_b,
        accounts.swap_token_a,
        accounts.swap_token_b,
        accounts.pool_mint,
        accounts.destination,
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
    for should_be_signer in [accounts.user_transfer_authority] {
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
pub const WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAllTokenTypesAccounts<'me, 'info> {
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub source: &'me AccountInfo<'info>,
    pub swap_token_a: &'me AccountInfo<'info>,
    pub swap_token_b: &'me AccountInfo<'info>,
    pub destination_token_a: &'me AccountInfo<'info>,
    pub destination_token_b: &'me AccountInfo<'info>,
    pub fee_account: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawAllTokenTypesKeys {
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub pool_mint: Pubkey,
    pub source: Pubkey,
    pub swap_token_a: Pubkey,
    pub swap_token_b: Pubkey,
    pub destination_token_a: Pubkey,
    pub destination_token_b: Pubkey,
    pub fee_account: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_program: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
}
impl From<WithdrawAllTokenTypesAccounts<'_, '_>> for WithdrawAllTokenTypesKeys {
    fn from(accounts: WithdrawAllTokenTypesAccounts) -> Self {
        Self {
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            pool_mint: *accounts.pool_mint.key,
            source: *accounts.source.key,
            swap_token_a: *accounts.swap_token_a.key,
            swap_token_b: *accounts.swap_token_b.key,
            destination_token_a: *accounts.destination_token_a.key,
            destination_token_b: *accounts.destination_token_b.key,
            fee_account: *accounts.fee_account.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_program: *accounts.token_program.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
        }
    }
}
impl From<WithdrawAllTokenTypesKeys>
for [AccountMeta; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawAllTokenTypesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
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
            swap: pubkeys[0],
            authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            pool_mint: pubkeys[3],
            source: pubkeys[4],
            swap_token_a: pubkeys[5],
            swap_token_b: pubkeys[6],
            destination_token_a: pubkeys[7],
            destination_token_b: pubkeys[8],
            fee_account: pubkeys[9],
            token_a_mint: pubkeys[10],
            token_b_mint: pubkeys[11],
            token_program: pubkeys[12],
            token_a_program: pubkeys[13],
            token_b_program: pubkeys[14],
        }
    }
}
impl<'info> From<WithdrawAllTokenTypesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_ALL_TOKEN_TYPES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAllTokenTypesAccounts<'_, 'info>) -> Self {
        [
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.pool_mint.clone(),
            accounts.source.clone(),
            accounts.swap_token_a.clone(),
            accounts.swap_token_b.clone(),
            accounts.destination_token_a.clone(),
            accounts.destination_token_b.clone(),
            accounts.fee_account.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_program.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
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
            swap: &arr[0],
            authority: &arr[1],
            user_transfer_authority: &arr[2],
            pool_mint: &arr[3],
            source: &arr[4],
            swap_token_a: &arr[5],
            swap_token_b: &arr[6],
            destination_token_a: &arr[7],
            destination_token_b: &arr[8],
            fee_account: &arr[9],
            token_a_mint: &arr[10],
            token_b_mint: &arr[11],
            token_program: &arr[12],
            token_a_program: &arr[13],
            token_b_program: &arr[14],
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
    withdraw_all_token_types_ix_with_program_id(CROPPER_LEGACY_PROGRAM_ID, keys, args)
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
        CROPPER_LEGACY_PROGRAM_ID,
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
        CROPPER_LEGACY_PROGRAM_ID,
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
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.source.key, keys.source),
        (*accounts.swap_token_a.key, keys.swap_token_a),
        (*accounts.swap_token_b.key, keys.swap_token_b),
        (*accounts.destination_token_a.key, keys.destination_token_a),
        (*accounts.destination_token_b.key, keys.destination_token_b),
        (*accounts.fee_account.key, keys.fee_account),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
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
        accounts.pool_mint,
        accounts.source,
        accounts.swap_token_a,
        accounts.swap_token_b,
        accounts.destination_token_a,
        accounts.destination_token_b,
        accounts.fee_account,
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
    for should_be_signer in [accounts.user_transfer_authority] {
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
pub const DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct DepositSingleTokenTypeExactAmountInAccounts<'me, 'info> {
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token: &'me AccountInfo<'info>,
    pub swap_token_a: &'me AccountInfo<'info>,
    pub swap_token_b: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub token_program_0: &'me AccountInfo<'info>,
    pub token_program_1: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositSingleTokenTypeExactAmountInKeys {
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token: Pubkey,
    pub swap_token_a: Pubkey,
    pub swap_token_b: Pubkey,
    pub pool_mint: Pubkey,
    pub destination: Pubkey,
    pub source_mint: Pubkey,
    pub token_program_0: Pubkey,
    pub token_program_1: Pubkey,
}
impl From<DepositSingleTokenTypeExactAmountInAccounts<'_, '_>>
for DepositSingleTokenTypeExactAmountInKeys {
    fn from(accounts: DepositSingleTokenTypeExactAmountInAccounts) -> Self {
        Self {
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token: *accounts.source_token.key,
            swap_token_a: *accounts.swap_token_a.key,
            swap_token_b: *accounts.swap_token_b.key,
            pool_mint: *accounts.pool_mint.key,
            destination: *accounts.destination.key,
            source_mint: *accounts.source_mint.key,
            token_program_0: *accounts.token_program_0.key,
            token_program_1: *accounts.token_program_1.key,
        }
    }
}
impl From<DepositSingleTokenTypeExactAmountInKeys>
for [AccountMeta; DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositSingleTokenTypeExactAmountInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_1,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_ACCOUNTS_LEN]>
for DepositSingleTokenTypeExactAmountInKeys {
    fn from(
        pubkeys: [Pubkey; DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap: pubkeys[0],
            authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            source_token: pubkeys[3],
            swap_token_a: pubkeys[4],
            swap_token_b: pubkeys[5],
            pool_mint: pubkeys[6],
            destination: pubkeys[7],
            source_mint: pubkeys[8],
            token_program_0: pubkeys[9],
            token_program_1: pubkeys[10],
        }
    }
}
impl<'info> From<DepositSingleTokenTypeExactAmountInAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositSingleTokenTypeExactAmountInAccounts<'_, 'info>) -> Self {
        [
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token.clone(),
            accounts.swap_token_a.clone(),
            accounts.swap_token_b.clone(),
            accounts.pool_mint.clone(),
            accounts.destination.clone(),
            accounts.source_mint.clone(),
            accounts.token_program_0.clone(),
            accounts.token_program_1.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_ACCOUNTS_LEN],
> for DepositSingleTokenTypeExactAmountInAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap: &arr[0],
            authority: &arr[1],
            user_transfer_authority: &arr[2],
            source_token: &arr[3],
            swap_token_a: &arr[4],
            swap_token_b: &arr[5],
            pool_mint: &arr[6],
            destination: &arr[7],
            source_mint: &arr[8],
            token_program_0: &arr[9],
            token_program_1: &arr[10],
        }
    }
}
pub const DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_DISCM: [u8; 1usize] = [4];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositSingleTokenTypeExactAmountInIxArgs {
    pub source_token_amount: u64,
    pub minimum_pool_token_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositSingleTokenTypeExactAmountInIxData(
    pub DepositSingleTokenTypeExactAmountInIxArgs,
);
impl From<DepositSingleTokenTypeExactAmountInIxArgs>
for DepositSingleTokenTypeExactAmountInIxData {
    fn from(args: DepositSingleTokenTypeExactAmountInIxArgs) -> Self {
        Self(args)
    }
}
impl DepositSingleTokenTypeExactAmountInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 1usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let source_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_pool_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositSingleTokenTypeExactAmountInIxArgs {
                source_token_amount,
                minimum_pool_token_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.source_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.minimum_pool_token_amount,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_single_token_type_exact_amount_in_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositSingleTokenTypeExactAmountInKeys,
    args: DepositSingleTokenTypeExactAmountInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DepositSingleTokenTypeExactAmountInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_single_token_type_exact_amount_in_ix(
    keys: DepositSingleTokenTypeExactAmountInKeys,
    args: DepositSingleTokenTypeExactAmountInIxArgs,
) -> std::io::Result<Instruction> {
    deposit_single_token_type_exact_amount_in_ix_with_program_id(
        CROPPER_LEGACY_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn deposit_single_token_type_exact_amount_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositSingleTokenTypeExactAmountInAccounts<'_, '_>,
    args: DepositSingleTokenTypeExactAmountInIxArgs,
) -> ProgramResult {
    let keys: DepositSingleTokenTypeExactAmountInKeys = accounts.into();
    let ix = deposit_single_token_type_exact_amount_in_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_single_token_type_exact_amount_in_invoke(
    accounts: DepositSingleTokenTypeExactAmountInAccounts<'_, '_>,
    args: DepositSingleTokenTypeExactAmountInIxArgs,
) -> ProgramResult {
    deposit_single_token_type_exact_amount_in_invoke_with_program_id(
        CROPPER_LEGACY_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn deposit_single_token_type_exact_amount_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositSingleTokenTypeExactAmountInAccounts<'_, '_>,
    args: DepositSingleTokenTypeExactAmountInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositSingleTokenTypeExactAmountInKeys = accounts.into();
    let ix = deposit_single_token_type_exact_amount_in_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_single_token_type_exact_amount_in_invoke_signed(
    accounts: DepositSingleTokenTypeExactAmountInAccounts<'_, '_>,
    args: DepositSingleTokenTypeExactAmountInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_single_token_type_exact_amount_in_invoke_signed_with_program_id(
        CROPPER_LEGACY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_single_token_type_exact_amount_in_verify_account_keys(
    accounts: DepositSingleTokenTypeExactAmountInAccounts<'_, '_>,
    keys: DepositSingleTokenTypeExactAmountInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.source_token.key, keys.source_token),
        (*accounts.swap_token_a.key, keys.swap_token_a),
        (*accounts.swap_token_b.key, keys.swap_token_b),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.destination.key, keys.destination),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.token_program_0.key, keys.token_program_0),
        (*accounts.token_program_1.key, keys.token_program_1),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_single_token_type_exact_amount_in_verify_writable_privileges<'me, 'info>(
    accounts: DepositSingleTokenTypeExactAmountInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token,
        accounts.swap_token_a,
        accounts.swap_token_b,
        accounts.pool_mint,
        accounts.destination,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_single_token_type_exact_amount_in_verify_signer_privileges<'me, 'info>(
    accounts: DepositSingleTokenTypeExactAmountInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_single_token_type_exact_amount_in_verify_account_privileges<'me, 'info>(
    accounts: DepositSingleTokenTypeExactAmountInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_single_token_type_exact_amount_in_verify_writable_privileges(accounts)?;
    deposit_single_token_type_exact_amount_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawSingleTokenTypeExactAmountOutAccounts<'me, 'info> {
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub pool_token_source: &'me AccountInfo<'info>,
    pub swap_token_a: &'me AccountInfo<'info>,
    pub swap_token_b: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub fee_account: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub token_program_0: &'me AccountInfo<'info>,
    pub token_program_1: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawSingleTokenTypeExactAmountOutKeys {
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_token_source: Pubkey,
    pub swap_token_a: Pubkey,
    pub swap_token_b: Pubkey,
    pub destination: Pubkey,
    pub fee_account: Pubkey,
    pub destination_mint: Pubkey,
    pub token_program_0: Pubkey,
    pub token_program_1: Pubkey,
}
impl From<WithdrawSingleTokenTypeExactAmountOutAccounts<'_, '_>>
for WithdrawSingleTokenTypeExactAmountOutKeys {
    fn from(accounts: WithdrawSingleTokenTypeExactAmountOutAccounts) -> Self {
        Self {
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            pool_mint: *accounts.pool_mint.key,
            pool_token_source: *accounts.pool_token_source.key,
            swap_token_a: *accounts.swap_token_a.key,
            swap_token_b: *accounts.swap_token_b.key,
            destination: *accounts.destination.key,
            fee_account: *accounts.fee_account.key,
            destination_mint: *accounts.destination_mint.key,
            token_program_0: *accounts.token_program_0.key,
            token_program_1: *accounts.token_program_1.key,
        }
    }
}
impl From<WithdrawSingleTokenTypeExactAmountOutKeys>
for [AccountMeta; WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawSingleTokenTypeExactAmountOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_1,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_ACCOUNTS_LEN]>
for WithdrawSingleTokenTypeExactAmountOutKeys {
    fn from(
        pubkeys: [Pubkey; WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap: pubkeys[0],
            authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            pool_mint: pubkeys[3],
            pool_token_source: pubkeys[4],
            swap_token_a: pubkeys[5],
            swap_token_b: pubkeys[6],
            destination: pubkeys[7],
            fee_account: pubkeys[8],
            destination_mint: pubkeys[9],
            token_program_0: pubkeys[10],
            token_program_1: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawSingleTokenTypeExactAmountOutAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'_, 'info>) -> Self {
        [
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.pool_mint.clone(),
            accounts.pool_token_source.clone(),
            accounts.swap_token_a.clone(),
            accounts.swap_token_b.clone(),
            accounts.destination.clone(),
            accounts.fee_account.clone(),
            accounts.destination_mint.clone(),
            accounts.token_program_0.clone(),
            accounts.token_program_1.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_ACCOUNTS_LEN],
> for WithdrawSingleTokenTypeExactAmountOutAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap: &arr[0],
            authority: &arr[1],
            user_transfer_authority: &arr[2],
            pool_mint: &arr[3],
            pool_token_source: &arr[4],
            swap_token_a: &arr[5],
            swap_token_b: &arr[6],
            destination: &arr[7],
            fee_account: &arr[8],
            destination_mint: &arr[9],
            token_program_0: &arr[10],
            token_program_1: &arr[11],
        }
    }
}
pub const WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_DISCM: [u8; 1usize] = [5];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawSingleTokenTypeExactAmountOutIxArgs {
    pub destination_token_amount: u64,
    pub maximum_pool_token_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawSingleTokenTypeExactAmountOutIxData(
    pub WithdrawSingleTokenTypeExactAmountOutIxArgs,
);
impl From<WithdrawSingleTokenTypeExactAmountOutIxArgs>
for WithdrawSingleTokenTypeExactAmountOutIxData {
    fn from(args: WithdrawSingleTokenTypeExactAmountOutIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawSingleTokenTypeExactAmountOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 1usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let destination_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_pool_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawSingleTokenTypeExactAmountOutIxArgs {
                destination_token_amount,
                maximum_pool_token_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.destination_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.maximum_pool_token_amount,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_single_token_type_exact_amount_out_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawSingleTokenTypeExactAmountOutKeys,
    args: WithdrawSingleTokenTypeExactAmountOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawSingleTokenTypeExactAmountOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_single_token_type_exact_amount_out_ix(
    keys: WithdrawSingleTokenTypeExactAmountOutKeys,
    args: WithdrawSingleTokenTypeExactAmountOutIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_single_token_type_exact_amount_out_ix_with_program_id(
        CROPPER_LEGACY_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn withdraw_single_token_type_exact_amount_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'_, '_>,
    args: WithdrawSingleTokenTypeExactAmountOutIxArgs,
) -> ProgramResult {
    let keys: WithdrawSingleTokenTypeExactAmountOutKeys = accounts.into();
    let ix = withdraw_single_token_type_exact_amount_out_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_single_token_type_exact_amount_out_invoke(
    accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'_, '_>,
    args: WithdrawSingleTokenTypeExactAmountOutIxArgs,
) -> ProgramResult {
    withdraw_single_token_type_exact_amount_out_invoke_with_program_id(
        CROPPER_LEGACY_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_single_token_type_exact_amount_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'_, '_>,
    args: WithdrawSingleTokenTypeExactAmountOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawSingleTokenTypeExactAmountOutKeys = accounts.into();
    let ix = withdraw_single_token_type_exact_amount_out_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_single_token_type_exact_amount_out_invoke_signed(
    accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'_, '_>,
    args: WithdrawSingleTokenTypeExactAmountOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_single_token_type_exact_amount_out_invoke_signed_with_program_id(
        CROPPER_LEGACY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_single_token_type_exact_amount_out_verify_account_keys(
    accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'_, '_>,
    keys: WithdrawSingleTokenTypeExactAmountOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.pool_token_source.key, keys.pool_token_source),
        (*accounts.swap_token_a.key, keys.swap_token_a),
        (*accounts.swap_token_b.key, keys.swap_token_b),
        (*accounts.destination.key, keys.destination),
        (*accounts.fee_account.key, keys.fee_account),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.token_program_0.key, keys.token_program_0),
        (*accounts.token_program_1.key, keys.token_program_1),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_single_token_type_exact_amount_out_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_mint,
        accounts.pool_token_source,
        accounts.swap_token_a,
        accounts.swap_token_b,
        accounts.destination,
        accounts.fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_single_token_type_exact_amount_out_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_single_token_type_exact_amount_out_verify_account_privileges<'me, 'info>(
    accounts: WithdrawSingleTokenTypeExactAmountOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_single_token_type_exact_amount_out_verify_writable_privileges(accounts)?;
    withdraw_single_token_type_exact_amount_out_verify_signer_privileges(accounts)?;
    Ok(())
}
