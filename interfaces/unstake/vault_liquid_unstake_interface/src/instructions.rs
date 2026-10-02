use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum VaultLiquidUnstakeProgramIx {
    CreateOrUpdateTokenMetadata,
    DepositSol(DepositSolIxArgs),
    FlashBorrow(FlashBorrowIxArgs),
    FlashRepay(FlashRepayIxArgs),
    InitializePool(InitializePoolIxArgs),
    LiquidUnstakeLst(LiquidUnstakeLstIxArgs),
    LiquidUnstakeLstWithSeed(LiquidUnstakeLstWithSeedIxArgs),
    LiquidUnstakeLstWithWrapped(LiquidUnstakeLstWithWrappedIxArgs),
    LiquidUnstakeLstWithWrappedSeed(LiquidUnstakeLstWithWrappedSeedIxArgs),
    LiquidUnstakeStakeAccount(LiquidUnstakeStakeAccountIxArgs),
    Update,
    UpdatePool(UpdatePoolIxArgs),
    UpsertLstInfo,
    WithdrawSol(WithdrawSolIxArgs),
    WithdrawStakeAccount(WithdrawStakeAccountIxArgs),
}
impl VaultLiquidUnstakeProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_OR_UPDATE_TOKEN_METADATA_IX_DISCM) {
            return Ok(Self::CreateOrUpdateTokenMetadata);
        }
        if buf.starts_with(&DEPOSIT_SOL_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_SOL_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DepositSol(DepositSolIxArgs { field_0 }));
        }
        if buf.starts_with(&FLASH_BORROW_IX_DISCM) {
            let mut reader = &buf[FLASH_BORROW_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::FlashBorrow(FlashBorrowIxArgs { field_0 }));
        }
        if buf.starts_with(&FLASH_REPAY_IX_DISCM) {
            let mut reader = &buf[FLASH_REPAY_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::FlashRepay(FlashRepayIxArgs { field_0 }));
        }
        if buf.starts_with(&INITIALIZE_POOL_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_POOL_IX_DISCM.len()..];
            let field_0: u32 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u32 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_5: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_6: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializePool(InitializePoolIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                    field_5,
                    field_6,
                }),
            );
        }
        if buf.starts_with(&LIQUID_UNSTAKE_LST_IX_DISCM) {
            let mut reader = &buf[LIQUID_UNSTAKE_LST_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_5: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidUnstakeLst(LiquidUnstakeLstIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                    field_5,
                }),
            );
        }
        if buf.starts_with(&LIQUID_UNSTAKE_LST_WITH_SEED_IX_DISCM) {
            let mut reader = &buf[LIQUID_UNSTAKE_LST_WITH_SEED_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_5: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_6: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidUnstakeLstWithSeed(LiquidUnstakeLstWithSeedIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                    field_5,
                    field_6,
                }),
            );
        }
        if buf.starts_with(&LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_DISCM) {
            let mut reader = &buf[LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_5: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidUnstakeLstWithWrapped(LiquidUnstakeLstWithWrappedIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                    field_5,
                }),
            );
        }
        if buf.starts_with(&LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_DISCM) {
            let mut reader = &buf[LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_5: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_6: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidUnstakeLstWithWrappedSeed(LiquidUnstakeLstWithWrappedSeedIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                    field_5,
                    field_6,
                }),
            );
        }
        if buf.starts_with(&LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_DISCM.len()..];
            let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidUnstakeStakeAccount(LiquidUnstakeStakeAccountIxArgs {
                    field_0,
                }),
            );
        }
        if buf.starts_with(&UPDATE_IX_DISCM) {
            return Ok(Self::Update);
        }
        if buf.starts_with(&UPDATE_POOL_IX_DISCM) {
            let mut reader = &buf[UPDATE_POOL_IX_DISCM.len()..];
            let field_0: u32 = crate::borsh_de_or_default(&mut reader)?;
            let field_1: u32 = crate::borsh_de_or_default(&mut reader)?;
            let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_3: u8 = crate::borsh_de_or_default(&mut reader)?;
            let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
            let field_5: u16 = crate::borsh_de_or_default(&mut reader)?;
            let field_6: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdatePool(UpdatePoolIxArgs {
                    field_0,
                    field_1,
                    field_2,
                    field_3,
                    field_4,
                    field_5,
                    field_6,
                }),
            );
        }
        if buf.starts_with(&UPSERT_LST_INFO_IX_DISCM) {
            return Ok(Self::UpsertLstInfo);
        }
        if buf.starts_with(&WITHDRAW_SOL_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_SOL_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::WithdrawSol(WithdrawSolIxArgs { field_0 }));
        }
        if buf.starts_with(&WITHDRAW_STAKE_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_STAKE_ACCOUNT_IX_DISCM.len()..];
            let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawStakeAccount(WithdrawStakeAccountIxArgs {
                    field_0,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateOrUpdateTokenMetadata => {
                writer.write_all(&CREATE_OR_UPDATE_TOKEN_METADATA_IX_DISCM)
            }
            Self::DepositSol(args) => {
                writer.write_all(&DEPOSIT_SOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::FlashBorrow(args) => {
                writer.write_all(&FLASH_BORROW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::FlashRepay(args) => {
                writer.write_all(&FLASH_REPAY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::InitializePool(args) => {
                writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_5, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_6, &mut writer)?;
                Ok(())
            }
            Self::LiquidUnstakeLst(args) => {
                writer.write_all(&LIQUID_UNSTAKE_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_5, &mut writer)?;
                Ok(())
            }
            Self::LiquidUnstakeLstWithSeed(args) => {
                writer.write_all(&LIQUID_UNSTAKE_LST_WITH_SEED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_5, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_6, &mut writer)?;
                Ok(())
            }
            Self::LiquidUnstakeLstWithWrapped(args) => {
                writer.write_all(&LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_5, &mut writer)?;
                Ok(())
            }
            Self::LiquidUnstakeLstWithWrappedSeed(args) => {
                writer.write_all(&LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_5, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_6, &mut writer)?;
                Ok(())
            }
            Self::LiquidUnstakeStakeAccount(args) => {
                writer.write_all(&LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::Update => writer.write_all(&UPDATE_IX_DISCM),
            Self::UpdatePool(args) => {
                writer.write_all(&UPDATE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_2, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_3, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_4, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_5, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.field_6, &mut writer)?;
                Ok(())
            }
            Self::UpsertLstInfo => writer.write_all(&UPSERT_LST_INFO_IX_DISCM),
            Self::WithdrawSol(args) => {
                writer.write_all(&WITHDRAW_SOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
                Ok(())
            }
            Self::WithdrawStakeAccount(args) => {
                writer.write_all(&WITHDRAW_STAKE_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field_0, &mut writer)?;
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
pub const CREATE_OR_UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateOrUpdateTokenMetadataAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub metadata_info: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOrUpdateTokenMetadataKeys {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub token_mint: Pubkey,
    pub metadata_program: Pubkey,
    pub metadata_info: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateOrUpdateTokenMetadataAccounts<'_, '_>>
for CreateOrUpdateTokenMetadataKeys {
    fn from(accounts: CreateOrUpdateTokenMetadataAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            token_mint: *accounts.token_mint.key,
            metadata_program: *accounts.metadata_program.key,
            metadata_info: *accounts.metadata_info.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateOrUpdateTokenMetadataKeys>
for [AccountMeta; CREATE_OR_UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOrUpdateTokenMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_OR_UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for CreateOrUpdateTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_OR_UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            authority: pubkeys[1],
            payer: pubkeys[2],
            token_mint: pubkeys[3],
            metadata_program: pubkeys[4],
            metadata_info: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateOrUpdateTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_OR_UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOrUpdateTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.token_mint.clone(),
            accounts.metadata_program.clone(),
            accounts.metadata_info.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_OR_UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for CreateOrUpdateTokenMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_OR_UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            authority: &arr[1],
            payer: &arr[2],
            token_mint: &arr[3],
            metadata_program: &arr[4],
            metadata_info: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CREATE_OR_UPDATE_TOKEN_METADATA_IX_DISCM: [u8; 8usize] = [
    203, 87, 105, 175, 156, 139, 235, 180,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOrUpdateTokenMetadataIxData;
impl CreateOrUpdateTokenMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_OR_UPDATE_TOKEN_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_OR_UPDATE_TOKEN_METADATA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_or_update_token_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOrUpdateTokenMetadataKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_OR_UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateOrUpdateTokenMetadataIxData.try_to_vec()?,
    })
}
pub fn create_or_update_token_metadata_ix(
    keys: CreateOrUpdateTokenMetadataKeys,
) -> std::io::Result<Instruction> {
    create_or_update_token_metadata_ix_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        keys,
    )
}
pub fn create_or_update_token_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOrUpdateTokenMetadataAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateOrUpdateTokenMetadataKeys = accounts.into();
    let ix = create_or_update_token_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_or_update_token_metadata_invoke(
    accounts: CreateOrUpdateTokenMetadataAccounts<'_, '_>,
) -> ProgramResult {
    create_or_update_token_metadata_invoke_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
    )
}
pub fn create_or_update_token_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOrUpdateTokenMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOrUpdateTokenMetadataKeys = accounts.into();
    let ix = create_or_update_token_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_or_update_token_metadata_invoke_signed(
    accounts: CreateOrUpdateTokenMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_or_update_token_metadata_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_or_update_token_metadata_verify_account_keys(
    accounts: CreateOrUpdateTokenMetadataAccounts<'_, '_>,
    keys: CreateOrUpdateTokenMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.authority.key, keys.authority),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.metadata_info.key, keys.metadata_info),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_or_update_token_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CreateOrUpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.payer,
        accounts.token_mint,
        accounts.metadata_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_or_update_token_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CreateOrUpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_or_update_token_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreateOrUpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_or_update_token_metadata_verify_writable_privileges(accounts)?;
    create_or_update_token_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_SOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct DepositSolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_lp_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositSolKeys {
    pub pool: Pubkey,
    pub sol_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub user: Pubkey,
    pub user_lp_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<DepositSolAccounts<'_, '_>> for DepositSolKeys {
    fn from(accounts: DepositSolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            sol_vault: *accounts.sol_vault.key,
            lp_mint: *accounts.lp_mint.key,
            user: *accounts.user.key,
            user_lp_account: *accounts.user_lp_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<DepositSolKeys> for [AccountMeta; DEPOSIT_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositSolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_SOL_IX_ACCOUNTS_LEN]> for DepositSolKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            sol_vault: pubkeys[1],
            lp_mint: pubkeys[2],
            user: pubkeys[3],
            user_lp_account: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
        }
    }
}
impl<'info> From<DepositSolAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_SOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositSolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.sol_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.user.clone(),
            accounts.user_lp_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_SOL_IX_ACCOUNTS_LEN]>
for DepositSolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            sol_vault: &arr[1],
            lp_mint: &arr[2],
            user: &arr[3],
            user_lp_account: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
        }
    }
}
pub const DEPOSIT_SOL_IX_DISCM: [u8; 8usize] = [108, 81, 78, 117, 125, 155, 56, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositSolIxArgs {
    pub field_0: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositSolIxData(pub DepositSolIxArgs);
impl From<DepositSolIxArgs> for DepositSolIxData {
    fn from(args: DepositSolIxArgs) -> Self {
        Self(args)
    }
}
impl DepositSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_SOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositSolIxArgs { field_0 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_SOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositSolKeys,
    args: DepositSolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_SOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositSolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_sol_ix(
    keys: DepositSolKeys,
    args: DepositSolIxArgs,
) -> std::io::Result<Instruction> {
    deposit_sol_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys, args)
}
pub fn deposit_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositSolAccounts<'_, '_>,
    args: DepositSolIxArgs,
) -> ProgramResult {
    let keys: DepositSolKeys = accounts.into();
    let ix = deposit_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_sol_invoke(
    accounts: DepositSolAccounts<'_, '_>,
    args: DepositSolIxArgs,
) -> ProgramResult {
    deposit_sol_invoke_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, accounts, args)
}
pub fn deposit_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositSolAccounts<'_, '_>,
    args: DepositSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositSolKeys = accounts.into();
    let ix = deposit_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_sol_invoke_signed(
    accounts: DepositSolAccounts<'_, '_>,
    args: DepositSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_sol_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_sol_verify_account_keys(
    accounts: DepositSolAccounts<'_, '_>,
    keys: DepositSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user.key, keys.user),
        (*accounts.user_lp_account.key, keys.user_lp_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_sol_verify_writable_privileges<'me, 'info>(
    accounts: DepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.sol_vault,
        accounts.lp_mint,
        accounts.user,
        accounts.user_lp_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_sol_verify_signer_privileges<'me, 'info>(
    accounts: DepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_sol_verify_account_privileges<'me, 'info>(
    accounts: DepositSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_sol_verify_writable_privileges(accounts)?;
    deposit_sol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASH_BORROW_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct FlashBorrowAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub borrower: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashBorrowKeys {
    pub pool: Pubkey,
    pub sol_vault: Pubkey,
    pub borrower: Pubkey,
    pub instructions: Pubkey,
    pub system_program: Pubkey,
}
impl From<FlashBorrowAccounts<'_, '_>> for FlashBorrowKeys {
    fn from(accounts: FlashBorrowAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            sol_vault: *accounts.sol_vault.key,
            borrower: *accounts.borrower.key,
            instructions: *accounts.instructions.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<FlashBorrowKeys> for [AccountMeta; FLASH_BORROW_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashBorrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrower,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions,
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
impl From<[Pubkey; FLASH_BORROW_IX_ACCOUNTS_LEN]> for FlashBorrowKeys {
    fn from(pubkeys: [Pubkey; FLASH_BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            sol_vault: pubkeys[1],
            borrower: pubkeys[2],
            instructions: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<FlashBorrowAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_BORROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashBorrowAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.sol_vault.clone(),
            accounts.borrower.clone(),
            accounts.instructions.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FLASH_BORROW_IX_ACCOUNTS_LEN]>
for FlashBorrowAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FLASH_BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            sol_vault: &arr[1],
            borrower: &arr[2],
            instructions: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const FLASH_BORROW_IX_DISCM: [u8; 8usize] = [166, 221, 220, 25, 61, 73, 127, 240];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashBorrowIxArgs {
    pub field_0: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashBorrowIxData(pub FlashBorrowIxArgs);
impl From<FlashBorrowIxArgs> for FlashBorrowIxData {
    fn from(args: FlashBorrowIxArgs) -> Self {
        Self(args)
    }
}
impl FlashBorrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_BORROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(FlashBorrowIxArgs { field_0 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_BORROW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn flash_borrow_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashBorrowKeys,
    args: FlashBorrowIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_BORROW_IX_ACCOUNTS_LEN] = keys.into();
    let data: FlashBorrowIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_borrow_ix(
    keys: FlashBorrowKeys,
    args: FlashBorrowIxArgs,
) -> std::io::Result<Instruction> {
    flash_borrow_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys, args)
}
pub fn flash_borrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashBorrowAccounts<'_, '_>,
    args: FlashBorrowIxArgs,
) -> ProgramResult {
    let keys: FlashBorrowKeys = accounts.into();
    let ix = flash_borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_borrow_invoke(
    accounts: FlashBorrowAccounts<'_, '_>,
    args: FlashBorrowIxArgs,
) -> ProgramResult {
    flash_borrow_invoke_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, accounts, args)
}
pub fn flash_borrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashBorrowAccounts<'_, '_>,
    args: FlashBorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashBorrowKeys = accounts.into();
    let ix = flash_borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_borrow_invoke_signed(
    accounts: FlashBorrowAccounts<'_, '_>,
    args: FlashBorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_borrow_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_borrow_verify_account_keys(
    accounts: FlashBorrowAccounts<'_, '_>,
    keys: FlashBorrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.borrower.key, keys.borrower),
        (*accounts.instructions.key, keys.instructions),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_borrow_verify_writable_privileges<'me, 'info>(
    accounts: FlashBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.sol_vault, accounts.borrower] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_borrow_verify_signer_privileges<'me, 'info>(
    accounts: FlashBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.borrower] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_borrow_verify_account_privileges<'me, 'info>(
    accounts: FlashBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_borrow_verify_writable_privileges(accounts)?;
    flash_borrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASH_REPAY_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct FlashRepayAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub borrower: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashRepayKeys {
    pub pool: Pubkey,
    pub sol_vault: Pubkey,
    pub borrower: Pubkey,
    pub manager_fee_account: Pubkey,
    pub instructions: Pubkey,
    pub system_program: Pubkey,
}
impl From<FlashRepayAccounts<'_, '_>> for FlashRepayKeys {
    fn from(accounts: FlashRepayAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            sol_vault: *accounts.sol_vault.key,
            borrower: *accounts.borrower.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            instructions: *accounts.instructions.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<FlashRepayKeys> for [AccountMeta; FLASH_REPAY_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashRepayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrower,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions,
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
impl From<[Pubkey; FLASH_REPAY_IX_ACCOUNTS_LEN]> for FlashRepayKeys {
    fn from(pubkeys: [Pubkey; FLASH_REPAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            sol_vault: pubkeys[1],
            borrower: pubkeys[2],
            manager_fee_account: pubkeys[3],
            instructions: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<FlashRepayAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_REPAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashRepayAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.sol_vault.clone(),
            accounts.borrower.clone(),
            accounts.manager_fee_account.clone(),
            accounts.instructions.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FLASH_REPAY_IX_ACCOUNTS_LEN]>
for FlashRepayAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FLASH_REPAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            sol_vault: &arr[1],
            borrower: &arr[2],
            manager_fee_account: &arr[3],
            instructions: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const FLASH_REPAY_IX_DISCM: [u8; 8usize] = [182, 143, 19, 23, 39, 221, 184, 78];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashRepayIxArgs {
    pub field_0: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashRepayIxData(pub FlashRepayIxArgs);
impl From<FlashRepayIxArgs> for FlashRepayIxData {
    fn from(args: FlashRepayIxArgs) -> Self {
        Self(args)
    }
}
impl FlashRepayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_REPAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(FlashRepayIxArgs { field_0 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_REPAY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn flash_repay_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashRepayKeys,
    args: FlashRepayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_REPAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: FlashRepayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_repay_ix(
    keys: FlashRepayKeys,
    args: FlashRepayIxArgs,
) -> std::io::Result<Instruction> {
    flash_repay_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys, args)
}
pub fn flash_repay_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashRepayAccounts<'_, '_>,
    args: FlashRepayIxArgs,
) -> ProgramResult {
    let keys: FlashRepayKeys = accounts.into();
    let ix = flash_repay_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_repay_invoke(
    accounts: FlashRepayAccounts<'_, '_>,
    args: FlashRepayIxArgs,
) -> ProgramResult {
    flash_repay_invoke_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, accounts, args)
}
pub fn flash_repay_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashRepayAccounts<'_, '_>,
    args: FlashRepayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashRepayKeys = accounts.into();
    let ix = flash_repay_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_repay_invoke_signed(
    accounts: FlashRepayAccounts<'_, '_>,
    args: FlashRepayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_repay_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_repay_verify_account_keys(
    accounts: FlashRepayAccounts<'_, '_>,
    keys: FlashRepayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.borrower.key, keys.borrower),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.instructions.key, keys.instructions),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_repay_verify_writable_privileges<'me, 'info>(
    accounts: FlashRepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.sol_vault,
        accounts.borrower,
        accounts.manager_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_repay_verify_signer_privileges<'me, 'info>(
    accounts: FlashRepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.borrower] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_repay_verify_account_privileges<'me, 'info>(
    accounts: FlashRepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_repay_verify_writable_privileges(accounts)?;
    flash_repay_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolKeys {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub sol_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub manager_fee_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializePoolAccounts<'_, '_>> for InitializePoolKeys {
    fn from(accounts: InitializePoolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            authority: *accounts.authority.key,
            sol_vault: *accounts.sol_vault.key,
            lp_mint: *accounts.lp_mint.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializePoolKeys> for [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]> for InitializePoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            authority: pubkeys[1],
            sol_vault: pubkeys[2],
            lp_mint: pubkeys[3],
            manager_fee_account: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<InitializePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.authority.clone(),
            accounts.sol_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.manager_fee_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]>
for InitializePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            authority: &arr[1],
            sol_vault: &arr[2],
            lp_mint: &arr[3],
            manager_fee_account: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const INITIALIZE_POOL_IX_DISCM: [u8; 8usize] = [95, 180, 10, 172, 84, 174, 232, 40];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePoolIxArgs {
    pub field_0: u32,
    pub field_1: u32,
    pub field_2: u64,
    pub field_3: u8,
    pub field_4: u64,
    pub field_5: u16,
    pub field_6: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolIxData(pub InitializePoolIxArgs);
impl From<InitializePoolIxArgs> for InitializePoolIxData {
    fn from(args: InitializePoolIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u32 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u32 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_6: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializePoolIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
                field_5,
                field_6,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_6, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_pool_ix(
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_pool_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys, args)
}
pub fn initialize_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pool_invoke(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    initialize_pool_invoke_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pool_invoke_signed(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_pool_verify_account_keys(
    accounts: InitializePoolAccounts<'_, '_>,
    keys: InitializePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.authority.key, keys.authority),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.authority,
        accounts.sol_vault,
        accounts.lp_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_verify_writable_privileges(accounts)?;
    initialize_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUID_UNSTAKE_LST_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct LiquidUnstakeLstAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_lst_account: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub user_sol_account: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub stake_pool: &'me AccountInfo<'info>,
    pub stake_pool_validator_list: &'me AccountInfo<'info>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'info>,
    pub stake_pool_manager_fee_account: &'me AccountInfo<'info>,
    pub stake_pool_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub stake_pool_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidUnstakeLstKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_lst_account: Pubkey,
    pub sol_vault: Pubkey,
    pub user_sol_account: Pubkey,
    pub manager_fee_account: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_validator_list: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_manager_fee_account: Pubkey,
    pub stake_pool_mint: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub stake_pool_program: Pubkey,
    pub system_program: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
}
impl From<LiquidUnstakeLstAccounts<'_, '_>> for LiquidUnstakeLstKeys {
    fn from(accounts: LiquidUnstakeLstAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_lst_account: *accounts.user_lst_account.key,
            sol_vault: *accounts.sol_vault.key,
            user_sol_account: *accounts.user_sol_account.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_validator_list: *accounts.stake_pool_validator_list.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_manager_fee_account: *accounts.stake_pool_manager_fee_account.key,
            stake_pool_mint: *accounts.stake_pool_mint.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            stake_pool_program: *accounts.stake_pool_program.key,
            system_program: *accounts.system_program.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<LiquidUnstakeLstKeys> for [AccountMeta; LIQUID_UNSTAKE_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidUnstakeLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lst_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_sol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUID_UNSTAKE_LST_IX_ACCOUNTS_LEN]> for LiquidUnstakeLstKeys {
    fn from(pubkeys: [Pubkey; LIQUID_UNSTAKE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            user_lst_account: pubkeys[3],
            sol_vault: pubkeys[4],
            user_sol_account: pubkeys[5],
            manager_fee_account: pubkeys[6],
            stake_pool: pubkeys[7],
            stake_pool_validator_list: pubkeys[8],
            stake_pool_withdraw_authority: pubkeys[9],
            stake_pool_manager_fee_account: pubkeys[10],
            stake_pool_mint: pubkeys[11],
            token_program: pubkeys[12],
            stake_program: pubkeys[13],
            stake_pool_program: pubkeys[14],
            system_program: pubkeys[15],
            clock: pubkeys[16],
            stake_history: pubkeys[17],
        }
    }
}
impl<'info> From<LiquidUnstakeLstAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUID_UNSTAKE_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidUnstakeLstAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.payer.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_lst_account.clone(),
            accounts.sol_vault.clone(),
            accounts.user_sol_account.clone(),
            accounts.manager_fee_account.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_validator_list.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_manager_fee_account.clone(),
            accounts.stake_pool_mint.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.stake_pool_program.clone(),
            accounts.system_program.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIQUID_UNSTAKE_LST_IX_ACCOUNTS_LEN]>
for LiquidUnstakeLstAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LIQUID_UNSTAKE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
            user_transfer_authority: &arr[2],
            user_lst_account: &arr[3],
            sol_vault: &arr[4],
            user_sol_account: &arr[5],
            manager_fee_account: &arr[6],
            stake_pool: &arr[7],
            stake_pool_validator_list: &arr[8],
            stake_pool_withdraw_authority: &arr[9],
            stake_pool_manager_fee_account: &arr[10],
            stake_pool_mint: &arr[11],
            token_program: &arr[12],
            stake_program: &arr[13],
            stake_pool_program: &arr[14],
            system_program: &arr[15],
            clock: &arr[16],
            stake_history: &arr[17],
        }
    }
}
pub const LIQUID_UNSTAKE_LST_IX_DISCM: [u8; 8usize] = [
    84, 174, 251, 245, 108, 64, 33, 185,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidUnstakeLstIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u64,
    pub field_3: u64,
    pub field_4: u64,
    pub field_5: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidUnstakeLstIxData(pub LiquidUnstakeLstIxArgs);
impl From<LiquidUnstakeLstIxArgs> for LiquidUnstakeLstIxData {
    fn from(args: LiquidUnstakeLstIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidUnstakeLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUID_UNSTAKE_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidUnstakeLstIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
                field_5,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUID_UNSTAKE_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_5, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquid_unstake_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidUnstakeLstKeys,
    args: LiquidUnstakeLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUID_UNSTAKE_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: LiquidUnstakeLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquid_unstake_lst_ix(
    keys: LiquidUnstakeLstKeys,
    args: LiquidUnstakeLstIxArgs,
) -> std::io::Result<Instruction> {
    liquid_unstake_lst_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys, args)
}
pub fn liquid_unstake_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeLstAccounts<'_, '_>,
    args: LiquidUnstakeLstIxArgs,
) -> ProgramResult {
    let keys: LiquidUnstakeLstKeys = accounts.into();
    let ix = liquid_unstake_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquid_unstake_lst_invoke(
    accounts: LiquidUnstakeLstAccounts<'_, '_>,
    args: LiquidUnstakeLstIxArgs,
) -> ProgramResult {
    liquid_unstake_lst_invoke_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn liquid_unstake_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeLstAccounts<'_, '_>,
    args: LiquidUnstakeLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidUnstakeLstKeys = accounts.into();
    let ix = liquid_unstake_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquid_unstake_lst_invoke_signed(
    accounts: LiquidUnstakeLstAccounts<'_, '_>,
    args: LiquidUnstakeLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquid_unstake_lst_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquid_unstake_lst_verify_account_keys(
    accounts: LiquidUnstakeLstAccounts<'_, '_>,
    keys: LiquidUnstakeLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_lst_account.key, keys.user_lst_account),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.user_sol_account.key, keys.user_sol_account),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.stake_pool.key, keys.stake_pool),
        (*accounts.stake_pool_validator_list.key, keys.stake_pool_validator_list),
        (
            *accounts.stake_pool_withdraw_authority.key,
            keys.stake_pool_withdraw_authority,
        ),
        (
            *accounts.stake_pool_manager_fee_account.key,
            keys.stake_pool_manager_fee_account,
        ),
        (*accounts.stake_pool_mint.key, keys.stake_pool_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.stake_pool_program.key, keys.stake_pool_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.clock.key, keys.clock),
        (*accounts.stake_history.key, keys.stake_history),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_verify_writable_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.payer,
        accounts.user_lst_account,
        accounts.sol_vault,
        accounts.user_sol_account,
        accounts.manager_fee_account,
        accounts.stake_pool,
        accounts.stake_pool_validator_list,
        accounts.stake_pool_manager_fee_account,
        accounts.stake_pool_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_verify_signer_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_verify_account_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquid_unstake_lst_verify_writable_privileges(accounts)?;
    liquid_unstake_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUID_UNSTAKE_LST_WITH_SEED_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct LiquidUnstakeLstWithSeedAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_lst_account: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub user_sol_account: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub stake_pool: &'me AccountInfo<'info>,
    pub stake_pool_validator_list: &'me AccountInfo<'info>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'info>,
    pub stake_pool_manager_fee_account: &'me AccountInfo<'info>,
    pub stake_pool_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub stake_pool_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidUnstakeLstWithSeedKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_lst_account: Pubkey,
    pub sol_vault: Pubkey,
    pub user_sol_account: Pubkey,
    pub manager_fee_account: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_validator_list: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_manager_fee_account: Pubkey,
    pub stake_pool_mint: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub stake_pool_program: Pubkey,
    pub system_program: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
}
impl From<LiquidUnstakeLstWithSeedAccounts<'_, '_>> for LiquidUnstakeLstWithSeedKeys {
    fn from(accounts: LiquidUnstakeLstWithSeedAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_lst_account: *accounts.user_lst_account.key,
            sol_vault: *accounts.sol_vault.key,
            user_sol_account: *accounts.user_sol_account.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_validator_list: *accounts.stake_pool_validator_list.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_manager_fee_account: *accounts.stake_pool_manager_fee_account.key,
            stake_pool_mint: *accounts.stake_pool_mint.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            stake_pool_program: *accounts.stake_pool_program.key,
            system_program: *accounts.system_program.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
        }
    }
}
impl From<LiquidUnstakeLstWithSeedKeys>
for [AccountMeta; LIQUID_UNSTAKE_LST_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidUnstakeLstWithSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lst_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_sol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUID_UNSTAKE_LST_WITH_SEED_IX_ACCOUNTS_LEN]>
for LiquidUnstakeLstWithSeedKeys {
    fn from(pubkeys: [Pubkey; LIQUID_UNSTAKE_LST_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            user_lst_account: pubkeys[3],
            sol_vault: pubkeys[4],
            user_sol_account: pubkeys[5],
            manager_fee_account: pubkeys[6],
            stake_pool: pubkeys[7],
            stake_pool_validator_list: pubkeys[8],
            stake_pool_withdraw_authority: pubkeys[9],
            stake_pool_manager_fee_account: pubkeys[10],
            stake_pool_mint: pubkeys[11],
            token_program: pubkeys[12],
            stake_program: pubkeys[13],
            stake_pool_program: pubkeys[14],
            system_program: pubkeys[15],
            clock: pubkeys[16],
            stake_history: pubkeys[17],
        }
    }
}
impl<'info> From<LiquidUnstakeLstWithSeedAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUID_UNSTAKE_LST_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidUnstakeLstWithSeedAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.payer.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_lst_account.clone(),
            accounts.sol_vault.clone(),
            accounts.user_sol_account.clone(),
            accounts.manager_fee_account.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_validator_list.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_manager_fee_account.clone(),
            accounts.stake_pool_mint.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.stake_pool_program.clone(),
            accounts.system_program.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LIQUID_UNSTAKE_LST_WITH_SEED_IX_ACCOUNTS_LEN]>
for LiquidUnstakeLstWithSeedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUID_UNSTAKE_LST_WITH_SEED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
            user_transfer_authority: &arr[2],
            user_lst_account: &arr[3],
            sol_vault: &arr[4],
            user_sol_account: &arr[5],
            manager_fee_account: &arr[6],
            stake_pool: &arr[7],
            stake_pool_validator_list: &arr[8],
            stake_pool_withdraw_authority: &arr[9],
            stake_pool_manager_fee_account: &arr[10],
            stake_pool_mint: &arr[11],
            token_program: &arr[12],
            stake_program: &arr[13],
            stake_pool_program: &arr[14],
            system_program: &arr[15],
            clock: &arr[16],
            stake_history: &arr[17],
        }
    }
}
pub const LIQUID_UNSTAKE_LST_WITH_SEED_IX_DISCM: [u8; 8usize] = [
    205, 199, 161, 101, 239, 109, 148, 163,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidUnstakeLstWithSeedIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u64,
    pub field_3: u64,
    pub field_4: u64,
    pub field_5: u8,
    pub field_6: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidUnstakeLstWithSeedIxData(pub LiquidUnstakeLstWithSeedIxArgs);
impl From<LiquidUnstakeLstWithSeedIxArgs> for LiquidUnstakeLstWithSeedIxData {
    fn from(args: LiquidUnstakeLstWithSeedIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidUnstakeLstWithSeedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUID_UNSTAKE_LST_WITH_SEED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_6: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidUnstakeLstWithSeedIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
                field_5,
                field_6,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUID_UNSTAKE_LST_WITH_SEED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_6, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquid_unstake_lst_with_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidUnstakeLstWithSeedKeys,
    args: LiquidUnstakeLstWithSeedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUID_UNSTAKE_LST_WITH_SEED_IX_ACCOUNTS_LEN] = keys.into();
    let data: LiquidUnstakeLstWithSeedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquid_unstake_lst_with_seed_ix(
    keys: LiquidUnstakeLstWithSeedKeys,
    args: LiquidUnstakeLstWithSeedIxArgs,
) -> std::io::Result<Instruction> {
    liquid_unstake_lst_with_seed_ix_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn liquid_unstake_lst_with_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeLstWithSeedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithSeedIxArgs,
) -> ProgramResult {
    let keys: LiquidUnstakeLstWithSeedKeys = accounts.into();
    let ix = liquid_unstake_lst_with_seed_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquid_unstake_lst_with_seed_invoke(
    accounts: LiquidUnstakeLstWithSeedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithSeedIxArgs,
) -> ProgramResult {
    liquid_unstake_lst_with_seed_invoke_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn liquid_unstake_lst_with_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeLstWithSeedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidUnstakeLstWithSeedKeys = accounts.into();
    let ix = liquid_unstake_lst_with_seed_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquid_unstake_lst_with_seed_invoke_signed(
    accounts: LiquidUnstakeLstWithSeedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquid_unstake_lst_with_seed_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquid_unstake_lst_with_seed_verify_account_keys(
    accounts: LiquidUnstakeLstWithSeedAccounts<'_, '_>,
    keys: LiquidUnstakeLstWithSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_lst_account.key, keys.user_lst_account),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.user_sol_account.key, keys.user_sol_account),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.stake_pool.key, keys.stake_pool),
        (*accounts.stake_pool_validator_list.key, keys.stake_pool_validator_list),
        (
            *accounts.stake_pool_withdraw_authority.key,
            keys.stake_pool_withdraw_authority,
        ),
        (
            *accounts.stake_pool_manager_fee_account.key,
            keys.stake_pool_manager_fee_account,
        ),
        (*accounts.stake_pool_mint.key, keys.stake_pool_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.stake_pool_program.key, keys.stake_pool_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.clock.key, keys.clock),
        (*accounts.stake_history.key, keys.stake_history),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_seed_verify_writable_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.payer,
        accounts.user_lst_account,
        accounts.sol_vault,
        accounts.user_sol_account,
        accounts.manager_fee_account,
        accounts.stake_pool,
        accounts.stake_pool_validator_list,
        accounts.stake_pool_manager_fee_account,
        accounts.stake_pool_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_seed_verify_signer_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_seed_verify_account_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquid_unstake_lst_with_seed_verify_writable_privileges(accounts)?;
    liquid_unstake_lst_with_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct LiquidUnstakeLstWithWrappedAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_lst_account: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub user_sol_account: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub stake_pool: &'me AccountInfo<'info>,
    pub stake_pool_validator_list: &'me AccountInfo<'info>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'info>,
    pub stake_pool_manager_fee_account: &'me AccountInfo<'info>,
    pub stake_pool_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub stake_pool_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidUnstakeLstWithWrappedKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_lst_account: Pubkey,
    pub sol_vault: Pubkey,
    pub user_sol_account: Pubkey,
    pub manager_fee_account: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_validator_list: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_manager_fee_account: Pubkey,
    pub stake_pool_mint: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub stake_pool_program: Pubkey,
    pub system_program: Pubkey,
    pub clock: Pubkey,
}
impl From<LiquidUnstakeLstWithWrappedAccounts<'_, '_>>
for LiquidUnstakeLstWithWrappedKeys {
    fn from(accounts: LiquidUnstakeLstWithWrappedAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_lst_account: *accounts.user_lst_account.key,
            sol_vault: *accounts.sol_vault.key,
            user_sol_account: *accounts.user_sol_account.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_validator_list: *accounts.stake_pool_validator_list.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_manager_fee_account: *accounts.stake_pool_manager_fee_account.key,
            stake_pool_mint: *accounts.stake_pool_mint.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            stake_pool_program: *accounts.stake_pool_program.key,
            system_program: *accounts.system_program.key,
            clock: *accounts.clock.key,
        }
    }
}
impl From<LiquidUnstakeLstWithWrappedKeys>
for [AccountMeta; LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidUnstakeLstWithWrappedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lst_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_sol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_ACCOUNTS_LEN]>
for LiquidUnstakeLstWithWrappedKeys {
    fn from(pubkeys: [Pubkey; LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            user_lst_account: pubkeys[3],
            sol_vault: pubkeys[4],
            user_sol_account: pubkeys[5],
            manager_fee_account: pubkeys[6],
            stake_pool: pubkeys[7],
            stake_pool_validator_list: pubkeys[8],
            stake_pool_withdraw_authority: pubkeys[9],
            stake_pool_manager_fee_account: pubkeys[10],
            stake_pool_mint: pubkeys[11],
            token_program: pubkeys[12],
            stake_program: pubkeys[13],
            stake_pool_program: pubkeys[14],
            system_program: pubkeys[15],
            clock: pubkeys[16],
        }
    }
}
impl<'info> From<LiquidUnstakeLstWithWrappedAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidUnstakeLstWithWrappedAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.payer.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_lst_account.clone(),
            accounts.sol_vault.clone(),
            accounts.user_sol_account.clone(),
            accounts.manager_fee_account.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_validator_list.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_manager_fee_account.clone(),
            accounts.stake_pool_mint.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.stake_pool_program.clone(),
            accounts.system_program.clone(),
            accounts.clock.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_ACCOUNTS_LEN]>
for LiquidUnstakeLstWithWrappedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
            user_transfer_authority: &arr[2],
            user_lst_account: &arr[3],
            sol_vault: &arr[4],
            user_sol_account: &arr[5],
            manager_fee_account: &arr[6],
            stake_pool: &arr[7],
            stake_pool_validator_list: &arr[8],
            stake_pool_withdraw_authority: &arr[9],
            stake_pool_manager_fee_account: &arr[10],
            stake_pool_mint: &arr[11],
            token_program: &arr[12],
            stake_program: &arr[13],
            stake_pool_program: &arr[14],
            system_program: &arr[15],
            clock: &arr[16],
        }
    }
}
pub const LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_DISCM: [u8; 8usize] = [
    88, 109, 224, 88, 64, 249, 243, 117,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidUnstakeLstWithWrappedIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u64,
    pub field_3: u64,
    pub field_4: u64,
    pub field_5: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidUnstakeLstWithWrappedIxData(pub LiquidUnstakeLstWithWrappedIxArgs);
impl From<LiquidUnstakeLstWithWrappedIxArgs> for LiquidUnstakeLstWithWrappedIxData {
    fn from(args: LiquidUnstakeLstWithWrappedIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidUnstakeLstWithWrappedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidUnstakeLstWithWrappedIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
                field_5,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_5, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquid_unstake_lst_with_wrapped_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidUnstakeLstWithWrappedKeys,
    args: LiquidUnstakeLstWithWrappedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUID_UNSTAKE_LST_WITH_WRAPPED_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LiquidUnstakeLstWithWrappedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquid_unstake_lst_with_wrapped_ix(
    keys: LiquidUnstakeLstWithWrappedKeys,
    args: LiquidUnstakeLstWithWrappedIxArgs,
) -> std::io::Result<Instruction> {
    liquid_unstake_lst_with_wrapped_ix_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn liquid_unstake_lst_with_wrapped_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeLstWithWrappedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithWrappedIxArgs,
) -> ProgramResult {
    let keys: LiquidUnstakeLstWithWrappedKeys = accounts.into();
    let ix = liquid_unstake_lst_with_wrapped_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquid_unstake_lst_with_wrapped_invoke(
    accounts: LiquidUnstakeLstWithWrappedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithWrappedIxArgs,
) -> ProgramResult {
    liquid_unstake_lst_with_wrapped_invoke_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn liquid_unstake_lst_with_wrapped_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeLstWithWrappedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithWrappedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidUnstakeLstWithWrappedKeys = accounts.into();
    let ix = liquid_unstake_lst_with_wrapped_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquid_unstake_lst_with_wrapped_invoke_signed(
    accounts: LiquidUnstakeLstWithWrappedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithWrappedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquid_unstake_lst_with_wrapped_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquid_unstake_lst_with_wrapped_verify_account_keys(
    accounts: LiquidUnstakeLstWithWrappedAccounts<'_, '_>,
    keys: LiquidUnstakeLstWithWrappedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_lst_account.key, keys.user_lst_account),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.user_sol_account.key, keys.user_sol_account),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.stake_pool.key, keys.stake_pool),
        (*accounts.stake_pool_validator_list.key, keys.stake_pool_validator_list),
        (
            *accounts.stake_pool_withdraw_authority.key,
            keys.stake_pool_withdraw_authority,
        ),
        (
            *accounts.stake_pool_manager_fee_account.key,
            keys.stake_pool_manager_fee_account,
        ),
        (*accounts.stake_pool_mint.key, keys.stake_pool_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.stake_pool_program.key, keys.stake_pool_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.clock.key, keys.clock),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_wrapped_verify_writable_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithWrappedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.payer,
        accounts.user_lst_account,
        accounts.sol_vault,
        accounts.user_sol_account,
        accounts.manager_fee_account,
        accounts.stake_pool,
        accounts.stake_pool_validator_list,
        accounts.stake_pool_manager_fee_account,
        accounts.stake_pool_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_wrapped_verify_signer_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithWrappedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_wrapped_verify_account_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithWrappedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquid_unstake_lst_with_wrapped_verify_writable_privileges(accounts)?;
    liquid_unstake_lst_with_wrapped_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct LiquidUnstakeLstWithWrappedSeedAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_lst_account: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub user_sol_account: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub stake_pool: &'me AccountInfo<'info>,
    pub stake_pool_validator_list: &'me AccountInfo<'info>,
    pub stake_pool_withdraw_authority: &'me AccountInfo<'info>,
    pub stake_pool_manager_fee_account: &'me AccountInfo<'info>,
    pub stake_pool_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub stake_pool_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidUnstakeLstWithWrappedSeedKeys {
    pub pool: Pubkey,
    pub payer: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_lst_account: Pubkey,
    pub sol_vault: Pubkey,
    pub user_sol_account: Pubkey,
    pub manager_fee_account: Pubkey,
    pub stake_pool: Pubkey,
    pub stake_pool_validator_list: Pubkey,
    pub stake_pool_withdraw_authority: Pubkey,
    pub stake_pool_manager_fee_account: Pubkey,
    pub stake_pool_mint: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub stake_pool_program: Pubkey,
    pub system_program: Pubkey,
    pub clock: Pubkey,
}
impl From<LiquidUnstakeLstWithWrappedSeedAccounts<'_, '_>>
for LiquidUnstakeLstWithWrappedSeedKeys {
    fn from(accounts: LiquidUnstakeLstWithWrappedSeedAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            payer: *accounts.payer.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_lst_account: *accounts.user_lst_account.key,
            sol_vault: *accounts.sol_vault.key,
            user_sol_account: *accounts.user_sol_account.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            stake_pool: *accounts.stake_pool.key,
            stake_pool_validator_list: *accounts.stake_pool_validator_list.key,
            stake_pool_withdraw_authority: *accounts.stake_pool_withdraw_authority.key,
            stake_pool_manager_fee_account: *accounts.stake_pool_manager_fee_account.key,
            stake_pool_mint: *accounts.stake_pool_mint.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            stake_pool_program: *accounts.stake_pool_program.key,
            system_program: *accounts.system_program.key,
            clock: *accounts.clock.key,
        }
    }
}
impl From<LiquidUnstakeLstWithWrappedSeedKeys>
for [AccountMeta; LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidUnstakeLstWithWrappedSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lst_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_sol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_ACCOUNTS_LEN]>
for LiquidUnstakeLstWithWrappedSeedKeys {
    fn from(
        pubkeys: [Pubkey; LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: pubkeys[0],
            payer: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            user_lst_account: pubkeys[3],
            sol_vault: pubkeys[4],
            user_sol_account: pubkeys[5],
            manager_fee_account: pubkeys[6],
            stake_pool: pubkeys[7],
            stake_pool_validator_list: pubkeys[8],
            stake_pool_withdraw_authority: pubkeys[9],
            stake_pool_manager_fee_account: pubkeys[10],
            stake_pool_mint: pubkeys[11],
            token_program: pubkeys[12],
            stake_program: pubkeys[13],
            stake_pool_program: pubkeys[14],
            system_program: pubkeys[15],
            clock: pubkeys[16],
        }
    }
}
impl<'info> From<LiquidUnstakeLstWithWrappedSeedAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.payer.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_lst_account.clone(),
            accounts.sol_vault.clone(),
            accounts.user_sol_account.clone(),
            accounts.manager_fee_account.clone(),
            accounts.stake_pool.clone(),
            accounts.stake_pool_validator_list.clone(),
            accounts.stake_pool_withdraw_authority.clone(),
            accounts.stake_pool_manager_fee_account.clone(),
            accounts.stake_pool_mint.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.stake_pool_program.clone(),
            accounts.system_program.clone(),
            accounts.clock.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_ACCOUNTS_LEN]>
for LiquidUnstakeLstWithWrappedSeedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            payer: &arr[1],
            user_transfer_authority: &arr[2],
            user_lst_account: &arr[3],
            sol_vault: &arr[4],
            user_sol_account: &arr[5],
            manager_fee_account: &arr[6],
            stake_pool: &arr[7],
            stake_pool_validator_list: &arr[8],
            stake_pool_withdraw_authority: &arr[9],
            stake_pool_manager_fee_account: &arr[10],
            stake_pool_mint: &arr[11],
            token_program: &arr[12],
            stake_program: &arr[13],
            stake_pool_program: &arr[14],
            system_program: &arr[15],
            clock: &arr[16],
        }
    }
}
pub const LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_DISCM: [u8; 8usize] = [
    81, 151, 125, 105, 38, 28, 199, 104,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidUnstakeLstWithWrappedSeedIxArgs {
    pub field_0: u64,
    pub field_1: u64,
    pub field_2: u64,
    pub field_3: u64,
    pub field_4: u64,
    pub field_5: u8,
    pub field_6: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidUnstakeLstWithWrappedSeedIxData(
    pub LiquidUnstakeLstWithWrappedSeedIxArgs,
);
impl From<LiquidUnstakeLstWithWrappedSeedIxArgs>
for LiquidUnstakeLstWithWrappedSeedIxData {
    fn from(args: LiquidUnstakeLstWithWrappedSeedIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidUnstakeLstWithWrappedSeedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_6: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidUnstakeLstWithWrappedSeedIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
                field_5,
                field_6,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_6, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquid_unstake_lst_with_wrapped_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidUnstakeLstWithWrappedSeedKeys,
    args: LiquidUnstakeLstWithWrappedSeedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUID_UNSTAKE_LST_WITH_WRAPPED_SEED_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LiquidUnstakeLstWithWrappedSeedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquid_unstake_lst_with_wrapped_seed_ix(
    keys: LiquidUnstakeLstWithWrappedSeedKeys,
    args: LiquidUnstakeLstWithWrappedSeedIxArgs,
) -> std::io::Result<Instruction> {
    liquid_unstake_lst_with_wrapped_seed_ix_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn liquid_unstake_lst_with_wrapped_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithWrappedSeedIxArgs,
) -> ProgramResult {
    let keys: LiquidUnstakeLstWithWrappedSeedKeys = accounts.into();
    let ix = liquid_unstake_lst_with_wrapped_seed_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn liquid_unstake_lst_with_wrapped_seed_invoke(
    accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithWrappedSeedIxArgs,
) -> ProgramResult {
    liquid_unstake_lst_with_wrapped_seed_invoke_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn liquid_unstake_lst_with_wrapped_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithWrappedSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidUnstakeLstWithWrappedSeedKeys = accounts.into();
    let ix = liquid_unstake_lst_with_wrapped_seed_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquid_unstake_lst_with_wrapped_seed_invoke_signed(
    accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'_, '_>,
    args: LiquidUnstakeLstWithWrappedSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquid_unstake_lst_with_wrapped_seed_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquid_unstake_lst_with_wrapped_seed_verify_account_keys(
    accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'_, '_>,
    keys: LiquidUnstakeLstWithWrappedSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.payer.key, keys.payer),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.user_lst_account.key, keys.user_lst_account),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.user_sol_account.key, keys.user_sol_account),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.stake_pool.key, keys.stake_pool),
        (*accounts.stake_pool_validator_list.key, keys.stake_pool_validator_list),
        (
            *accounts.stake_pool_withdraw_authority.key,
            keys.stake_pool_withdraw_authority,
        ),
        (
            *accounts.stake_pool_manager_fee_account.key,
            keys.stake_pool_manager_fee_account,
        ),
        (*accounts.stake_pool_mint.key, keys.stake_pool_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.stake_pool_program.key, keys.stake_pool_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.clock.key, keys.clock),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_wrapped_seed_verify_writable_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.payer,
        accounts.user_lst_account,
        accounts.sol_vault,
        accounts.user_sol_account,
        accounts.manager_fee_account,
        accounts.stake_pool,
        accounts.stake_pool_validator_list,
        accounts.stake_pool_manager_fee_account,
        accounts.stake_pool_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_wrapped_seed_verify_signer_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquid_unstake_lst_with_wrapped_seed_verify_account_privileges<'me, 'info>(
    accounts: LiquidUnstakeLstWithWrappedSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquid_unstake_lst_with_wrapped_seed_verify_writable_privileges(accounts)?;
    liquid_unstake_lst_with_wrapped_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct LiquidUnstakeStakeAccountAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_account_info: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub user_sol_account: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidUnstakeStakeAccountKeys {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub stake_account: Pubkey,
    pub stake_account_info: Pubkey,
    pub sol_vault: Pubkey,
    pub user_sol_account: Pubkey,
    pub manager_fee_account: Pubkey,
    pub stake_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub clock: Pubkey,
}
impl From<LiquidUnstakeStakeAccountAccounts<'_, '_>> for LiquidUnstakeStakeAccountKeys {
    fn from(accounts: LiquidUnstakeStakeAccountAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            user: *accounts.user.key,
            stake_account: *accounts.stake_account.key,
            stake_account_info: *accounts.stake_account_info.key,
            sol_vault: *accounts.sol_vault.key,
            user_sol_account: *accounts.user_sol_account.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            stake_program: *accounts.stake_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            clock: *accounts.clock.key,
        }
    }
}
impl From<LiquidUnstakeStakeAccountKeys>
for [AccountMeta; LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidUnstakeStakeAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_sol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
for LiquidUnstakeStakeAccountKeys {
    fn from(pubkeys: [Pubkey; LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            user: pubkeys[1],
            stake_account: pubkeys[2],
            stake_account_info: pubkeys[3],
            sol_vault: pubkeys[4],
            user_sol_account: pubkeys[5],
            manager_fee_account: pubkeys[6],
            stake_program: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            clock: pubkeys[10],
        }
    }
}
impl<'info> From<LiquidUnstakeStakeAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidUnstakeStakeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.user.clone(),
            accounts.stake_account.clone(),
            accounts.stake_account_info.clone(),
            accounts.sol_vault.clone(),
            accounts.user_sol_account.clone(),
            accounts.manager_fee_account.clone(),
            accounts.stake_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.clock.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
for LiquidUnstakeStakeAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            user: &arr[1],
            stake_account: &arr[2],
            stake_account_info: &arr[3],
            sol_vault: &arr[4],
            user_sol_account: &arr[5],
            manager_fee_account: &arr[6],
            stake_program: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            clock: &arr[10],
        }
    }
}
pub const LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    6, 242, 242, 0, 61, 230, 96, 58,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidUnstakeStakeAccountIxArgs {
    pub field_0: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidUnstakeStakeAccountIxData(pub LiquidUnstakeStakeAccountIxArgs);
impl From<LiquidUnstakeStakeAccountIxArgs> for LiquidUnstakeStakeAccountIxData {
    fn from(args: LiquidUnstakeStakeAccountIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidUnstakeStakeAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidUnstakeStakeAccountIxArgs {
                field_0,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquid_unstake_stake_account_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidUnstakeStakeAccountKeys,
    args: LiquidUnstakeStakeAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUID_UNSTAKE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: LiquidUnstakeStakeAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquid_unstake_stake_account_ix(
    keys: LiquidUnstakeStakeAccountKeys,
    args: LiquidUnstakeStakeAccountIxArgs,
) -> std::io::Result<Instruction> {
    liquid_unstake_stake_account_ix_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn liquid_unstake_stake_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeStakeAccountAccounts<'_, '_>,
    args: LiquidUnstakeStakeAccountIxArgs,
) -> ProgramResult {
    let keys: LiquidUnstakeStakeAccountKeys = accounts.into();
    let ix = liquid_unstake_stake_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquid_unstake_stake_account_invoke(
    accounts: LiquidUnstakeStakeAccountAccounts<'_, '_>,
    args: LiquidUnstakeStakeAccountIxArgs,
) -> ProgramResult {
    liquid_unstake_stake_account_invoke_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn liquid_unstake_stake_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidUnstakeStakeAccountAccounts<'_, '_>,
    args: LiquidUnstakeStakeAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidUnstakeStakeAccountKeys = accounts.into();
    let ix = liquid_unstake_stake_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquid_unstake_stake_account_invoke_signed(
    accounts: LiquidUnstakeStakeAccountAccounts<'_, '_>,
    args: LiquidUnstakeStakeAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquid_unstake_stake_account_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquid_unstake_stake_account_verify_account_keys(
    accounts: LiquidUnstakeStakeAccountAccounts<'_, '_>,
    keys: LiquidUnstakeStakeAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.user.key, keys.user),
        (*accounts.stake_account.key, keys.stake_account),
        (*accounts.stake_account_info.key, keys.stake_account_info),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.user_sol_account.key, keys.user_sol_account),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.clock.key, keys.clock),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquid_unstake_stake_account_verify_writable_privileges<'me, 'info>(
    accounts: LiquidUnstakeStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.user,
        accounts.stake_account,
        accounts.stake_account_info,
        accounts.sol_vault,
        accounts.user_sol_account,
        accounts.manager_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquid_unstake_stake_account_verify_signer_privileges<'me, 'info>(
    accounts: LiquidUnstakeStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquid_unstake_stake_account_verify_account_privileges<'me, 'info>(
    accounts: LiquidUnstakeStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquid_unstake_stake_account_verify_writable_privileges(accounts)?;
    liquid_unstake_stake_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateKeys {
    pub pool: Pubkey,
    pub sol_vault: Pubkey,
    pub stake_program: Pubkey,
    pub token_program: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateAccounts<'_, '_>> for UpdateKeys {
    fn from(accounts: UpdateAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            sol_vault: *accounts.sol_vault.key,
            stake_program: *accounts.stake_program.key,
            token_program: *accounts.token_program.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateKeys> for [AccountMeta; UPDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
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
impl From<[Pubkey; UPDATE_IX_ACCOUNTS_LEN]> for UpdateKeys {
    fn from(pubkeys: [Pubkey; UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            sol_vault: pubkeys[1],
            stake_program: pubkeys[2],
            token_program: pubkeys[3],
            clock: pubkeys[4],
            stake_history: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<UpdateAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.sol_vault.clone(),
            accounts.stake_program.clone(),
            accounts.token_program.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_IX_ACCOUNTS_LEN]>
for UpdateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            sol_vault: &arr[1],
            stake_program: &arr[2],
            token_program: &arr[3],
            clock: &arr[4],
            stake_history: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const UPDATE_IX_DISCM: [u8; 8usize] = [219, 200, 88, 176, 158, 63, 253, 127];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateIxData;
impl UpdateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateIxData.try_to_vec()?,
    })
}
pub fn update_ix(keys: UpdateKeys) -> std::io::Result<Instruction> {
    update_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys)
}
pub fn update_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateKeys = accounts.into();
    let ix = update_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_invoke(accounts: UpdateAccounts<'_, '_>) -> ProgramResult {
    update_invoke_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, accounts)
}
pub fn update_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateKeys = accounts.into();
    let ix = update_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_invoke_signed(
    accounts: UpdateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_verify_account_keys(
    accounts: UpdateAccounts<'_, '_>,
    keys: UpdateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.clock.key, keys.clock),
        (*accounts.stake_history.key, keys.stake_history),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_verify_writable_privileges<'me, 'info>(
    accounts: UpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.sol_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_verify_account_privileges<'me, 'info>(
    accounts: UpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_POOL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePoolKeys {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub manager_fee_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<UpdatePoolAccounts<'_, '_>> for UpdatePoolKeys {
    fn from(accounts: UpdatePoolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            authority: *accounts.authority.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<UpdatePoolKeys> for [AccountMeta; UPDATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_POOL_IX_ACCOUNTS_LEN]> for UpdatePoolKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            authority: pubkeys[1],
            manager_fee_account: pubkeys[2],
            system_program: pubkeys[3],
            token_program: pubkeys[4],
            rent: pubkeys[5],
        }
    }
}
impl<'info> From<UpdatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.authority.clone(),
            accounts.manager_fee_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_POOL_IX_ACCOUNTS_LEN]>
for UpdatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            authority: &arr[1],
            manager_fee_account: &arr[2],
            system_program: &arr[3],
            token_program: &arr[4],
            rent: &arr[5],
        }
    }
}
pub const UPDATE_POOL_IX_DISCM: [u8; 8usize] = [239, 214, 170, 78, 36, 35, 30, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePoolIxArgs {
    pub field_0: u32,
    pub field_1: u32,
    pub field_2: u64,
    pub field_3: u8,
    pub field_4: u64,
    pub field_5: u16,
    pub field_6: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePoolIxData(pub UpdatePoolIxArgs);
impl From<UpdatePoolIxArgs> for UpdatePoolIxData {
    fn from(args: UpdatePoolIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u32 = crate::borsh_de_or_default(&mut reader)?;
        let field_1: u32 = crate::borsh_de_or_default(&mut reader)?;
        let field_2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_3: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_6: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdatePoolIxArgs {
                field_0,
                field_1,
                field_2,
                field_3,
                field_4,
                field_5,
                field_6,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.field_6, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePoolKeys,
    args: UpdatePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_pool_ix(
    keys: UpdatePoolKeys,
    args: UpdatePoolIxArgs,
) -> std::io::Result<Instruction> {
    update_pool_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys, args)
}
pub fn update_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolAccounts<'_, '_>,
    args: UpdatePoolIxArgs,
) -> ProgramResult {
    let keys: UpdatePoolKeys = accounts.into();
    let ix = update_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pool_invoke(
    accounts: UpdatePoolAccounts<'_, '_>,
    args: UpdatePoolIxArgs,
) -> ProgramResult {
    update_pool_invoke_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, accounts, args)
}
pub fn update_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolAccounts<'_, '_>,
    args: UpdatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePoolKeys = accounts.into();
    let ix = update_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pool_invoke_signed(
    accounts: UpdatePoolAccounts<'_, '_>,
    args: UpdatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pool_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_pool_verify_account_keys(
    accounts: UpdatePoolAccounts<'_, '_>,
    keys: UpdatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.authority.key, keys.authority),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_pool_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pool_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pool_verify_account_privileges<'me, 'info>(
    accounts: UpdatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pool_verify_writable_privileges(accounts)?;
    update_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPSERT_LST_INFO_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpsertLstInfoAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub stake_pool: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpsertLstInfoKeys {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub lst_mint: Pubkey,
    pub stake_pool: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpsertLstInfoAccounts<'_, '_>> for UpsertLstInfoKeys {
    fn from(accounts: UpsertLstInfoAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            authority: *accounts.authority.key,
            lst_mint: *accounts.lst_mint.key,
            stake_pool: *accounts.stake_pool.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpsertLstInfoKeys> for [AccountMeta; UPSERT_LST_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: UpsertLstInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool,
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
impl From<[Pubkey; UPSERT_LST_INFO_IX_ACCOUNTS_LEN]> for UpsertLstInfoKeys {
    fn from(pubkeys: [Pubkey; UPSERT_LST_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            authority: pubkeys[1],
            lst_mint: pubkeys[2],
            stake_pool: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<UpsertLstInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; UPSERT_LST_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpsertLstInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.authority.clone(),
            accounts.lst_mint.clone(),
            accounts.stake_pool.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPSERT_LST_INFO_IX_ACCOUNTS_LEN]>
for UpsertLstInfoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPSERT_LST_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            authority: &arr[1],
            lst_mint: &arr[2],
            stake_pool: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const UPSERT_LST_INFO_IX_DISCM: [u8; 8usize] = [191, 203, 33, 89, 39, 85, 151, 125];
#[derive(Clone, Debug, PartialEq)]
pub struct UpsertLstInfoIxData;
impl UpsertLstInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPSERT_LST_INFO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPSERT_LST_INFO_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn upsert_lst_info_ix_with_program_id(
    program_id: Pubkey,
    keys: UpsertLstInfoKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPSERT_LST_INFO_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpsertLstInfoIxData.try_to_vec()?,
    })
}
pub fn upsert_lst_info_ix(keys: UpsertLstInfoKeys) -> std::io::Result<Instruction> {
    upsert_lst_info_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys)
}
pub fn upsert_lst_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpsertLstInfoAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpsertLstInfoKeys = accounts.into();
    let ix = upsert_lst_info_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn upsert_lst_info_invoke(accounts: UpsertLstInfoAccounts<'_, '_>) -> ProgramResult {
    upsert_lst_info_invoke_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, accounts)
}
pub fn upsert_lst_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpsertLstInfoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpsertLstInfoKeys = accounts.into();
    let ix = upsert_lst_info_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn upsert_lst_info_invoke_signed(
    accounts: UpsertLstInfoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    upsert_lst_info_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn upsert_lst_info_verify_account_keys(
    accounts: UpsertLstInfoAccounts<'_, '_>,
    keys: UpsertLstInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.authority.key, keys.authority),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.stake_pool.key, keys.stake_pool),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn upsert_lst_info_verify_writable_privileges<'me, 'info>(
    accounts: UpsertLstInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn upsert_lst_info_verify_signer_privileges<'me, 'info>(
    accounts: UpsertLstInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn upsert_lst_info_verify_account_privileges<'me, 'info>(
    accounts: UpsertLstInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    upsert_lst_info_verify_writable_privileges(accounts)?;
    upsert_lst_info_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_SOL_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawSolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_lp_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawSolKeys {
    pub pool: Pubkey,
    pub sol_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub user: Pubkey,
    pub user_lp_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawSolAccounts<'_, '_>> for WithdrawSolKeys {
    fn from(accounts: WithdrawSolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            sol_vault: *accounts.sol_vault.key,
            lp_mint: *accounts.lp_mint.key,
            user: *accounts.user.key,
            user_lp_account: *accounts.user_lp_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawSolKeys> for [AccountMeta; WITHDRAW_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawSolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
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
impl From<[Pubkey; WITHDRAW_SOL_IX_ACCOUNTS_LEN]> for WithdrawSolKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            sol_vault: pubkeys[1],
            lp_mint: pubkeys[2],
            user: pubkeys[3],
            user_lp_account: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawSolAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_SOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawSolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.sol_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.user.clone(),
            accounts.user_lp_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_SOL_IX_ACCOUNTS_LEN]>
for WithdrawSolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            sol_vault: &arr[1],
            lp_mint: &arr[2],
            user: &arr[3],
            user_lp_account: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_SOL_IX_DISCM: [u8; 8usize] = [145, 131, 74, 136, 65, 137, 42, 38];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawSolIxArgs {
    pub field_0: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawSolIxData(pub WithdrawSolIxArgs);
impl From<WithdrawSolIxArgs> for WithdrawSolIxData {
    fn from(args: WithdrawSolIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_SOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawSolIxArgs { field_0 }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_SOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawSolKeys,
    args: WithdrawSolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_SOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawSolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_sol_ix(
    keys: WithdrawSolKeys,
    args: WithdrawSolIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_sol_ix_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, keys, args)
}
pub fn withdraw_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSolAccounts<'_, '_>,
    args: WithdrawSolIxArgs,
) -> ProgramResult {
    let keys: WithdrawSolKeys = accounts.into();
    let ix = withdraw_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_sol_invoke(
    accounts: WithdrawSolAccounts<'_, '_>,
    args: WithdrawSolIxArgs,
) -> ProgramResult {
    withdraw_sol_invoke_with_program_id(VAULT_LIQUID_UNSTAKE_PROGRAM_ID, accounts, args)
}
pub fn withdraw_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSolAccounts<'_, '_>,
    args: WithdrawSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawSolKeys = accounts.into();
    let ix = withdraw_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_sol_invoke_signed(
    accounts: WithdrawSolAccounts<'_, '_>,
    args: WithdrawSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_sol_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_sol_verify_account_keys(
    accounts: WithdrawSolAccounts<'_, '_>,
    keys: WithdrawSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user.key, keys.user),
        (*accounts.user_lp_account.key, keys.user_lp_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_sol_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.sol_vault,
        accounts.lp_mint,
        accounts.user,
        accounts.user_lp_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_sol_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_sol_verify_account_privileges<'me, 'info>(
    accounts: WithdrawSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_sol_verify_writable_privileges(accounts)?;
    withdraw_sol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_STAKE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawStakeAccountAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_lp_account: &'me AccountInfo<'info>,
    pub stake_account_destination: &'me AccountInfo<'info>,
    pub stake_account_source: &'me AccountInfo<'info>,
    pub stake_account_info_source: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawStakeAccountKeys {
    pub pool: Pubkey,
    pub sol_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub user: Pubkey,
    pub user_lp_account: Pubkey,
    pub stake_account_destination: Pubkey,
    pub stake_account_source: Pubkey,
    pub stake_account_info_source: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
    pub clock: Pubkey,
}
impl From<WithdrawStakeAccountAccounts<'_, '_>> for WithdrawStakeAccountKeys {
    fn from(accounts: WithdrawStakeAccountAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            sol_vault: *accounts.sol_vault.key,
            lp_mint: *accounts.lp_mint.key,
            user: *accounts.user.key,
            user_lp_account: *accounts.user_lp_account.key,
            stake_account_destination: *accounts.stake_account_destination.key,
            stake_account_source: *accounts.stake_account_source.key,
            stake_account_info_source: *accounts.stake_account_info_source.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
            clock: *accounts.clock.key,
        }
    }
}
impl From<WithdrawStakeAccountKeys>
for [AccountMeta; WITHDRAW_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawStakeAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account_destination,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account_info_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
for WithdrawStakeAccountKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            sol_vault: pubkeys[1],
            lp_mint: pubkeys[2],
            user: pubkeys[3],
            user_lp_account: pubkeys[4],
            stake_account_destination: pubkeys[5],
            stake_account_source: pubkeys[6],
            stake_account_info_source: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            stake_program: pubkeys[10],
            clock: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawStakeAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawStakeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.sol_vault.clone(),
            accounts.lp_mint.clone(),
            accounts.user.clone(),
            accounts.user_lp_account.clone(),
            accounts.stake_account_destination.clone(),
            accounts.stake_account_source.clone(),
            accounts.stake_account_info_source.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
            accounts.clock.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
for WithdrawStakeAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_STAKE_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            sol_vault: &arr[1],
            lp_mint: &arr[2],
            user: &arr[3],
            user_lp_account: &arr[4],
            stake_account_destination: &arr[5],
            stake_account_source: &arr[6],
            stake_account_info_source: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            stake_program: &arr[10],
            clock: &arr[11],
        }
    }
}
pub const WITHDRAW_STAKE_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    211, 85, 184, 65, 183, 177, 233, 217,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawStakeAccountIxArgs {
    pub field_0: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawStakeAccountIxData(pub WithdrawStakeAccountIxArgs);
impl From<WithdrawStakeAccountIxArgs> for WithdrawStakeAccountIxData {
    fn from(args: WithdrawStakeAccountIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawStakeAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_STAKE_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawStakeAccountIxArgs {
                field_0,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_STAKE_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field_0, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_stake_account_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawStakeAccountKeys,
    args: WithdrawStakeAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawStakeAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_stake_account_ix(
    keys: WithdrawStakeAccountKeys,
    args: WithdrawStakeAccountIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_stake_account_ix_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn withdraw_stake_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawStakeAccountAccounts<'_, '_>,
    args: WithdrawStakeAccountIxArgs,
) -> ProgramResult {
    let keys: WithdrawStakeAccountKeys = accounts.into();
    let ix = withdraw_stake_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_stake_account_invoke(
    accounts: WithdrawStakeAccountAccounts<'_, '_>,
    args: WithdrawStakeAccountIxArgs,
) -> ProgramResult {
    withdraw_stake_account_invoke_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_stake_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawStakeAccountAccounts<'_, '_>,
    args: WithdrawStakeAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawStakeAccountKeys = accounts.into();
    let ix = withdraw_stake_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_stake_account_invoke_signed(
    accounts: WithdrawStakeAccountAccounts<'_, '_>,
    args: WithdrawStakeAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_stake_account_invoke_signed_with_program_id(
        VAULT_LIQUID_UNSTAKE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_stake_account_verify_account_keys(
    accounts: WithdrawStakeAccountAccounts<'_, '_>,
    keys: WithdrawStakeAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user.key, keys.user),
        (*accounts.user_lp_account.key, keys.user_lp_account),
        (*accounts.stake_account_destination.key, keys.stake_account_destination),
        (*accounts.stake_account_source.key, keys.stake_account_source),
        (*accounts.stake_account_info_source.key, keys.stake_account_info_source),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.clock.key, keys.clock),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_stake_account_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.sol_vault,
        accounts.lp_mint,
        accounts.user,
        accounts.user_lp_account,
        accounts.stake_account_destination,
        accounts.stake_account_source,
        accounts.stake_account_info_source,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_stake_account_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user, accounts.stake_account_destination] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_stake_account_verify_account_privileges<'me, 'info>(
    accounts: WithdrawStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_stake_account_verify_writable_privileges(accounts)?;
    withdraw_stake_account_verify_signer_privileges(accounts)?;
    Ok(())
}
