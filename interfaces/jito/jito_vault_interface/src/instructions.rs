use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum JitoVaultProgramIx {
    InitializeConfig(InitializeConfigIxArgs),
    InitializeVault(InitializeVaultIxArgs),
    InitializeVaultWithMint,
    InitializeVaultOperatorDelegation,
    InitializeVaultNcnTicket,
    InitializeVaultNcnSlasherOperatorTicket,
    InitializeVaultNcnSlasherTicket,
    WarmupVaultNcnTicket,
    CooldownVaultNcnTicket,
    WarmupVaultNcnSlasherTicket,
    CooldownVaultNcnSlasherTicket,
    MintTo(MintToIxArgs),
    EnqueueWithdrawal(EnqueueWithdrawalIxArgs),
    ChangeWithdrawalTicketOwner,
    BurnWithdrawalTicket,
    SetDepositCapacity(SetDepositCapacityIxArgs),
    SetFees(SetFeesIxArgs),
    SetProgramFee(SetProgramFeeIxArgs),
    SetProgramFeeWallet,
    SetIsPaused(SetIsPausedIxArgs),
    DelegateTokenAccount,
    RevokeDelegateTokenAccount,
    SetAdmin,
    SetSecondaryAdmin(SetSecondaryAdminIxArgs),
    AddDelegation(AddDelegationIxArgs),
    CooldownDelegation(CooldownDelegationIxArgs),
    UpdateVaultBalance,
    InitializeVaultUpdateStateTracker(InitializeVaultUpdateStateTrackerIxArgs),
    CrankVaultUpdateStateTracker,
    CloseVaultUpdateStateTracker(CloseVaultUpdateStateTrackerIxArgs),
    CreateTokenMetadata(CreateTokenMetadataIxArgs),
    UpdateTokenMetadata(UpdateTokenMetadataIxArgs),
    SetConfigAdmin,
    SetConfigSecondaryAdmin(SetConfigSecondaryAdminIxArgs),
}
impl JitoVaultProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            INITIALIZE_CONFIG_IX_DISCM => {
                Ok(
                    Self::InitializeConfig(
                        InitializeConfigIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_VAULT_IX_DISCM => {
                Ok(
                    Self::InitializeVault(
                        InitializeVaultIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_VAULT_WITH_MINT_IX_DISCM => Ok(Self::InitializeVaultWithMint),
            INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_DISCM => {
                Ok(Self::InitializeVaultOperatorDelegation)
            }
            INITIALIZE_VAULT_NCN_TICKET_IX_DISCM => Ok(Self::InitializeVaultNcnTicket),
            INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_DISCM => {
                Ok(Self::InitializeVaultNcnSlasherOperatorTicket)
            }
            INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_DISCM => {
                Ok(Self::InitializeVaultNcnSlasherTicket)
            }
            WARMUP_VAULT_NCN_TICKET_IX_DISCM => Ok(Self::WarmupVaultNcnTicket),
            COOLDOWN_VAULT_NCN_TICKET_IX_DISCM => Ok(Self::CooldownVaultNcnTicket),
            WARMUP_VAULT_NCN_SLASHER_TICKET_IX_DISCM => {
                Ok(Self::WarmupVaultNcnSlasherTicket)
            }
            COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_DISCM => {
                Ok(Self::CooldownVaultNcnSlasherTicket)
            }
            MINT_TO_IX_DISCM => Ok(Self::MintTo(MintToIxArgs::deserialize(&mut reader)?)),
            ENQUEUE_WITHDRAWAL_IX_DISCM => {
                Ok(
                    Self::EnqueueWithdrawal(
                        EnqueueWithdrawalIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CHANGE_WITHDRAWAL_TICKET_OWNER_IX_DISCM => {
                Ok(Self::ChangeWithdrawalTicketOwner)
            }
            BURN_WITHDRAWAL_TICKET_IX_DISCM => Ok(Self::BurnWithdrawalTicket),
            SET_DEPOSIT_CAPACITY_IX_DISCM => {
                Ok(
                    Self::SetDepositCapacity(
                        SetDepositCapacityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_FEES_IX_DISCM => {
                Ok(Self::SetFees(SetFeesIxArgs::deserialize(&mut reader)?))
            }
            SET_PROGRAM_FEE_IX_DISCM => {
                Ok(Self::SetProgramFee(SetProgramFeeIxArgs::deserialize(&mut reader)?))
            }
            SET_PROGRAM_FEE_WALLET_IX_DISCM => Ok(Self::SetProgramFeeWallet),
            SET_IS_PAUSED_IX_DISCM => {
                Ok(Self::SetIsPaused(SetIsPausedIxArgs::deserialize(&mut reader)?))
            }
            DELEGATE_TOKEN_ACCOUNT_IX_DISCM => Ok(Self::DelegateTokenAccount),
            REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_DISCM => {
                Ok(Self::RevokeDelegateTokenAccount)
            }
            SET_ADMIN_IX_DISCM => Ok(Self::SetAdmin),
            SET_SECONDARY_ADMIN_IX_DISCM => {
                Ok(
                    Self::SetSecondaryAdmin(
                        SetSecondaryAdminIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            ADD_DELEGATION_IX_DISCM => {
                Ok(Self::AddDelegation(AddDelegationIxArgs::deserialize(&mut reader)?))
            }
            COOLDOWN_DELEGATION_IX_DISCM => {
                Ok(
                    Self::CooldownDelegation(
                        CooldownDelegationIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_VAULT_BALANCE_IX_DISCM => Ok(Self::UpdateVaultBalance),
            INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM => {
                Ok(
                    Self::InitializeVaultUpdateStateTracker(
                        InitializeVaultUpdateStateTrackerIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            CRANK_VAULT_UPDATE_STATE_TRACKER_IX_DISCM => {
                Ok(Self::CrankVaultUpdateStateTracker)
            }
            CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM => {
                Ok(
                    Self::CloseVaultUpdateStateTracker(
                        CloseVaultUpdateStateTrackerIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CREATE_TOKEN_METADATA_IX_DISCM => {
                Ok(
                    Self::CreateTokenMetadata(
                        CreateTokenMetadataIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_TOKEN_METADATA_IX_DISCM => {
                Ok(
                    Self::UpdateTokenMetadata(
                        UpdateTokenMetadataIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_CONFIG_ADMIN_IX_DISCM => Ok(Self::SetConfigAdmin),
            SET_CONFIG_SECONDARY_ADMIN_IX_DISCM => {
                Ok(
                    Self::SetConfigSecondaryAdmin(
                        SetConfigSecondaryAdminIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeConfig(args) => {
                writer.write_all(&[INITIALIZE_CONFIG_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeVault(args) => {
                writer.write_all(&[INITIALIZE_VAULT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeVaultWithMint => {
                writer.write_all(&[INITIALIZE_VAULT_WITH_MINT_IX_DISCM])
            }
            Self::InitializeVaultOperatorDelegation => {
                writer.write_all(&[INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_DISCM])
            }
            Self::InitializeVaultNcnTicket => {
                writer.write_all(&[INITIALIZE_VAULT_NCN_TICKET_IX_DISCM])
            }
            Self::InitializeVaultNcnSlasherOperatorTicket => {
                writer
                    .write_all(&[INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_DISCM])
            }
            Self::InitializeVaultNcnSlasherTicket => {
                writer.write_all(&[INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_DISCM])
            }
            Self::WarmupVaultNcnTicket => {
                writer.write_all(&[WARMUP_VAULT_NCN_TICKET_IX_DISCM])
            }
            Self::CooldownVaultNcnTicket => {
                writer.write_all(&[COOLDOWN_VAULT_NCN_TICKET_IX_DISCM])
            }
            Self::WarmupVaultNcnSlasherTicket => {
                writer.write_all(&[WARMUP_VAULT_NCN_SLASHER_TICKET_IX_DISCM])
            }
            Self::CooldownVaultNcnSlasherTicket => {
                writer.write_all(&[COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_DISCM])
            }
            Self::MintTo(args) => {
                writer.write_all(&[MINT_TO_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::EnqueueWithdrawal(args) => {
                writer.write_all(&[ENQUEUE_WITHDRAWAL_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::ChangeWithdrawalTicketOwner => {
                writer.write_all(&[CHANGE_WITHDRAWAL_TICKET_OWNER_IX_DISCM])
            }
            Self::BurnWithdrawalTicket => {
                writer.write_all(&[BURN_WITHDRAWAL_TICKET_IX_DISCM])
            }
            Self::SetDepositCapacity(args) => {
                writer.write_all(&[SET_DEPOSIT_CAPACITY_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SetFees(args) => {
                writer.write_all(&[SET_FEES_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SetProgramFee(args) => {
                writer.write_all(&[SET_PROGRAM_FEE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SetProgramFeeWallet => {
                writer.write_all(&[SET_PROGRAM_FEE_WALLET_IX_DISCM])
            }
            Self::SetIsPaused(args) => {
                writer.write_all(&[SET_IS_PAUSED_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::DelegateTokenAccount => {
                writer.write_all(&[DELEGATE_TOKEN_ACCOUNT_IX_DISCM])
            }
            Self::RevokeDelegateTokenAccount => {
                writer.write_all(&[REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_DISCM])
            }
            Self::SetAdmin => writer.write_all(&[SET_ADMIN_IX_DISCM]),
            Self::SetSecondaryAdmin(args) => {
                writer.write_all(&[SET_SECONDARY_ADMIN_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::AddDelegation(args) => {
                writer.write_all(&[ADD_DELEGATION_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CooldownDelegation(args) => {
                writer.write_all(&[COOLDOWN_DELEGATION_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::UpdateVaultBalance => {
                writer.write_all(&[UPDATE_VAULT_BALANCE_IX_DISCM])
            }
            Self::InitializeVaultUpdateStateTracker(args) => {
                writer.write_all(&[INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CrankVaultUpdateStateTracker => {
                writer.write_all(&[CRANK_VAULT_UPDATE_STATE_TRACKER_IX_DISCM])
            }
            Self::CloseVaultUpdateStateTracker(args) => {
                writer.write_all(&[CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CreateTokenMetadata(args) => {
                writer.write_all(&[CREATE_TOKEN_METADATA_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::UpdateTokenMetadata(args) => {
                writer.write_all(&[UPDATE_TOKEN_METADATA_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SetConfigAdmin => writer.write_all(&[SET_CONFIG_ADMIN_IX_DISCM]),
            Self::SetConfigSecondaryAdmin(args) => {
                writer.write_all(&[SET_CONFIG_SECONDARY_ADMIN_IX_DISCM])?;
                args.serialize(&mut writer)
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
pub const INITIALIZE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub restaking_program: &'me AccountInfo<'info>,
    pub program_fee_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub restaking_program: Pubkey,
    pub program_fee_wallet: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeConfigAccounts<'_, '_>> for InitializeConfigKeys {
    fn from(accounts: InitializeConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            restaking_program: *accounts.restaking_program.key,
            program_fee_wallet: *accounts.program_fee_wallet.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeConfigKeys> for [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.restaking_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_fee_wallet,
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
impl From<[Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]> for InitializeConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            restaking_program: pubkeys[2],
            program_fee_wallet: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.admin.clone(),
            accounts.restaking_program.clone(),
            accounts.program_fee_wallet.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            restaking_program: &arr[2],
            program_fee_wallet: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INITIALIZE_CONFIG_IX_DISCM: u8 = 0u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct InitializeConfigIxArgs {
    pub program_fee_bps: u16,
}
impl InitializeConfigIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let program_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { program_fee_bps })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfigIxData(pub InitializeConfigIxArgs);
impl From<InitializeConfigIxArgs> for InitializeConfigIxData {
    fn from(args: InitializeConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(InitializeConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_CONFIG_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_config_ix(
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_config_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn initialize_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_config_invoke(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    initialize_config_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn initialize_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_config_invoke_signed(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_config_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_config_verify_account_keys(
    accounts: InitializeConfigAccounts<'_, '_>,
    keys: InitializeConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.admin.key, &keys.admin),
        (accounts.restaking_program.key, &keys.restaking_program),
        (accounts.program_fee_wallet.key, &keys.program_fee_wallet),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_config_verify_writable_privileges(accounts)?;
    initialize_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vrt_mint: &'me AccountInfo<'info>,
    pub st_mint: &'me AccountInfo<'info>,
    pub admin_st_token_account: &'me AccountInfo<'info>,
    pub vault_st_token_account: &'me AccountInfo<'info>,
    pub burn_vault: &'me AccountInfo<'info>,
    pub burn_vault_vrt_token_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub vrt_mint: Pubkey,
    pub st_mint: Pubkey,
    pub admin_st_token_account: Pubkey,
    pub vault_st_token_account: Pubkey,
    pub burn_vault: Pubkey,
    pub burn_vault_vrt_token_account: Pubkey,
    pub admin: Pubkey,
    pub base: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<InitializeVaultAccounts<'_, '_>> for InitializeVaultKeys {
    fn from(accounts: InitializeVaultAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            vrt_mint: *accounts.vrt_mint.key,
            st_mint: *accounts.st_mint.key,
            admin_st_token_account: *accounts.admin_st_token_account.key,
            vault_st_token_account: *accounts.vault_st_token_account.key,
            burn_vault: *accounts.burn_vault.key,
            burn_vault_vrt_token_account: *accounts.burn_vault_vrt_token_account.key,
            admin: *accounts.admin.key,
            base: *accounts.base.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<InitializeVaultKeys> for [AccountMeta; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vrt_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.st_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin_st_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_st_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.burn_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.burn_vault_vrt_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]> for InitializeVaultKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            vrt_mint: pubkeys[2],
            st_mint: pubkeys[3],
            admin_st_token_account: pubkeys[4],
            vault_st_token_account: pubkeys[5],
            burn_vault: pubkeys[6],
            burn_vault_vrt_token_account: pubkeys[7],
            admin: pubkeys[8],
            base: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<InitializeVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.vrt_mint.clone(),
            accounts.st_mint.clone(),
            accounts.admin_st_token_account.clone(),
            accounts.vault_st_token_account.clone(),
            accounts.burn_vault.clone(),
            accounts.burn_vault_vrt_token_account.clone(),
            accounts.admin.clone(),
            accounts.base.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]>
for InitializeVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            vrt_mint: &arr[2],
            st_mint: &arr[3],
            admin_st_token_account: &arr[4],
            vault_st_token_account: &arr[5],
            burn_vault: &arr[6],
            burn_vault_vrt_token_account: &arr[7],
            admin: &arr[8],
            base: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
        }
    }
}
pub const INITIALIZE_VAULT_IX_DISCM: u8 = 1u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct InitializeVaultIxArgs {
    pub deposit_fee_bps: u16,
    pub withdrawal_fee_bps: u16,
    pub reward_fee_bps: u16,
    pub decimals: u8,
    pub initialize_token_amount: u64,
}
impl InitializeVaultIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let deposit_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reward_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let initialize_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            deposit_fee_bps,
            withdrawal_fee_bps,
            reward_fee_bps,
            decimals,
            initialize_token_amount,
        })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultIxData(pub InitializeVaultIxArgs);
impl From<InitializeVaultIxArgs> for InitializeVaultIxData {
    fn from(args: InitializeVaultIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(InitializeVaultIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_VAULT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultKeys,
    args: InitializeVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_vault_ix(
    keys: InitializeVaultKeys,
    args: InitializeVaultIxArgs,
) -> std::io::Result<Instruction> {
    initialize_vault_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn initialize_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultAccounts<'_, '_>,
    args: InitializeVaultIxArgs,
) -> ProgramResult {
    let keys: InitializeVaultKeys = accounts.into();
    let ix = initialize_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_invoke(
    accounts: InitializeVaultAccounts<'_, '_>,
    args: InitializeVaultIxArgs,
) -> ProgramResult {
    initialize_vault_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn initialize_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultAccounts<'_, '_>,
    args: InitializeVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultKeys = accounts.into();
    let ix = initialize_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_invoke_signed(
    accounts: InitializeVaultAccounts<'_, '_>,
    args: InitializeVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_vault_verify_account_keys(
    accounts: InitializeVaultAccounts<'_, '_>,
    keys: InitializeVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.vrt_mint.key, &keys.vrt_mint),
        (accounts.st_mint.key, &keys.st_mint),
        (accounts.admin_st_token_account.key, &keys.admin_st_token_account),
        (accounts.vault_st_token_account.key, &keys.vault_st_token_account),
        (accounts.burn_vault.key, &keys.burn_vault),
        (accounts.burn_vault_vrt_token_account.key, &keys.burn_vault_vrt_token_account),
        (accounts.admin.key, &keys.admin),
        (accounts.base.key, &keys.base),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
        (accounts.associated_token_program.key, &keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.config,
        accounts.vault,
        accounts.vrt_mint,
        accounts.admin_st_token_account,
        accounts.vault_st_token_account,
        accounts.burn_vault_vrt_token_account,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vrt_mint, accounts.admin, accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_verify_writable_privileges(accounts)?;
    initialize_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_WITH_MINT_IX_DISCM: u8 = 2u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultWithMintIxData;
impl InitializeVaultWithMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_VAULT_WITH_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_VAULT_WITH_MINT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_with_mint_ix_with_program_id(
    program_id: Pubkey,
) -> std::io::Result<Instruction> {
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: InitializeVaultWithMintIxData.try_to_vec()?,
    })
}
pub fn initialize_vault_with_mint_ix() -> std::io::Result<Instruction> {
    initialize_vault_with_mint_ix_with_program_id(JITO_VAULT_PROGRAM_ID)
}
pub fn initialize_vault_with_mint_invoke_with_program_id(
    program_id: Pubkey,
) -> ProgramResult {
    let ix = initialize_vault_with_mint_ix_with_program_id(program_id)?;
    invoke(&ix, &[])
}
pub fn initialize_vault_with_mint_invoke() -> ProgramResult {
    initialize_vault_with_mint_invoke_with_program_id(JITO_VAULT_PROGRAM_ID)
}
pub fn initialize_vault_with_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_vault_with_mint_ix_with_program_id(program_id)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn initialize_vault_with_mint_invoke_signed(seeds: &[&[&[u8]]]) -> ProgramResult {
    initialize_vault_with_mint_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        seeds,
    )
}
pub const INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultOperatorDelegationAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub operator_vault_ticket: &'me AccountInfo<'info>,
    pub vault_operator_delegation: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultOperatorDelegationKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub operator: Pubkey,
    pub operator_vault_ticket: Pubkey,
    pub vault_operator_delegation: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeVaultOperatorDelegationAccounts<'_, '_>>
for InitializeVaultOperatorDelegationKeys {
    fn from(accounts: InitializeVaultOperatorDelegationAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            operator: *accounts.operator.key,
            operator_vault_ticket: *accounts.operator_vault_ticket.key,
            vault_operator_delegation: *accounts.vault_operator_delegation.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeVaultOperatorDelegationKeys>
for [AccountMeta; INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultOperatorDelegationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator_vault_ticket,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_operator_delegation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_ACCOUNTS_LEN]>
for InitializeVaultOperatorDelegationKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            operator: pubkeys[2],
            operator_vault_ticket: pubkeys[3],
            vault_operator_delegation: pubkeys[4],
            admin: pubkeys[5],
            payer: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeVaultOperatorDelegationAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultOperatorDelegationAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.operator.clone(),
            accounts.operator_vault_ticket.clone(),
            accounts.vault_operator_delegation.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_ACCOUNTS_LEN]>
for InitializeVaultOperatorDelegationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            operator: &arr[2],
            operator_vault_ticket: &arr[3],
            vault_operator_delegation: &arr[4],
            admin: &arr[5],
            payer: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_DISCM: u8 = 3u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultOperatorDelegationIxData;
impl InitializeVaultOperatorDelegationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_operator_delegation_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultOperatorDelegationKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_OPERATOR_DELEGATION_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeVaultOperatorDelegationIxData.try_to_vec()?,
    })
}
pub fn initialize_vault_operator_delegation_ix(
    keys: InitializeVaultOperatorDelegationKeys,
) -> std::io::Result<Instruction> {
    initialize_vault_operator_delegation_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn initialize_vault_operator_delegation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultOperatorDelegationAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeVaultOperatorDelegationKeys = accounts.into();
    let ix = initialize_vault_operator_delegation_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_operator_delegation_invoke(
    accounts: InitializeVaultOperatorDelegationAccounts<'_, '_>,
) -> ProgramResult {
    initialize_vault_operator_delegation_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_vault_operator_delegation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultOperatorDelegationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultOperatorDelegationKeys = accounts.into();
    let ix = initialize_vault_operator_delegation_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_operator_delegation_invoke_signed(
    accounts: InitializeVaultOperatorDelegationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_operator_delegation_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_vault_operator_delegation_verify_account_keys(
    accounts: InitializeVaultOperatorDelegationAccounts<'_, '_>,
    keys: InitializeVaultOperatorDelegationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.operator.key, &keys.operator),
        (accounts.operator_vault_ticket.key, &keys.operator_vault_ticket),
        (accounts.vault_operator_delegation.key, &keys.vault_operator_delegation),
        (accounts.admin.key, &keys.admin),
        (accounts.payer.key, &keys.payer),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_operator_delegation_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultOperatorDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.operator,
        accounts.vault_operator_delegation,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_operator_delegation_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultOperatorDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_operator_delegation_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultOperatorDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_operator_delegation_verify_writable_privileges(accounts)?;
    initialize_vault_operator_delegation_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultNcnTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub ncn_vault_ticket: &'me AccountInfo<'info>,
    pub vault_ncn_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultNcnTicketKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub ncn_vault_ticket: Pubkey,
    pub vault_ncn_ticket: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeVaultNcnTicketAccounts<'_, '_>> for InitializeVaultNcnTicketKeys {
    fn from(accounts: InitializeVaultNcnTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            ncn: *accounts.ncn.key,
            ncn_vault_ticket: *accounts.ncn_vault_ticket.key,
            vault_ncn_ticket: *accounts.vault_ncn_ticket.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeVaultNcnTicketKeys>
for [AccountMeta; INITIALIZE_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultNcnTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_ticket,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_ncn_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]>
for InitializeVaultNcnTicketKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            ncn: pubkeys[2],
            ncn_vault_ticket: pubkeys[3],
            vault_ncn_ticket: pubkeys[4],
            admin: pubkeys[5],
            payer: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeVaultNcnTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultNcnTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.ncn.clone(),
            accounts.ncn_vault_ticket.clone(),
            accounts.vault_ncn_ticket.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]>
for InitializeVaultNcnTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            ncn: &arr[2],
            ncn_vault_ticket: &arr[3],
            vault_ncn_ticket: &arr[4],
            admin: &arr[5],
            payer: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const INITIALIZE_VAULT_NCN_TICKET_IX_DISCM: u8 = 4u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultNcnTicketIxData;
impl InitializeVaultNcnTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_VAULT_NCN_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_VAULT_NCN_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_ncn_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultNcnTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeVaultNcnTicketIxData.try_to_vec()?,
    })
}
pub fn initialize_vault_ncn_ticket_ix(
    keys: InitializeVaultNcnTicketKeys,
) -> std::io::Result<Instruction> {
    initialize_vault_ncn_ticket_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn initialize_vault_ncn_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultNcnTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeVaultNcnTicketKeys = accounts.into();
    let ix = initialize_vault_ncn_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_ncn_ticket_invoke(
    accounts: InitializeVaultNcnTicketAccounts<'_, '_>,
) -> ProgramResult {
    initialize_vault_ncn_ticket_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn initialize_vault_ncn_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultNcnTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultNcnTicketKeys = accounts.into();
    let ix = initialize_vault_ncn_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_ncn_ticket_invoke_signed(
    accounts: InitializeVaultNcnTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_ncn_ticket_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_vault_ncn_ticket_verify_account_keys(
    accounts: InitializeVaultNcnTicketAccounts<'_, '_>,
    keys: InitializeVaultNcnTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn.key, &keys.ncn),
        (accounts.ncn_vault_ticket.key, &keys.ncn_vault_ticket),
        (accounts.vault_ncn_ticket.key, &keys.vault_ncn_ticket),
        (accounts.admin.key, &keys.admin),
        (accounts.payer.key, &keys.payer),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_ticket_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_ncn_ticket,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_ticket_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_ticket_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_ncn_ticket_verify_writable_privileges(accounts)?;
    initialize_vault_ncn_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultNcnSlasherOperatorTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub slasher: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault_ncn_slasher_ticket: &'me AccountInfo<'info>,
    pub vault_ncn_slasher_operator_ticket: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultNcnSlasherOperatorTicketKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub slasher: Pubkey,
    pub operator: Pubkey,
    pub vault_ncn_slasher_ticket: Pubkey,
    pub vault_ncn_slasher_operator_ticket: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeVaultNcnSlasherOperatorTicketAccounts<'_, '_>>
for InitializeVaultNcnSlasherOperatorTicketKeys {
    fn from(accounts: InitializeVaultNcnSlasherOperatorTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            ncn: *accounts.ncn.key,
            slasher: *accounts.slasher.key,
            operator: *accounts.operator.key,
            vault_ncn_slasher_ticket: *accounts.vault_ncn_slasher_ticket.key,
            vault_ncn_slasher_operator_ticket: *accounts
                .vault_ncn_slasher_operator_ticket
                .key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeVaultNcnSlasherOperatorTicketKeys>
for [AccountMeta; INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultNcnSlasherOperatorTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slasher,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_ncn_slasher_ticket,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_ncn_slasher_operator_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_ACCOUNTS_LEN]>
for InitializeVaultNcnSlasherOperatorTicketKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            ncn: pubkeys[2],
            slasher: pubkeys[3],
            operator: pubkeys[4],
            vault_ncn_slasher_ticket: pubkeys[5],
            vault_ncn_slasher_operator_ticket: pubkeys[6],
            payer: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeVaultNcnSlasherOperatorTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.ncn.clone(),
            accounts.slasher.clone(),
            accounts.operator.clone(),
            accounts.vault_ncn_slasher_ticket.clone(),
            accounts.vault_ncn_slasher_operator_ticket.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_ACCOUNTS_LEN],
> for InitializeVaultNcnSlasherOperatorTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            ncn: &arr[2],
            slasher: &arr[3],
            operator: &arr[4],
            vault_ncn_slasher_ticket: &arr[5],
            vault_ncn_slasher_operator_ticket: &arr[6],
            payer: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_DISCM: u8 = 5u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultNcnSlasherOperatorTicketIxData;
impl InitializeVaultNcnSlasherOperatorTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_ncn_slasher_operator_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultNcnSlasherOperatorTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_NCN_SLASHER_OPERATOR_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeVaultNcnSlasherOperatorTicketIxData.try_to_vec()?,
    })
}
pub fn initialize_vault_ncn_slasher_operator_ticket_ix(
    keys: InitializeVaultNcnSlasherOperatorTicketKeys,
) -> std::io::Result<Instruction> {
    initialize_vault_ncn_slasher_operator_ticket_ix_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        keys,
    )
}
pub fn initialize_vault_ncn_slasher_operator_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeVaultNcnSlasherOperatorTicketKeys = accounts.into();
    let ix = initialize_vault_ncn_slasher_operator_ticket_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_ncn_slasher_operator_ticket_invoke(
    accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'_, '_>,
) -> ProgramResult {
    initialize_vault_ncn_slasher_operator_ticket_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_vault_ncn_slasher_operator_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultNcnSlasherOperatorTicketKeys = accounts.into();
    let ix = initialize_vault_ncn_slasher_operator_ticket_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_ncn_slasher_operator_ticket_invoke_signed(
    accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_ncn_slasher_operator_ticket_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_vault_ncn_slasher_operator_ticket_verify_account_keys(
    accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'_, '_>,
    keys: InitializeVaultNcnSlasherOperatorTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn.key, &keys.ncn),
        (accounts.slasher.key, &keys.slasher),
        (accounts.operator.key, &keys.operator),
        (accounts.vault_ncn_slasher_ticket.key, &keys.vault_ncn_slasher_ticket),
        (
            accounts.vault_ncn_slasher_operator_ticket.key,
            &keys.vault_ncn_slasher_operator_ticket,
        ),
        (accounts.payer.key, &keys.payer),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_slasher_operator_ticket_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_ncn_slasher_operator_ticket,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_slasher_operator_ticket_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_slasher_operator_ticket_verify_account_privileges<
    'me,
    'info,
>(
    accounts: InitializeVaultNcnSlasherOperatorTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_ncn_slasher_operator_ticket_verify_writable_privileges(accounts)?;
    initialize_vault_ncn_slasher_operator_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultNcnSlasherTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub slasher: &'me AccountInfo<'info>,
    pub ncn_slasher_ticket: &'me AccountInfo<'info>,
    pub vault_slasher_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultNcnSlasherTicketKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub slasher: Pubkey,
    pub ncn_slasher_ticket: Pubkey,
    pub vault_slasher_ticket: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeVaultNcnSlasherTicketAccounts<'_, '_>>
for InitializeVaultNcnSlasherTicketKeys {
    fn from(accounts: InitializeVaultNcnSlasherTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            ncn: *accounts.ncn.key,
            slasher: *accounts.slasher.key,
            ncn_slasher_ticket: *accounts.ncn_slasher_ticket.key,
            vault_slasher_ticket: *accounts.vault_slasher_ticket.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeVaultNcnSlasherTicketKeys>
for [AccountMeta; INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultNcnSlasherTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slasher,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_slasher_ticket,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_slasher_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for InitializeVaultNcnSlasherTicketKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            ncn: pubkeys[2],
            slasher: pubkeys[3],
            ncn_slasher_ticket: pubkeys[4],
            vault_slasher_ticket: pubkeys[5],
            admin: pubkeys[6],
            payer: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeVaultNcnSlasherTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultNcnSlasherTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.ncn.clone(),
            accounts.slasher.clone(),
            accounts.ncn_slasher_ticket.clone(),
            accounts.vault_slasher_ticket.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for InitializeVaultNcnSlasherTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            ncn: &arr[2],
            slasher: &arr[3],
            ncn_slasher_ticket: &arr[4],
            vault_slasher_ticket: &arr[5],
            admin: &arr[6],
            payer: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_DISCM: u8 = 6u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultNcnSlasherTicketIxData;
impl InitializeVaultNcnSlasherTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_ncn_slasher_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultNcnSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeVaultNcnSlasherTicketIxData.try_to_vec()?,
    })
}
pub fn initialize_vault_ncn_slasher_ticket_ix(
    keys: InitializeVaultNcnSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    initialize_vault_ncn_slasher_ticket_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn initialize_vault_ncn_slasher_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultNcnSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeVaultNcnSlasherTicketKeys = accounts.into();
    let ix = initialize_vault_ncn_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_ncn_slasher_ticket_invoke(
    accounts: InitializeVaultNcnSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    initialize_vault_ncn_slasher_ticket_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_vault_ncn_slasher_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultNcnSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultNcnSlasherTicketKeys = accounts.into();
    let ix = initialize_vault_ncn_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_ncn_slasher_ticket_invoke_signed(
    accounts: InitializeVaultNcnSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_ncn_slasher_ticket_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_vault_ncn_slasher_ticket_verify_account_keys(
    accounts: InitializeVaultNcnSlasherTicketAccounts<'_, '_>,
    keys: InitializeVaultNcnSlasherTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn.key, &keys.ncn),
        (accounts.slasher.key, &keys.slasher),
        (accounts.ncn_slasher_ticket.key, &keys.ncn_slasher_ticket),
        (accounts.vault_slasher_ticket.key, &keys.vault_slasher_ticket),
        (accounts.admin.key, &keys.admin),
        (accounts.payer.key, &keys.payer),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_slasher_ticket_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault_slasher_ticket, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_slasher_ticket_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_ncn_slasher_ticket_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_ncn_slasher_ticket_verify_writable_privileges(accounts)?;
    initialize_vault_ncn_slasher_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WARMUP_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct WarmupVaultNcnTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub vault_ncn_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WarmupVaultNcnTicketKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub vault_ncn_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<WarmupVaultNcnTicketAccounts<'_, '_>> for WarmupVaultNcnTicketKeys {
    fn from(accounts: WarmupVaultNcnTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            ncn: *accounts.ncn.key,
            vault_ncn_ticket: *accounts.vault_ncn_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<WarmupVaultNcnTicketKeys>
for [AccountMeta; WARMUP_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: WarmupVaultNcnTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_ncn_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WARMUP_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]>
for WarmupVaultNcnTicketKeys {
    fn from(pubkeys: [Pubkey; WARMUP_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            ncn: pubkeys[2],
            vault_ncn_ticket: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<WarmupVaultNcnTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; WARMUP_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: WarmupVaultNcnTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.ncn.clone(),
            accounts.vault_ncn_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WARMUP_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]>
for WarmupVaultNcnTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WARMUP_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            ncn: &arr[2],
            vault_ncn_ticket: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const WARMUP_VAULT_NCN_TICKET_IX_DISCM: u8 = 7u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WarmupVaultNcnTicketIxData;
impl WarmupVaultNcnTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WARMUP_VAULT_NCN_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WARMUP_VAULT_NCN_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn warmup_vault_ncn_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: WarmupVaultNcnTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WARMUP_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WarmupVaultNcnTicketIxData.try_to_vec()?,
    })
}
pub fn warmup_vault_ncn_ticket_ix(
    keys: WarmupVaultNcnTicketKeys,
) -> std::io::Result<Instruction> {
    warmup_vault_ncn_ticket_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn warmup_vault_ncn_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WarmupVaultNcnTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WarmupVaultNcnTicketKeys = accounts.into();
    let ix = warmup_vault_ncn_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn warmup_vault_ncn_ticket_invoke(
    accounts: WarmupVaultNcnTicketAccounts<'_, '_>,
) -> ProgramResult {
    warmup_vault_ncn_ticket_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn warmup_vault_ncn_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WarmupVaultNcnTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WarmupVaultNcnTicketKeys = accounts.into();
    let ix = warmup_vault_ncn_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn warmup_vault_ncn_ticket_invoke_signed(
    accounts: WarmupVaultNcnTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    warmup_vault_ncn_ticket_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn warmup_vault_ncn_ticket_verify_account_keys(
    accounts: WarmupVaultNcnTicketAccounts<'_, '_>,
    keys: WarmupVaultNcnTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn.key, &keys.ncn),
        (accounts.vault_ncn_ticket.key, &keys.vault_ncn_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn warmup_vault_ncn_ticket_verify_writable_privileges<'me, 'info>(
    accounts: WarmupVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.vault_ncn_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn warmup_vault_ncn_ticket_verify_signer_privileges<'me, 'info>(
    accounts: WarmupVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn warmup_vault_ncn_ticket_verify_account_privileges<'me, 'info>(
    accounts: WarmupVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    warmup_vault_ncn_ticket_verify_writable_privileges(accounts)?;
    warmup_vault_ncn_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COOLDOWN_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CooldownVaultNcnTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub vault_ncn_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CooldownVaultNcnTicketKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub vault_ncn_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<CooldownVaultNcnTicketAccounts<'_, '_>> for CooldownVaultNcnTicketKeys {
    fn from(accounts: CooldownVaultNcnTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            ncn: *accounts.ncn.key,
            vault_ncn_ticket: *accounts.vault_ncn_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<CooldownVaultNcnTicketKeys>
for [AccountMeta; COOLDOWN_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CooldownVaultNcnTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_ncn_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COOLDOWN_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]>
for CooldownVaultNcnTicketKeys {
    fn from(pubkeys: [Pubkey; COOLDOWN_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            ncn: pubkeys[2],
            vault_ncn_ticket: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<CooldownVaultNcnTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; COOLDOWN_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CooldownVaultNcnTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.ncn.clone(),
            accounts.vault_ncn_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COOLDOWN_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN]>
for CooldownVaultNcnTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COOLDOWN_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            ncn: &arr[2],
            vault_ncn_ticket: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const COOLDOWN_VAULT_NCN_TICKET_IX_DISCM: u8 = 8u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CooldownVaultNcnTicketIxData;
impl CooldownVaultNcnTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COOLDOWN_VAULT_NCN_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COOLDOWN_VAULT_NCN_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cooldown_vault_ncn_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: CooldownVaultNcnTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COOLDOWN_VAULT_NCN_TICKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CooldownVaultNcnTicketIxData.try_to_vec()?,
    })
}
pub fn cooldown_vault_ncn_ticket_ix(
    keys: CooldownVaultNcnTicketKeys,
) -> std::io::Result<Instruction> {
    cooldown_vault_ncn_ticket_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn cooldown_vault_ncn_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CooldownVaultNcnTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CooldownVaultNcnTicketKeys = accounts.into();
    let ix = cooldown_vault_ncn_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cooldown_vault_ncn_ticket_invoke(
    accounts: CooldownVaultNcnTicketAccounts<'_, '_>,
) -> ProgramResult {
    cooldown_vault_ncn_ticket_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn cooldown_vault_ncn_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CooldownVaultNcnTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CooldownVaultNcnTicketKeys = accounts.into();
    let ix = cooldown_vault_ncn_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cooldown_vault_ncn_ticket_invoke_signed(
    accounts: CooldownVaultNcnTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cooldown_vault_ncn_ticket_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cooldown_vault_ncn_ticket_verify_account_keys(
    accounts: CooldownVaultNcnTicketAccounts<'_, '_>,
    keys: CooldownVaultNcnTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn.key, &keys.ncn),
        (accounts.vault_ncn_ticket.key, &keys.vault_ncn_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cooldown_vault_ncn_ticket_verify_writable_privileges<'me, 'info>(
    accounts: CooldownVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault_ncn_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cooldown_vault_ncn_ticket_verify_signer_privileges<'me, 'info>(
    accounts: CooldownVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cooldown_vault_ncn_ticket_verify_account_privileges<'me, 'info>(
    accounts: CooldownVaultNcnTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cooldown_vault_ncn_ticket_verify_writable_privileges(accounts)?;
    cooldown_vault_ncn_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WARMUP_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WarmupVaultNcnSlasherTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub slasher: &'me AccountInfo<'info>,
    pub vault_slasher_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WarmupVaultNcnSlasherTicketKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub slasher: Pubkey,
    pub vault_slasher_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<WarmupVaultNcnSlasherTicketAccounts<'_, '_>>
for WarmupVaultNcnSlasherTicketKeys {
    fn from(accounts: WarmupVaultNcnSlasherTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            ncn: *accounts.ncn.key,
            slasher: *accounts.slasher.key,
            vault_slasher_ticket: *accounts.vault_slasher_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<WarmupVaultNcnSlasherTicketKeys>
for [AccountMeta; WARMUP_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: WarmupVaultNcnSlasherTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slasher,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_slasher_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WARMUP_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for WarmupVaultNcnSlasherTicketKeys {
    fn from(pubkeys: [Pubkey; WARMUP_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            ncn: pubkeys[2],
            slasher: pubkeys[3],
            vault_slasher_ticket: pubkeys[4],
            admin: pubkeys[5],
        }
    }
}
impl<'info> From<WarmupVaultNcnSlasherTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; WARMUP_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: WarmupVaultNcnSlasherTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.ncn.clone(),
            accounts.slasher.clone(),
            accounts.vault_slasher_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WARMUP_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for WarmupVaultNcnSlasherTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WARMUP_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            ncn: &arr[2],
            slasher: &arr[3],
            vault_slasher_ticket: &arr[4],
            admin: &arr[5],
        }
    }
}
pub const WARMUP_VAULT_NCN_SLASHER_TICKET_IX_DISCM: u8 = 9u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WarmupVaultNcnSlasherTicketIxData;
impl WarmupVaultNcnSlasherTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WARMUP_VAULT_NCN_SLASHER_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WARMUP_VAULT_NCN_SLASHER_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn warmup_vault_ncn_slasher_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: WarmupVaultNcnSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WARMUP_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WarmupVaultNcnSlasherTicketIxData.try_to_vec()?,
    })
}
pub fn warmup_vault_ncn_slasher_ticket_ix(
    keys: WarmupVaultNcnSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    warmup_vault_ncn_slasher_ticket_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn warmup_vault_ncn_slasher_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WarmupVaultNcnSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WarmupVaultNcnSlasherTicketKeys = accounts.into();
    let ix = warmup_vault_ncn_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn warmup_vault_ncn_slasher_ticket_invoke(
    accounts: WarmupVaultNcnSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    warmup_vault_ncn_slasher_ticket_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
    )
}
pub fn warmup_vault_ncn_slasher_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WarmupVaultNcnSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WarmupVaultNcnSlasherTicketKeys = accounts.into();
    let ix = warmup_vault_ncn_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn warmup_vault_ncn_slasher_ticket_invoke_signed(
    accounts: WarmupVaultNcnSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    warmup_vault_ncn_slasher_ticket_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn warmup_vault_ncn_slasher_ticket_verify_account_keys(
    accounts: WarmupVaultNcnSlasherTicketAccounts<'_, '_>,
    keys: WarmupVaultNcnSlasherTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn.key, &keys.ncn),
        (accounts.slasher.key, &keys.slasher),
        (accounts.vault_slasher_ticket.key, &keys.vault_slasher_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn warmup_vault_ncn_slasher_ticket_verify_writable_privileges<'me, 'info>(
    accounts: WarmupVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault_slasher_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn warmup_vault_ncn_slasher_ticket_verify_signer_privileges<'me, 'info>(
    accounts: WarmupVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn warmup_vault_ncn_slasher_ticket_verify_account_privileges<'me, 'info>(
    accounts: WarmupVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    warmup_vault_ncn_slasher_ticket_verify_writable_privileges(accounts)?;
    warmup_vault_ncn_slasher_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CooldownVaultNcnSlasherTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub slasher: &'me AccountInfo<'info>,
    pub vault_ncn_slasher_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CooldownVaultNcnSlasherTicketKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub slasher: Pubkey,
    pub vault_ncn_slasher_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<CooldownVaultNcnSlasherTicketAccounts<'_, '_>>
for CooldownVaultNcnSlasherTicketKeys {
    fn from(accounts: CooldownVaultNcnSlasherTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            ncn: *accounts.ncn.key,
            slasher: *accounts.slasher.key,
            vault_ncn_slasher_ticket: *accounts.vault_ncn_slasher_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<CooldownVaultNcnSlasherTicketKeys>
for [AccountMeta; COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CooldownVaultNcnSlasherTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slasher,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_ncn_slasher_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for CooldownVaultNcnSlasherTicketKeys {
    fn from(
        pubkeys: [Pubkey; COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            ncn: pubkeys[2],
            slasher: pubkeys[3],
            vault_ncn_slasher_ticket: pubkeys[4],
            admin: pubkeys[5],
        }
    }
}
impl<'info> From<CooldownVaultNcnSlasherTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CooldownVaultNcnSlasherTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.ncn.clone(),
            accounts.slasher.clone(),
            accounts.vault_ncn_slasher_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for CooldownVaultNcnSlasherTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            ncn: &arr[2],
            slasher: &arr[3],
            vault_ncn_slasher_ticket: &arr[4],
            admin: &arr[5],
        }
    }
}
pub const COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_DISCM: u8 = 10u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CooldownVaultNcnSlasherTicketIxData;
impl CooldownVaultNcnSlasherTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cooldown_vault_ncn_slasher_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: CooldownVaultNcnSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COOLDOWN_VAULT_NCN_SLASHER_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CooldownVaultNcnSlasherTicketIxData.try_to_vec()?,
    })
}
pub fn cooldown_vault_ncn_slasher_ticket_ix(
    keys: CooldownVaultNcnSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    cooldown_vault_ncn_slasher_ticket_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn cooldown_vault_ncn_slasher_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CooldownVaultNcnSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CooldownVaultNcnSlasherTicketKeys = accounts.into();
    let ix = cooldown_vault_ncn_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cooldown_vault_ncn_slasher_ticket_invoke(
    accounts: CooldownVaultNcnSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    cooldown_vault_ncn_slasher_ticket_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
    )
}
pub fn cooldown_vault_ncn_slasher_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CooldownVaultNcnSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CooldownVaultNcnSlasherTicketKeys = accounts.into();
    let ix = cooldown_vault_ncn_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cooldown_vault_ncn_slasher_ticket_invoke_signed(
    accounts: CooldownVaultNcnSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cooldown_vault_ncn_slasher_ticket_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cooldown_vault_ncn_slasher_ticket_verify_account_keys(
    accounts: CooldownVaultNcnSlasherTicketAccounts<'_, '_>,
    keys: CooldownVaultNcnSlasherTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn.key, &keys.ncn),
        (accounts.slasher.key, &keys.slasher),
        (accounts.vault_ncn_slasher_ticket.key, &keys.vault_ncn_slasher_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cooldown_vault_ncn_slasher_ticket_verify_writable_privileges<'me, 'info>(
    accounts: CooldownVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault_ncn_slasher_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cooldown_vault_ncn_slasher_ticket_verify_signer_privileges<'me, 'info>(
    accounts: CooldownVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cooldown_vault_ncn_slasher_ticket_verify_account_privileges<'me, 'info>(
    accounts: CooldownVaultNcnSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cooldown_vault_ncn_slasher_ticket_verify_writable_privileges(accounts)?;
    cooldown_vault_ncn_slasher_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_TO_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct MintToAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vrt_mint: &'me AccountInfo<'info>,
    pub depositor: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub depositor_vrt_token_account: &'me AccountInfo<'info>,
    pub vault_fee_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub mint_signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct MintToKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub vrt_mint: Pubkey,
    pub depositor: Pubkey,
    pub depositor_token_account: Pubkey,
    pub vault_token_account: Pubkey,
    pub depositor_vrt_token_account: Pubkey,
    pub vault_fee_token_account: Pubkey,
    pub token_program: Pubkey,
    pub mint_signer: Pubkey,
}
impl From<MintToAccounts<'_, '_>> for MintToKeys {
    fn from(accounts: MintToAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            vrt_mint: *accounts.vrt_mint.key,
            depositor: *accounts.depositor.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            vault_token_account: *accounts.vault_token_account.key,
            depositor_vrt_token_account: *accounts.depositor_vrt_token_account.key,
            vault_fee_token_account: *accounts.vault_fee_token_account.key,
            token_program: *accounts.token_program.key,
            mint_signer: *accounts.mint_signer.key,
        }
    }
}
impl From<MintToKeys> for [AccountMeta; MINT_TO_IX_ACCOUNTS_LEN] {
    fn from(keys: MintToKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vrt_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_vrt_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_fee_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_signer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_TO_IX_ACCOUNTS_LEN]> for MintToKeys {
    fn from(pubkeys: [Pubkey; MINT_TO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            vrt_mint: pubkeys[2],
            depositor: pubkeys[3],
            depositor_token_account: pubkeys[4],
            vault_token_account: pubkeys[5],
            depositor_vrt_token_account: pubkeys[6],
            vault_fee_token_account: pubkeys[7],
            token_program: pubkeys[8],
            mint_signer: pubkeys[9],
        }
    }
}
impl<'info> From<MintToAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintToAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.vrt_mint.clone(),
            accounts.depositor.clone(),
            accounts.depositor_token_account.clone(),
            accounts.vault_token_account.clone(),
            accounts.depositor_vrt_token_account.clone(),
            accounts.vault_fee_token_account.clone(),
            accounts.token_program.clone(),
            accounts.mint_signer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN]>
for MintToAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            vrt_mint: &arr[2],
            depositor: &arr[3],
            depositor_token_account: &arr[4],
            vault_token_account: &arr[5],
            depositor_vrt_token_account: &arr[6],
            vault_fee_token_account: &arr[7],
            token_program: &arr[8],
            mint_signer: &arr[9],
        }
    }
}
pub const MINT_TO_IX_DISCM: u8 = 11u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct MintToIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
impl MintToIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_in, min_amount_out })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintToIxData(pub MintToIxArgs);
impl From<MintToIxArgs> for MintToIxData {
    fn from(args: MintToIxArgs) -> Self {
        Self(args)
    }
}
impl MintToIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != MINT_TO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MintToIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MINT_TO_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_to_ix_with_program_id(
    program_id: Pubkey,
    keys: MintToKeys,
    args: MintToIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_TO_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintToIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_to_ix(keys: MintToKeys, args: MintToIxArgs) -> std::io::Result<Instruction> {
    mint_to_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn mint_to_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintToAccounts<'_, '_>,
    args: MintToIxArgs,
) -> ProgramResult {
    let keys: MintToKeys = accounts.into();
    let ix = mint_to_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_to_invoke(
    accounts: MintToAccounts<'_, '_>,
    args: MintToIxArgs,
) -> ProgramResult {
    mint_to_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn mint_to_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintToAccounts<'_, '_>,
    args: MintToIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintToKeys = accounts.into();
    let ix = mint_to_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_to_invoke_signed(
    accounts: MintToAccounts<'_, '_>,
    args: MintToIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_to_invoke_signed_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_to_verify_account_keys(
    accounts: MintToAccounts<'_, '_>,
    keys: MintToKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.vrt_mint.key, &keys.vrt_mint),
        (accounts.depositor.key, &keys.depositor),
        (accounts.depositor_token_account.key, &keys.depositor_token_account),
        (accounts.vault_token_account.key, &keys.vault_token_account),
        (accounts.depositor_vrt_token_account.key, &keys.depositor_vrt_token_account),
        (accounts.vault_fee_token_account.key, &keys.vault_fee_token_account),
        (accounts.token_program.key, &keys.token_program),
        (accounts.mint_signer.key, &keys.mint_signer),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn mint_to_verify_writable_privileges<'me, 'info>(
    accounts: MintToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vrt_mint,
        accounts.depositor,
        accounts.depositor_token_account,
        accounts.vault_token_account,
        accounts.depositor_vrt_token_account,
        accounts.vault_fee_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_to_verify_signer_privileges<'me, 'info>(
    accounts: MintToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.depositor, accounts.mint_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_to_verify_account_privileges<'me, 'info>(
    accounts: MintToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_to_verify_writable_privileges(accounts)?;
    mint_to_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ENQUEUE_WITHDRAWAL_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct EnqueueWithdrawalAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_staker_withdrawal_ticket: &'me AccountInfo<'info>,
    pub vault_staker_withdrawal_ticket_token_account: &'me AccountInfo<'info>,
    pub staker: &'me AccountInfo<'info>,
    pub staker_vrt_token_account: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub burn_signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct EnqueueWithdrawalKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub vault_staker_withdrawal_ticket: Pubkey,
    pub vault_staker_withdrawal_ticket_token_account: Pubkey,
    pub staker: Pubkey,
    pub staker_vrt_token_account: Pubkey,
    pub base: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub burn_signer: Pubkey,
}
impl From<EnqueueWithdrawalAccounts<'_, '_>> for EnqueueWithdrawalKeys {
    fn from(accounts: EnqueueWithdrawalAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            vault_staker_withdrawal_ticket: *accounts.vault_staker_withdrawal_ticket.key,
            vault_staker_withdrawal_ticket_token_account: *accounts
                .vault_staker_withdrawal_ticket_token_account
                .key,
            staker: *accounts.staker.key,
            staker_vrt_token_account: *accounts.staker_vrt_token_account.key,
            base: *accounts.base.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            burn_signer: *accounts.burn_signer.key,
        }
    }
}
impl From<EnqueueWithdrawalKeys> for [AccountMeta; ENQUEUE_WITHDRAWAL_IX_ACCOUNTS_LEN] {
    fn from(keys: EnqueueWithdrawalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_staker_withdrawal_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_staker_withdrawal_ticket_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker_vrt_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
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
                pubkey: keys.burn_signer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ENQUEUE_WITHDRAWAL_IX_ACCOUNTS_LEN]> for EnqueueWithdrawalKeys {
    fn from(pubkeys: [Pubkey; ENQUEUE_WITHDRAWAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            vault_staker_withdrawal_ticket: pubkeys[2],
            vault_staker_withdrawal_ticket_token_account: pubkeys[3],
            staker: pubkeys[4],
            staker_vrt_token_account: pubkeys[5],
            base: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            burn_signer: pubkeys[9],
        }
    }
}
impl<'info> From<EnqueueWithdrawalAccounts<'_, 'info>>
for [AccountInfo<'info>; ENQUEUE_WITHDRAWAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: EnqueueWithdrawalAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.vault_staker_withdrawal_ticket.clone(),
            accounts.vault_staker_withdrawal_ticket_token_account.clone(),
            accounts.staker.clone(),
            accounts.staker_vrt_token_account.clone(),
            accounts.base.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.burn_signer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENQUEUE_WITHDRAWAL_IX_ACCOUNTS_LEN]>
for EnqueueWithdrawalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ENQUEUE_WITHDRAWAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            vault_staker_withdrawal_ticket: &arr[2],
            vault_staker_withdrawal_ticket_token_account: &arr[3],
            staker: &arr[4],
            staker_vrt_token_account: &arr[5],
            base: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            burn_signer: &arr[9],
        }
    }
}
pub const ENQUEUE_WITHDRAWAL_IX_DISCM: u8 = 12u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct EnqueueWithdrawalIxArgs {
    pub amount: u64,
}
impl EnqueueWithdrawalIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnqueueWithdrawalIxData(pub EnqueueWithdrawalIxArgs);
impl From<EnqueueWithdrawalIxArgs> for EnqueueWithdrawalIxData {
    fn from(args: EnqueueWithdrawalIxArgs) -> Self {
        Self(args)
    }
}
impl EnqueueWithdrawalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != ENQUEUE_WITHDRAWAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(EnqueueWithdrawalIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[ENQUEUE_WITHDRAWAL_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn enqueue_withdrawal_ix_with_program_id(
    program_id: Pubkey,
    keys: EnqueueWithdrawalKeys,
    args: EnqueueWithdrawalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ENQUEUE_WITHDRAWAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: EnqueueWithdrawalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn enqueue_withdrawal_ix(
    keys: EnqueueWithdrawalKeys,
    args: EnqueueWithdrawalIxArgs,
) -> std::io::Result<Instruction> {
    enqueue_withdrawal_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn enqueue_withdrawal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EnqueueWithdrawalAccounts<'_, '_>,
    args: EnqueueWithdrawalIxArgs,
) -> ProgramResult {
    let keys: EnqueueWithdrawalKeys = accounts.into();
    let ix = enqueue_withdrawal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn enqueue_withdrawal_invoke(
    accounts: EnqueueWithdrawalAccounts<'_, '_>,
    args: EnqueueWithdrawalIxArgs,
) -> ProgramResult {
    enqueue_withdrawal_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn enqueue_withdrawal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EnqueueWithdrawalAccounts<'_, '_>,
    args: EnqueueWithdrawalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EnqueueWithdrawalKeys = accounts.into();
    let ix = enqueue_withdrawal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn enqueue_withdrawal_invoke_signed(
    accounts: EnqueueWithdrawalAccounts<'_, '_>,
    args: EnqueueWithdrawalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    enqueue_withdrawal_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn enqueue_withdrawal_verify_account_keys(
    accounts: EnqueueWithdrawalAccounts<'_, '_>,
    keys: EnqueueWithdrawalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (
            accounts.vault_staker_withdrawal_ticket.key,
            &keys.vault_staker_withdrawal_ticket,
        ),
        (
            accounts.vault_staker_withdrawal_ticket_token_account.key,
            &keys.vault_staker_withdrawal_ticket_token_account,
        ),
        (accounts.staker.key, &keys.staker),
        (accounts.staker_vrt_token_account.key, &keys.staker_vrt_token_account),
        (accounts.base.key, &keys.base),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.burn_signer.key, &keys.burn_signer),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn enqueue_withdrawal_verify_writable_privileges<'me, 'info>(
    accounts: EnqueueWithdrawalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_staker_withdrawal_ticket,
        accounts.vault_staker_withdrawal_ticket_token_account,
        accounts.staker,
        accounts.staker_vrt_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn enqueue_withdrawal_verify_signer_privileges<'me, 'info>(
    accounts: EnqueueWithdrawalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker, accounts.base, accounts.burn_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn enqueue_withdrawal_verify_account_privileges<'me, 'info>(
    accounts: EnqueueWithdrawalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    enqueue_withdrawal_verify_writable_privileges(accounts)?;
    enqueue_withdrawal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_WITHDRAWAL_TICKET_OWNER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ChangeWithdrawalTicketOwnerAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_staker_withdrawal_ticket: &'me AccountInfo<'info>,
    pub old_owner: &'me AccountInfo<'info>,
    pub new_owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ChangeWithdrawalTicketOwnerKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub vault_staker_withdrawal_ticket: Pubkey,
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
}
impl From<ChangeWithdrawalTicketOwnerAccounts<'_, '_>>
for ChangeWithdrawalTicketOwnerKeys {
    fn from(accounts: ChangeWithdrawalTicketOwnerAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            vault_staker_withdrawal_ticket: *accounts.vault_staker_withdrawal_ticket.key,
            old_owner: *accounts.old_owner.key,
            new_owner: *accounts.new_owner.key,
        }
    }
}
impl From<ChangeWithdrawalTicketOwnerKeys>
for [AccountMeta; CHANGE_WITHDRAWAL_TICKET_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeWithdrawalTicketOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_staker_withdrawal_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_owner,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_WITHDRAWAL_TICKET_OWNER_IX_ACCOUNTS_LEN]>
for ChangeWithdrawalTicketOwnerKeys {
    fn from(pubkeys: [Pubkey; CHANGE_WITHDRAWAL_TICKET_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            vault_staker_withdrawal_ticket: pubkeys[2],
            old_owner: pubkeys[3],
            new_owner: pubkeys[4],
        }
    }
}
impl<'info> From<ChangeWithdrawalTicketOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_WITHDRAWAL_TICKET_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeWithdrawalTicketOwnerAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.vault_staker_withdrawal_ticket.clone(),
            accounts.old_owner.clone(),
            accounts.new_owner.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CHANGE_WITHDRAWAL_TICKET_OWNER_IX_ACCOUNTS_LEN]>
for ChangeWithdrawalTicketOwnerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHANGE_WITHDRAWAL_TICKET_OWNER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            vault_staker_withdrawal_ticket: &arr[2],
            old_owner: &arr[3],
            new_owner: &arr[4],
        }
    }
}
pub const CHANGE_WITHDRAWAL_TICKET_OWNER_IX_DISCM: u8 = 13u8;
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeWithdrawalTicketOwnerIxData;
impl ChangeWithdrawalTicketOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CHANGE_WITHDRAWAL_TICKET_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CHANGE_WITHDRAWAL_TICKET_OWNER_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_withdrawal_ticket_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeWithdrawalTicketOwnerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_WITHDRAWAL_TICKET_OWNER_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ChangeWithdrawalTicketOwnerIxData.try_to_vec()?,
    })
}
pub fn change_withdrawal_ticket_owner_ix(
    keys: ChangeWithdrawalTicketOwnerKeys,
) -> std::io::Result<Instruction> {
    change_withdrawal_ticket_owner_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn change_withdrawal_ticket_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeWithdrawalTicketOwnerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ChangeWithdrawalTicketOwnerKeys = accounts.into();
    let ix = change_withdrawal_ticket_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_withdrawal_ticket_owner_invoke(
    accounts: ChangeWithdrawalTicketOwnerAccounts<'_, '_>,
) -> ProgramResult {
    change_withdrawal_ticket_owner_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
    )
}
pub fn change_withdrawal_ticket_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeWithdrawalTicketOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeWithdrawalTicketOwnerKeys = accounts.into();
    let ix = change_withdrawal_ticket_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_withdrawal_ticket_owner_invoke_signed(
    accounts: ChangeWithdrawalTicketOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_withdrawal_ticket_owner_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn change_withdrawal_ticket_owner_verify_account_keys(
    accounts: ChangeWithdrawalTicketOwnerAccounts<'_, '_>,
    keys: ChangeWithdrawalTicketOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (
            accounts.vault_staker_withdrawal_ticket.key,
            &keys.vault_staker_withdrawal_ticket,
        ),
        (accounts.old_owner.key, &keys.old_owner),
        (accounts.new_owner.key, &keys.new_owner),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn change_withdrawal_ticket_owner_verify_writable_privileges<'me, 'info>(
    accounts: ChangeWithdrawalTicketOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault_staker_withdrawal_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_withdrawal_ticket_owner_verify_signer_privileges<'me, 'info>(
    accounts: ChangeWithdrawalTicketOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.old_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_withdrawal_ticket_owner_verify_account_privileges<'me, 'info>(
    accounts: ChangeWithdrawalTicketOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_withdrawal_ticket_owner_verify_writable_privileges(accounts)?;
    change_withdrawal_ticket_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_WITHDRAWAL_TICKET_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct BurnWithdrawalTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub vrt_mint: &'me AccountInfo<'info>,
    pub staker: &'me AccountInfo<'info>,
    pub staker_token_account: &'me AccountInfo<'info>,
    pub vault_staker_withdrawal_ticket: &'me AccountInfo<'info>,
    pub vault_staker_withdrawal_ticket_token_account: &'me AccountInfo<'info>,
    pub vault_fee_token_account: &'me AccountInfo<'info>,
    pub program_fee_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub burn_signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct BurnWithdrawalTicketKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub vault_token_account: Pubkey,
    pub vrt_mint: Pubkey,
    pub staker: Pubkey,
    pub staker_token_account: Pubkey,
    pub vault_staker_withdrawal_ticket: Pubkey,
    pub vault_staker_withdrawal_ticket_token_account: Pubkey,
    pub vault_fee_token_account: Pubkey,
    pub program_fee_token_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub burn_signer: Pubkey,
}
impl From<BurnWithdrawalTicketAccounts<'_, '_>> for BurnWithdrawalTicketKeys {
    fn from(accounts: BurnWithdrawalTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            vault_token_account: *accounts.vault_token_account.key,
            vrt_mint: *accounts.vrt_mint.key,
            staker: *accounts.staker.key,
            staker_token_account: *accounts.staker_token_account.key,
            vault_staker_withdrawal_ticket: *accounts.vault_staker_withdrawal_ticket.key,
            vault_staker_withdrawal_ticket_token_account: *accounts
                .vault_staker_withdrawal_ticket_token_account
                .key,
            vault_fee_token_account: *accounts.vault_fee_token_account.key,
            program_fee_token_account: *accounts.program_fee_token_account.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            burn_signer: *accounts.burn_signer.key,
        }
    }
}
impl From<BurnWithdrawalTicketKeys>
for [AccountMeta; BURN_WITHDRAWAL_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnWithdrawalTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vrt_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_staker_withdrawal_ticket,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_staker_withdrawal_ticket_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_fee_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_fee_token_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.burn_signer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BURN_WITHDRAWAL_TICKET_IX_ACCOUNTS_LEN]>
for BurnWithdrawalTicketKeys {
    fn from(pubkeys: [Pubkey; BURN_WITHDRAWAL_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            vault_token_account: pubkeys[2],
            vrt_mint: pubkeys[3],
            staker: pubkeys[4],
            staker_token_account: pubkeys[5],
            vault_staker_withdrawal_ticket: pubkeys[6],
            vault_staker_withdrawal_ticket_token_account: pubkeys[7],
            vault_fee_token_account: pubkeys[8],
            program_fee_token_account: pubkeys[9],
            token_program: pubkeys[10],
            system_program: pubkeys[11],
            burn_signer: pubkeys[12],
        }
    }
}
impl<'info> From<BurnWithdrawalTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; BURN_WITHDRAWAL_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: BurnWithdrawalTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.vault_token_account.clone(),
            accounts.vrt_mint.clone(),
            accounts.staker.clone(),
            accounts.staker_token_account.clone(),
            accounts.vault_staker_withdrawal_ticket.clone(),
            accounts.vault_staker_withdrawal_ticket_token_account.clone(),
            accounts.vault_fee_token_account.clone(),
            accounts.program_fee_token_account.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.burn_signer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_WITHDRAWAL_TICKET_IX_ACCOUNTS_LEN]>
for BurnWithdrawalTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BURN_WITHDRAWAL_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            vault_token_account: &arr[2],
            vrt_mint: &arr[3],
            staker: &arr[4],
            staker_token_account: &arr[5],
            vault_staker_withdrawal_ticket: &arr[6],
            vault_staker_withdrawal_ticket_token_account: &arr[7],
            vault_fee_token_account: &arr[8],
            program_fee_token_account: &arr[9],
            token_program: &arr[10],
            system_program: &arr[11],
            burn_signer: &arr[12],
        }
    }
}
pub const BURN_WITHDRAWAL_TICKET_IX_DISCM: u8 = 14u8;
#[derive(Clone, Debug, PartialEq)]
pub struct BurnWithdrawalTicketIxData;
impl BurnWithdrawalTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != BURN_WITHDRAWAL_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[BURN_WITHDRAWAL_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn burn_withdrawal_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: BurnWithdrawalTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_WITHDRAWAL_TICKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: BurnWithdrawalTicketIxData.try_to_vec()?,
    })
}
pub fn burn_withdrawal_ticket_ix(
    keys: BurnWithdrawalTicketKeys,
) -> std::io::Result<Instruction> {
    burn_withdrawal_ticket_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn burn_withdrawal_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BurnWithdrawalTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: BurnWithdrawalTicketKeys = accounts.into();
    let ix = burn_withdrawal_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn burn_withdrawal_ticket_invoke(
    accounts: BurnWithdrawalTicketAccounts<'_, '_>,
) -> ProgramResult {
    burn_withdrawal_ticket_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn burn_withdrawal_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BurnWithdrawalTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnWithdrawalTicketKeys = accounts.into();
    let ix = burn_withdrawal_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn burn_withdrawal_ticket_invoke_signed(
    accounts: BurnWithdrawalTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    burn_withdrawal_ticket_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn burn_withdrawal_ticket_verify_account_keys(
    accounts: BurnWithdrawalTicketAccounts<'_, '_>,
    keys: BurnWithdrawalTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.vault_token_account.key, &keys.vault_token_account),
        (accounts.vrt_mint.key, &keys.vrt_mint),
        (accounts.staker.key, &keys.staker),
        (accounts.staker_token_account.key, &keys.staker_token_account),
        (
            accounts.vault_staker_withdrawal_ticket.key,
            &keys.vault_staker_withdrawal_ticket,
        ),
        (
            accounts.vault_staker_withdrawal_ticket_token_account.key,
            &keys.vault_staker_withdrawal_ticket_token_account,
        ),
        (accounts.vault_fee_token_account.key, &keys.vault_fee_token_account),
        (accounts.program_fee_token_account.key, &keys.program_fee_token_account),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.burn_signer.key, &keys.burn_signer),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn burn_withdrawal_ticket_verify_writable_privileges<'me, 'info>(
    accounts: BurnWithdrawalTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_token_account,
        accounts.vrt_mint,
        accounts.staker,
        accounts.staker_token_account,
        accounts.vault_staker_withdrawal_ticket,
        accounts.vault_staker_withdrawal_ticket_token_account,
        accounts.vault_fee_token_account,
        accounts.program_fee_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_withdrawal_ticket_verify_signer_privileges<'me, 'info>(
    accounts: BurnWithdrawalTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.burn_signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_withdrawal_ticket_verify_account_privileges<'me, 'info>(
    accounts: BurnWithdrawalTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_withdrawal_ticket_verify_writable_privileges(accounts)?;
    burn_withdrawal_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_DEPOSIT_CAPACITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetDepositCapacityAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetDepositCapacityKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub admin: Pubkey,
}
impl From<SetDepositCapacityAccounts<'_, '_>> for SetDepositCapacityKeys {
    fn from(accounts: SetDepositCapacityAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<SetDepositCapacityKeys>
for [AccountMeta; SET_DEPOSIT_CAPACITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetDepositCapacityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_DEPOSIT_CAPACITY_IX_ACCOUNTS_LEN]> for SetDepositCapacityKeys {
    fn from(pubkeys: [Pubkey; SET_DEPOSIT_CAPACITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<SetDepositCapacityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_DEPOSIT_CAPACITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetDepositCapacityAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.vault.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_DEPOSIT_CAPACITY_IX_ACCOUNTS_LEN]>
for SetDepositCapacityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_DEPOSIT_CAPACITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const SET_DEPOSIT_CAPACITY_IX_DISCM: u8 = 15u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetDepositCapacityIxArgs {
    pub amount: u64,
}
impl SetDepositCapacityIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetDepositCapacityIxData(pub SetDepositCapacityIxArgs);
impl From<SetDepositCapacityIxArgs> for SetDepositCapacityIxData {
    fn from(args: SetDepositCapacityIxArgs) -> Self {
        Self(args)
    }
}
impl SetDepositCapacityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_DEPOSIT_CAPACITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SetDepositCapacityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_DEPOSIT_CAPACITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_deposit_capacity_ix_with_program_id(
    program_id: Pubkey,
    keys: SetDepositCapacityKeys,
    args: SetDepositCapacityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_DEPOSIT_CAPACITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetDepositCapacityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_deposit_capacity_ix(
    keys: SetDepositCapacityKeys,
    args: SetDepositCapacityIxArgs,
) -> std::io::Result<Instruction> {
    set_deposit_capacity_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn set_deposit_capacity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetDepositCapacityAccounts<'_, '_>,
    args: SetDepositCapacityIxArgs,
) -> ProgramResult {
    let keys: SetDepositCapacityKeys = accounts.into();
    let ix = set_deposit_capacity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_deposit_capacity_invoke(
    accounts: SetDepositCapacityAccounts<'_, '_>,
    args: SetDepositCapacityIxArgs,
) -> ProgramResult {
    set_deposit_capacity_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn set_deposit_capacity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetDepositCapacityAccounts<'_, '_>,
    args: SetDepositCapacityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetDepositCapacityKeys = accounts.into();
    let ix = set_deposit_capacity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_deposit_capacity_invoke_signed(
    accounts: SetDepositCapacityAccounts<'_, '_>,
    args: SetDepositCapacityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_deposit_capacity_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_deposit_capacity_verify_account_keys(
    accounts: SetDepositCapacityAccounts<'_, '_>,
    keys: SetDepositCapacityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_deposit_capacity_verify_writable_privileges<'me, 'info>(
    accounts: SetDepositCapacityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_deposit_capacity_verify_signer_privileges<'me, 'info>(
    accounts: SetDepositCapacityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_deposit_capacity_verify_account_privileges<'me, 'info>(
    accounts: SetDepositCapacityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_deposit_capacity_verify_writable_privileges(accounts)?;
    set_deposit_capacity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetFeesAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetFeesKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub admin: Pubkey,
}
impl From<SetFeesAccounts<'_, '_>> for SetFeesKeys {
    fn from(accounts: SetFeesAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<SetFeesKeys> for [AccountMeta; SET_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEES_IX_ACCOUNTS_LEN]> for SetFeesKeys {
    fn from(pubkeys: [Pubkey; SET_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<SetFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeesAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.vault.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEES_IX_ACCOUNTS_LEN]>
for SetFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const SET_FEES_IX_DISCM: u8 = 16u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetFeesIxArgs {
    pub deposit_fee_bps: Option<u16>,
    pub withdrawal_fee_bps: Option<u16>,
    pub reward_fee_bps: Option<u16>,
}
impl SetFeesIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let deposit_fee_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_fee_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let reward_fee_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            deposit_fee_bps,
            withdrawal_fee_bps,
            reward_fee_bps,
        })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeesIxData(pub SetFeesIxArgs);
impl From<SetFeesIxArgs> for SetFeesIxData {
    fn from(args: SetFeesIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SetFeesIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_FEES_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeesKeys,
    args: SetFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fees_ix(
    keys: SetFeesKeys,
    args: SetFeesIxArgs,
) -> std::io::Result<Instruction> {
    set_fees_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn set_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeesAccounts<'_, '_>,
    args: SetFeesIxArgs,
) -> ProgramResult {
    let keys: SetFeesKeys = accounts.into();
    let ix = set_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fees_invoke(
    accounts: SetFeesAccounts<'_, '_>,
    args: SetFeesIxArgs,
) -> ProgramResult {
    set_fees_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn set_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeesAccounts<'_, '_>,
    args: SetFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeesKeys = accounts.into();
    let ix = set_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fees_invoke_signed(
    accounts: SetFeesAccounts<'_, '_>,
    args: SetFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fees_invoke_signed_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_fees_verify_account_keys(
    accounts: SetFeesAccounts<'_, '_>,
    keys: SetFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_fees_verify_writable_privileges<'me, 'info>(
    accounts: SetFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fees_verify_signer_privileges<'me, 'info>(
    accounts: SetFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fees_verify_account_privileges<'me, 'info>(
    accounts: SetFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fees_verify_writable_privileges(accounts)?;
    set_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PROGRAM_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetProgramFeeAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetProgramFeeKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
}
impl From<SetProgramFeeAccounts<'_, '_>> for SetProgramFeeKeys {
    fn from(accounts: SetProgramFeeAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<SetProgramFeeKeys> for [AccountMeta; SET_PROGRAM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetProgramFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PROGRAM_FEE_IX_ACCOUNTS_LEN]> for SetProgramFeeKeys {
    fn from(pubkeys: [Pubkey; SET_PROGRAM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<SetProgramFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PROGRAM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetProgramFeeAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PROGRAM_FEE_IX_ACCOUNTS_LEN]>
for SetProgramFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PROGRAM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const SET_PROGRAM_FEE_IX_DISCM: u8 = 17u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetProgramFeeIxArgs {
    pub new_fee_bps: u16,
}
impl SetProgramFeeIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { new_fee_bps })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetProgramFeeIxData(pub SetProgramFeeIxArgs);
impl From<SetProgramFeeIxArgs> for SetProgramFeeIxData {
    fn from(args: SetProgramFeeIxArgs) -> Self {
        Self(args)
    }
}
impl SetProgramFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_PROGRAM_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SetProgramFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_PROGRAM_FEE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_program_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: SetProgramFeeKeys,
    args: SetProgramFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PROGRAM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetProgramFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_program_fee_ix(
    keys: SetProgramFeeKeys,
    args: SetProgramFeeIxArgs,
) -> std::io::Result<Instruction> {
    set_program_fee_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn set_program_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetProgramFeeAccounts<'_, '_>,
    args: SetProgramFeeIxArgs,
) -> ProgramResult {
    let keys: SetProgramFeeKeys = accounts.into();
    let ix = set_program_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_program_fee_invoke(
    accounts: SetProgramFeeAccounts<'_, '_>,
    args: SetProgramFeeIxArgs,
) -> ProgramResult {
    set_program_fee_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn set_program_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetProgramFeeAccounts<'_, '_>,
    args: SetProgramFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetProgramFeeKeys = accounts.into();
    let ix = set_program_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_program_fee_invoke_signed(
    accounts: SetProgramFeeAccounts<'_, '_>,
    args: SetProgramFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_program_fee_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_program_fee_verify_account_keys(
    accounts: SetProgramFeeAccounts<'_, '_>,
    keys: SetProgramFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_program_fee_verify_writable_privileges<'me, 'info>(
    accounts: SetProgramFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_program_fee_verify_signer_privileges<'me, 'info>(
    accounts: SetProgramFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_program_fee_verify_account_privileges<'me, 'info>(
    accounts: SetProgramFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_program_fee_verify_writable_privileges(accounts)?;
    set_program_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PROGRAM_FEE_WALLET_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetProgramFeeWalletAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub program_fee_admin: &'me AccountInfo<'info>,
    pub new_fee_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetProgramFeeWalletKeys {
    pub config: Pubkey,
    pub program_fee_admin: Pubkey,
    pub new_fee_wallet: Pubkey,
}
impl From<SetProgramFeeWalletAccounts<'_, '_>> for SetProgramFeeWalletKeys {
    fn from(accounts: SetProgramFeeWalletAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            program_fee_admin: *accounts.program_fee_admin.key,
            new_fee_wallet: *accounts.new_fee_wallet.key,
        }
    }
}
impl From<SetProgramFeeWalletKeys>
for [AccountMeta; SET_PROGRAM_FEE_WALLET_IX_ACCOUNTS_LEN] {
    fn from(keys: SetProgramFeeWalletKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_fee_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_fee_wallet,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PROGRAM_FEE_WALLET_IX_ACCOUNTS_LEN]> for SetProgramFeeWalletKeys {
    fn from(pubkeys: [Pubkey; SET_PROGRAM_FEE_WALLET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            program_fee_admin: pubkeys[1],
            new_fee_wallet: pubkeys[2],
        }
    }
}
impl<'info> From<SetProgramFeeWalletAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PROGRAM_FEE_WALLET_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetProgramFeeWalletAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.program_fee_admin.clone(),
            accounts.new_fee_wallet.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PROGRAM_FEE_WALLET_IX_ACCOUNTS_LEN]>
for SetProgramFeeWalletAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PROGRAM_FEE_WALLET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            program_fee_admin: &arr[1],
            new_fee_wallet: &arr[2],
        }
    }
}
pub const SET_PROGRAM_FEE_WALLET_IX_DISCM: u8 = 18u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SetProgramFeeWalletIxData;
impl SetProgramFeeWalletIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_PROGRAM_FEE_WALLET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_PROGRAM_FEE_WALLET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_program_fee_wallet_ix_with_program_id(
    program_id: Pubkey,
    keys: SetProgramFeeWalletKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PROGRAM_FEE_WALLET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetProgramFeeWalletIxData.try_to_vec()?,
    })
}
pub fn set_program_fee_wallet_ix(
    keys: SetProgramFeeWalletKeys,
) -> std::io::Result<Instruction> {
    set_program_fee_wallet_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn set_program_fee_wallet_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetProgramFeeWalletAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetProgramFeeWalletKeys = accounts.into();
    let ix = set_program_fee_wallet_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_program_fee_wallet_invoke(
    accounts: SetProgramFeeWalletAccounts<'_, '_>,
) -> ProgramResult {
    set_program_fee_wallet_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn set_program_fee_wallet_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetProgramFeeWalletAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetProgramFeeWalletKeys = accounts.into();
    let ix = set_program_fee_wallet_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_program_fee_wallet_invoke_signed(
    accounts: SetProgramFeeWalletAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_program_fee_wallet_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_program_fee_wallet_verify_account_keys(
    accounts: SetProgramFeeWalletAccounts<'_, '_>,
    keys: SetProgramFeeWalletKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.program_fee_admin.key, &keys.program_fee_admin),
        (accounts.new_fee_wallet.key, &keys.new_fee_wallet),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_program_fee_wallet_verify_writable_privileges<'me, 'info>(
    accounts: SetProgramFeeWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_program_fee_wallet_verify_signer_privileges<'me, 'info>(
    accounts: SetProgramFeeWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.program_fee_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_program_fee_wallet_verify_account_privileges<'me, 'info>(
    accounts: SetProgramFeeWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_program_fee_wallet_verify_writable_privileges(accounts)?;
    set_program_fee_wallet_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_IS_PAUSED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetIsPausedAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetIsPausedKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub admin: Pubkey,
}
impl From<SetIsPausedAccounts<'_, '_>> for SetIsPausedKeys {
    fn from(accounts: SetIsPausedAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<SetIsPausedKeys> for [AccountMeta; SET_IS_PAUSED_IX_ACCOUNTS_LEN] {
    fn from(keys: SetIsPausedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_IS_PAUSED_IX_ACCOUNTS_LEN]> for SetIsPausedKeys {
    fn from(pubkeys: [Pubkey; SET_IS_PAUSED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<SetIsPausedAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_IS_PAUSED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetIsPausedAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.vault.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_IS_PAUSED_IX_ACCOUNTS_LEN]>
for SetIsPausedAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_IS_PAUSED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const SET_IS_PAUSED_IX_DISCM: u8 = 19u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetIsPausedIxArgs {
    pub is_paused: bool,
}
impl SetIsPausedIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { is_paused })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetIsPausedIxData(pub SetIsPausedIxArgs);
impl From<SetIsPausedIxArgs> for SetIsPausedIxData {
    fn from(args: SetIsPausedIxArgs) -> Self {
        Self(args)
    }
}
impl SetIsPausedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_IS_PAUSED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SetIsPausedIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_IS_PAUSED_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_is_paused_ix_with_program_id(
    program_id: Pubkey,
    keys: SetIsPausedKeys,
    args: SetIsPausedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_IS_PAUSED_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetIsPausedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_is_paused_ix(
    keys: SetIsPausedKeys,
    args: SetIsPausedIxArgs,
) -> std::io::Result<Instruction> {
    set_is_paused_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn set_is_paused_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetIsPausedAccounts<'_, '_>,
    args: SetIsPausedIxArgs,
) -> ProgramResult {
    let keys: SetIsPausedKeys = accounts.into();
    let ix = set_is_paused_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_is_paused_invoke(
    accounts: SetIsPausedAccounts<'_, '_>,
    args: SetIsPausedIxArgs,
) -> ProgramResult {
    set_is_paused_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn set_is_paused_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetIsPausedAccounts<'_, '_>,
    args: SetIsPausedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetIsPausedKeys = accounts.into();
    let ix = set_is_paused_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_is_paused_invoke_signed(
    accounts: SetIsPausedAccounts<'_, '_>,
    args: SetIsPausedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_is_paused_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_is_paused_verify_account_keys(
    accounts: SetIsPausedAccounts<'_, '_>,
    keys: SetIsPausedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_is_paused_verify_writable_privileges<'me, 'info>(
    accounts: SetIsPausedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_is_paused_verify_signer_privileges<'me, 'info>(
    accounts: SetIsPausedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_is_paused_verify_account_privileges<'me, 'info>(
    accounts: SetIsPausedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_is_paused_verify_writable_privileges(accounts)?;
    set_is_paused_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DelegateTokenAccountAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub delegate_asset_admin: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub delegate: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DelegateTokenAccountKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub delegate_asset_admin: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub delegate: Pubkey,
    pub token_program: Pubkey,
}
impl From<DelegateTokenAccountAccounts<'_, '_>> for DelegateTokenAccountKeys {
    fn from(accounts: DelegateTokenAccountAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            delegate_asset_admin: *accounts.delegate_asset_admin.key,
            token_mint: *accounts.token_mint.key,
            token_account: *accounts.token_account.key,
            delegate: *accounts.delegate.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DelegateTokenAccountKeys>
for [AccountMeta; DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: DelegateTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegate_asset_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegate,
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
impl From<[Pubkey; DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for DelegateTokenAccountKeys {
    fn from(pubkeys: [Pubkey; DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            delegate_asset_admin: pubkeys[2],
            token_mint: pubkeys[3],
            token_account: pubkeys[4],
            delegate: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<DelegateTokenAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DelegateTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.delegate_asset_admin.clone(),
            accounts.token_mint.clone(),
            accounts.token_account.clone(),
            accounts.delegate.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for DelegateTokenAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            delegate_asset_admin: &arr[2],
            token_mint: &arr[3],
            token_account: &arr[4],
            delegate: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const DELEGATE_TOKEN_ACCOUNT_IX_DISCM: u8 = 20u8;
#[derive(Clone, Debug, PartialEq)]
pub struct DelegateTokenAccountIxData;
impl DelegateTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != DELEGATE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DELEGATE_TOKEN_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delegate_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: DelegateTokenAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DelegateTokenAccountIxData.try_to_vec()?,
    })
}
pub fn delegate_token_account_ix(
    keys: DelegateTokenAccountKeys,
) -> std::io::Result<Instruction> {
    delegate_token_account_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn delegate_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DelegateTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DelegateTokenAccountKeys = accounts.into();
    let ix = delegate_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn delegate_token_account_invoke(
    accounts: DelegateTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    delegate_token_account_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn delegate_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DelegateTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DelegateTokenAccountKeys = accounts.into();
    let ix = delegate_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delegate_token_account_invoke_signed(
    accounts: DelegateTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delegate_token_account_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn delegate_token_account_verify_account_keys(
    accounts: DelegateTokenAccountAccounts<'_, '_>,
    keys: DelegateTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.delegate_asset_admin.key, &keys.delegate_asset_admin),
        (accounts.token_mint.key, &keys.token_mint),
        (accounts.token_account.key, &keys.token_account),
        (accounts.delegate.key, &keys.delegate),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn delegate_token_account_verify_writable_privileges<'me, 'info>(
    accounts: DelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delegate_token_account_verify_signer_privileges<'me, 'info>(
    accounts: DelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_asset_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delegate_token_account_verify_account_privileges<'me, 'info>(
    accounts: DelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delegate_token_account_verify_writable_privileges(accounts)?;
    delegate_token_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RevokeDelegateTokenAccountAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub delegate_asset_admin: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RevokeDelegateTokenAccountKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub delegate_asset_admin: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<RevokeDelegateTokenAccountAccounts<'_, '_>>
for RevokeDelegateTokenAccountKeys {
    fn from(accounts: RevokeDelegateTokenAccountAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            delegate_asset_admin: *accounts.delegate_asset_admin.key,
            token_mint: *accounts.token_mint.key,
            token_account: *accounts.token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RevokeDelegateTokenAccountKeys>
for [AccountMeta; REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeDelegateTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegate_asset_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_account,
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
impl From<[Pubkey; REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for RevokeDelegateTokenAccountKeys {
    fn from(pubkeys: [Pubkey; REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            delegate_asset_admin: pubkeys[2],
            token_mint: pubkeys[3],
            token_account: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<RevokeDelegateTokenAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeDelegateTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.delegate_asset_admin.clone(),
            accounts.token_mint.clone(),
            accounts.token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for RevokeDelegateTokenAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            delegate_asset_admin: &arr[2],
            token_mint: &arr[3],
            token_account: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_DISCM: u8 = 21u8;
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeDelegateTokenAccountIxData;
impl RevokeDelegateTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_delegate_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeDelegateTokenAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RevokeDelegateTokenAccountIxData.try_to_vec()?,
    })
}
pub fn revoke_delegate_token_account_ix(
    keys: RevokeDelegateTokenAccountKeys,
) -> std::io::Result<Instruction> {
    revoke_delegate_token_account_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn revoke_delegate_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeDelegateTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RevokeDelegateTokenAccountKeys = accounts.into();
    let ix = revoke_delegate_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_delegate_token_account_invoke(
    accounts: RevokeDelegateTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    revoke_delegate_token_account_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn revoke_delegate_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeDelegateTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeDelegateTokenAccountKeys = accounts.into();
    let ix = revoke_delegate_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_delegate_token_account_invoke_signed(
    accounts: RevokeDelegateTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_delegate_token_account_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn revoke_delegate_token_account_verify_account_keys(
    accounts: RevokeDelegateTokenAccountAccounts<'_, '_>,
    keys: RevokeDelegateTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.delegate_asset_admin.key, &keys.delegate_asset_admin),
        (accounts.token_mint.key, &keys.token_mint),
        (accounts.token_account.key, &keys.token_account),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn revoke_delegate_token_account_verify_writable_privileges<'me, 'info>(
    accounts: RevokeDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_delegate_token_account_verify_signer_privileges<'me, 'info>(
    accounts: RevokeDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_asset_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_delegate_token_account_verify_account_privileges<'me, 'info>(
    accounts: RevokeDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_delegate_token_account_verify_writable_privileges(accounts)?;
    revoke_delegate_token_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ADMIN_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetAdminAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub old_admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetAdminKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
}
impl From<SetAdminAccounts<'_, '_>> for SetAdminKeys {
    fn from(accounts: SetAdminAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            old_admin: *accounts.old_admin.key,
            new_admin: *accounts.new_admin.key,
        }
    }
}
impl From<SetAdminKeys> for [AccountMeta; SET_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_ADMIN_IX_ACCOUNTS_LEN]> for SetAdminKeys {
    fn from(pubkeys: [Pubkey; SET_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            old_admin: pubkeys[2],
            new_admin: pubkeys[3],
        }
    }
}
impl<'info> From<SetAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.old_admin.clone(),
            accounts.new_admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ADMIN_IX_ACCOUNTS_LEN]>
for SetAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            old_admin: &arr[2],
            new_admin: &arr[3],
        }
    }
}
pub const SET_ADMIN_IX_DISCM: u8 = 22u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SetAdminIxData;
impl SetAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_ADMIN_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetAdminIxData.try_to_vec()?,
    })
}
pub fn set_admin_ix(keys: SetAdminKeys) -> std::io::Result<Instruction> {
    set_admin_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn set_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetAdminKeys = accounts.into();
    let ix = set_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_admin_invoke(accounts: SetAdminAccounts<'_, '_>) -> ProgramResult {
    set_admin_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn set_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetAdminKeys = accounts.into();
    let ix = set_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_admin_invoke_signed(
    accounts: SetAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_admin_invoke_signed_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, seeds)
}
pub fn set_admin_verify_account_keys(
    accounts: SetAdminAccounts<'_, '_>,
    keys: SetAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.old_admin.key, &keys.old_admin),
        (accounts.new_admin.key, &keys.new_admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_admin_verify_writable_privileges<'me, 'info>(
    accounts: SetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.old_admin, accounts.new_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_admin_verify_account_privileges<'me, 'info>(
    accounts: SetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_admin_verify_writable_privileges(accounts)?;
    set_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetSecondaryAdminAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetSecondaryAdminKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub admin: Pubkey,
    pub new_admin: Pubkey,
}
impl From<SetSecondaryAdminAccounts<'_, '_>> for SetSecondaryAdminKeys {
    fn from(accounts: SetSecondaryAdminAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            admin: *accounts.admin.key,
            new_admin: *accounts.new_admin.key,
        }
    }
}
impl From<SetSecondaryAdminKeys> for [AccountMeta; SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetSecondaryAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_admin,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]> for SetSecondaryAdminKeys {
    fn from(pubkeys: [Pubkey; SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            admin: pubkeys[2],
            new_admin: pubkeys[3],
        }
    }
}
impl<'info> From<SetSecondaryAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetSecondaryAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.admin.clone(),
            accounts.new_admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]>
for SetSecondaryAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            admin: &arr[2],
            new_admin: &arr[3],
        }
    }
}
pub const SET_SECONDARY_ADMIN_IX_DISCM: u8 = 23u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetSecondaryAdminIxArgs {
    pub vault_admin_role: VaultAdminRole,
}
impl SetSecondaryAdminIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault_admin_role: VaultAdminRole = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { vault_admin_role })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetSecondaryAdminIxData(pub SetSecondaryAdminIxArgs);
impl From<SetSecondaryAdminIxArgs> for SetSecondaryAdminIxData {
    fn from(args: SetSecondaryAdminIxArgs) -> Self {
        Self(args)
    }
}
impl SetSecondaryAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_SECONDARY_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SetSecondaryAdminIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_SECONDARY_ADMIN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_secondary_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetSecondaryAdminKeys,
    args: SetSecondaryAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetSecondaryAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_secondary_admin_ix(
    keys: SetSecondaryAdminKeys,
    args: SetSecondaryAdminIxArgs,
) -> std::io::Result<Instruction> {
    set_secondary_admin_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn set_secondary_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetSecondaryAdminAccounts<'_, '_>,
    args: SetSecondaryAdminIxArgs,
) -> ProgramResult {
    let keys: SetSecondaryAdminKeys = accounts.into();
    let ix = set_secondary_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_secondary_admin_invoke(
    accounts: SetSecondaryAdminAccounts<'_, '_>,
    args: SetSecondaryAdminIxArgs,
) -> ProgramResult {
    set_secondary_admin_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn set_secondary_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetSecondaryAdminAccounts<'_, '_>,
    args: SetSecondaryAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetSecondaryAdminKeys = accounts.into();
    let ix = set_secondary_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_secondary_admin_invoke_signed(
    accounts: SetSecondaryAdminAccounts<'_, '_>,
    args: SetSecondaryAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_secondary_admin_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_secondary_admin_verify_account_keys(
    accounts: SetSecondaryAdminAccounts<'_, '_>,
    keys: SetSecondaryAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.admin.key, &keys.admin),
        (accounts.new_admin.key, &keys.new_admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_secondary_admin_verify_writable_privileges<'me, 'info>(
    accounts: SetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_secondary_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_secondary_admin_verify_account_privileges<'me, 'info>(
    accounts: SetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_secondary_admin_verify_writable_privileges(accounts)?;
    set_secondary_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_DELEGATION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AddDelegationAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault_operator_delegation: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddDelegationKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub operator: Pubkey,
    pub vault_operator_delegation: Pubkey,
    pub admin: Pubkey,
}
impl From<AddDelegationAccounts<'_, '_>> for AddDelegationKeys {
    fn from(accounts: AddDelegationAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            operator: *accounts.operator.key,
            vault_operator_delegation: *accounts.vault_operator_delegation.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<AddDelegationKeys> for [AccountMeta; ADD_DELEGATION_IX_ACCOUNTS_LEN] {
    fn from(keys: AddDelegationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_operator_delegation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_DELEGATION_IX_ACCOUNTS_LEN]> for AddDelegationKeys {
    fn from(pubkeys: [Pubkey; ADD_DELEGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            operator: pubkeys[2],
            vault_operator_delegation: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<AddDelegationAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_DELEGATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddDelegationAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.operator.clone(),
            accounts.vault_operator_delegation.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_DELEGATION_IX_ACCOUNTS_LEN]>
for AddDelegationAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_DELEGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            operator: &arr[2],
            vault_operator_delegation: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const ADD_DELEGATION_IX_DISCM: u8 = 24u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct AddDelegationIxArgs {
    pub amount: u64,
}
impl AddDelegationIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddDelegationIxData(pub AddDelegationIxArgs);
impl From<AddDelegationIxArgs> for AddDelegationIxData {
    fn from(args: AddDelegationIxArgs) -> Self {
        Self(args)
    }
}
impl AddDelegationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != ADD_DELEGATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AddDelegationIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[ADD_DELEGATION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_delegation_ix_with_program_id(
    program_id: Pubkey,
    keys: AddDelegationKeys,
    args: AddDelegationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_DELEGATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddDelegationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_delegation_ix(
    keys: AddDelegationKeys,
    args: AddDelegationIxArgs,
) -> std::io::Result<Instruction> {
    add_delegation_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn add_delegation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddDelegationAccounts<'_, '_>,
    args: AddDelegationIxArgs,
) -> ProgramResult {
    let keys: AddDelegationKeys = accounts.into();
    let ix = add_delegation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_delegation_invoke(
    accounts: AddDelegationAccounts<'_, '_>,
    args: AddDelegationIxArgs,
) -> ProgramResult {
    add_delegation_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn add_delegation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddDelegationAccounts<'_, '_>,
    args: AddDelegationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddDelegationKeys = accounts.into();
    let ix = add_delegation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_delegation_invoke_signed(
    accounts: AddDelegationAccounts<'_, '_>,
    args: AddDelegationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_delegation_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_delegation_verify_account_keys(
    accounts: AddDelegationAccounts<'_, '_>,
    keys: AddDelegationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.operator.key, &keys.operator),
        (accounts.vault_operator_delegation.key, &keys.vault_operator_delegation),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_delegation_verify_writable_privileges<'me, 'info>(
    accounts: AddDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.vault_operator_delegation] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_delegation_verify_signer_privileges<'me, 'info>(
    accounts: AddDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_delegation_verify_account_privileges<'me, 'info>(
    accounts: AddDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_delegation_verify_writable_privileges(accounts)?;
    add_delegation_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COOLDOWN_DELEGATION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CooldownDelegationAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault_operator_delegation: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CooldownDelegationKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub operator: Pubkey,
    pub vault_operator_delegation: Pubkey,
    pub admin: Pubkey,
}
impl From<CooldownDelegationAccounts<'_, '_>> for CooldownDelegationKeys {
    fn from(accounts: CooldownDelegationAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            operator: *accounts.operator.key,
            vault_operator_delegation: *accounts.vault_operator_delegation.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<CooldownDelegationKeys>
for [AccountMeta; COOLDOWN_DELEGATION_IX_ACCOUNTS_LEN] {
    fn from(keys: CooldownDelegationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_operator_delegation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COOLDOWN_DELEGATION_IX_ACCOUNTS_LEN]> for CooldownDelegationKeys {
    fn from(pubkeys: [Pubkey; COOLDOWN_DELEGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            operator: pubkeys[2],
            vault_operator_delegation: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<CooldownDelegationAccounts<'_, 'info>>
for [AccountInfo<'info>; COOLDOWN_DELEGATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CooldownDelegationAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.operator.clone(),
            accounts.vault_operator_delegation.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COOLDOWN_DELEGATION_IX_ACCOUNTS_LEN]>
for CooldownDelegationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COOLDOWN_DELEGATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            operator: &arr[2],
            vault_operator_delegation: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const COOLDOWN_DELEGATION_IX_DISCM: u8 = 25u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct CooldownDelegationIxArgs {
    pub amount: u64,
}
impl CooldownDelegationIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CooldownDelegationIxData(pub CooldownDelegationIxArgs);
impl From<CooldownDelegationIxArgs> for CooldownDelegationIxData {
    fn from(args: CooldownDelegationIxArgs) -> Self {
        Self(args)
    }
}
impl CooldownDelegationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COOLDOWN_DELEGATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CooldownDelegationIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COOLDOWN_DELEGATION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cooldown_delegation_ix_with_program_id(
    program_id: Pubkey,
    keys: CooldownDelegationKeys,
    args: CooldownDelegationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COOLDOWN_DELEGATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: CooldownDelegationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cooldown_delegation_ix(
    keys: CooldownDelegationKeys,
    args: CooldownDelegationIxArgs,
) -> std::io::Result<Instruction> {
    cooldown_delegation_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn cooldown_delegation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CooldownDelegationAccounts<'_, '_>,
    args: CooldownDelegationIxArgs,
) -> ProgramResult {
    let keys: CooldownDelegationKeys = accounts.into();
    let ix = cooldown_delegation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cooldown_delegation_invoke(
    accounts: CooldownDelegationAccounts<'_, '_>,
    args: CooldownDelegationIxArgs,
) -> ProgramResult {
    cooldown_delegation_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn cooldown_delegation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CooldownDelegationAccounts<'_, '_>,
    args: CooldownDelegationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CooldownDelegationKeys = accounts.into();
    let ix = cooldown_delegation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cooldown_delegation_invoke_signed(
    accounts: CooldownDelegationAccounts<'_, '_>,
    args: CooldownDelegationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cooldown_delegation_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cooldown_delegation_verify_account_keys(
    accounts: CooldownDelegationAccounts<'_, '_>,
    keys: CooldownDelegationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.operator.key, &keys.operator),
        (accounts.vault_operator_delegation.key, &keys.vault_operator_delegation),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cooldown_delegation_verify_writable_privileges<'me, 'info>(
    accounts: CooldownDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.vault_operator_delegation] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cooldown_delegation_verify_signer_privileges<'me, 'info>(
    accounts: CooldownDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cooldown_delegation_verify_account_privileges<'me, 'info>(
    accounts: CooldownDelegationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cooldown_delegation_verify_writable_privileges(accounts)?;
    cooldown_delegation_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_VAULT_BALANCE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateVaultBalanceAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub vrt_mint: &'me AccountInfo<'info>,
    pub vault_fee_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateVaultBalanceKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub vault_token_account: Pubkey,
    pub vrt_mint: Pubkey,
    pub vault_fee_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<UpdateVaultBalanceAccounts<'_, '_>> for UpdateVaultBalanceKeys {
    fn from(accounts: UpdateVaultBalanceAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            vault_token_account: *accounts.vault_token_account.key,
            vrt_mint: *accounts.vrt_mint.key,
            vault_fee_token_account: *accounts.vault_fee_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<UpdateVaultBalanceKeys>
for [AccountMeta; UPDATE_VAULT_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateVaultBalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vrt_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_fee_token_account,
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
impl From<[Pubkey; UPDATE_VAULT_BALANCE_IX_ACCOUNTS_LEN]> for UpdateVaultBalanceKeys {
    fn from(pubkeys: [Pubkey; UPDATE_VAULT_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            vault_token_account: pubkeys[2],
            vrt_mint: pubkeys[3],
            vault_fee_token_account: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateVaultBalanceAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_VAULT_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateVaultBalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.vault_token_account.clone(),
            accounts.vrt_mint.clone(),
            accounts.vault_fee_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_VAULT_BALANCE_IX_ACCOUNTS_LEN]>
for UpdateVaultBalanceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_VAULT_BALANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            vault_token_account: &arr[2],
            vrt_mint: &arr[3],
            vault_fee_token_account: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const UPDATE_VAULT_BALANCE_IX_DISCM: u8 = 26u8;
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateVaultBalanceIxData;
impl UpdateVaultBalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_VAULT_BALANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_VAULT_BALANCE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_vault_balance_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateVaultBalanceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_VAULT_BALANCE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateVaultBalanceIxData.try_to_vec()?,
    })
}
pub fn update_vault_balance_ix(
    keys: UpdateVaultBalanceKeys,
) -> std::io::Result<Instruction> {
    update_vault_balance_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn update_vault_balance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateVaultBalanceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateVaultBalanceKeys = accounts.into();
    let ix = update_vault_balance_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_vault_balance_invoke(
    accounts: UpdateVaultBalanceAccounts<'_, '_>,
) -> ProgramResult {
    update_vault_balance_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn update_vault_balance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateVaultBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateVaultBalanceKeys = accounts.into();
    let ix = update_vault_balance_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_vault_balance_invoke_signed(
    accounts: UpdateVaultBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_vault_balance_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_vault_balance_verify_account_keys(
    accounts: UpdateVaultBalanceAccounts<'_, '_>,
    keys: UpdateVaultBalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.vault_token_account.key, &keys.vault_token_account),
        (accounts.vrt_mint.key, &keys.vrt_mint),
        (accounts.vault_fee_token_account.key, &keys.vault_fee_token_account),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_vault_balance_verify_writable_privileges<'me, 'info>(
    accounts: UpdateVaultBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vrt_mint,
        accounts.vault_fee_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_vault_balance_verify_account_privileges<'me, 'info>(
    accounts: UpdateVaultBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_vault_balance_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultUpdateStateTrackerAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_update_state_tracker: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultUpdateStateTrackerKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub vault_update_state_tracker: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeVaultUpdateStateTrackerAccounts<'_, '_>>
for InitializeVaultUpdateStateTrackerKeys {
    fn from(accounts: InitializeVaultUpdateStateTrackerAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            vault_update_state_tracker: *accounts.vault_update_state_tracker.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeVaultUpdateStateTrackerKeys>
for [AccountMeta; INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultUpdateStateTrackerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_update_state_tracker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
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
impl From<[Pubkey; INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN]>
for InitializeVaultUpdateStateTrackerKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            vault_update_state_tracker: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeVaultUpdateStateTrackerAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultUpdateStateTrackerAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.vault_update_state_tracker.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN]>
for InitializeVaultUpdateStateTrackerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            vault_update_state_tracker: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM: u8 = 27u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct InitializeVaultUpdateStateTrackerIxArgs {
    pub withdrawal_allocation_method: WithdrawalAllocationMethod,
}
impl InitializeVaultUpdateStateTrackerIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let withdrawal_allocation_method: WithdrawalAllocationMethod = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            withdrawal_allocation_method,
        })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultUpdateStateTrackerIxData(
    pub InitializeVaultUpdateStateTrackerIxArgs,
);
impl From<InitializeVaultUpdateStateTrackerIxArgs>
for InitializeVaultUpdateStateTrackerIxData {
    fn from(args: InitializeVaultUpdateStateTrackerIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeVaultUpdateStateTrackerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(InitializeVaultUpdateStateTrackerIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_update_state_tracker_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultUpdateStateTrackerKeys,
    args: InitializeVaultUpdateStateTrackerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeVaultUpdateStateTrackerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_vault_update_state_tracker_ix(
    keys: InitializeVaultUpdateStateTrackerKeys,
    args: InitializeVaultUpdateStateTrackerIxArgs,
) -> std::io::Result<Instruction> {
    initialize_vault_update_state_tracker_ix_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_vault_update_state_tracker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultUpdateStateTrackerAccounts<'_, '_>,
    args: InitializeVaultUpdateStateTrackerIxArgs,
) -> ProgramResult {
    let keys: InitializeVaultUpdateStateTrackerKeys = accounts.into();
    let ix = initialize_vault_update_state_tracker_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_update_state_tracker_invoke(
    accounts: InitializeVaultUpdateStateTrackerAccounts<'_, '_>,
    args: InitializeVaultUpdateStateTrackerIxArgs,
) -> ProgramResult {
    initialize_vault_update_state_tracker_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_vault_update_state_tracker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultUpdateStateTrackerAccounts<'_, '_>,
    args: InitializeVaultUpdateStateTrackerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultUpdateStateTrackerKeys = accounts.into();
    let ix = initialize_vault_update_state_tracker_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_update_state_tracker_invoke_signed(
    accounts: InitializeVaultUpdateStateTrackerAccounts<'_, '_>,
    args: InitializeVaultUpdateStateTrackerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_update_state_tracker_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_vault_update_state_tracker_verify_account_keys(
    accounts: InitializeVaultUpdateStateTrackerAccounts<'_, '_>,
    keys: InitializeVaultUpdateStateTrackerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.vault_update_state_tracker.key, &keys.vault_update_state_tracker),
        (accounts.payer.key, &keys.payer),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_update_state_tracker_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultUpdateStateTrackerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_update_state_tracker,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_update_state_tracker_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultUpdateStateTrackerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_update_state_tracker_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CRANK_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CrankVaultUpdateStateTrackerAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault_operator_delegation: &'me AccountInfo<'info>,
    pub vault_update_state_tracker: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CrankVaultUpdateStateTrackerKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub operator: Pubkey,
    pub vault_operator_delegation: Pubkey,
    pub vault_update_state_tracker: Pubkey,
}
impl From<CrankVaultUpdateStateTrackerAccounts<'_, '_>>
for CrankVaultUpdateStateTrackerKeys {
    fn from(accounts: CrankVaultUpdateStateTrackerAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            operator: *accounts.operator.key,
            vault_operator_delegation: *accounts.vault_operator_delegation.key,
            vault_update_state_tracker: *accounts.vault_update_state_tracker.key,
        }
    }
}
impl From<CrankVaultUpdateStateTrackerKeys>
for [AccountMeta; CRANK_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] {
    fn from(keys: CrankVaultUpdateStateTrackerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_operator_delegation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_update_state_tracker,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CRANK_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN]>
for CrankVaultUpdateStateTrackerKeys {
    fn from(
        pubkeys: [Pubkey; CRANK_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            operator: pubkeys[2],
            vault_operator_delegation: pubkeys[3],
            vault_update_state_tracker: pubkeys[4],
        }
    }
}
impl<'info> From<CrankVaultUpdateStateTrackerAccounts<'_, 'info>>
for [AccountInfo<'info>; CRANK_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CrankVaultUpdateStateTrackerAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.operator.clone(),
            accounts.vault_operator_delegation.clone(),
            accounts.vault_update_state_tracker.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CRANK_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN]>
for CrankVaultUpdateStateTrackerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CRANK_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            operator: &arr[2],
            vault_operator_delegation: &arr[3],
            vault_update_state_tracker: &arr[4],
        }
    }
}
pub const CRANK_VAULT_UPDATE_STATE_TRACKER_IX_DISCM: u8 = 28u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CrankVaultUpdateStateTrackerIxData;
impl CrankVaultUpdateStateTrackerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CRANK_VAULT_UPDATE_STATE_TRACKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CRANK_VAULT_UPDATE_STATE_TRACKER_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn crank_vault_update_state_tracker_ix_with_program_id(
    program_id: Pubkey,
    keys: CrankVaultUpdateStateTrackerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CRANK_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CrankVaultUpdateStateTrackerIxData.try_to_vec()?,
    })
}
pub fn crank_vault_update_state_tracker_ix(
    keys: CrankVaultUpdateStateTrackerKeys,
) -> std::io::Result<Instruction> {
    crank_vault_update_state_tracker_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn crank_vault_update_state_tracker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CrankVaultUpdateStateTrackerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CrankVaultUpdateStateTrackerKeys = accounts.into();
    let ix = crank_vault_update_state_tracker_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn crank_vault_update_state_tracker_invoke(
    accounts: CrankVaultUpdateStateTrackerAccounts<'_, '_>,
) -> ProgramResult {
    crank_vault_update_state_tracker_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
    )
}
pub fn crank_vault_update_state_tracker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CrankVaultUpdateStateTrackerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CrankVaultUpdateStateTrackerKeys = accounts.into();
    let ix = crank_vault_update_state_tracker_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn crank_vault_update_state_tracker_invoke_signed(
    accounts: CrankVaultUpdateStateTrackerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    crank_vault_update_state_tracker_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn crank_vault_update_state_tracker_verify_account_keys(
    accounts: CrankVaultUpdateStateTrackerAccounts<'_, '_>,
    keys: CrankVaultUpdateStateTrackerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.operator.key, &keys.operator),
        (accounts.vault_operator_delegation.key, &keys.vault_operator_delegation),
        (accounts.vault_update_state_tracker.key, &keys.vault_update_state_tracker),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn crank_vault_update_state_tracker_verify_writable_privileges<'me, 'info>(
    accounts: CrankVaultUpdateStateTrackerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_operator_delegation,
        accounts.vault_update_state_tracker,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn crank_vault_update_state_tracker_verify_account_privileges<'me, 'info>(
    accounts: CrankVaultUpdateStateTrackerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    crank_vault_update_state_tracker_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseVaultUpdateStateTrackerAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_update_state_tracker: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CloseVaultUpdateStateTrackerKeys {
    pub config: Pubkey,
    pub vault: Pubkey,
    pub vault_update_state_tracker: Pubkey,
    pub payer: Pubkey,
}
impl From<CloseVaultUpdateStateTrackerAccounts<'_, '_>>
for CloseVaultUpdateStateTrackerKeys {
    fn from(accounts: CloseVaultUpdateStateTrackerAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            vault: *accounts.vault.key,
            vault_update_state_tracker: *accounts.vault_update_state_tracker.key,
            payer: *accounts.payer.key,
        }
    }
}
impl From<CloseVaultUpdateStateTrackerKeys>
for [AccountMeta; CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseVaultUpdateStateTrackerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_update_state_tracker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN]>
for CloseVaultUpdateStateTrackerKeys {
    fn from(
        pubkeys: [Pubkey; CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            vault: pubkeys[1],
            vault_update_state_tracker: pubkeys[2],
            payer: pubkeys[3],
        }
    }
}
impl<'info> From<CloseVaultUpdateStateTrackerAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseVaultUpdateStateTrackerAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.vault.clone(),
            accounts.vault_update_state_tracker.clone(),
            accounts.payer.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN]>
for CloseVaultUpdateStateTrackerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            vault: &arr[1],
            vault_update_state_tracker: &arr[2],
            payer: &arr[3],
        }
    }
}
pub const CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM: u8 = 29u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct CloseVaultUpdateStateTrackerIxArgs {
    pub ncn_epoch: u64,
}
impl CloseVaultUpdateStateTrackerIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ncn_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { ncn_epoch })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseVaultUpdateStateTrackerIxData(pub CloseVaultUpdateStateTrackerIxArgs);
impl From<CloseVaultUpdateStateTrackerIxArgs> for CloseVaultUpdateStateTrackerIxData {
    fn from(args: CloseVaultUpdateStateTrackerIxArgs) -> Self {
        Self(args)
    }
}
impl CloseVaultUpdateStateTrackerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CloseVaultUpdateStateTrackerIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_vault_update_state_tracker_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseVaultUpdateStateTrackerKeys,
    args: CloseVaultUpdateStateTrackerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_VAULT_UPDATE_STATE_TRACKER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CloseVaultUpdateStateTrackerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_vault_update_state_tracker_ix(
    keys: CloseVaultUpdateStateTrackerKeys,
    args: CloseVaultUpdateStateTrackerIxArgs,
) -> std::io::Result<Instruction> {
    close_vault_update_state_tracker_ix_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn close_vault_update_state_tracker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseVaultUpdateStateTrackerAccounts<'_, '_>,
    args: CloseVaultUpdateStateTrackerIxArgs,
) -> ProgramResult {
    let keys: CloseVaultUpdateStateTrackerKeys = accounts.into();
    let ix = close_vault_update_state_tracker_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn close_vault_update_state_tracker_invoke(
    accounts: CloseVaultUpdateStateTrackerAccounts<'_, '_>,
    args: CloseVaultUpdateStateTrackerIxArgs,
) -> ProgramResult {
    close_vault_update_state_tracker_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn close_vault_update_state_tracker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseVaultUpdateStateTrackerAccounts<'_, '_>,
    args: CloseVaultUpdateStateTrackerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseVaultUpdateStateTrackerKeys = accounts.into();
    let ix = close_vault_update_state_tracker_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_vault_update_state_tracker_invoke_signed(
    accounts: CloseVaultUpdateStateTrackerAccounts<'_, '_>,
    args: CloseVaultUpdateStateTrackerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_vault_update_state_tracker_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn close_vault_update_state_tracker_verify_account_keys(
    accounts: CloseVaultUpdateStateTrackerAccounts<'_, '_>,
    keys: CloseVaultUpdateStateTrackerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.vault.key, &keys.vault),
        (accounts.vault_update_state_tracker.key, &keys.vault_update_state_tracker),
        (accounts.payer.key, &keys.payer),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn close_vault_update_state_tracker_verify_writable_privileges<'me, 'info>(
    accounts: CloseVaultUpdateStateTrackerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_update_state_tracker,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_vault_update_state_tracker_verify_signer_privileges<'me, 'info>(
    accounts: CloseVaultUpdateStateTrackerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_vault_update_state_tracker_verify_account_privileges<'me, 'info>(
    accounts: CloseVaultUpdateStateTrackerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_vault_update_state_tracker_verify_writable_privileges(accounts)?;
    close_vault_update_state_tracker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenMetadataAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub vrt_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub mpl_token_metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenMetadataKeys {
    pub vault: Pubkey,
    pub admin: Pubkey,
    pub vrt_mint: Pubkey,
    pub payer: Pubkey,
    pub metadata: Pubkey,
    pub mpl_token_metadata_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTokenMetadataAccounts<'_, '_>> for CreateTokenMetadataKeys {
    fn from(accounts: CreateTokenMetadataAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            admin: *accounts.admin.key,
            vrt_mint: *accounts.vrt_mint.key,
            payer: *accounts.payer.key,
            metadata: *accounts.metadata.key,
            mpl_token_metadata_program: *accounts.mpl_token_metadata_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTokenMetadataKeys>
for [AccountMeta; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vrt_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mpl_token_metadata_program,
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
impl From<[Pubkey; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]> for CreateTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            admin: pubkeys[1],
            vrt_mint: pubkeys[2],
            payer: pubkeys[3],
            metadata: pubkeys[4],
            mpl_token_metadata_program: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.admin.clone(),
            accounts.vrt_mint.clone(),
            accounts.payer.clone(),
            accounts.metadata.clone(),
            accounts.mpl_token_metadata_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for CreateTokenMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            admin: &arr[1],
            vrt_mint: &arr[2],
            payer: &arr[3],
            metadata: &arr[4],
            mpl_token_metadata_program: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CREATE_TOKEN_METADATA_IX_DISCM: u8 = 30u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct CreateTokenMetadataIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl CreateTokenMetadataIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name, symbol, uri })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenMetadataIxData(pub CreateTokenMetadataIxArgs);
impl From<CreateTokenMetadataIxArgs> for CreateTokenMetadataIxData {
    fn from(args: CreateTokenMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTokenMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_TOKEN_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CreateTokenMetadataIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_TOKEN_METADATA_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenMetadataKeys,
    args: CreateTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTokenMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_token_metadata_ix(
    keys: CreateTokenMetadataKeys,
    args: CreateTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    create_token_metadata_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn create_token_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenMetadataAccounts<'_, '_>,
    args: CreateTokenMetadataIxArgs,
) -> ProgramResult {
    let keys: CreateTokenMetadataKeys = accounts.into();
    let ix = create_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_metadata_invoke(
    accounts: CreateTokenMetadataAccounts<'_, '_>,
    args: CreateTokenMetadataIxArgs,
) -> ProgramResult {
    create_token_metadata_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn create_token_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenMetadataAccounts<'_, '_>,
    args: CreateTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenMetadataKeys = accounts.into();
    let ix = create_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_metadata_invoke_signed(
    accounts: CreateTokenMetadataAccounts<'_, '_>,
    args: CreateTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_metadata_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_token_metadata_verify_account_keys(
    accounts: CreateTokenMetadataAccounts<'_, '_>,
    keys: CreateTokenMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.vault.key, &keys.vault),
        (accounts.admin.key, &keys.admin),
        (accounts.vrt_mint.key, &keys.vrt_mint),
        (accounts.payer.key, &keys.payer),
        (accounts.metadata.key, &keys.metadata),
        (accounts.mpl_token_metadata_program.key, &keys.mpl_token_metadata_program),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_token_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_metadata_verify_writable_privileges(accounts)?;
    create_token_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTokenMetadataAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub vrt_mint: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub mpl_token_metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateTokenMetadataKeys {
    pub vault: Pubkey,
    pub admin: Pubkey,
    pub vrt_mint: Pubkey,
    pub metadata: Pubkey,
    pub mpl_token_metadata_program: Pubkey,
}
impl From<UpdateTokenMetadataAccounts<'_, '_>> for UpdateTokenMetadataKeys {
    fn from(accounts: UpdateTokenMetadataAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            admin: *accounts.admin.key,
            vrt_mint: *accounts.vrt_mint.key,
            metadata: *accounts.metadata.key,
            mpl_token_metadata_program: *accounts.mpl_token_metadata_program.key,
        }
    }
}
impl From<UpdateTokenMetadataKeys>
for [AccountMeta; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTokenMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vrt_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mpl_token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]> for UpdateTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            admin: pubkeys[1],
            vrt_mint: pubkeys[2],
            metadata: pubkeys[3],
            mpl_token_metadata_program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.admin.clone(),
            accounts.vrt_mint.clone(),
            accounts.metadata.clone(),
            accounts.mpl_token_metadata_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for UpdateTokenMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            admin: &arr[1],
            vrt_mint: &arr[2],
            metadata: &arr[3],
            mpl_token_metadata_program: &arr[4],
        }
    }
}
pub const UPDATE_TOKEN_METADATA_IX_DISCM: u8 = 31u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct UpdateTokenMetadataIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl UpdateTokenMetadataIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name, symbol, uri })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenMetadataIxData(pub UpdateTokenMetadataIxArgs);
impl From<UpdateTokenMetadataIxArgs> for UpdateTokenMetadataIxData {
    fn from(args: UpdateTokenMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTokenMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_TOKEN_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UpdateTokenMetadataIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_TOKEN_METADATA_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_token_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTokenMetadataKeys,
    args: UpdateTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTokenMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_token_metadata_ix(
    keys: UpdateTokenMetadataKeys,
    args: UpdateTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    update_token_metadata_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn update_token_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
) -> ProgramResult {
    let keys: UpdateTokenMetadataKeys = accounts.into();
    let ix = update_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_token_metadata_invoke(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
) -> ProgramResult {
    update_token_metadata_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts, args)
}
pub fn update_token_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTokenMetadataKeys = accounts.into();
    let ix = update_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_token_metadata_invoke_signed(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    args: UpdateTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_token_metadata_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_token_metadata_verify_account_keys(
    accounts: UpdateTokenMetadataAccounts<'_, '_>,
    keys: UpdateTokenMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.vault.key, &keys.vault),
        (accounts.admin.key, &keys.admin),
        (accounts.vrt_mint.key, &keys.vrt_mint),
        (accounts.metadata.key, &keys.metadata),
        (accounts.mpl_token_metadata_program.key, &keys.mpl_token_metadata_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_token_metadata_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_token_metadata_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_token_metadata_verify_account_privileges<'me, 'info>(
    accounts: UpdateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_token_metadata_verify_writable_privileges(accounts)?;
    update_token_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_CONFIG_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetConfigAdminAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub old_admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetConfigAdminKeys {
    pub config: Pubkey,
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
}
impl From<SetConfigAdminAccounts<'_, '_>> for SetConfigAdminKeys {
    fn from(accounts: SetConfigAdminAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            old_admin: *accounts.old_admin.key,
            new_admin: *accounts.new_admin.key,
        }
    }
}
impl From<SetConfigAdminKeys> for [AccountMeta; SET_CONFIG_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetConfigAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_admin,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_CONFIG_ADMIN_IX_ACCOUNTS_LEN]> for SetConfigAdminKeys {
    fn from(pubkeys: [Pubkey; SET_CONFIG_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            old_admin: pubkeys[1],
            new_admin: pubkeys[2],
        }
    }
}
impl<'info> From<SetConfigAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CONFIG_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetConfigAdminAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.old_admin.clone(), accounts.new_admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_CONFIG_ADMIN_IX_ACCOUNTS_LEN]>
for SetConfigAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_CONFIG_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            old_admin: &arr[1],
            new_admin: &arr[2],
        }
    }
}
pub const SET_CONFIG_ADMIN_IX_DISCM: u8 = 32u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SetConfigAdminIxData;
impl SetConfigAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_CONFIG_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_CONFIG_ADMIN_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_config_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetConfigAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CONFIG_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetConfigAdminIxData.try_to_vec()?,
    })
}
pub fn set_config_admin_ix(keys: SetConfigAdminKeys) -> std::io::Result<Instruction> {
    set_config_admin_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys)
}
pub fn set_config_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetConfigAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetConfigAdminKeys = accounts.into();
    let ix = set_config_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_config_admin_invoke(
    accounts: SetConfigAdminAccounts<'_, '_>,
) -> ProgramResult {
    set_config_admin_invoke_with_program_id(JITO_VAULT_PROGRAM_ID, accounts)
}
pub fn set_config_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetConfigAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetConfigAdminKeys = accounts.into();
    let ix = set_config_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_config_admin_invoke_signed(
    accounts: SetConfigAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_config_admin_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_config_admin_verify_account_keys(
    accounts: SetConfigAdminAccounts<'_, '_>,
    keys: SetConfigAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.old_admin.key, &keys.old_admin),
        (accounts.new_admin.key, &keys.new_admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_config_admin_verify_writable_privileges<'me, 'info>(
    accounts: SetConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_config_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.old_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_config_admin_verify_account_privileges<'me, 'info>(
    accounts: SetConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_config_admin_verify_writable_privileges(accounts)?;
    set_config_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_CONFIG_SECONDARY_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetConfigSecondaryAdminAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetConfigSecondaryAdminKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub new_admin: Pubkey,
}
impl From<SetConfigSecondaryAdminAccounts<'_, '_>> for SetConfigSecondaryAdminKeys {
    fn from(accounts: SetConfigSecondaryAdminAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            new_admin: *accounts.new_admin.key,
        }
    }
}
impl From<SetConfigSecondaryAdminKeys>
for [AccountMeta; SET_CONFIG_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetConfigSecondaryAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_admin,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_CONFIG_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]>
for SetConfigSecondaryAdminKeys {
    fn from(pubkeys: [Pubkey; SET_CONFIG_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            new_admin: pubkeys[2],
        }
    }
}
impl<'info> From<SetConfigSecondaryAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CONFIG_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetConfigSecondaryAdminAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.admin.clone(), accounts.new_admin.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_CONFIG_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]>
for SetConfigSecondaryAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_CONFIG_SECONDARY_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            new_admin: &arr[2],
        }
    }
}
pub const SET_CONFIG_SECONDARY_ADMIN_IX_DISCM: u8 = 33u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetConfigSecondaryAdminIxArgs {
    pub config_admin_role: ConfigAdminRole,
}
impl SetConfigSecondaryAdminIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config_admin_role: ConfigAdminRole = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { config_admin_role })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetConfigSecondaryAdminIxData(pub SetConfigSecondaryAdminIxArgs);
impl From<SetConfigSecondaryAdminIxArgs> for SetConfigSecondaryAdminIxData {
    fn from(args: SetConfigSecondaryAdminIxArgs) -> Self {
        Self(args)
    }
}
impl SetConfigSecondaryAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_CONFIG_SECONDARY_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SetConfigSecondaryAdminIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_CONFIG_SECONDARY_ADMIN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_config_secondary_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetConfigSecondaryAdminKeys,
    args: SetConfigSecondaryAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CONFIG_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetConfigSecondaryAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_config_secondary_admin_ix(
    keys: SetConfigSecondaryAdminKeys,
    args: SetConfigSecondaryAdminIxArgs,
) -> std::io::Result<Instruction> {
    set_config_secondary_admin_ix_with_program_id(JITO_VAULT_PROGRAM_ID, keys, args)
}
pub fn set_config_secondary_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetConfigSecondaryAdminAccounts<'_, '_>,
    args: SetConfigSecondaryAdminIxArgs,
) -> ProgramResult {
    let keys: SetConfigSecondaryAdminKeys = accounts.into();
    let ix = set_config_secondary_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_config_secondary_admin_invoke(
    accounts: SetConfigSecondaryAdminAccounts<'_, '_>,
    args: SetConfigSecondaryAdminIxArgs,
) -> ProgramResult {
    set_config_secondary_admin_invoke_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_config_secondary_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetConfigSecondaryAdminAccounts<'_, '_>,
    args: SetConfigSecondaryAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetConfigSecondaryAdminKeys = accounts.into();
    let ix = set_config_secondary_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_config_secondary_admin_invoke_signed(
    accounts: SetConfigSecondaryAdminAccounts<'_, '_>,
    args: SetConfigSecondaryAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_config_secondary_admin_invoke_signed_with_program_id(
        JITO_VAULT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_config_secondary_admin_verify_account_keys(
    accounts: SetConfigSecondaryAdminAccounts<'_, '_>,
    keys: SetConfigSecondaryAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.admin.key, &keys.admin),
        (accounts.new_admin.key, &keys.new_admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_config_secondary_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetConfigSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_config_secondary_admin_verify_account_privileges<'me, 'info>(
    accounts: SetConfigSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_config_secondary_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
