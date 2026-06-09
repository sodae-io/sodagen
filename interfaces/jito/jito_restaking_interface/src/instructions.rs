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
pub enum JitoRestakingProgramIx {
    InitializeConfig,
    InitializeNcn,
    InitializeOperator(InitializeOperatorIxArgs),
    InitializeNcnVaultSlasherTicket(InitializeNcnVaultSlasherTicketIxArgs),
    InitializeNcnVaultTicket,
    InitializeOperatorVaultTicket,
    InitializeNcnOperatorState,
    WarmupNcnVaultTicket,
    CooldownNcnVaultTicket,
    NcnWarmupOperator,
    NcnCooldownOperator,
    OperatorWarmupNcn,
    OperatorCooldownNcn,
    WarmupNcnVaultSlasherTicket,
    CooldownNcnVaultSlasherTicket,
    WarmupOperatorVaultTicket,
    CooldownOperatorVaultTicket,
    NcnSetAdmin,
    NcnSetSecondaryAdmin(NcnSetSecondaryAdminIxArgs),
    OperatorSetAdmin,
    OperatorSetSecondaryAdmin(OperatorSetSecondaryAdminIxArgs),
    OperatorSetFee(OperatorSetFeeIxArgs),
    NcnDelegateTokenAccount,
    OperatorDelegateTokenAccount,
    SetConfigAdmin,
}
impl JitoRestakingProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            INITIALIZE_CONFIG_IX_DISCM => Ok(Self::InitializeConfig),
            INITIALIZE_NCN_IX_DISCM => Ok(Self::InitializeNcn),
            INITIALIZE_OPERATOR_IX_DISCM => {
                Ok(
                    Self::InitializeOperator(
                        InitializeOperatorIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_DISCM => {
                Ok(
                    Self::InitializeNcnVaultSlasherTicket(
                        InitializeNcnVaultSlasherTicketIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_NCN_VAULT_TICKET_IX_DISCM => Ok(Self::InitializeNcnVaultTicket),
            INITIALIZE_OPERATOR_VAULT_TICKET_IX_DISCM => {
                Ok(Self::InitializeOperatorVaultTicket)
            }
            INITIALIZE_NCN_OPERATOR_STATE_IX_DISCM => {
                Ok(Self::InitializeNcnOperatorState)
            }
            WARMUP_NCN_VAULT_TICKET_IX_DISCM => Ok(Self::WarmupNcnVaultTicket),
            COOLDOWN_NCN_VAULT_TICKET_IX_DISCM => Ok(Self::CooldownNcnVaultTicket),
            NCN_WARMUP_OPERATOR_IX_DISCM => Ok(Self::NcnWarmupOperator),
            NCN_COOLDOWN_OPERATOR_IX_DISCM => Ok(Self::NcnCooldownOperator),
            OPERATOR_WARMUP_NCN_IX_DISCM => Ok(Self::OperatorWarmupNcn),
            OPERATOR_COOLDOWN_NCN_IX_DISCM => Ok(Self::OperatorCooldownNcn),
            WARMUP_NCN_VAULT_SLASHER_TICKET_IX_DISCM => {
                Ok(Self::WarmupNcnVaultSlasherTicket)
            }
            COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_DISCM => {
                Ok(Self::CooldownNcnVaultSlasherTicket)
            }
            WARMUP_OPERATOR_VAULT_TICKET_IX_DISCM => Ok(Self::WarmupOperatorVaultTicket),
            COOLDOWN_OPERATOR_VAULT_TICKET_IX_DISCM => {
                Ok(Self::CooldownOperatorVaultTicket)
            }
            NCN_SET_ADMIN_IX_DISCM => Ok(Self::NcnSetAdmin),
            NCN_SET_SECONDARY_ADMIN_IX_DISCM => {
                Ok(
                    Self::NcnSetSecondaryAdmin(
                        NcnSetSecondaryAdminIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            OPERATOR_SET_ADMIN_IX_DISCM => Ok(Self::OperatorSetAdmin),
            OPERATOR_SET_SECONDARY_ADMIN_IX_DISCM => {
                Ok(
                    Self::OperatorSetSecondaryAdmin(
                        OperatorSetSecondaryAdminIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            OPERATOR_SET_FEE_IX_DISCM => {
                Ok(Self::OperatorSetFee(OperatorSetFeeIxArgs::deserialize(&mut reader)?))
            }
            NCN_DELEGATE_TOKEN_ACCOUNT_IX_DISCM => Ok(Self::NcnDelegateTokenAccount),
            OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_DISCM => {
                Ok(Self::OperatorDelegateTokenAccount)
            }
            SET_CONFIG_ADMIN_IX_DISCM => Ok(Self::SetConfigAdmin),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeConfig => writer.write_all(&[INITIALIZE_CONFIG_IX_DISCM]),
            Self::InitializeNcn => writer.write_all(&[INITIALIZE_NCN_IX_DISCM]),
            Self::InitializeOperator(args) => {
                writer.write_all(&[INITIALIZE_OPERATOR_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeNcnVaultSlasherTicket(args) => {
                writer.write_all(&[INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeNcnVaultTicket => {
                writer.write_all(&[INITIALIZE_NCN_VAULT_TICKET_IX_DISCM])
            }
            Self::InitializeOperatorVaultTicket => {
                writer.write_all(&[INITIALIZE_OPERATOR_VAULT_TICKET_IX_DISCM])
            }
            Self::InitializeNcnOperatorState => {
                writer.write_all(&[INITIALIZE_NCN_OPERATOR_STATE_IX_DISCM])
            }
            Self::WarmupNcnVaultTicket => {
                writer.write_all(&[WARMUP_NCN_VAULT_TICKET_IX_DISCM])
            }
            Self::CooldownNcnVaultTicket => {
                writer.write_all(&[COOLDOWN_NCN_VAULT_TICKET_IX_DISCM])
            }
            Self::NcnWarmupOperator => writer.write_all(&[NCN_WARMUP_OPERATOR_IX_DISCM]),
            Self::NcnCooldownOperator => {
                writer.write_all(&[NCN_COOLDOWN_OPERATOR_IX_DISCM])
            }
            Self::OperatorWarmupNcn => writer.write_all(&[OPERATOR_WARMUP_NCN_IX_DISCM]),
            Self::OperatorCooldownNcn => {
                writer.write_all(&[OPERATOR_COOLDOWN_NCN_IX_DISCM])
            }
            Self::WarmupNcnVaultSlasherTicket => {
                writer.write_all(&[WARMUP_NCN_VAULT_SLASHER_TICKET_IX_DISCM])
            }
            Self::CooldownNcnVaultSlasherTicket => {
                writer.write_all(&[COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_DISCM])
            }
            Self::WarmupOperatorVaultTicket => {
                writer.write_all(&[WARMUP_OPERATOR_VAULT_TICKET_IX_DISCM])
            }
            Self::CooldownOperatorVaultTicket => {
                writer.write_all(&[COOLDOWN_OPERATOR_VAULT_TICKET_IX_DISCM])
            }
            Self::NcnSetAdmin => writer.write_all(&[NCN_SET_ADMIN_IX_DISCM]),
            Self::NcnSetSecondaryAdmin(args) => {
                writer.write_all(&[NCN_SET_SECONDARY_ADMIN_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::OperatorSetAdmin => writer.write_all(&[OPERATOR_SET_ADMIN_IX_DISCM]),
            Self::OperatorSetSecondaryAdmin(args) => {
                writer.write_all(&[OPERATOR_SET_SECONDARY_ADMIN_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::OperatorSetFee(args) => {
                writer.write_all(&[OPERATOR_SET_FEE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::NcnDelegateTokenAccount => {
                writer.write_all(&[NCN_DELEGATE_TOKEN_ACCOUNT_IX_DISCM])
            }
            Self::OperatorDelegateTokenAccount => {
                writer.write_all(&[OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_DISCM])
            }
            Self::SetConfigAdmin => writer.write_all(&[SET_CONFIG_ADMIN_IX_DISCM]),
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
pub const INITIALIZE_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub vault_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeConfigAccounts<'_, '_>> for InitializeConfigKeys {
    fn from(accounts: InitializeConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            vault_program: *accounts.vault_program.key,
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
                pubkey: keys.vault_program,
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
            vault_program: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.admin.clone(),
            accounts.vault_program.clone(),
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
            vault_program: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_CONFIG_IX_DISCM: u8 = 0u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfigIxData;
impl InitializeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_CONFIG_IX_DISCM])
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeConfigIxData.try_to_vec()?,
    })
}
pub fn initialize_config_ix(keys: InitializeConfigKeys) -> std::io::Result<Instruction> {
    initialize_config_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn initialize_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_config_invoke(
    accounts: InitializeConfigAccounts<'_, '_>,
) -> ProgramResult {
    initialize_config_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn initialize_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_config_invoke_signed(
    accounts: InitializeConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_config_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
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
        (accounts.vault_program.key, &keys.vault_program),
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
pub const INITIALIZE_NCN_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeNcnAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeNcnKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub admin: Pubkey,
    pub base: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeNcnAccounts<'_, '_>> for InitializeNcnKeys {
    fn from(accounts: InitializeNcnAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            admin: *accounts.admin.key,
            base: *accounts.base.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeNcnKeys> for [AccountMeta; INITIALIZE_NCN_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeNcnKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ncn,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_NCN_IX_ACCOUNTS_LEN]> for InitializeNcnKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_NCN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            admin: pubkeys[2],
            base: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeNcnAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_NCN_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeNcnAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.admin.clone(),
            accounts.base.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_NCN_IX_ACCOUNTS_LEN]>
for InitializeNcnAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_NCN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            admin: &arr[2],
            base: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INITIALIZE_NCN_IX_DISCM: u8 = 1u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeNcnIxData;
impl InitializeNcnIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_NCN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_NCN_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ncn_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeNcnKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_NCN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeNcnIxData.try_to_vec()?,
    })
}
pub fn initialize_ncn_ix(keys: InitializeNcnKeys) -> std::io::Result<Instruction> {
    initialize_ncn_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn initialize_ncn_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNcnAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeNcnKeys = accounts.into();
    let ix = initialize_ncn_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_ncn_invoke(accounts: InitializeNcnAccounts<'_, '_>) -> ProgramResult {
    initialize_ncn_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn initialize_ncn_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNcnAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeNcnKeys = accounts.into();
    let ix = initialize_ncn_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_ncn_invoke_signed(
    accounts: InitializeNcnAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_ncn_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_ncn_verify_account_keys(
    accounts: InitializeNcnAccounts<'_, '_>,
    keys: InitializeNcnKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.admin.key, &keys.admin),
        (accounts.base.key, &keys.base),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_ncn_verify_writable_privileges<'me, 'info>(
    accounts: InitializeNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.ncn, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_ncn_verify_signer_privileges<'me, 'info>(
    accounts: InitializeNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_ncn_verify_account_privileges<'me, 'info>(
    accounts: InitializeNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_ncn_verify_writable_privileges(accounts)?;
    initialize_ncn_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_OPERATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeOperatorAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeOperatorKeys {
    pub config: Pubkey,
    pub operator: Pubkey,
    pub admin: Pubkey,
    pub base: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeOperatorAccounts<'_, '_>> for InitializeOperatorKeys {
    fn from(accounts: InitializeOperatorAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            operator: *accounts.operator.key,
            admin: *accounts.admin.key,
            base: *accounts.base.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeOperatorKeys>
for [AccountMeta; INITIALIZE_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
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
        ]
    }
}
impl From<[Pubkey; INITIALIZE_OPERATOR_IX_ACCOUNTS_LEN]> for InitializeOperatorKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            operator: pubkeys[1],
            admin: pubkeys[2],
            base: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.operator.clone(),
            accounts.admin.clone(),
            accounts.base.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_OPERATOR_IX_ACCOUNTS_LEN]>
for InitializeOperatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            operator: &arr[1],
            admin: &arr[2],
            base: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INITIALIZE_OPERATOR_IX_DISCM: u8 = 2u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct InitializeOperatorIxArgs {
    pub operator_fee_bps: u16,
}
impl InitializeOperatorIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let operator_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { operator_fee_bps })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeOperatorIxData(pub InitializeOperatorIxArgs);
impl From<InitializeOperatorIxArgs> for InitializeOperatorIxData {
    fn from(args: InitializeOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_OPERATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(InitializeOperatorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_OPERATOR_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeOperatorKeys,
    args: InitializeOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_operator_ix(
    keys: InitializeOperatorKeys,
    args: InitializeOperatorIxArgs,
) -> std::io::Result<Instruction> {
    initialize_operator_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys, args)
}
pub fn initialize_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOperatorAccounts<'_, '_>,
    args: InitializeOperatorIxArgs,
) -> ProgramResult {
    let keys: InitializeOperatorKeys = accounts.into();
    let ix = initialize_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_operator_invoke(
    accounts: InitializeOperatorAccounts<'_, '_>,
    args: InitializeOperatorIxArgs,
) -> ProgramResult {
    initialize_operator_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts, args)
}
pub fn initialize_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOperatorAccounts<'_, '_>,
    args: InitializeOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeOperatorKeys = accounts.into();
    let ix = initialize_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_operator_invoke_signed(
    accounts: InitializeOperatorAccounts<'_, '_>,
    args: InitializeOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_operator_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_operator_verify_account_keys(
    accounts: InitializeOperatorAccounts<'_, '_>,
    keys: InitializeOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.operator.key, &keys.operator),
        (accounts.admin.key, &keys.admin),
        (accounts.base.key, &keys.base),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_operator_verify_writable_privileges<'me, 'info>(
    accounts: InitializeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config, accounts.operator, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_operator_verify_signer_privileges<'me, 'info>(
    accounts: InitializeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_operator_verify_account_privileges<'me, 'info>(
    accounts: InitializeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_operator_verify_writable_privileges(accounts)?;
    initialize_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeNcnVaultSlasherTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub slasher: &'me AccountInfo<'info>,
    pub ncn_vault_ticket: &'me AccountInfo<'info>,
    pub ncn_vault_slasher_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeNcnVaultSlasherTicketKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub vault: Pubkey,
    pub slasher: Pubkey,
    pub ncn_vault_ticket: Pubkey,
    pub ncn_vault_slasher_ticket: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeNcnVaultSlasherTicketAccounts<'_, '_>>
for InitializeNcnVaultSlasherTicketKeys {
    fn from(accounts: InitializeNcnVaultSlasherTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            vault: *accounts.vault.key,
            slasher: *accounts.slasher.key,
            ncn_vault_ticket: *accounts.ncn_vault_ticket.key,
            ncn_vault_slasher_ticket: *accounts.ncn_vault_slasher_ticket.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeNcnVaultSlasherTicketKeys>
for [AccountMeta; INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeNcnVaultSlasherTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slasher,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_ticket,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_slasher_ticket,
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
impl From<[Pubkey; INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for InitializeNcnVaultSlasherTicketKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            vault: pubkeys[2],
            slasher: pubkeys[3],
            ncn_vault_ticket: pubkeys[4],
            ncn_vault_slasher_ticket: pubkeys[5],
            admin: pubkeys[6],
            payer: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeNcnVaultSlasherTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeNcnVaultSlasherTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.vault.clone(),
            accounts.slasher.clone(),
            accounts.ncn_vault_ticket.clone(),
            accounts.ncn_vault_slasher_ticket.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for InitializeNcnVaultSlasherTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            vault: &arr[2],
            slasher: &arr[3],
            ncn_vault_ticket: &arr[4],
            ncn_vault_slasher_ticket: &arr[5],
            admin: &arr[6],
            payer: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_DISCM: u8 = 3u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct InitializeNcnVaultSlasherTicketIxArgs {
    pub max_slashable_per_epoch: u64,
}
impl InitializeNcnVaultSlasherTicketIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_slashable_per_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { max_slashable_per_epoch })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeNcnVaultSlasherTicketIxData(
    pub InitializeNcnVaultSlasherTicketIxArgs,
);
impl From<InitializeNcnVaultSlasherTicketIxArgs>
for InitializeNcnVaultSlasherTicketIxData {
    fn from(args: InitializeNcnVaultSlasherTicketIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeNcnVaultSlasherTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(InitializeNcnVaultSlasherTicketIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ncn_vault_slasher_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeNcnVaultSlasherTicketKeys,
    args: InitializeNcnVaultSlasherTicketIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeNcnVaultSlasherTicketIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_ncn_vault_slasher_ticket_ix(
    keys: InitializeNcnVaultSlasherTicketKeys,
    args: InitializeNcnVaultSlasherTicketIxArgs,
) -> std::io::Result<Instruction> {
    initialize_ncn_vault_slasher_ticket_ix_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_ncn_vault_slasher_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNcnVaultSlasherTicketAccounts<'_, '_>,
    args: InitializeNcnVaultSlasherTicketIxArgs,
) -> ProgramResult {
    let keys: InitializeNcnVaultSlasherTicketKeys = accounts.into();
    let ix = initialize_ncn_vault_slasher_ticket_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_ncn_vault_slasher_ticket_invoke(
    accounts: InitializeNcnVaultSlasherTicketAccounts<'_, '_>,
    args: InitializeNcnVaultSlasherTicketIxArgs,
) -> ProgramResult {
    initialize_ncn_vault_slasher_ticket_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_ncn_vault_slasher_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNcnVaultSlasherTicketAccounts<'_, '_>,
    args: InitializeNcnVaultSlasherTicketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeNcnVaultSlasherTicketKeys = accounts.into();
    let ix = initialize_ncn_vault_slasher_ticket_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_ncn_vault_slasher_ticket_invoke_signed(
    accounts: InitializeNcnVaultSlasherTicketAccounts<'_, '_>,
    args: InitializeNcnVaultSlasherTicketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_ncn_vault_slasher_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_ncn_vault_slasher_ticket_verify_account_keys(
    accounts: InitializeNcnVaultSlasherTicketAccounts<'_, '_>,
    keys: InitializeNcnVaultSlasherTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.vault.key, &keys.vault),
        (accounts.slasher.key, &keys.slasher),
        (accounts.ncn_vault_ticket.key, &keys.ncn_vault_ticket),
        (accounts.ncn_vault_slasher_ticket.key, &keys.ncn_vault_slasher_ticket),
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
pub fn initialize_ncn_vault_slasher_ticket_verify_writable_privileges<'me, 'info>(
    accounts: InitializeNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.ncn,
        accounts.ncn_vault_slasher_ticket,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_ncn_vault_slasher_ticket_verify_signer_privileges<'me, 'info>(
    accounts: InitializeNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_ncn_vault_slasher_ticket_verify_account_privileges<'me, 'info>(
    accounts: InitializeNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_ncn_vault_slasher_ticket_verify_writable_privileges(accounts)?;
    initialize_ncn_vault_slasher_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeNcnVaultTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn_vault_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeNcnVaultTicketKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub vault: Pubkey,
    pub ncn_vault_ticket: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeNcnVaultTicketAccounts<'_, '_>> for InitializeNcnVaultTicketKeys {
    fn from(accounts: InitializeNcnVaultTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            vault: *accounts.vault.key,
            ncn_vault_ticket: *accounts.ncn_vault_ticket.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeNcnVaultTicketKeys>
for [AccountMeta; INITIALIZE_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeNcnVaultTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_ticket,
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
impl From<[Pubkey; INITIALIZE_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for InitializeNcnVaultTicketKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            vault: pubkeys[2],
            ncn_vault_ticket: pubkeys[3],
            admin: pubkeys[4],
            payer: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeNcnVaultTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeNcnVaultTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.vault.clone(),
            accounts.ncn_vault_ticket.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for InitializeNcnVaultTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            vault: &arr[2],
            ncn_vault_ticket: &arr[3],
            admin: &arr[4],
            payer: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_NCN_VAULT_TICKET_IX_DISCM: u8 = 4u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeNcnVaultTicketIxData;
impl InitializeNcnVaultTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_NCN_VAULT_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_NCN_VAULT_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ncn_vault_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeNcnVaultTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeNcnVaultTicketIxData.try_to_vec()?,
    })
}
pub fn initialize_ncn_vault_ticket_ix(
    keys: InitializeNcnVaultTicketKeys,
) -> std::io::Result<Instruction> {
    initialize_ncn_vault_ticket_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn initialize_ncn_vault_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNcnVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeNcnVaultTicketKeys = accounts.into();
    let ix = initialize_ncn_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_ncn_vault_ticket_invoke(
    accounts: InitializeNcnVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    initialize_ncn_vault_ticket_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_ncn_vault_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNcnVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeNcnVaultTicketKeys = accounts.into();
    let ix = initialize_ncn_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_ncn_vault_ticket_invoke_signed(
    accounts: InitializeNcnVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_ncn_vault_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_ncn_vault_ticket_verify_account_keys(
    accounts: InitializeNcnVaultTicketAccounts<'_, '_>,
    keys: InitializeNcnVaultTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn_vault_ticket.key, &keys.ncn_vault_ticket),
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
pub fn initialize_ncn_vault_ticket_verify_writable_privileges<'me, 'info>(
    accounts: InitializeNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn, accounts.ncn_vault_ticket, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_ncn_vault_ticket_verify_signer_privileges<'me, 'info>(
    accounts: InitializeNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_ncn_vault_ticket_verify_account_privileges<'me, 'info>(
    accounts: InitializeNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_ncn_vault_ticket_verify_writable_privileges(accounts)?;
    initialize_ncn_vault_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeOperatorVaultTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub operator_vault_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeOperatorVaultTicketKeys {
    pub config: Pubkey,
    pub operator: Pubkey,
    pub vault: Pubkey,
    pub operator_vault_ticket: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeOperatorVaultTicketAccounts<'_, '_>>
for InitializeOperatorVaultTicketKeys {
    fn from(accounts: InitializeOperatorVaultTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            operator: *accounts.operator.key,
            vault: *accounts.vault.key,
            operator_vault_ticket: *accounts.operator_vault_ticket.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeOperatorVaultTicketKeys>
for [AccountMeta; INITIALIZE_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeOperatorVaultTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator_vault_ticket,
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
impl From<[Pubkey; INITIALIZE_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for InitializeOperatorVaultTicketKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            operator: pubkeys[1],
            vault: pubkeys[2],
            operator_vault_ticket: pubkeys[3],
            admin: pubkeys[4],
            payer: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeOperatorVaultTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeOperatorVaultTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.operator.clone(),
            accounts.vault.clone(),
            accounts.operator_vault_ticket.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for InitializeOperatorVaultTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            operator: &arr[1],
            vault: &arr[2],
            operator_vault_ticket: &arr[3],
            admin: &arr[4],
            payer: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_OPERATOR_VAULT_TICKET_IX_DISCM: u8 = 5u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeOperatorVaultTicketIxData;
impl InitializeOperatorVaultTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_OPERATOR_VAULT_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_OPERATOR_VAULT_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_operator_vault_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeOperatorVaultTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeOperatorVaultTicketIxData.try_to_vec()?,
    })
}
pub fn initialize_operator_vault_ticket_ix(
    keys: InitializeOperatorVaultTicketKeys,
) -> std::io::Result<Instruction> {
    initialize_operator_vault_ticket_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn initialize_operator_vault_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOperatorVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeOperatorVaultTicketKeys = accounts.into();
    let ix = initialize_operator_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_operator_vault_ticket_invoke(
    accounts: InitializeOperatorVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    initialize_operator_vault_ticket_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_operator_vault_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOperatorVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeOperatorVaultTicketKeys = accounts.into();
    let ix = initialize_operator_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_operator_vault_ticket_invoke_signed(
    accounts: InitializeOperatorVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_operator_vault_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_operator_vault_ticket_verify_account_keys(
    accounts: InitializeOperatorVaultTicketAccounts<'_, '_>,
    keys: InitializeOperatorVaultTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.operator.key, &keys.operator),
        (accounts.vault.key, &keys.vault),
        (accounts.operator_vault_ticket.key, &keys.operator_vault_ticket),
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
pub fn initialize_operator_vault_ticket_verify_writable_privileges<'me, 'info>(
    accounts: InitializeOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.operator,
        accounts.operator_vault_ticket,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_operator_vault_ticket_verify_signer_privileges<'me, 'info>(
    accounts: InitializeOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_operator_vault_ticket_verify_account_privileges<'me, 'info>(
    accounts: InitializeOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_operator_vault_ticket_verify_writable_privileges(accounts)?;
    initialize_operator_vault_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_NCN_OPERATOR_STATE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeNcnOperatorStateAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub ncn_operator_state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeNcnOperatorStateKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub operator: Pubkey,
    pub ncn_operator_state: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeNcnOperatorStateAccounts<'_, '_>>
for InitializeNcnOperatorStateKeys {
    fn from(accounts: InitializeNcnOperatorStateAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            operator: *accounts.operator.key,
            ncn_operator_state: *accounts.ncn_operator_state.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeNcnOperatorStateKeys>
for [AccountMeta; INITIALIZE_NCN_OPERATOR_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeNcnOperatorStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ncn_operator_state,
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
impl From<[Pubkey; INITIALIZE_NCN_OPERATOR_STATE_IX_ACCOUNTS_LEN]>
for InitializeNcnOperatorStateKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_NCN_OPERATOR_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            operator: pubkeys[2],
            ncn_operator_state: pubkeys[3],
            admin: pubkeys[4],
            payer: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeNcnOperatorStateAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_NCN_OPERATOR_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeNcnOperatorStateAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.operator.clone(),
            accounts.ncn_operator_state.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_NCN_OPERATOR_STATE_IX_ACCOUNTS_LEN]>
for InitializeNcnOperatorStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_NCN_OPERATOR_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            operator: &arr[2],
            ncn_operator_state: &arr[3],
            admin: &arr[4],
            payer: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_NCN_OPERATOR_STATE_IX_DISCM: u8 = 6u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeNcnOperatorStateIxData;
impl InitializeNcnOperatorStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_NCN_OPERATOR_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_NCN_OPERATOR_STATE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ncn_operator_state_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeNcnOperatorStateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_NCN_OPERATOR_STATE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeNcnOperatorStateIxData.try_to_vec()?,
    })
}
pub fn initialize_ncn_operator_state_ix(
    keys: InitializeNcnOperatorStateKeys,
) -> std::io::Result<Instruction> {
    initialize_ncn_operator_state_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn initialize_ncn_operator_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNcnOperatorStateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeNcnOperatorStateKeys = accounts.into();
    let ix = initialize_ncn_operator_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_ncn_operator_state_invoke(
    accounts: InitializeNcnOperatorStateAccounts<'_, '_>,
) -> ProgramResult {
    initialize_ncn_operator_state_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_ncn_operator_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNcnOperatorStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeNcnOperatorStateKeys = accounts.into();
    let ix = initialize_ncn_operator_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_ncn_operator_state_invoke_signed(
    accounts: InitializeNcnOperatorStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_ncn_operator_state_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_ncn_operator_state_verify_account_keys(
    accounts: InitializeNcnOperatorStateAccounts<'_, '_>,
    keys: InitializeNcnOperatorStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.operator.key, &keys.operator),
        (accounts.ncn_operator_state.key, &keys.ncn_operator_state),
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
pub fn initialize_ncn_operator_state_verify_writable_privileges<'me, 'info>(
    accounts: InitializeNcnOperatorStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.ncn,
        accounts.operator,
        accounts.ncn_operator_state,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_ncn_operator_state_verify_signer_privileges<'me, 'info>(
    accounts: InitializeNcnOperatorStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_ncn_operator_state_verify_account_privileges<'me, 'info>(
    accounts: InitializeNcnOperatorStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_ncn_operator_state_verify_writable_privileges(accounts)?;
    initialize_ncn_operator_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WARMUP_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct WarmupNcnVaultTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn_vault_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WarmupNcnVaultTicketKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub vault: Pubkey,
    pub ncn_vault_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<WarmupNcnVaultTicketAccounts<'_, '_>> for WarmupNcnVaultTicketKeys {
    fn from(accounts: WarmupNcnVaultTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            vault: *accounts.vault.key,
            ncn_vault_ticket: *accounts.ncn_vault_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<WarmupNcnVaultTicketKeys>
for [AccountMeta; WARMUP_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: WarmupNcnVaultTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_ticket,
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
impl From<[Pubkey; WARMUP_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for WarmupNcnVaultTicketKeys {
    fn from(pubkeys: [Pubkey; WARMUP_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            vault: pubkeys[2],
            ncn_vault_ticket: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<WarmupNcnVaultTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; WARMUP_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: WarmupNcnVaultTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.vault.clone(),
            accounts.ncn_vault_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WARMUP_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for WarmupNcnVaultTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WARMUP_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            vault: &arr[2],
            ncn_vault_ticket: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const WARMUP_NCN_VAULT_TICKET_IX_DISCM: u8 = 7u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WarmupNcnVaultTicketIxData;
impl WarmupNcnVaultTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WARMUP_NCN_VAULT_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WARMUP_NCN_VAULT_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn warmup_ncn_vault_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: WarmupNcnVaultTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WARMUP_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WarmupNcnVaultTicketIxData.try_to_vec()?,
    })
}
pub fn warmup_ncn_vault_ticket_ix(
    keys: WarmupNcnVaultTicketKeys,
) -> std::io::Result<Instruction> {
    warmup_ncn_vault_ticket_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn warmup_ncn_vault_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WarmupNcnVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WarmupNcnVaultTicketKeys = accounts.into();
    let ix = warmup_ncn_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn warmup_ncn_vault_ticket_invoke(
    accounts: WarmupNcnVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    warmup_ncn_vault_ticket_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn warmup_ncn_vault_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WarmupNcnVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WarmupNcnVaultTicketKeys = accounts.into();
    let ix = warmup_ncn_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn warmup_ncn_vault_ticket_invoke_signed(
    accounts: WarmupNcnVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    warmup_ncn_vault_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn warmup_ncn_vault_ticket_verify_account_keys(
    accounts: WarmupNcnVaultTicketAccounts<'_, '_>,
    keys: WarmupNcnVaultTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn_vault_ticket.key, &keys.ncn_vault_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn warmup_ncn_vault_ticket_verify_writable_privileges<'me, 'info>(
    accounts: WarmupNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn_vault_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn warmup_ncn_vault_ticket_verify_signer_privileges<'me, 'info>(
    accounts: WarmupNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn warmup_ncn_vault_ticket_verify_account_privileges<'me, 'info>(
    accounts: WarmupNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    warmup_ncn_vault_ticket_verify_writable_privileges(accounts)?;
    warmup_ncn_vault_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COOLDOWN_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CooldownNcnVaultTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub ncn_vault_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CooldownNcnVaultTicketKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub vault: Pubkey,
    pub ncn_vault_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<CooldownNcnVaultTicketAccounts<'_, '_>> for CooldownNcnVaultTicketKeys {
    fn from(accounts: CooldownNcnVaultTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            vault: *accounts.vault.key,
            ncn_vault_ticket: *accounts.ncn_vault_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<CooldownNcnVaultTicketKeys>
for [AccountMeta; COOLDOWN_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CooldownNcnVaultTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_ticket,
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
impl From<[Pubkey; COOLDOWN_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for CooldownNcnVaultTicketKeys {
    fn from(pubkeys: [Pubkey; COOLDOWN_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            vault: pubkeys[2],
            ncn_vault_ticket: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<CooldownNcnVaultTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; COOLDOWN_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CooldownNcnVaultTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.vault.clone(),
            accounts.ncn_vault_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COOLDOWN_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for CooldownNcnVaultTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COOLDOWN_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            vault: &arr[2],
            ncn_vault_ticket: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const COOLDOWN_NCN_VAULT_TICKET_IX_DISCM: u8 = 8u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CooldownNcnVaultTicketIxData;
impl CooldownNcnVaultTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COOLDOWN_NCN_VAULT_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COOLDOWN_NCN_VAULT_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cooldown_ncn_vault_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: CooldownNcnVaultTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COOLDOWN_NCN_VAULT_TICKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CooldownNcnVaultTicketIxData.try_to_vec()?,
    })
}
pub fn cooldown_ncn_vault_ticket_ix(
    keys: CooldownNcnVaultTicketKeys,
) -> std::io::Result<Instruction> {
    cooldown_ncn_vault_ticket_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn cooldown_ncn_vault_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CooldownNcnVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CooldownNcnVaultTicketKeys = accounts.into();
    let ix = cooldown_ncn_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cooldown_ncn_vault_ticket_invoke(
    accounts: CooldownNcnVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    cooldown_ncn_vault_ticket_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn cooldown_ncn_vault_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CooldownNcnVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CooldownNcnVaultTicketKeys = accounts.into();
    let ix = cooldown_ncn_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cooldown_ncn_vault_ticket_invoke_signed(
    accounts: CooldownNcnVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cooldown_ncn_vault_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cooldown_ncn_vault_ticket_verify_account_keys(
    accounts: CooldownNcnVaultTicketAccounts<'_, '_>,
    keys: CooldownNcnVaultTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.vault.key, &keys.vault),
        (accounts.ncn_vault_ticket.key, &keys.ncn_vault_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cooldown_ncn_vault_ticket_verify_writable_privileges<'me, 'info>(
    accounts: CooldownNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn_vault_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cooldown_ncn_vault_ticket_verify_signer_privileges<'me, 'info>(
    accounts: CooldownNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cooldown_ncn_vault_ticket_verify_account_privileges<'me, 'info>(
    accounts: CooldownNcnVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cooldown_ncn_vault_ticket_verify_writable_privileges(accounts)?;
    cooldown_ncn_vault_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NCN_WARMUP_OPERATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct NcnWarmupOperatorAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub ncn_operator_state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct NcnWarmupOperatorKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub operator: Pubkey,
    pub ncn_operator_state: Pubkey,
    pub admin: Pubkey,
}
impl From<NcnWarmupOperatorAccounts<'_, '_>> for NcnWarmupOperatorKeys {
    fn from(accounts: NcnWarmupOperatorAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            operator: *accounts.operator.key,
            ncn_operator_state: *accounts.ncn_operator_state.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<NcnWarmupOperatorKeys> for [AccountMeta; NCN_WARMUP_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: NcnWarmupOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_operator_state,
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
impl From<[Pubkey; NCN_WARMUP_OPERATOR_IX_ACCOUNTS_LEN]> for NcnWarmupOperatorKeys {
    fn from(pubkeys: [Pubkey; NCN_WARMUP_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            operator: pubkeys[2],
            ncn_operator_state: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<NcnWarmupOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; NCN_WARMUP_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: NcnWarmupOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.operator.clone(),
            accounts.ncn_operator_state.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NCN_WARMUP_OPERATOR_IX_ACCOUNTS_LEN]>
for NcnWarmupOperatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; NCN_WARMUP_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            operator: &arr[2],
            ncn_operator_state: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const NCN_WARMUP_OPERATOR_IX_DISCM: u8 = 9u8;
#[derive(Clone, Debug, PartialEq)]
pub struct NcnWarmupOperatorIxData;
impl NcnWarmupOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NCN_WARMUP_OPERATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NCN_WARMUP_OPERATOR_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn ncn_warmup_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: NcnWarmupOperatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NCN_WARMUP_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: NcnWarmupOperatorIxData.try_to_vec()?,
    })
}
pub fn ncn_warmup_operator_ix(
    keys: NcnWarmupOperatorKeys,
) -> std::io::Result<Instruction> {
    ncn_warmup_operator_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn ncn_warmup_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NcnWarmupOperatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: NcnWarmupOperatorKeys = accounts.into();
    let ix = ncn_warmup_operator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn ncn_warmup_operator_invoke(
    accounts: NcnWarmupOperatorAccounts<'_, '_>,
) -> ProgramResult {
    ncn_warmup_operator_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn ncn_warmup_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NcnWarmupOperatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NcnWarmupOperatorKeys = accounts.into();
    let ix = ncn_warmup_operator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn ncn_warmup_operator_invoke_signed(
    accounts: NcnWarmupOperatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    ncn_warmup_operator_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn ncn_warmup_operator_verify_account_keys(
    accounts: NcnWarmupOperatorAccounts<'_, '_>,
    keys: NcnWarmupOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.operator.key, &keys.operator),
        (accounts.ncn_operator_state.key, &keys.ncn_operator_state),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn ncn_warmup_operator_verify_writable_privileges<'me, 'info>(
    accounts: NcnWarmupOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn_operator_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn ncn_warmup_operator_verify_signer_privileges<'me, 'info>(
    accounts: NcnWarmupOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn ncn_warmup_operator_verify_account_privileges<'me, 'info>(
    accounts: NcnWarmupOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    ncn_warmup_operator_verify_writable_privileges(accounts)?;
    ncn_warmup_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NCN_COOLDOWN_OPERATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct NcnCooldownOperatorAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub ncn_operator_state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct NcnCooldownOperatorKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub operator: Pubkey,
    pub ncn_operator_state: Pubkey,
    pub admin: Pubkey,
}
impl From<NcnCooldownOperatorAccounts<'_, '_>> for NcnCooldownOperatorKeys {
    fn from(accounts: NcnCooldownOperatorAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            operator: *accounts.operator.key,
            ncn_operator_state: *accounts.ncn_operator_state.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<NcnCooldownOperatorKeys>
for [AccountMeta; NCN_COOLDOWN_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: NcnCooldownOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_operator_state,
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
impl From<[Pubkey; NCN_COOLDOWN_OPERATOR_IX_ACCOUNTS_LEN]> for NcnCooldownOperatorKeys {
    fn from(pubkeys: [Pubkey; NCN_COOLDOWN_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            operator: pubkeys[2],
            ncn_operator_state: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<NcnCooldownOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; NCN_COOLDOWN_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: NcnCooldownOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.operator.clone(),
            accounts.ncn_operator_state.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NCN_COOLDOWN_OPERATOR_IX_ACCOUNTS_LEN]>
for NcnCooldownOperatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; NCN_COOLDOWN_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            operator: &arr[2],
            ncn_operator_state: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const NCN_COOLDOWN_OPERATOR_IX_DISCM: u8 = 10u8;
#[derive(Clone, Debug, PartialEq)]
pub struct NcnCooldownOperatorIxData;
impl NcnCooldownOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NCN_COOLDOWN_OPERATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NCN_COOLDOWN_OPERATOR_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn ncn_cooldown_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: NcnCooldownOperatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NCN_COOLDOWN_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: NcnCooldownOperatorIxData.try_to_vec()?,
    })
}
pub fn ncn_cooldown_operator_ix(
    keys: NcnCooldownOperatorKeys,
) -> std::io::Result<Instruction> {
    ncn_cooldown_operator_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn ncn_cooldown_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NcnCooldownOperatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: NcnCooldownOperatorKeys = accounts.into();
    let ix = ncn_cooldown_operator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn ncn_cooldown_operator_invoke(
    accounts: NcnCooldownOperatorAccounts<'_, '_>,
) -> ProgramResult {
    ncn_cooldown_operator_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn ncn_cooldown_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NcnCooldownOperatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NcnCooldownOperatorKeys = accounts.into();
    let ix = ncn_cooldown_operator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn ncn_cooldown_operator_invoke_signed(
    accounts: NcnCooldownOperatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    ncn_cooldown_operator_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn ncn_cooldown_operator_verify_account_keys(
    accounts: NcnCooldownOperatorAccounts<'_, '_>,
    keys: NcnCooldownOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.operator.key, &keys.operator),
        (accounts.ncn_operator_state.key, &keys.ncn_operator_state),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn ncn_cooldown_operator_verify_writable_privileges<'me, 'info>(
    accounts: NcnCooldownOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn_operator_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn ncn_cooldown_operator_verify_signer_privileges<'me, 'info>(
    accounts: NcnCooldownOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn ncn_cooldown_operator_verify_account_privileges<'me, 'info>(
    accounts: NcnCooldownOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    ncn_cooldown_operator_verify_writable_privileges(accounts)?;
    ncn_cooldown_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPERATOR_WARMUP_NCN_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct OperatorWarmupNcnAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub ncn_operator_state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct OperatorWarmupNcnKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub operator: Pubkey,
    pub ncn_operator_state: Pubkey,
    pub admin: Pubkey,
}
impl From<OperatorWarmupNcnAccounts<'_, '_>> for OperatorWarmupNcnKeys {
    fn from(accounts: OperatorWarmupNcnAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            operator: *accounts.operator.key,
            ncn_operator_state: *accounts.ncn_operator_state.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<OperatorWarmupNcnKeys> for [AccountMeta; OPERATOR_WARMUP_NCN_IX_ACCOUNTS_LEN] {
    fn from(keys: OperatorWarmupNcnKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_operator_state,
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
impl From<[Pubkey; OPERATOR_WARMUP_NCN_IX_ACCOUNTS_LEN]> for OperatorWarmupNcnKeys {
    fn from(pubkeys: [Pubkey; OPERATOR_WARMUP_NCN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            operator: pubkeys[2],
            ncn_operator_state: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<OperatorWarmupNcnAccounts<'_, 'info>>
for [AccountInfo<'info>; OPERATOR_WARMUP_NCN_IX_ACCOUNTS_LEN] {
    fn from(accounts: OperatorWarmupNcnAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.operator.clone(),
            accounts.ncn_operator_state.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPERATOR_WARMUP_NCN_IX_ACCOUNTS_LEN]>
for OperatorWarmupNcnAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPERATOR_WARMUP_NCN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            operator: &arr[2],
            ncn_operator_state: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const OPERATOR_WARMUP_NCN_IX_DISCM: u8 = 11u8;
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorWarmupNcnIxData;
impl OperatorWarmupNcnIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPERATOR_WARMUP_NCN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPERATOR_WARMUP_NCN_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn operator_warmup_ncn_ix_with_program_id(
    program_id: Pubkey,
    keys: OperatorWarmupNcnKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPERATOR_WARMUP_NCN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: OperatorWarmupNcnIxData.try_to_vec()?,
    })
}
pub fn operator_warmup_ncn_ix(
    keys: OperatorWarmupNcnKeys,
) -> std::io::Result<Instruction> {
    operator_warmup_ncn_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn operator_warmup_ncn_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OperatorWarmupNcnAccounts<'_, '_>,
) -> ProgramResult {
    let keys: OperatorWarmupNcnKeys = accounts.into();
    let ix = operator_warmup_ncn_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn operator_warmup_ncn_invoke(
    accounts: OperatorWarmupNcnAccounts<'_, '_>,
) -> ProgramResult {
    operator_warmup_ncn_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn operator_warmup_ncn_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OperatorWarmupNcnAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OperatorWarmupNcnKeys = accounts.into();
    let ix = operator_warmup_ncn_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn operator_warmup_ncn_invoke_signed(
    accounts: OperatorWarmupNcnAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    operator_warmup_ncn_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn operator_warmup_ncn_verify_account_keys(
    accounts: OperatorWarmupNcnAccounts<'_, '_>,
    keys: OperatorWarmupNcnKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.operator.key, &keys.operator),
        (accounts.ncn_operator_state.key, &keys.ncn_operator_state),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn operator_warmup_ncn_verify_writable_privileges<'me, 'info>(
    accounts: OperatorWarmupNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn_operator_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn operator_warmup_ncn_verify_signer_privileges<'me, 'info>(
    accounts: OperatorWarmupNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn operator_warmup_ncn_verify_account_privileges<'me, 'info>(
    accounts: OperatorWarmupNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    operator_warmup_ncn_verify_writable_privileges(accounts)?;
    operator_warmup_ncn_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPERATOR_COOLDOWN_NCN_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct OperatorCooldownNcnAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub ncn_operator_state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct OperatorCooldownNcnKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub operator: Pubkey,
    pub ncn_operator_state: Pubkey,
    pub admin: Pubkey,
}
impl From<OperatorCooldownNcnAccounts<'_, '_>> for OperatorCooldownNcnKeys {
    fn from(accounts: OperatorCooldownNcnAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            operator: *accounts.operator.key,
            ncn_operator_state: *accounts.ncn_operator_state.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<OperatorCooldownNcnKeys>
for [AccountMeta; OPERATOR_COOLDOWN_NCN_IX_ACCOUNTS_LEN] {
    fn from(keys: OperatorCooldownNcnKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_operator_state,
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
impl From<[Pubkey; OPERATOR_COOLDOWN_NCN_IX_ACCOUNTS_LEN]> for OperatorCooldownNcnKeys {
    fn from(pubkeys: [Pubkey; OPERATOR_COOLDOWN_NCN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            operator: pubkeys[2],
            ncn_operator_state: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<OperatorCooldownNcnAccounts<'_, 'info>>
for [AccountInfo<'info>; OPERATOR_COOLDOWN_NCN_IX_ACCOUNTS_LEN] {
    fn from(accounts: OperatorCooldownNcnAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.operator.clone(),
            accounts.ncn_operator_state.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPERATOR_COOLDOWN_NCN_IX_ACCOUNTS_LEN]>
for OperatorCooldownNcnAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPERATOR_COOLDOWN_NCN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            operator: &arr[2],
            ncn_operator_state: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const OPERATOR_COOLDOWN_NCN_IX_DISCM: u8 = 12u8;
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorCooldownNcnIxData;
impl OperatorCooldownNcnIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPERATOR_COOLDOWN_NCN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPERATOR_COOLDOWN_NCN_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn operator_cooldown_ncn_ix_with_program_id(
    program_id: Pubkey,
    keys: OperatorCooldownNcnKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPERATOR_COOLDOWN_NCN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: OperatorCooldownNcnIxData.try_to_vec()?,
    })
}
pub fn operator_cooldown_ncn_ix(
    keys: OperatorCooldownNcnKeys,
) -> std::io::Result<Instruction> {
    operator_cooldown_ncn_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn operator_cooldown_ncn_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OperatorCooldownNcnAccounts<'_, '_>,
) -> ProgramResult {
    let keys: OperatorCooldownNcnKeys = accounts.into();
    let ix = operator_cooldown_ncn_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn operator_cooldown_ncn_invoke(
    accounts: OperatorCooldownNcnAccounts<'_, '_>,
) -> ProgramResult {
    operator_cooldown_ncn_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn operator_cooldown_ncn_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OperatorCooldownNcnAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OperatorCooldownNcnKeys = accounts.into();
    let ix = operator_cooldown_ncn_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn operator_cooldown_ncn_invoke_signed(
    accounts: OperatorCooldownNcnAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    operator_cooldown_ncn_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn operator_cooldown_ncn_verify_account_keys(
    accounts: OperatorCooldownNcnAccounts<'_, '_>,
    keys: OperatorCooldownNcnKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.operator.key, &keys.operator),
        (accounts.ncn_operator_state.key, &keys.ncn_operator_state),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn operator_cooldown_ncn_verify_writable_privileges<'me, 'info>(
    accounts: OperatorCooldownNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn_operator_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn operator_cooldown_ncn_verify_signer_privileges<'me, 'info>(
    accounts: OperatorCooldownNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn operator_cooldown_ncn_verify_account_privileges<'me, 'info>(
    accounts: OperatorCooldownNcnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    operator_cooldown_ncn_verify_writable_privileges(accounts)?;
    operator_cooldown_ncn_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WARMUP_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WarmupNcnVaultSlasherTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub slasher: &'me AccountInfo<'info>,
    pub ncn_vault_ticket: &'me AccountInfo<'info>,
    pub ncn_vault_slasher_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WarmupNcnVaultSlasherTicketKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub vault: Pubkey,
    pub slasher: Pubkey,
    pub ncn_vault_ticket: Pubkey,
    pub ncn_vault_slasher_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<WarmupNcnVaultSlasherTicketAccounts<'_, '_>>
for WarmupNcnVaultSlasherTicketKeys {
    fn from(accounts: WarmupNcnVaultSlasherTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            vault: *accounts.vault.key,
            slasher: *accounts.slasher.key,
            ncn_vault_ticket: *accounts.ncn_vault_ticket.key,
            ncn_vault_slasher_ticket: *accounts.ncn_vault_slasher_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<WarmupNcnVaultSlasherTicketKeys>
for [AccountMeta; WARMUP_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: WarmupNcnVaultSlasherTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slasher,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_ticket,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_slasher_ticket,
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
impl From<[Pubkey; WARMUP_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for WarmupNcnVaultSlasherTicketKeys {
    fn from(pubkeys: [Pubkey; WARMUP_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            vault: pubkeys[2],
            slasher: pubkeys[3],
            ncn_vault_ticket: pubkeys[4],
            ncn_vault_slasher_ticket: pubkeys[5],
            admin: pubkeys[6],
        }
    }
}
impl<'info> From<WarmupNcnVaultSlasherTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; WARMUP_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: WarmupNcnVaultSlasherTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.vault.clone(),
            accounts.slasher.clone(),
            accounts.ncn_vault_ticket.clone(),
            accounts.ncn_vault_slasher_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WARMUP_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for WarmupNcnVaultSlasherTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WARMUP_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            vault: &arr[2],
            slasher: &arr[3],
            ncn_vault_ticket: &arr[4],
            ncn_vault_slasher_ticket: &arr[5],
            admin: &arr[6],
        }
    }
}
pub const WARMUP_NCN_VAULT_SLASHER_TICKET_IX_DISCM: u8 = 13u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WarmupNcnVaultSlasherTicketIxData;
impl WarmupNcnVaultSlasherTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WARMUP_NCN_VAULT_SLASHER_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WARMUP_NCN_VAULT_SLASHER_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn warmup_ncn_vault_slasher_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: WarmupNcnVaultSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WARMUP_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WarmupNcnVaultSlasherTicketIxData.try_to_vec()?,
    })
}
pub fn warmup_ncn_vault_slasher_ticket_ix(
    keys: WarmupNcnVaultSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    warmup_ncn_vault_slasher_ticket_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn warmup_ncn_vault_slasher_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WarmupNcnVaultSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WarmupNcnVaultSlasherTicketKeys = accounts.into();
    let ix = warmup_ncn_vault_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn warmup_ncn_vault_slasher_ticket_invoke(
    accounts: WarmupNcnVaultSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    warmup_ncn_vault_slasher_ticket_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn warmup_ncn_vault_slasher_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WarmupNcnVaultSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WarmupNcnVaultSlasherTicketKeys = accounts.into();
    let ix = warmup_ncn_vault_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn warmup_ncn_vault_slasher_ticket_invoke_signed(
    accounts: WarmupNcnVaultSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    warmup_ncn_vault_slasher_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn warmup_ncn_vault_slasher_ticket_verify_account_keys(
    accounts: WarmupNcnVaultSlasherTicketAccounts<'_, '_>,
    keys: WarmupNcnVaultSlasherTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.vault.key, &keys.vault),
        (accounts.slasher.key, &keys.slasher),
        (accounts.ncn_vault_ticket.key, &keys.ncn_vault_ticket),
        (accounts.ncn_vault_slasher_ticket.key, &keys.ncn_vault_slasher_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn warmup_ncn_vault_slasher_ticket_verify_writable_privileges<'me, 'info>(
    accounts: WarmupNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn_vault_slasher_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn warmup_ncn_vault_slasher_ticket_verify_signer_privileges<'me, 'info>(
    accounts: WarmupNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn warmup_ncn_vault_slasher_ticket_verify_account_privileges<'me, 'info>(
    accounts: WarmupNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    warmup_ncn_vault_slasher_ticket_verify_writable_privileges(accounts)?;
    warmup_ncn_vault_slasher_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CooldownNcnVaultSlasherTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub ncn: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub slasher: &'me AccountInfo<'info>,
    pub ncn_vault_slasher_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CooldownNcnVaultSlasherTicketKeys {
    pub config: Pubkey,
    pub ncn: Pubkey,
    pub vault: Pubkey,
    pub slasher: Pubkey,
    pub ncn_vault_slasher_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<CooldownNcnVaultSlasherTicketAccounts<'_, '_>>
for CooldownNcnVaultSlasherTicketKeys {
    fn from(accounts: CooldownNcnVaultSlasherTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            ncn: *accounts.ncn.key,
            vault: *accounts.vault.key,
            slasher: *accounts.slasher.key,
            ncn_vault_slasher_ticket: *accounts.ncn_vault_slasher_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<CooldownNcnVaultSlasherTicketKeys>
for [AccountMeta; COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CooldownNcnVaultSlasherTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.slasher,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ncn_vault_slasher_ticket,
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
impl From<[Pubkey; COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for CooldownNcnVaultSlasherTicketKeys {
    fn from(
        pubkeys: [Pubkey; COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: pubkeys[0],
            ncn: pubkeys[1],
            vault: pubkeys[2],
            slasher: pubkeys[3],
            ncn_vault_slasher_ticket: pubkeys[4],
            admin: pubkeys[5],
        }
    }
}
impl<'info> From<CooldownNcnVaultSlasherTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CooldownNcnVaultSlasherTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.ncn.clone(),
            accounts.vault.clone(),
            accounts.slasher.clone(),
            accounts.ncn_vault_slasher_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN]>
for CooldownNcnVaultSlasherTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            ncn: &arr[1],
            vault: &arr[2],
            slasher: &arr[3],
            ncn_vault_slasher_ticket: &arr[4],
            admin: &arr[5],
        }
    }
}
pub const COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_DISCM: u8 = 14u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CooldownNcnVaultSlasherTicketIxData;
impl CooldownNcnVaultSlasherTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cooldown_ncn_vault_slasher_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: CooldownNcnVaultSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COOLDOWN_NCN_VAULT_SLASHER_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CooldownNcnVaultSlasherTicketIxData.try_to_vec()?,
    })
}
pub fn cooldown_ncn_vault_slasher_ticket_ix(
    keys: CooldownNcnVaultSlasherTicketKeys,
) -> std::io::Result<Instruction> {
    cooldown_ncn_vault_slasher_ticket_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn cooldown_ncn_vault_slasher_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CooldownNcnVaultSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CooldownNcnVaultSlasherTicketKeys = accounts.into();
    let ix = cooldown_ncn_vault_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cooldown_ncn_vault_slasher_ticket_invoke(
    accounts: CooldownNcnVaultSlasherTicketAccounts<'_, '_>,
) -> ProgramResult {
    cooldown_ncn_vault_slasher_ticket_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn cooldown_ncn_vault_slasher_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CooldownNcnVaultSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CooldownNcnVaultSlasherTicketKeys = accounts.into();
    let ix = cooldown_ncn_vault_slasher_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cooldown_ncn_vault_slasher_ticket_invoke_signed(
    accounts: CooldownNcnVaultSlasherTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cooldown_ncn_vault_slasher_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cooldown_ncn_vault_slasher_ticket_verify_account_keys(
    accounts: CooldownNcnVaultSlasherTicketAccounts<'_, '_>,
    keys: CooldownNcnVaultSlasherTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.ncn.key, &keys.ncn),
        (accounts.vault.key, &keys.vault),
        (accounts.slasher.key, &keys.slasher),
        (accounts.ncn_vault_slasher_ticket.key, &keys.ncn_vault_slasher_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cooldown_ncn_vault_slasher_ticket_verify_writable_privileges<'me, 'info>(
    accounts: CooldownNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn_vault_slasher_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cooldown_ncn_vault_slasher_ticket_verify_signer_privileges<'me, 'info>(
    accounts: CooldownNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cooldown_ncn_vault_slasher_ticket_verify_account_privileges<'me, 'info>(
    accounts: CooldownNcnVaultSlasherTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cooldown_ncn_vault_slasher_ticket_verify_writable_privileges(accounts)?;
    cooldown_ncn_vault_slasher_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WARMUP_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct WarmupOperatorVaultTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub operator_vault_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WarmupOperatorVaultTicketKeys {
    pub config: Pubkey,
    pub operator: Pubkey,
    pub vault: Pubkey,
    pub operator_vault_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<WarmupOperatorVaultTicketAccounts<'_, '_>> for WarmupOperatorVaultTicketKeys {
    fn from(accounts: WarmupOperatorVaultTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            operator: *accounts.operator.key,
            vault: *accounts.vault.key,
            operator_vault_ticket: *accounts.operator_vault_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<WarmupOperatorVaultTicketKeys>
for [AccountMeta; WARMUP_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: WarmupOperatorVaultTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator_vault_ticket,
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
impl From<[Pubkey; WARMUP_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for WarmupOperatorVaultTicketKeys {
    fn from(pubkeys: [Pubkey; WARMUP_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            operator: pubkeys[1],
            vault: pubkeys[2],
            operator_vault_ticket: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<WarmupOperatorVaultTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; WARMUP_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: WarmupOperatorVaultTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.operator.clone(),
            accounts.vault.clone(),
            accounts.operator_vault_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WARMUP_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for WarmupOperatorVaultTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WARMUP_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            operator: &arr[1],
            vault: &arr[2],
            operator_vault_ticket: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const WARMUP_OPERATOR_VAULT_TICKET_IX_DISCM: u8 = 15u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WarmupOperatorVaultTicketIxData;
impl WarmupOperatorVaultTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WARMUP_OPERATOR_VAULT_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WARMUP_OPERATOR_VAULT_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn warmup_operator_vault_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: WarmupOperatorVaultTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WARMUP_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WarmupOperatorVaultTicketIxData.try_to_vec()?,
    })
}
pub fn warmup_operator_vault_ticket_ix(
    keys: WarmupOperatorVaultTicketKeys,
) -> std::io::Result<Instruction> {
    warmup_operator_vault_ticket_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn warmup_operator_vault_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WarmupOperatorVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WarmupOperatorVaultTicketKeys = accounts.into();
    let ix = warmup_operator_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn warmup_operator_vault_ticket_invoke(
    accounts: WarmupOperatorVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    warmup_operator_vault_ticket_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn warmup_operator_vault_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WarmupOperatorVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WarmupOperatorVaultTicketKeys = accounts.into();
    let ix = warmup_operator_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn warmup_operator_vault_ticket_invoke_signed(
    accounts: WarmupOperatorVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    warmup_operator_vault_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn warmup_operator_vault_ticket_verify_account_keys(
    accounts: WarmupOperatorVaultTicketAccounts<'_, '_>,
    keys: WarmupOperatorVaultTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.operator.key, &keys.operator),
        (accounts.vault.key, &keys.vault),
        (accounts.operator_vault_ticket.key, &keys.operator_vault_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn warmup_operator_vault_ticket_verify_writable_privileges<'me, 'info>(
    accounts: WarmupOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.operator_vault_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn warmup_operator_vault_ticket_verify_signer_privileges<'me, 'info>(
    accounts: WarmupOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn warmup_operator_vault_ticket_verify_account_privileges<'me, 'info>(
    accounts: WarmupOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    warmup_operator_vault_ticket_verify_writable_privileges(accounts)?;
    warmup_operator_vault_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COOLDOWN_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CooldownOperatorVaultTicketAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub operator_vault_ticket: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CooldownOperatorVaultTicketKeys {
    pub config: Pubkey,
    pub operator: Pubkey,
    pub vault: Pubkey,
    pub operator_vault_ticket: Pubkey,
    pub admin: Pubkey,
}
impl From<CooldownOperatorVaultTicketAccounts<'_, '_>>
for CooldownOperatorVaultTicketKeys {
    fn from(accounts: CooldownOperatorVaultTicketAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            operator: *accounts.operator.key,
            vault: *accounts.vault.key,
            operator_vault_ticket: *accounts.operator_vault_ticket.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<CooldownOperatorVaultTicketKeys>
for [AccountMeta; COOLDOWN_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(keys: CooldownOperatorVaultTicketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator_vault_ticket,
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
impl From<[Pubkey; COOLDOWN_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for CooldownOperatorVaultTicketKeys {
    fn from(pubkeys: [Pubkey; COOLDOWN_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            operator: pubkeys[1],
            vault: pubkeys[2],
            operator_vault_ticket: pubkeys[3],
            admin: pubkeys[4],
        }
    }
}
impl<'info> From<CooldownOperatorVaultTicketAccounts<'_, 'info>>
for [AccountInfo<'info>; COOLDOWN_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CooldownOperatorVaultTicketAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.operator.clone(),
            accounts.vault.clone(),
            accounts.operator_vault_ticket.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COOLDOWN_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN]>
for CooldownOperatorVaultTicketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COOLDOWN_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            operator: &arr[1],
            vault: &arr[2],
            operator_vault_ticket: &arr[3],
            admin: &arr[4],
        }
    }
}
pub const COOLDOWN_OPERATOR_VAULT_TICKET_IX_DISCM: u8 = 16u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CooldownOperatorVaultTicketIxData;
impl CooldownOperatorVaultTicketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COOLDOWN_OPERATOR_VAULT_TICKET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COOLDOWN_OPERATOR_VAULT_TICKET_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cooldown_operator_vault_ticket_ix_with_program_id(
    program_id: Pubkey,
    keys: CooldownOperatorVaultTicketKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COOLDOWN_OPERATOR_VAULT_TICKET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CooldownOperatorVaultTicketIxData.try_to_vec()?,
    })
}
pub fn cooldown_operator_vault_ticket_ix(
    keys: CooldownOperatorVaultTicketKeys,
) -> std::io::Result<Instruction> {
    cooldown_operator_vault_ticket_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn cooldown_operator_vault_ticket_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CooldownOperatorVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CooldownOperatorVaultTicketKeys = accounts.into();
    let ix = cooldown_operator_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cooldown_operator_vault_ticket_invoke(
    accounts: CooldownOperatorVaultTicketAccounts<'_, '_>,
) -> ProgramResult {
    cooldown_operator_vault_ticket_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn cooldown_operator_vault_ticket_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CooldownOperatorVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CooldownOperatorVaultTicketKeys = accounts.into();
    let ix = cooldown_operator_vault_ticket_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cooldown_operator_vault_ticket_invoke_signed(
    accounts: CooldownOperatorVaultTicketAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cooldown_operator_vault_ticket_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cooldown_operator_vault_ticket_verify_account_keys(
    accounts: CooldownOperatorVaultTicketAccounts<'_, '_>,
    keys: CooldownOperatorVaultTicketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.operator.key, &keys.operator),
        (accounts.vault.key, &keys.vault),
        (accounts.operator_vault_ticket.key, &keys.operator_vault_ticket),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cooldown_operator_vault_ticket_verify_writable_privileges<'me, 'info>(
    accounts: CooldownOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.operator_vault_ticket] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cooldown_operator_vault_ticket_verify_signer_privileges<'me, 'info>(
    accounts: CooldownOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cooldown_operator_vault_ticket_verify_account_privileges<'me, 'info>(
    accounts: CooldownOperatorVaultTicketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cooldown_operator_vault_ticket_verify_writable_privileges(accounts)?;
    cooldown_operator_vault_ticket_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NCN_SET_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct NcnSetAdminAccounts<'me, 'info> {
    pub ncn: &'me AccountInfo<'info>,
    pub old_admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct NcnSetAdminKeys {
    pub ncn: Pubkey,
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
}
impl From<NcnSetAdminAccounts<'_, '_>> for NcnSetAdminKeys {
    fn from(accounts: NcnSetAdminAccounts) -> Self {
        Self {
            ncn: *accounts.ncn.key,
            old_admin: *accounts.old_admin.key,
            new_admin: *accounts.new_admin.key,
        }
    }
}
impl From<NcnSetAdminKeys> for [AccountMeta; NCN_SET_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: NcnSetAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ncn,
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
impl From<[Pubkey; NCN_SET_ADMIN_IX_ACCOUNTS_LEN]> for NcnSetAdminKeys {
    fn from(pubkeys: [Pubkey; NCN_SET_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ncn: pubkeys[0],
            old_admin: pubkeys[1],
            new_admin: pubkeys[2],
        }
    }
}
impl<'info> From<NcnSetAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; NCN_SET_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: NcnSetAdminAccounts<'_, 'info>) -> Self {
        [accounts.ncn.clone(), accounts.old_admin.clone(), accounts.new_admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NCN_SET_ADMIN_IX_ACCOUNTS_LEN]>
for NcnSetAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NCN_SET_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ncn: &arr[0],
            old_admin: &arr[1],
            new_admin: &arr[2],
        }
    }
}
pub const NCN_SET_ADMIN_IX_DISCM: u8 = 17u8;
#[derive(Clone, Debug, PartialEq)]
pub struct NcnSetAdminIxData;
impl NcnSetAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NCN_SET_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NCN_SET_ADMIN_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn ncn_set_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: NcnSetAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NCN_SET_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: NcnSetAdminIxData.try_to_vec()?,
    })
}
pub fn ncn_set_admin_ix(keys: NcnSetAdminKeys) -> std::io::Result<Instruction> {
    ncn_set_admin_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn ncn_set_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NcnSetAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: NcnSetAdminKeys = accounts.into();
    let ix = ncn_set_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn ncn_set_admin_invoke(accounts: NcnSetAdminAccounts<'_, '_>) -> ProgramResult {
    ncn_set_admin_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn ncn_set_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NcnSetAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NcnSetAdminKeys = accounts.into();
    let ix = ncn_set_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn ncn_set_admin_invoke_signed(
    accounts: NcnSetAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    ncn_set_admin_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn ncn_set_admin_verify_account_keys(
    accounts: NcnSetAdminAccounts<'_, '_>,
    keys: NcnSetAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.ncn.key, &keys.ncn),
        (accounts.old_admin.key, &keys.old_admin),
        (accounts.new_admin.key, &keys.new_admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn ncn_set_admin_verify_writable_privileges<'me, 'info>(
    accounts: NcnSetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn ncn_set_admin_verify_signer_privileges<'me, 'info>(
    accounts: NcnSetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.old_admin, accounts.new_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn ncn_set_admin_verify_account_privileges<'me, 'info>(
    accounts: NcnSetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    ncn_set_admin_verify_writable_privileges(accounts)?;
    ncn_set_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NCN_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct NcnSetSecondaryAdminAccounts<'me, 'info> {
    pub ncn: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct NcnSetSecondaryAdminKeys {
    pub ncn: Pubkey,
    pub admin: Pubkey,
    pub new_admin: Pubkey,
}
impl From<NcnSetSecondaryAdminAccounts<'_, '_>> for NcnSetSecondaryAdminKeys {
    fn from(accounts: NcnSetSecondaryAdminAccounts) -> Self {
        Self {
            ncn: *accounts.ncn.key,
            admin: *accounts.admin.key,
            new_admin: *accounts.new_admin.key,
        }
    }
}
impl From<NcnSetSecondaryAdminKeys>
for [AccountMeta; NCN_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: NcnSetSecondaryAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ncn,
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
impl From<[Pubkey; NCN_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]>
for NcnSetSecondaryAdminKeys {
    fn from(pubkeys: [Pubkey; NCN_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ncn: pubkeys[0],
            admin: pubkeys[1],
            new_admin: pubkeys[2],
        }
    }
}
impl<'info> From<NcnSetSecondaryAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; NCN_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: NcnSetSecondaryAdminAccounts<'_, 'info>) -> Self {
        [accounts.ncn.clone(), accounts.admin.clone(), accounts.new_admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NCN_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]>
for NcnSetSecondaryAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; NCN_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ncn: &arr[0],
            admin: &arr[1],
            new_admin: &arr[2],
        }
    }
}
pub const NCN_SET_SECONDARY_ADMIN_IX_DISCM: u8 = 18u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct NcnSetSecondaryAdminIxArgs {
    pub ncn_admin_role: NcnAdminRole,
}
impl NcnSetSecondaryAdminIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ncn_admin_role: NcnAdminRole = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { ncn_admin_role })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct NcnSetSecondaryAdminIxData(pub NcnSetSecondaryAdminIxArgs);
impl From<NcnSetSecondaryAdminIxArgs> for NcnSetSecondaryAdminIxData {
    fn from(args: NcnSetSecondaryAdminIxArgs) -> Self {
        Self(args)
    }
}
impl NcnSetSecondaryAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NCN_SET_SECONDARY_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(NcnSetSecondaryAdminIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NCN_SET_SECONDARY_ADMIN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn ncn_set_secondary_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: NcnSetSecondaryAdminKeys,
    args: NcnSetSecondaryAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NCN_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: NcnSetSecondaryAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn ncn_set_secondary_admin_ix(
    keys: NcnSetSecondaryAdminKeys,
    args: NcnSetSecondaryAdminIxArgs,
) -> std::io::Result<Instruction> {
    ncn_set_secondary_admin_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys, args)
}
pub fn ncn_set_secondary_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NcnSetSecondaryAdminAccounts<'_, '_>,
    args: NcnSetSecondaryAdminIxArgs,
) -> ProgramResult {
    let keys: NcnSetSecondaryAdminKeys = accounts.into();
    let ix = ncn_set_secondary_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn ncn_set_secondary_admin_invoke(
    accounts: NcnSetSecondaryAdminAccounts<'_, '_>,
    args: NcnSetSecondaryAdminIxArgs,
) -> ProgramResult {
    ncn_set_secondary_admin_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn ncn_set_secondary_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NcnSetSecondaryAdminAccounts<'_, '_>,
    args: NcnSetSecondaryAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NcnSetSecondaryAdminKeys = accounts.into();
    let ix = ncn_set_secondary_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn ncn_set_secondary_admin_invoke_signed(
    accounts: NcnSetSecondaryAdminAccounts<'_, '_>,
    args: NcnSetSecondaryAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    ncn_set_secondary_admin_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn ncn_set_secondary_admin_verify_account_keys(
    accounts: NcnSetSecondaryAdminAccounts<'_, '_>,
    keys: NcnSetSecondaryAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.ncn.key, &keys.ncn),
        (accounts.admin.key, &keys.admin),
        (accounts.new_admin.key, &keys.new_admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn ncn_set_secondary_admin_verify_writable_privileges<'me, 'info>(
    accounts: NcnSetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.ncn] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn ncn_set_secondary_admin_verify_signer_privileges<'me, 'info>(
    accounts: NcnSetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn ncn_set_secondary_admin_verify_account_privileges<'me, 'info>(
    accounts: NcnSetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    ncn_set_secondary_admin_verify_writable_privileges(accounts)?;
    ncn_set_secondary_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPERATOR_SET_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct OperatorSetAdminAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub old_admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct OperatorSetAdminKeys {
    pub operator: Pubkey,
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
}
impl From<OperatorSetAdminAccounts<'_, '_>> for OperatorSetAdminKeys {
    fn from(accounts: OperatorSetAdminAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            old_admin: *accounts.old_admin.key,
            new_admin: *accounts.new_admin.key,
        }
    }
}
impl From<OperatorSetAdminKeys> for [AccountMeta; OPERATOR_SET_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: OperatorSetAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
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
impl From<[Pubkey; OPERATOR_SET_ADMIN_IX_ACCOUNTS_LEN]> for OperatorSetAdminKeys {
    fn from(pubkeys: [Pubkey; OPERATOR_SET_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            old_admin: pubkeys[1],
            new_admin: pubkeys[2],
        }
    }
}
impl<'info> From<OperatorSetAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; OPERATOR_SET_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: OperatorSetAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.old_admin.clone(),
            accounts.new_admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPERATOR_SET_ADMIN_IX_ACCOUNTS_LEN]>
for OperatorSetAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPERATOR_SET_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: &arr[0],
            old_admin: &arr[1],
            new_admin: &arr[2],
        }
    }
}
pub const OPERATOR_SET_ADMIN_IX_DISCM: u8 = 19u8;
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorSetAdminIxData;
impl OperatorSetAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPERATOR_SET_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPERATOR_SET_ADMIN_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn operator_set_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: OperatorSetAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPERATOR_SET_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: OperatorSetAdminIxData.try_to_vec()?,
    })
}
pub fn operator_set_admin_ix(
    keys: OperatorSetAdminKeys,
) -> std::io::Result<Instruction> {
    operator_set_admin_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn operator_set_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: OperatorSetAdminKeys = accounts.into();
    let ix = operator_set_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn operator_set_admin_invoke(
    accounts: OperatorSetAdminAccounts<'_, '_>,
) -> ProgramResult {
    operator_set_admin_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
}
pub fn operator_set_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OperatorSetAdminKeys = accounts.into();
    let ix = operator_set_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn operator_set_admin_invoke_signed(
    accounts: OperatorSetAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    operator_set_admin_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn operator_set_admin_verify_account_keys(
    accounts: OperatorSetAdminAccounts<'_, '_>,
    keys: OperatorSetAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.operator.key, &keys.operator),
        (accounts.old_admin.key, &keys.old_admin),
        (accounts.new_admin.key, &keys.new_admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn operator_set_admin_verify_writable_privileges<'me, 'info>(
    accounts: OperatorSetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.operator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn operator_set_admin_verify_signer_privileges<'me, 'info>(
    accounts: OperatorSetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.old_admin, accounts.new_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn operator_set_admin_verify_account_privileges<'me, 'info>(
    accounts: OperatorSetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    operator_set_admin_verify_writable_privileges(accounts)?;
    operator_set_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPERATOR_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct OperatorSetSecondaryAdminAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct OperatorSetSecondaryAdminKeys {
    pub operator: Pubkey,
    pub admin: Pubkey,
    pub new_admin: Pubkey,
}
impl From<OperatorSetSecondaryAdminAccounts<'_, '_>> for OperatorSetSecondaryAdminKeys {
    fn from(accounts: OperatorSetSecondaryAdminAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            admin: *accounts.admin.key,
            new_admin: *accounts.new_admin.key,
        }
    }
}
impl From<OperatorSetSecondaryAdminKeys>
for [AccountMeta; OPERATOR_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: OperatorSetSecondaryAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
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
impl From<[Pubkey; OPERATOR_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]>
for OperatorSetSecondaryAdminKeys {
    fn from(pubkeys: [Pubkey; OPERATOR_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            admin: pubkeys[1],
            new_admin: pubkeys[2],
        }
    }
}
impl<'info> From<OperatorSetSecondaryAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; OPERATOR_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: OperatorSetSecondaryAdminAccounts<'_, 'info>) -> Self {
        [accounts.operator.clone(), accounts.admin.clone(), accounts.new_admin.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; OPERATOR_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN]>
for OperatorSetSecondaryAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPERATOR_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            admin: &arr[1],
            new_admin: &arr[2],
        }
    }
}
pub const OPERATOR_SET_SECONDARY_ADMIN_IX_DISCM: u8 = 20u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct OperatorSetSecondaryAdminIxArgs {
    pub operator_admin_role: OperatorAdminRole,
}
impl OperatorSetSecondaryAdminIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let operator_admin_role: OperatorAdminRole = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { operator_admin_role })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorSetSecondaryAdminIxData(pub OperatorSetSecondaryAdminIxArgs);
impl From<OperatorSetSecondaryAdminIxArgs> for OperatorSetSecondaryAdminIxData {
    fn from(args: OperatorSetSecondaryAdminIxArgs) -> Self {
        Self(args)
    }
}
impl OperatorSetSecondaryAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPERATOR_SET_SECONDARY_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OperatorSetSecondaryAdminIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPERATOR_SET_SECONDARY_ADMIN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn operator_set_secondary_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: OperatorSetSecondaryAdminKeys,
    args: OperatorSetSecondaryAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPERATOR_SET_SECONDARY_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: OperatorSetSecondaryAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn operator_set_secondary_admin_ix(
    keys: OperatorSetSecondaryAdminKeys,
    args: OperatorSetSecondaryAdminIxArgs,
) -> std::io::Result<Instruction> {
    operator_set_secondary_admin_ix_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn operator_set_secondary_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetSecondaryAdminAccounts<'_, '_>,
    args: OperatorSetSecondaryAdminIxArgs,
) -> ProgramResult {
    let keys: OperatorSetSecondaryAdminKeys = accounts.into();
    let ix = operator_set_secondary_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn operator_set_secondary_admin_invoke(
    accounts: OperatorSetSecondaryAdminAccounts<'_, '_>,
    args: OperatorSetSecondaryAdminIxArgs,
) -> ProgramResult {
    operator_set_secondary_admin_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn operator_set_secondary_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetSecondaryAdminAccounts<'_, '_>,
    args: OperatorSetSecondaryAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OperatorSetSecondaryAdminKeys = accounts.into();
    let ix = operator_set_secondary_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn operator_set_secondary_admin_invoke_signed(
    accounts: OperatorSetSecondaryAdminAccounts<'_, '_>,
    args: OperatorSetSecondaryAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    operator_set_secondary_admin_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn operator_set_secondary_admin_verify_account_keys(
    accounts: OperatorSetSecondaryAdminAccounts<'_, '_>,
    keys: OperatorSetSecondaryAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.operator.key, &keys.operator),
        (accounts.admin.key, &keys.admin),
        (accounts.new_admin.key, &keys.new_admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn operator_set_secondary_admin_verify_writable_privileges<'me, 'info>(
    accounts: OperatorSetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.operator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn operator_set_secondary_admin_verify_signer_privileges<'me, 'info>(
    accounts: OperatorSetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn operator_set_secondary_admin_verify_account_privileges<'me, 'info>(
    accounts: OperatorSetSecondaryAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    operator_set_secondary_admin_verify_writable_privileges(accounts)?;
    operator_set_secondary_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPERATOR_SET_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct OperatorSetFeeAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct OperatorSetFeeKeys {
    pub config: Pubkey,
    pub operator: Pubkey,
    pub admin: Pubkey,
}
impl From<OperatorSetFeeAccounts<'_, '_>> for OperatorSetFeeKeys {
    fn from(accounts: OperatorSetFeeAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            operator: *accounts.operator.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<OperatorSetFeeKeys> for [AccountMeta; OPERATOR_SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: OperatorSetFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
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
impl From<[Pubkey; OPERATOR_SET_FEE_IX_ACCOUNTS_LEN]> for OperatorSetFeeKeys {
    fn from(pubkeys: [Pubkey; OPERATOR_SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            operator: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<OperatorSetFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; OPERATOR_SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: OperatorSetFeeAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.operator.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPERATOR_SET_FEE_IX_ACCOUNTS_LEN]>
for OperatorSetFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPERATOR_SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            operator: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const OPERATOR_SET_FEE_IX_DISCM: u8 = 21u8;
#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct OperatorSetFeeIxArgs {
    pub new_fee_bps: u16,
}
impl OperatorSetFeeIxArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { new_fee_bps })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorSetFeeIxData(pub OperatorSetFeeIxArgs);
impl From<OperatorSetFeeIxArgs> for OperatorSetFeeIxData {
    fn from(args: OperatorSetFeeIxArgs) -> Self {
        Self(args)
    }
}
impl OperatorSetFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPERATOR_SET_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OperatorSetFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPERATOR_SET_FEE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn operator_set_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: OperatorSetFeeKeys,
    args: OperatorSetFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPERATOR_SET_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: OperatorSetFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn operator_set_fee_ix(
    keys: OperatorSetFeeKeys,
    args: OperatorSetFeeIxArgs,
) -> std::io::Result<Instruction> {
    operator_set_fee_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys, args)
}
pub fn operator_set_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetFeeAccounts<'_, '_>,
    args: OperatorSetFeeIxArgs,
) -> ProgramResult {
    let keys: OperatorSetFeeKeys = accounts.into();
    let ix = operator_set_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn operator_set_fee_invoke(
    accounts: OperatorSetFeeAccounts<'_, '_>,
    args: OperatorSetFeeIxArgs,
) -> ProgramResult {
    operator_set_fee_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts, args)
}
pub fn operator_set_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetFeeAccounts<'_, '_>,
    args: OperatorSetFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OperatorSetFeeKeys = accounts.into();
    let ix = operator_set_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn operator_set_fee_invoke_signed(
    accounts: OperatorSetFeeAccounts<'_, '_>,
    args: OperatorSetFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    operator_set_fee_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn operator_set_fee_verify_account_keys(
    accounts: OperatorSetFeeAccounts<'_, '_>,
    keys: OperatorSetFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.config.key, &keys.config),
        (accounts.operator.key, &keys.operator),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn operator_set_fee_verify_writable_privileges<'me, 'info>(
    accounts: OperatorSetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.operator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn operator_set_fee_verify_signer_privileges<'me, 'info>(
    accounts: OperatorSetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn operator_set_fee_verify_account_privileges<'me, 'info>(
    accounts: OperatorSetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    operator_set_fee_verify_writable_privileges(accounts)?;
    operator_set_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NCN_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct NcnDelegateTokenAccountAccounts<'me, 'info> {
    pub ncn: &'me AccountInfo<'info>,
    pub delegate_admin: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub delegate: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct NcnDelegateTokenAccountKeys {
    pub ncn: Pubkey,
    pub delegate_admin: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub delegate: Pubkey,
    pub token_program: Pubkey,
}
impl From<NcnDelegateTokenAccountAccounts<'_, '_>> for NcnDelegateTokenAccountKeys {
    fn from(accounts: NcnDelegateTokenAccountAccounts) -> Self {
        Self {
            ncn: *accounts.ncn.key,
            delegate_admin: *accounts.delegate_admin.key,
            token_mint: *accounts.token_mint.key,
            token_account: *accounts.token_account.key,
            delegate: *accounts.delegate.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<NcnDelegateTokenAccountKeys>
for [AccountMeta; NCN_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: NcnDelegateTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.ncn,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegate_admin,
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
impl From<[Pubkey; NCN_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for NcnDelegateTokenAccountKeys {
    fn from(pubkeys: [Pubkey; NCN_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            ncn: pubkeys[0],
            delegate_admin: pubkeys[1],
            token_mint: pubkeys[2],
            token_account: pubkeys[3],
            delegate: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<NcnDelegateTokenAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; NCN_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: NcnDelegateTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.ncn.clone(),
            accounts.delegate_admin.clone(),
            accounts.token_mint.clone(),
            accounts.token_account.clone(),
            accounts.delegate.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; NCN_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for NcnDelegateTokenAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; NCN_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            ncn: &arr[0],
            delegate_admin: &arr[1],
            token_mint: &arr[2],
            token_account: &arr[3],
            delegate: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const NCN_DELEGATE_TOKEN_ACCOUNT_IX_DISCM: u8 = 22u8;
#[derive(Clone, Debug, PartialEq)]
pub struct NcnDelegateTokenAccountIxData;
impl NcnDelegateTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NCN_DELEGATE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NCN_DELEGATE_TOKEN_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn ncn_delegate_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: NcnDelegateTokenAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NCN_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: NcnDelegateTokenAccountIxData.try_to_vec()?,
    })
}
pub fn ncn_delegate_token_account_ix(
    keys: NcnDelegateTokenAccountKeys,
) -> std::io::Result<Instruction> {
    ncn_delegate_token_account_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn ncn_delegate_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NcnDelegateTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: NcnDelegateTokenAccountKeys = accounts.into();
    let ix = ncn_delegate_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn ncn_delegate_token_account_invoke(
    accounts: NcnDelegateTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    ncn_delegate_token_account_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn ncn_delegate_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NcnDelegateTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NcnDelegateTokenAccountKeys = accounts.into();
    let ix = ncn_delegate_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn ncn_delegate_token_account_invoke_signed(
    accounts: NcnDelegateTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    ncn_delegate_token_account_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn ncn_delegate_token_account_verify_account_keys(
    accounts: NcnDelegateTokenAccountAccounts<'_, '_>,
    keys: NcnDelegateTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.ncn.key, &keys.ncn),
        (accounts.delegate_admin.key, &keys.delegate_admin),
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
pub fn ncn_delegate_token_account_verify_writable_privileges<'me, 'info>(
    accounts: NcnDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn ncn_delegate_token_account_verify_signer_privileges<'me, 'info>(
    accounts: NcnDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn ncn_delegate_token_account_verify_account_privileges<'me, 'info>(
    accounts: NcnDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    ncn_delegate_token_account_verify_writable_privileges(accounts)?;
    ncn_delegate_token_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct OperatorDelegateTokenAccountAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub delegate_admin: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub delegate: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct OperatorDelegateTokenAccountKeys {
    pub operator: Pubkey,
    pub delegate_admin: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub delegate: Pubkey,
    pub token_program: Pubkey,
}
impl From<OperatorDelegateTokenAccountAccounts<'_, '_>>
for OperatorDelegateTokenAccountKeys {
    fn from(accounts: OperatorDelegateTokenAccountAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            delegate_admin: *accounts.delegate_admin.key,
            token_mint: *accounts.token_mint.key,
            token_account: *accounts.token_account.key,
            delegate: *accounts.delegate.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<OperatorDelegateTokenAccountKeys>
for [AccountMeta; OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: OperatorDelegateTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegate_admin,
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
impl From<[Pubkey; OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for OperatorDelegateTokenAccountKeys {
    fn from(pubkeys: [Pubkey; OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            delegate_admin: pubkeys[1],
            token_mint: pubkeys[2],
            token_account: pubkeys[3],
            delegate: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<OperatorDelegateTokenAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: OperatorDelegateTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.delegate_admin.clone(),
            accounts.token_mint.clone(),
            accounts.token_account.clone(),
            accounts.delegate.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
for OperatorDelegateTokenAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            delegate_admin: &arr[1],
            token_mint: &arr[2],
            token_account: &arr[3],
            delegate: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_DISCM: u8 = 23u8;
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorDelegateTokenAccountIxData;
impl OperatorDelegateTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn operator_delegate_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: OperatorDelegateTokenAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPERATOR_DELEGATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: OperatorDelegateTokenAccountIxData.try_to_vec()?,
    })
}
pub fn operator_delegate_token_account_ix(
    keys: OperatorDelegateTokenAccountKeys,
) -> std::io::Result<Instruction> {
    operator_delegate_token_account_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
}
pub fn operator_delegate_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OperatorDelegateTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: OperatorDelegateTokenAccountKeys = accounts.into();
    let ix = operator_delegate_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn operator_delegate_token_account_invoke(
    accounts: OperatorDelegateTokenAccountAccounts<'_, '_>,
) -> ProgramResult {
    operator_delegate_token_account_invoke_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
    )
}
pub fn operator_delegate_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OperatorDelegateTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OperatorDelegateTokenAccountKeys = accounts.into();
    let ix = operator_delegate_token_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn operator_delegate_token_account_invoke_signed(
    accounts: OperatorDelegateTokenAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    operator_delegate_token_account_invoke_signed_with_program_id(
        JITO_RESTAKING_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn operator_delegate_token_account_verify_account_keys(
    accounts: OperatorDelegateTokenAccountAccounts<'_, '_>,
    keys: OperatorDelegateTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.operator.key, &keys.operator),
        (accounts.delegate_admin.key, &keys.delegate_admin),
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
pub fn operator_delegate_token_account_verify_writable_privileges<'me, 'info>(
    accounts: OperatorDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn operator_delegate_token_account_verify_signer_privileges<'me, 'info>(
    accounts: OperatorDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn operator_delegate_token_account_verify_account_privileges<'me, 'info>(
    accounts: OperatorDelegateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    operator_delegate_token_account_verify_writable_privileges(accounts)?;
    operator_delegate_token_account_verify_signer_privileges(accounts)?;
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
pub const SET_CONFIG_ADMIN_IX_DISCM: u8 = 24u8;
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
    set_config_admin_ix_with_program_id(JITO_RESTAKING_PROGRAM_ID, keys)
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
    set_config_admin_invoke_with_program_id(JITO_RESTAKING_PROGRAM_ID, accounts)
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
        JITO_RESTAKING_PROGRAM_ID,
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
