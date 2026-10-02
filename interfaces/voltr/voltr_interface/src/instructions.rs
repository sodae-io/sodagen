use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum VoltrProgramIx {
    AcceptProtocolAdmin,
    AcceptVaultAdmin,
    AddAdaptor,
    CalibrateHighWaterMark,
    CalibrateHighWaterMarkUnsafe,
    CancelRequestWithdrawVault,
    CloseStrategy,
    CreateLpMetadata(CreateLpMetadataIxArgs),
    DepositStrategy(DepositStrategyIxArgs),
    DepositVault(DepositVaultIxArgs),
    DirectWithdrawStrategy(DirectWithdrawStrategyIxArgs),
    DirectWithdrawStrategyWithTolerance(DirectWithdrawStrategyWithToleranceIxArgs),
    HarvestFee,
    InitProtocol(InitProtocolIxArgs),
    InitializeDirectWithdrawStrategy(InitializeDirectWithdrawStrategyIxArgs),
    InitializeStrategy(InitializeStrategyIxArgs),
    InitializeVault(InitializeVaultIxArgs),
    InstantWithdrawStrategy(InstantWithdrawStrategyIxArgs),
    InstantWithdrawStrategyWithTolerance(InstantWithdrawStrategyWithToleranceIxArgs),
    InstantWithdrawVault(InstantWithdrawVaultIxArgs),
    RemoveAdaptor,
    RequestWithdrawVault(RequestWithdrawVaultIxArgs),
    UpdateProtocol(UpdateProtocolIxArgs),
    UpdateVaultAdaptorPolicy(UpdateVaultAdaptorPolicyIxArgs),
    UpdateVaultConfig(UpdateVaultConfigIxArgs),
    UpdateVaultProtocolFee(UpdateVaultProtocolFeeIxArgs),
    WithdrawStrategy(WithdrawStrategyIxArgs),
    WithdrawVault,
}
impl VoltrProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ACCEPT_PROTOCOL_ADMIN_IX_DISCM) {
            return Ok(Self::AcceptProtocolAdmin);
        }
        if buf.starts_with(&ACCEPT_VAULT_ADMIN_IX_DISCM) {
            return Ok(Self::AcceptVaultAdmin);
        }
        if buf.starts_with(&ADD_ADAPTOR_IX_DISCM) {
            return Ok(Self::AddAdaptor);
        }
        if buf.starts_with(&CALIBRATE_HIGH_WATER_MARK_IX_DISCM) {
            return Ok(Self::CalibrateHighWaterMark);
        }
        if buf.starts_with(&CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_DISCM) {
            return Ok(Self::CalibrateHighWaterMarkUnsafe);
        }
        if buf.starts_with(&CANCEL_REQUEST_WITHDRAW_VAULT_IX_DISCM) {
            return Ok(Self::CancelRequestWithdrawVault);
        }
        if buf.starts_with(&CLOSE_STRATEGY_IX_DISCM) {
            return Ok(Self::CloseStrategy);
        }
        if buf.starts_with(&CREATE_LP_METADATA_IX_DISCM) {
            let mut reader = &buf[CREATE_LP_METADATA_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateLpMetadata(CreateLpMetadataIxArgs {
                    name,
                    symbol,
                    uri,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_STRATEGY_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_STRATEGY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_discriminator: Option<Vec<u8>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::DepositStrategy(DepositStrategyIxArgs {
                    amount,
                    instruction_discriminator,
                    additional_args,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_VAULT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_VAULT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DepositVault(DepositVaultIxArgs { amount }));
        }
        if buf.starts_with(&DIRECT_WITHDRAW_STRATEGY_IX_DISCM) {
            let mut reader = &buf[DIRECT_WITHDRAW_STRATEGY_IX_DISCM.len()..];
            let user_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DirectWithdrawStrategy(DirectWithdrawStrategyIxArgs {
                    user_args,
                }),
            );
        }
        if buf.starts_with(&DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM) {
            let mut reader = &buf[DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM
                .len()..];
            let user_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            let tolerance: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DirectWithdrawStrategyWithTolerance(DirectWithdrawStrategyWithToleranceIxArgs {
                    user_args,
                    tolerance,
                }),
            );
        }
        if buf.starts_with(&HARVEST_FEE_IX_DISCM) {
            return Ok(Self::HarvestFee);
        }
        if buf.starts_with(&INIT_PROTOCOL_IX_DISCM) {
            let mut reader = &buf[INIT_PROTOCOL_IX_DISCM.len()..];
            let treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let operational_state: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitProtocol(InitProtocolIxArgs {
                    treasury,
                    operational_state,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_DISCM.len()..];
            let instruction_discriminator: Option<Vec<u8>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let allow_user_args: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeDirectWithdrawStrategy(InitializeDirectWithdrawStrategyIxArgs {
                    instruction_discriminator,
                    additional_args,
                    allow_user_args,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_STRATEGY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_STRATEGY_IX_DISCM.len()..];
            let instruction_discriminator: Option<Vec<u8>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::InitializeStrategy(InitializeStrategyIxArgs {
                    instruction_discriminator,
                    additional_args,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_VAULT_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_VAULT_IX_DISCM.len()..];
            let config = if reader.is_empty() {
                Default::default()
            } else {
                <VaultInitializationInput>::deserialize(&mut reader)?
            };
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let description: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeVault(InitializeVaultIxArgs {
                    config,
                    name,
                    description,
                }),
            );
        }
        if buf.starts_with(&INSTANT_WITHDRAW_STRATEGY_IX_DISCM) {
            let mut reader = &buf[INSTANT_WITHDRAW_STRATEGY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
            let user_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InstantWithdrawStrategy(InstantWithdrawStrategyIxArgs {
                    amount,
                    is_amount_in_lp,
                    is_withdraw_all,
                    user_args,
                }),
            );
        }
        if buf.starts_with(&INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM) {
            let mut reader = &buf[INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM
                .len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
            let user_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            let tolerance: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InstantWithdrawStrategyWithTolerance(InstantWithdrawStrategyWithToleranceIxArgs {
                    amount,
                    is_amount_in_lp,
                    is_withdraw_all,
                    user_args,
                    tolerance,
                }),
            );
        }
        if buf.starts_with(&INSTANT_WITHDRAW_VAULT_IX_DISCM) {
            let mut reader = &buf[INSTANT_WITHDRAW_VAULT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InstantWithdrawVault(InstantWithdrawVaultIxArgs {
                    amount,
                    is_amount_in_lp,
                    is_withdraw_all,
                }),
            );
        }
        if buf.starts_with(&REMOVE_ADAPTOR_IX_DISCM) {
            return Ok(Self::RemoveAdaptor);
        }
        if buf.starts_with(&REQUEST_WITHDRAW_VAULT_IX_DISCM) {
            let mut reader = &buf[REQUEST_WITHDRAW_VAULT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RequestWithdrawVault(RequestWithdrawVaultIxArgs {
                    amount,
                    is_amount_in_lp,
                    is_withdraw_all,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PROTOCOL_IX_DISCM) {
            let mut reader = &buf[UPDATE_PROTOCOL_IX_DISCM.len()..];
            let field: ProtocolConfigField = crate::borsh_de_or_default(&mut reader)?;
            let data: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateProtocol(UpdateProtocolIxArgs {
                    field,
                    data,
                }),
            );
        }
        if buf.starts_with(&UPDATE_VAULT_ADAPTOR_POLICY_IX_DISCM) {
            let mut reader = &buf[UPDATE_VAULT_ADAPTOR_POLICY_IX_DISCM.len()..];
            let allow_any_adaptor: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateVaultAdaptorPolicy(UpdateVaultAdaptorPolicyIxArgs {
                    allow_any_adaptor,
                }),
            );
        }
        if buf.starts_with(&UPDATE_VAULT_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_VAULT_CONFIG_IX_DISCM.len()..];
            let field: VaultConfigField = crate::borsh_de_or_default(&mut reader)?;
            let data: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateVaultConfig(UpdateVaultConfigIxArgs {
                    field,
                    data,
                }),
            );
        }
        if buf.starts_with(&UPDATE_VAULT_PROTOCOL_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_VAULT_PROTOCOL_FEE_IX_DISCM.len()..];
            let field: ProtocolFeeField = crate::borsh_de_or_default(&mut reader)?;
            let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateVaultProtocolFee(UpdateVaultProtocolFeeIxArgs {
                    field,
                    fee_bps,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_STRATEGY_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_STRATEGY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_discriminator: Option<Vec<u8>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::WithdrawStrategy(WithdrawStrategyIxArgs {
                    amount,
                    instruction_discriminator,
                    additional_args,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_VAULT_IX_DISCM) {
            return Ok(Self::WithdrawVault);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AcceptProtocolAdmin => {
                writer.write_all(&ACCEPT_PROTOCOL_ADMIN_IX_DISCM)
            }
            Self::AcceptVaultAdmin => writer.write_all(&ACCEPT_VAULT_ADMIN_IX_DISCM),
            Self::AddAdaptor => writer.write_all(&ADD_ADAPTOR_IX_DISCM),
            Self::CalibrateHighWaterMark => {
                writer.write_all(&CALIBRATE_HIGH_WATER_MARK_IX_DISCM)
            }
            Self::CalibrateHighWaterMarkUnsafe => {
                writer.write_all(&CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_DISCM)
            }
            Self::CancelRequestWithdrawVault => {
                writer.write_all(&CANCEL_REQUEST_WITHDRAW_VAULT_IX_DISCM)
            }
            Self::CloseStrategy => writer.write_all(&CLOSE_STRATEGY_IX_DISCM),
            Self::CreateLpMetadata(args) => {
                writer.write_all(&CREATE_LP_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                Ok(())
            }
            Self::DepositStrategy(args) => {
                writer.write_all(&DEPOSIT_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.instruction_discriminator,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.additional_args, &mut writer)?;
                Ok(())
            }
            Self::DepositVault(args) => {
                writer.write_all(&DEPOSIT_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::DirectWithdrawStrategy(args) => {
                writer.write_all(&DIRECT_WITHDRAW_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.user_args, &mut writer)?;
                Ok(())
            }
            Self::DirectWithdrawStrategyWithTolerance(args) => {
                writer.write_all(&DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.user_args, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tolerance, &mut writer)?;
                Ok(())
            }
            Self::HarvestFee => writer.write_all(&HARVEST_FEE_IX_DISCM),
            Self::InitProtocol(args) => {
                writer.write_all(&INIT_PROTOCOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.treasury, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.operational_state, &mut writer)?;
                Ok(())
            }
            Self::InitializeDirectWithdrawStrategy(args) => {
                writer.write_all(&INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.instruction_discriminator,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.additional_args, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.allow_user_args, &mut writer)?;
                Ok(())
            }
            Self::InitializeStrategy(args) => {
                writer.write_all(&INITIALIZE_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.instruction_discriminator,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.additional_args, &mut writer)?;
                Ok(())
            }
            Self::InitializeVault(args) => {
                writer.write_all(&INITIALIZE_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.description, &mut writer)?;
                Ok(())
            }
            Self::InstantWithdrawStrategy(args) => {
                writer.write_all(&INSTANT_WITHDRAW_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_amount_in_lp, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_withdraw_all, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user_args, &mut writer)?;
                Ok(())
            }
            Self::InstantWithdrawStrategyWithTolerance(args) => {
                writer.write_all(&INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_amount_in_lp, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_withdraw_all, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.user_args, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tolerance, &mut writer)?;
                Ok(())
            }
            Self::InstantWithdrawVault(args) => {
                writer.write_all(&INSTANT_WITHDRAW_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_amount_in_lp, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_withdraw_all, &mut writer)?;
                Ok(())
            }
            Self::RemoveAdaptor => writer.write_all(&REMOVE_ADAPTOR_IX_DISCM),
            Self::RequestWithdrawVault(args) => {
                writer.write_all(&REQUEST_WITHDRAW_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_amount_in_lp, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_withdraw_all, &mut writer)?;
                Ok(())
            }
            Self::UpdateProtocol(args) => {
                writer.write_all(&UPDATE_PROTOCOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::UpdateVaultAdaptorPolicy(args) => {
                writer.write_all(&UPDATE_VAULT_ADAPTOR_POLICY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.allow_any_adaptor, &mut writer)?;
                Ok(())
            }
            Self::UpdateVaultConfig(args) => {
                writer.write_all(&UPDATE_VAULT_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::UpdateVaultProtocolFee(args) => {
                writer.write_all(&UPDATE_VAULT_PROTOCOL_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.field, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_bps, &mut writer)?;
                Ok(())
            }
            Self::WithdrawStrategy(args) => {
                writer.write_all(&WITHDRAW_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.instruction_discriminator,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.additional_args, &mut writer)?;
                Ok(())
            }
            Self::WithdrawVault => writer.write_all(&WITHDRAW_VAULT_IX_DISCM),
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
pub const ACCEPT_PROTOCOL_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AcceptProtocolAdminAccounts<'me, 'info> {
    pub pending_admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptProtocolAdminKeys {
    pub pending_admin: Pubkey,
    pub protocol: Pubkey,
}
impl From<AcceptProtocolAdminAccounts<'_, '_>> for AcceptProtocolAdminKeys {
    fn from(accounts: AcceptProtocolAdminAccounts) -> Self {
        Self {
            pending_admin: *accounts.pending_admin.key,
            protocol: *accounts.protocol.key,
        }
    }
}
impl From<AcceptProtocolAdminKeys>
for [AccountMeta; ACCEPT_PROTOCOL_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptProtocolAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ACCEPT_PROTOCOL_ADMIN_IX_ACCOUNTS_LEN]> for AcceptProtocolAdminKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_PROTOCOL_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_admin: pubkeys[0],
            protocol: pubkeys[1],
        }
    }
}
impl<'info> From<AcceptProtocolAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_PROTOCOL_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptProtocolAdminAccounts<'_, 'info>) -> Self {
        [accounts.pending_admin.clone(), accounts.protocol.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ACCEPT_PROTOCOL_ADMIN_IX_ACCOUNTS_LEN]>
for AcceptProtocolAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ACCEPT_PROTOCOL_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pending_admin: &arr[0],
            protocol: &arr[1],
        }
    }
}
pub const ACCEPT_PROTOCOL_ADMIN_IX_DISCM: [u8; 8usize] = [
    76, 35, 211, 183, 82, 72, 131, 36,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptProtocolAdminIxData;
impl AcceptProtocolAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_PROTOCOL_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_PROTOCOL_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_protocol_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptProtocolAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_PROTOCOL_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AcceptProtocolAdminIxData.try_to_vec()?,
    })
}
pub fn accept_protocol_admin_ix(
    keys: AcceptProtocolAdminKeys,
) -> std::io::Result<Instruction> {
    accept_protocol_admin_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn accept_protocol_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptProtocolAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AcceptProtocolAdminKeys = accounts.into();
    let ix = accept_protocol_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_protocol_admin_invoke(
    accounts: AcceptProtocolAdminAccounts<'_, '_>,
) -> ProgramResult {
    accept_protocol_admin_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn accept_protocol_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptProtocolAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptProtocolAdminKeys = accounts.into();
    let ix = accept_protocol_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_protocol_admin_invoke_signed(
    accounts: AcceptProtocolAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_protocol_admin_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn accept_protocol_admin_verify_account_keys(
    accounts: AcceptProtocolAdminAccounts<'_, '_>,
    keys: AcceptProtocolAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_admin.key, keys.pending_admin),
        (*accounts.protocol.key, keys.protocol),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_protocol_admin_verify_writable_privileges<'me, 'info>(
    accounts: AcceptProtocolAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.protocol] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_protocol_admin_verify_signer_privileges<'me, 'info>(
    accounts: AcceptProtocolAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_protocol_admin_verify_account_privileges<'me, 'info>(
    accounts: AcceptProtocolAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_protocol_admin_verify_writable_privileges(accounts)?;
    accept_protocol_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ACCEPT_VAULT_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AcceptVaultAdminAccounts<'me, 'info> {
    pub pending_admin: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptVaultAdminKeys {
    pub pending_admin: Pubkey,
    pub vault: Pubkey,
}
impl From<AcceptVaultAdminAccounts<'_, '_>> for AcceptVaultAdminKeys {
    fn from(accounts: AcceptVaultAdminAccounts) -> Self {
        Self {
            pending_admin: *accounts.pending_admin.key,
            vault: *accounts.vault.key,
        }
    }
}
impl From<AcceptVaultAdminKeys> for [AccountMeta; ACCEPT_VAULT_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptVaultAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ACCEPT_VAULT_ADMIN_IX_ACCOUNTS_LEN]> for AcceptVaultAdminKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_VAULT_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_admin: pubkeys[0],
            vault: pubkeys[1],
        }
    }
}
impl<'info> From<AcceptVaultAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_VAULT_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptVaultAdminAccounts<'_, 'info>) -> Self {
        [accounts.pending_admin.clone(), accounts.vault.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ACCEPT_VAULT_ADMIN_IX_ACCOUNTS_LEN]>
for AcceptVaultAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ACCEPT_VAULT_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_admin: &arr[0],
            vault: &arr[1],
        }
    }
}
pub const ACCEPT_VAULT_ADMIN_IX_DISCM: [u8; 8usize] = [
    214, 137, 204, 56, 78, 214, 90, 125,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptVaultAdminIxData;
impl AcceptVaultAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_VAULT_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_VAULT_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_vault_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptVaultAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_VAULT_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AcceptVaultAdminIxData.try_to_vec()?,
    })
}
pub fn accept_vault_admin_ix(
    keys: AcceptVaultAdminKeys,
) -> std::io::Result<Instruction> {
    accept_vault_admin_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn accept_vault_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptVaultAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AcceptVaultAdminKeys = accounts.into();
    let ix = accept_vault_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_vault_admin_invoke(
    accounts: AcceptVaultAdminAccounts<'_, '_>,
) -> ProgramResult {
    accept_vault_admin_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn accept_vault_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptVaultAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptVaultAdminKeys = accounts.into();
    let ix = accept_vault_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_vault_admin_invoke_signed(
    accounts: AcceptVaultAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_vault_admin_invoke_signed_with_program_id(VOLTR_PROGRAM_ID, accounts, seeds)
}
pub fn accept_vault_admin_verify_account_keys(
    accounts: AcceptVaultAdminAccounts<'_, '_>,
    keys: AcceptVaultAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_admin.key, keys.pending_admin),
        (*accounts.vault.key, keys.vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_vault_admin_verify_writable_privileges<'me, 'info>(
    accounts: AcceptVaultAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_vault_admin_verify_signer_privileges<'me, 'info>(
    accounts: AcceptVaultAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_vault_admin_verify_account_privileges<'me, 'info>(
    accounts: AcceptVaultAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_vault_admin_verify_writable_privileges(accounts)?;
    accept_vault_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_ADAPTOR_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AddAdaptorAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddAdaptorKeys {
    pub payer: Pubkey,
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddAdaptorAccounts<'_, '_>> for AddAdaptorKeys {
    fn from(accounts: AddAdaptorAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            adaptor_program: *accounts.adaptor_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddAdaptorKeys> for [AccountMeta; ADD_ADAPTOR_IX_ACCOUNTS_LEN] {
    fn from(keys: AddAdaptorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
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
impl From<[Pubkey; ADD_ADAPTOR_IX_ACCOUNTS_LEN]> for AddAdaptorKeys {
    fn from(pubkeys: [Pubkey; ADD_ADAPTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            admin: pubkeys[1],
            protocol: pubkeys[2],
            vault: pubkeys[3],
            adaptor_add_receipt: pubkeys[4],
            adaptor_program: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<AddAdaptorAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_ADAPTOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddAdaptorAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.adaptor_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_ADAPTOR_IX_ACCOUNTS_LEN]>
for AddAdaptorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_ADAPTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            admin: &arr[1],
            protocol: &arr[2],
            vault: &arr[3],
            adaptor_add_receipt: &arr[4],
            adaptor_program: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const ADD_ADAPTOR_IX_DISCM: [u8; 8usize] = [161, 145, 203, 248, 211, 202, 203, 67];
#[derive(Clone, Debug, PartialEq)]
pub struct AddAdaptorIxData;
impl AddAdaptorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_ADAPTOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_ADAPTOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_adaptor_ix_with_program_id(
    program_id: Pubkey,
    keys: AddAdaptorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_ADAPTOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AddAdaptorIxData.try_to_vec()?,
    })
}
pub fn add_adaptor_ix(keys: AddAdaptorKeys) -> std::io::Result<Instruction> {
    add_adaptor_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn add_adaptor_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddAdaptorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AddAdaptorKeys = accounts.into();
    let ix = add_adaptor_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_adaptor_invoke(accounts: AddAdaptorAccounts<'_, '_>) -> ProgramResult {
    add_adaptor_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn add_adaptor_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddAdaptorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddAdaptorKeys = accounts.into();
    let ix = add_adaptor_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_adaptor_invoke_signed(
    accounts: AddAdaptorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_adaptor_invoke_signed_with_program_id(VOLTR_PROGRAM_ID, accounts, seeds)
}
pub fn add_adaptor_verify_account_keys(
    accounts: AddAdaptorAccounts<'_, '_>,
    keys: AddAdaptorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_adaptor_verify_writable_privileges<'me, 'info>(
    accounts: AddAdaptorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.vault,
        accounts.adaptor_add_receipt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_adaptor_verify_signer_privileges<'me, 'info>(
    accounts: AddAdaptorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_adaptor_verify_account_privileges<'me, 'info>(
    accounts: AddAdaptorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_adaptor_verify_writable_privileges(accounts)?;
    add_adaptor_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CALIBRATE_HIGH_WATER_MARK_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CalibrateHighWaterMarkAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub admin_lp_ata: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CalibrateHighWaterMarkKeys {
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub admin_lp_ata: Pubkey,
    pub lp_token_program: Pubkey,
}
impl From<CalibrateHighWaterMarkAccounts<'_, '_>> for CalibrateHighWaterMarkKeys {
    fn from(accounts: CalibrateHighWaterMarkAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            admin_lp_ata: *accounts.admin_lp_ata.key,
            lp_token_program: *accounts.lp_token_program.key,
        }
    }
}
impl From<CalibrateHighWaterMarkKeys>
for [AccountMeta; CALIBRATE_HIGH_WATER_MARK_IX_ACCOUNTS_LEN] {
    fn from(keys: CalibrateHighWaterMarkKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CALIBRATE_HIGH_WATER_MARK_IX_ACCOUNTS_LEN]>
for CalibrateHighWaterMarkKeys {
    fn from(pubkeys: [Pubkey; CALIBRATE_HIGH_WATER_MARK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            vault_lp_mint: pubkeys[3],
            admin_lp_ata: pubkeys[4],
            lp_token_program: pubkeys[5],
        }
    }
}
impl<'info> From<CalibrateHighWaterMarkAccounts<'_, 'info>>
for [AccountInfo<'info>; CALIBRATE_HIGH_WATER_MARK_IX_ACCOUNTS_LEN] {
    fn from(accounts: CalibrateHighWaterMarkAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.admin_lp_ata.clone(),
            accounts.lp_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CALIBRATE_HIGH_WATER_MARK_IX_ACCOUNTS_LEN]>
for CalibrateHighWaterMarkAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CALIBRATE_HIGH_WATER_MARK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            vault_lp_mint: &arr[3],
            admin_lp_ata: &arr[4],
            lp_token_program: &arr[5],
        }
    }
}
pub const CALIBRATE_HIGH_WATER_MARK_IX_DISCM: [u8; 8usize] = [
    178, 116, 38, 9, 23, 20, 91, 154,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CalibrateHighWaterMarkIxData;
impl CalibrateHighWaterMarkIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CALIBRATE_HIGH_WATER_MARK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CALIBRATE_HIGH_WATER_MARK_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn calibrate_high_water_mark_ix_with_program_id(
    program_id: Pubkey,
    keys: CalibrateHighWaterMarkKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CALIBRATE_HIGH_WATER_MARK_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CalibrateHighWaterMarkIxData.try_to_vec()?,
    })
}
pub fn calibrate_high_water_mark_ix(
    keys: CalibrateHighWaterMarkKeys,
) -> std::io::Result<Instruction> {
    calibrate_high_water_mark_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn calibrate_high_water_mark_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CalibrateHighWaterMarkAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CalibrateHighWaterMarkKeys = accounts.into();
    let ix = calibrate_high_water_mark_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn calibrate_high_water_mark_invoke(
    accounts: CalibrateHighWaterMarkAccounts<'_, '_>,
) -> ProgramResult {
    calibrate_high_water_mark_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn calibrate_high_water_mark_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CalibrateHighWaterMarkAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CalibrateHighWaterMarkKeys = accounts.into();
    let ix = calibrate_high_water_mark_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn calibrate_high_water_mark_invoke_signed(
    accounts: CalibrateHighWaterMarkAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    calibrate_high_water_mark_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn calibrate_high_water_mark_verify_account_keys(
    accounts: CalibrateHighWaterMarkAccounts<'_, '_>,
    keys: CalibrateHighWaterMarkKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.admin_lp_ata.key, keys.admin_lp_ata),
        (*accounts.lp_token_program.key, keys.lp_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn calibrate_high_water_mark_verify_writable_privileges<'me, 'info>(
    accounts: CalibrateHighWaterMarkAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_lp_mint,
        accounts.admin_lp_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn calibrate_high_water_mark_verify_signer_privileges<'me, 'info>(
    accounts: CalibrateHighWaterMarkAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn calibrate_high_water_mark_verify_account_privileges<'me, 'info>(
    accounts: CalibrateHighWaterMarkAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    calibrate_high_water_mark_verify_writable_privileges(accounts)?;
    calibrate_high_water_mark_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CalibrateHighWaterMarkUnsafeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CalibrateHighWaterMarkUnsafeKeys {
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
}
impl From<CalibrateHighWaterMarkUnsafeAccounts<'_, '_>>
for CalibrateHighWaterMarkUnsafeKeys {
    fn from(accounts: CalibrateHighWaterMarkUnsafeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
        }
    }
}
impl From<CalibrateHighWaterMarkUnsafeKeys>
for [AccountMeta; CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_ACCOUNTS_LEN] {
    fn from(keys: CalibrateHighWaterMarkUnsafeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_ACCOUNTS_LEN]>
for CalibrateHighWaterMarkUnsafeKeys {
    fn from(
        pubkeys: [Pubkey; CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            vault_lp_mint: pubkeys[3],
        }
    }
}
impl<'info> From<CalibrateHighWaterMarkUnsafeAccounts<'_, 'info>>
for [AccountInfo<'info>; CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CalibrateHighWaterMarkUnsafeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_lp_mint.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_ACCOUNTS_LEN]>
for CalibrateHighWaterMarkUnsafeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            vault_lp_mint: &arr[3],
        }
    }
}
pub const CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_DISCM: [u8; 8usize] = [
    162, 16, 96, 202, 178, 8, 187, 235,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CalibrateHighWaterMarkUnsafeIxData;
impl CalibrateHighWaterMarkUnsafeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn calibrate_high_water_mark_unsafe_ix_with_program_id(
    program_id: Pubkey,
    keys: CalibrateHighWaterMarkUnsafeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CALIBRATE_HIGH_WATER_MARK_UNSAFE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CalibrateHighWaterMarkUnsafeIxData.try_to_vec()?,
    })
}
pub fn calibrate_high_water_mark_unsafe_ix(
    keys: CalibrateHighWaterMarkUnsafeKeys,
) -> std::io::Result<Instruction> {
    calibrate_high_water_mark_unsafe_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn calibrate_high_water_mark_unsafe_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CalibrateHighWaterMarkUnsafeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CalibrateHighWaterMarkUnsafeKeys = accounts.into();
    let ix = calibrate_high_water_mark_unsafe_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn calibrate_high_water_mark_unsafe_invoke(
    accounts: CalibrateHighWaterMarkUnsafeAccounts<'_, '_>,
) -> ProgramResult {
    calibrate_high_water_mark_unsafe_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn calibrate_high_water_mark_unsafe_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CalibrateHighWaterMarkUnsafeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CalibrateHighWaterMarkUnsafeKeys = accounts.into();
    let ix = calibrate_high_water_mark_unsafe_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn calibrate_high_water_mark_unsafe_invoke_signed(
    accounts: CalibrateHighWaterMarkUnsafeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    calibrate_high_water_mark_unsafe_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn calibrate_high_water_mark_unsafe_verify_account_keys(
    accounts: CalibrateHighWaterMarkUnsafeAccounts<'_, '_>,
    keys: CalibrateHighWaterMarkUnsafeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn calibrate_high_water_mark_unsafe_verify_writable_privileges<'me, 'info>(
    accounts: CalibrateHighWaterMarkUnsafeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.vault_lp_mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn calibrate_high_water_mark_unsafe_verify_signer_privileges<'me, 'info>(
    accounts: CalibrateHighWaterMarkUnsafeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn calibrate_high_water_mark_unsafe_verify_account_privileges<'me, 'info>(
    accounts: CalibrateHighWaterMarkUnsafeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    calibrate_high_water_mark_unsafe_verify_writable_privileges(accounts)?;
    calibrate_high_water_mark_unsafe_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CancelRequestWithdrawVaultAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub user_lp_ata: &'me AccountInfo<'info>,
    pub request_withdraw_lp_ata: &'me AccountInfo<'info>,
    pub request_withdraw_vault_receipt: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelRequestWithdrawVaultKeys {
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub user_lp_ata: Pubkey,
    pub request_withdraw_lp_ata: Pubkey,
    pub request_withdraw_vault_receipt: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CancelRequestWithdrawVaultAccounts<'_, '_>>
for CancelRequestWithdrawVaultKeys {
    fn from(accounts: CancelRequestWithdrawVaultAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            user_lp_ata: *accounts.user_lp_ata.key,
            request_withdraw_lp_ata: *accounts.request_withdraw_lp_ata.key,
            request_withdraw_vault_receipt: *accounts.request_withdraw_vault_receipt.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CancelRequestWithdrawVaultKeys>
for [AccountMeta; CANCEL_REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelRequestWithdrawVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_vault_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; CANCEL_REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]>
for CancelRequestWithdrawVaultKeys {
    fn from(pubkeys: [Pubkey; CANCEL_REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            vault_lp_mint: pubkeys[3],
            user_lp_ata: pubkeys[4],
            request_withdraw_lp_ata: pubkeys[5],
            request_withdraw_vault_receipt: pubkeys[6],
            lp_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<CancelRequestWithdrawVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelRequestWithdrawVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.user_lp_ata.clone(),
            accounts.request_withdraw_lp_ata.clone(),
            accounts.request_withdraw_vault_receipt.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CANCEL_REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]>
for CancelRequestWithdrawVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            vault_lp_mint: &arr[3],
            user_lp_ata: &arr[4],
            request_withdraw_lp_ata: &arr[5],
            request_withdraw_vault_receipt: &arr[6],
            lp_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const CANCEL_REQUEST_WITHDRAW_VAULT_IX_DISCM: [u8; 8usize] = [
    231, 54, 14, 6, 223, 124, 127, 238,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelRequestWithdrawVaultIxData;
impl CancelRequestWithdrawVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_REQUEST_WITHDRAW_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_REQUEST_WITHDRAW_VAULT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_request_withdraw_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelRequestWithdrawVaultKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelRequestWithdrawVaultIxData.try_to_vec()?,
    })
}
pub fn cancel_request_withdraw_vault_ix(
    keys: CancelRequestWithdrawVaultKeys,
) -> std::io::Result<Instruction> {
    cancel_request_withdraw_vault_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn cancel_request_withdraw_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelRequestWithdrawVaultAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelRequestWithdrawVaultKeys = accounts.into();
    let ix = cancel_request_withdraw_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_request_withdraw_vault_invoke(
    accounts: CancelRequestWithdrawVaultAccounts<'_, '_>,
) -> ProgramResult {
    cancel_request_withdraw_vault_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn cancel_request_withdraw_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelRequestWithdrawVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelRequestWithdrawVaultKeys = accounts.into();
    let ix = cancel_request_withdraw_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_request_withdraw_vault_invoke_signed(
    accounts: CancelRequestWithdrawVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_request_withdraw_vault_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cancel_request_withdraw_vault_verify_account_keys(
    accounts: CancelRequestWithdrawVaultAccounts<'_, '_>,
    keys: CancelRequestWithdrawVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.user_lp_ata.key, keys.user_lp_ata),
        (*accounts.request_withdraw_lp_ata.key, keys.request_withdraw_lp_ata),
        (
            *accounts.request_withdraw_vault_receipt.key,
            keys.request_withdraw_vault_receipt,
        ),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_request_withdraw_vault_verify_writable_privileges<'me, 'info>(
    accounts: CancelRequestWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_transfer_authority,
        accounts.vault,
        accounts.vault_lp_mint,
        accounts.user_lp_ata,
        accounts.request_withdraw_lp_ata,
        accounts.request_withdraw_vault_receipt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_request_withdraw_vault_verify_signer_privileges<'me, 'info>(
    accounts: CancelRequestWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_request_withdraw_vault_verify_account_privileges<'me, 'info>(
    accounts: CancelRequestWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_request_withdraw_vault_verify_writable_privileges(accounts)?;
    cancel_request_withdraw_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_STRATEGY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CloseStrategyAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub manager: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseStrategyKeys {
    pub payer: Pubkey,
    pub manager: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseStrategyAccounts<'_, '_>> for CloseStrategyKeys {
    fn from(accounts: CloseStrategyAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            manager: *accounts.manager.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseStrategyKeys> for [AccountMeta; CLOSE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
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
impl From<[Pubkey; CLOSE_STRATEGY_IX_ACCOUNTS_LEN]> for CloseStrategyKeys {
    fn from(pubkeys: [Pubkey; CLOSE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            manager: pubkeys[1],
            protocol: pubkeys[2],
            vault: pubkeys[3],
            strategy: pubkeys[4],
            strategy_init_receipt: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CloseStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.manager.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_STRATEGY_IX_ACCOUNTS_LEN]>
for CloseStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            manager: &arr[1],
            protocol: &arr[2],
            vault: &arr[3],
            strategy: &arr[4],
            strategy_init_receipt: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CLOSE_STRATEGY_IX_DISCM: [u8; 8usize] = [56, 247, 170, 246, 89, 221, 134, 200];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseStrategyIxData;
impl CloseStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_STRATEGY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseStrategyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseStrategyIxData.try_to_vec()?,
    })
}
pub fn close_strategy_ix(keys: CloseStrategyKeys) -> std::io::Result<Instruction> {
    close_strategy_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn close_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseStrategyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseStrategyKeys = accounts.into();
    let ix = close_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_strategy_invoke(accounts: CloseStrategyAccounts<'_, '_>) -> ProgramResult {
    close_strategy_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn close_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseStrategyKeys = accounts.into();
    let ix = close_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_strategy_invoke_signed(
    accounts: CloseStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_strategy_invoke_signed_with_program_id(VOLTR_PROGRAM_ID, accounts, seeds)
}
pub fn close_strategy_verify_account_keys(
    accounts: CloseStrategyAccounts<'_, '_>,
    keys: CloseStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.manager.key, keys.manager),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_strategy_verify_writable_privileges<'me, 'info>(
    accounts: CloseStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.strategy_init_receipt] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_strategy_verify_signer_privileges<'me, 'info>(
    accounts: CloseStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_strategy_verify_account_privileges<'me, 'info>(
    accounts: CloseStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_strategy_verify_writable_privileges(accounts)?;
    close_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_LP_METADATA_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateLpMetadataAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint_auth: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateLpMetadataKeys {
    pub payer: Pubkey,
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub vault_lp_mint_auth: Pubkey,
    pub metadata_account: Pubkey,
    pub metadata_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateLpMetadataAccounts<'_, '_>> for CreateLpMetadataKeys {
    fn from(accounts: CreateLpMetadataAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            admin: *accounts.admin.key,
            vault: *accounts.vault.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            vault_lp_mint_auth: *accounts.vault_lp_mint_auth.key,
            metadata_account: *accounts.metadata_account.key,
            metadata_program: *accounts.metadata_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateLpMetadataKeys> for [AccountMeta; CREATE_LP_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateLpMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; CREATE_LP_METADATA_IX_ACCOUNTS_LEN]> for CreateLpMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_LP_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            admin: pubkeys[1],
            vault: pubkeys[2],
            vault_lp_mint: pubkeys[3],
            vault_lp_mint_auth: pubkeys[4],
            metadata_account: pubkeys[5],
            metadata_program: pubkeys[6],
            rent: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<CreateLpMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_LP_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateLpMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.admin.clone(),
            accounts.vault.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.vault_lp_mint_auth.clone(),
            accounts.metadata_account.clone(),
            accounts.metadata_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_LP_METADATA_IX_ACCOUNTS_LEN]>
for CreateLpMetadataAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_LP_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            admin: &arr[1],
            vault: &arr[2],
            vault_lp_mint: &arr[3],
            vault_lp_mint_auth: &arr[4],
            metadata_account: &arr[5],
            metadata_program: &arr[6],
            rent: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const CREATE_LP_METADATA_IX_DISCM: [u8; 8usize] = [
    148, 193, 160, 116, 87, 25, 123, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateLpMetadataIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateLpMetadataIxData(pub CreateLpMetadataIxArgs);
impl From<CreateLpMetadataIxArgs> for CreateLpMetadataIxData {
    fn from(args: CreateLpMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl CreateLpMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_LP_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateLpMetadataIxArgs {
                name,
                symbol,
                uri,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_LP_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_lp_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateLpMetadataKeys,
    args: CreateLpMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_LP_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateLpMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_lp_metadata_ix(
    keys: CreateLpMetadataKeys,
    args: CreateLpMetadataIxArgs,
) -> std::io::Result<Instruction> {
    create_lp_metadata_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn create_lp_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateLpMetadataAccounts<'_, '_>,
    args: CreateLpMetadataIxArgs,
) -> ProgramResult {
    let keys: CreateLpMetadataKeys = accounts.into();
    let ix = create_lp_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_lp_metadata_invoke(
    accounts: CreateLpMetadataAccounts<'_, '_>,
    args: CreateLpMetadataIxArgs,
) -> ProgramResult {
    create_lp_metadata_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn create_lp_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateLpMetadataAccounts<'_, '_>,
    args: CreateLpMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateLpMetadataKeys = accounts.into();
    let ix = create_lp_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_lp_metadata_invoke_signed(
    accounts: CreateLpMetadataAccounts<'_, '_>,
    args: CreateLpMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_lp_metadata_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_lp_metadata_verify_account_keys(
    accounts: CreateLpMetadataAccounts<'_, '_>,
    keys: CreateLpMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.admin.key, keys.admin),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.vault_lp_mint_auth.key, keys.vault_lp_mint_auth),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_lp_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CreateLpMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.metadata_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_lp_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CreateLpMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_lp_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreateLpMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_lp_metadata_verify_writable_privileges(accounts)?;
    create_lp_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DepositStrategyAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub vault_asset_idle_auth: &'me AccountInfo<'info>,
    pub vault_strategy_auth: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub vault_asset_idle_ata: &'me AccountInfo<'info>,
    pub vault_strategy_asset_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositStrategyKeys {
    pub manager: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub vault_asset_idle_auth: Pubkey,
    pub vault_strategy_auth: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub vault_asset_idle_ata: Pubkey,
    pub vault_strategy_asset_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub adaptor_program: Pubkey,
}
impl From<DepositStrategyAccounts<'_, '_>> for DepositStrategyKeys {
    fn from(accounts: DepositStrategyAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            vault_asset_idle_auth: *accounts.vault_asset_idle_auth.key,
            vault_strategy_auth: *accounts.vault_strategy_auth.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            vault_asset_idle_ata: *accounts.vault_asset_idle_ata.key,
            vault_strategy_asset_ata: *accounts.vault_strategy_asset_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            adaptor_program: *accounts.adaptor_program.key,
        }
    }
}
impl From<DepositStrategyKeys> for [AccountMeta; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN]> for DepositStrategyKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            strategy: pubkeys[3],
            adaptor_add_receipt: pubkeys[4],
            strategy_init_receipt: pubkeys[5],
            vault_asset_idle_auth: pubkeys[6],
            vault_strategy_auth: pubkeys[7],
            vault_asset_mint: pubkeys[8],
            vault_lp_mint: pubkeys[9],
            vault_asset_idle_ata: pubkeys[10],
            vault_strategy_asset_ata: pubkeys[11],
            asset_token_program: pubkeys[12],
            adaptor_program: pubkeys[13],
        }
    }
}
impl<'info> From<DepositStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.vault_asset_idle_auth.clone(),
            accounts.vault_strategy_auth.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.vault_asset_idle_ata.clone(),
            accounts.vault_strategy_asset_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.adaptor_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN]>
for DepositStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            strategy: &arr[3],
            adaptor_add_receipt: &arr[4],
            strategy_init_receipt: &arr[5],
            vault_asset_idle_auth: &arr[6],
            vault_strategy_auth: &arr[7],
            vault_asset_mint: &arr[8],
            vault_lp_mint: &arr[9],
            vault_asset_idle_ata: &arr[10],
            vault_strategy_asset_ata: &arr[11],
            asset_token_program: &arr[12],
            adaptor_program: &arr[13],
        }
    }
}
pub const DEPOSIT_STRATEGY_IX_DISCM: [u8; 8usize] = [
    246, 82, 57, 226, 131, 222, 253, 249,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositStrategyIxArgs {
    pub amount: u64,
    pub instruction_discriminator: Option<Vec<u8>>,
    pub additional_args: Option<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositStrategyIxData(pub DepositStrategyIxArgs);
impl From<DepositStrategyIxArgs> for DepositStrategyIxData {
    fn from(args: DepositStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl DepositStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_discriminator: Option<Vec<u8>> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositStrategyIxArgs {
                amount,
                instruction_discriminator,
                additional_args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.instruction_discriminator,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.additional_args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositStrategyKeys,
    args: DepositStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_strategy_ix(
    keys: DepositStrategyKeys,
    args: DepositStrategyIxArgs,
) -> std::io::Result<Instruction> {
    deposit_strategy_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn deposit_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositStrategyAccounts<'_, '_>,
    args: DepositStrategyIxArgs,
) -> ProgramResult {
    let keys: DepositStrategyKeys = accounts.into();
    let ix = deposit_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_strategy_invoke(
    accounts: DepositStrategyAccounts<'_, '_>,
    args: DepositStrategyIxArgs,
) -> ProgramResult {
    deposit_strategy_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn deposit_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositStrategyAccounts<'_, '_>,
    args: DepositStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositStrategyKeys = accounts.into();
    let ix = deposit_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_strategy_invoke_signed(
    accounts: DepositStrategyAccounts<'_, '_>,
    args: DepositStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_strategy_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_strategy_verify_account_keys(
    accounts: DepositStrategyAccounts<'_, '_>,
    keys: DepositStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.vault_asset_idle_auth.key, keys.vault_asset_idle_auth),
        (*accounts.vault_strategy_auth.key, keys.vault_strategy_auth),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.vault_asset_idle_ata.key, keys.vault_asset_idle_ata),
        (*accounts.vault_strategy_asset_ata.key, keys.vault_strategy_asset_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.adaptor_program.key, keys.adaptor_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_strategy_verify_writable_privileges<'me, 'info>(
    accounts: DepositStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy_init_receipt,
        accounts.vault_asset_idle_auth,
        accounts.vault_strategy_auth,
        accounts.vault_asset_mint,
        accounts.vault_asset_idle_ata,
        accounts.vault_strategy_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_strategy_verify_signer_privileges<'me, 'info>(
    accounts: DepositStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_strategy_verify_account_privileges<'me, 'info>(
    accounts: DepositStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_strategy_verify_writable_privileges(accounts)?;
    deposit_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_VAULT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct DepositVaultAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_idle_ata: &'me AccountInfo<'info>,
    pub vault_asset_idle_auth: &'me AccountInfo<'info>,
    pub user_lp_ata: &'me AccountInfo<'info>,
    pub vault_lp_mint_auth: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositVaultKeys {
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_idle_ata: Pubkey,
    pub vault_asset_idle_auth: Pubkey,
    pub user_lp_ata: Pubkey,
    pub vault_lp_mint_auth: Pubkey,
    pub asset_token_program: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DepositVaultAccounts<'_, '_>> for DepositVaultKeys {
    fn from(accounts: DepositVaultAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_idle_ata: *accounts.vault_asset_idle_ata.key,
            vault_asset_idle_auth: *accounts.vault_asset_idle_auth.key,
            user_lp_ata: *accounts.user_lp_ata.key,
            vault_lp_mint_auth: *accounts.vault_lp_mint_auth.key,
            asset_token_program: *accounts.asset_token_program.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DepositVaultKeys> for [AccountMeta; DEPOSIT_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; DEPOSIT_VAULT_IX_ACCOUNTS_LEN]> for DepositVaultKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            vault_asset_mint: pubkeys[3],
            vault_lp_mint: pubkeys[4],
            user_asset_ata: pubkeys[5],
            vault_asset_idle_ata: pubkeys[6],
            vault_asset_idle_auth: pubkeys[7],
            user_lp_ata: pubkeys[8],
            vault_lp_mint_auth: pubkeys[9],
            asset_token_program: pubkeys[10],
            lp_token_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<DepositVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_idle_ata.clone(),
            accounts.vault_asset_idle_auth.clone(),
            accounts.user_lp_ata.clone(),
            accounts.vault_lp_mint_auth.clone(),
            accounts.asset_token_program.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_VAULT_IX_ACCOUNTS_LEN]>
for DepositVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            vault_asset_mint: &arr[3],
            vault_lp_mint: &arr[4],
            user_asset_ata: &arr[5],
            vault_asset_idle_ata: &arr[6],
            vault_asset_idle_auth: &arr[7],
            user_lp_ata: &arr[8],
            vault_lp_mint_auth: &arr[9],
            asset_token_program: &arr[10],
            lp_token_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const DEPOSIT_VAULT_IX_DISCM: [u8; 8usize] = [126, 224, 21, 255, 228, 53, 117, 33];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositVaultIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositVaultIxData(pub DepositVaultIxArgs);
impl From<DepositVaultIxArgs> for DepositVaultIxData {
    fn from(args: DepositVaultIxArgs) -> Self {
        Self(args)
    }
}
impl DepositVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositVaultIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositVaultKeys,
    args: DepositVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_vault_ix(
    keys: DepositVaultKeys,
    args: DepositVaultIxArgs,
) -> std::io::Result<Instruction> {
    deposit_vault_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn deposit_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositVaultAccounts<'_, '_>,
    args: DepositVaultIxArgs,
) -> ProgramResult {
    let keys: DepositVaultKeys = accounts.into();
    let ix = deposit_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_vault_invoke(
    accounts: DepositVaultAccounts<'_, '_>,
    args: DepositVaultIxArgs,
) -> ProgramResult {
    deposit_vault_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn deposit_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositVaultAccounts<'_, '_>,
    args: DepositVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositVaultKeys = accounts.into();
    let ix = deposit_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_vault_invoke_signed(
    accounts: DepositVaultAccounts<'_, '_>,
    args: DepositVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_vault_invoke_signed_with_program_id(VOLTR_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_vault_verify_account_keys(
    accounts: DepositVaultAccounts<'_, '_>,
    keys: DepositVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_idle_ata.key, keys.vault_asset_idle_ata),
        (*accounts.vault_asset_idle_auth.key, keys.vault_asset_idle_auth),
        (*accounts.user_lp_ata.key, keys.user_lp_ata),
        (*accounts.vault_lp_mint_auth.key, keys.vault_lp_mint_auth),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_vault_verify_writable_privileges<'me, 'info>(
    accounts: DepositVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_lp_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_idle_ata,
        accounts.user_lp_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_vault_verify_signer_privileges<'me, 'info>(
    accounts: DepositVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_vault_verify_account_privileges<'me, 'info>(
    accounts: DepositVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_vault_verify_writable_privileges(accounts)?;
    deposit_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct DirectWithdrawStrategyAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub direct_withdraw_init_receipt: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub request_withdraw_lp_ata: &'me AccountInfo<'info>,
    pub vault_strategy_auth: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_strategy_asset_ata: &'me AccountInfo<'info>,
    pub request_withdraw_vault_receipt: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DirectWithdrawStrategyKeys {
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub direct_withdraw_init_receipt: Pubkey,
    pub strategy: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub request_withdraw_lp_ata: Pubkey,
    pub vault_strategy_auth: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_strategy_asset_ata: Pubkey,
    pub request_withdraw_vault_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DirectWithdrawStrategyAccounts<'_, '_>> for DirectWithdrawStrategyKeys {
    fn from(accounts: DirectWithdrawStrategyAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            direct_withdraw_init_receipt: *accounts.direct_withdraw_init_receipt.key,
            strategy: *accounts.strategy.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            request_withdraw_lp_ata: *accounts.request_withdraw_lp_ata.key,
            vault_strategy_auth: *accounts.vault_strategy_auth.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_strategy_asset_ata: *accounts.vault_strategy_asset_ata.key,
            request_withdraw_vault_receipt: *accounts.request_withdraw_vault_receipt.key,
            adaptor_program: *accounts.adaptor_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DirectWithdrawStrategyKeys>
for [AccountMeta; DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: DirectWithdrawStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.direct_withdraw_init_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_vault_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]>
for DirectWithdrawStrategyKeys {
    fn from(pubkeys: [Pubkey; DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            adaptor_add_receipt: pubkeys[3],
            strategy_init_receipt: pubkeys[4],
            direct_withdraw_init_receipt: pubkeys[5],
            strategy: pubkeys[6],
            vault_asset_mint: pubkeys[7],
            vault_lp_mint: pubkeys[8],
            request_withdraw_lp_ata: pubkeys[9],
            vault_strategy_auth: pubkeys[10],
            user_asset_ata: pubkeys[11],
            vault_strategy_asset_ata: pubkeys[12],
            request_withdraw_vault_receipt: pubkeys[13],
            adaptor_program: pubkeys[14],
            asset_token_program: pubkeys[15],
            lp_token_program: pubkeys[16],
            system_program: pubkeys[17],
        }
    }
}
impl<'info> From<DirectWithdrawStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DirectWithdrawStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.direct_withdraw_init_receipt.clone(),
            accounts.strategy.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.request_withdraw_lp_ata.clone(),
            accounts.vault_strategy_auth.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_strategy_asset_ata.clone(),
            accounts.request_withdraw_vault_receipt.clone(),
            accounts.adaptor_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]>
for DirectWithdrawStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            adaptor_add_receipt: &arr[3],
            strategy_init_receipt: &arr[4],
            direct_withdraw_init_receipt: &arr[5],
            strategy: &arr[6],
            vault_asset_mint: &arr[7],
            vault_lp_mint: &arr[8],
            request_withdraw_lp_ata: &arr[9],
            vault_strategy_auth: &arr[10],
            user_asset_ata: &arr[11],
            vault_strategy_asset_ata: &arr[12],
            request_withdraw_vault_receipt: &arr[13],
            adaptor_program: &arr[14],
            asset_token_program: &arr[15],
            lp_token_program: &arr[16],
            system_program: &arr[17],
        }
    }
}
pub const DIRECT_WITHDRAW_STRATEGY_IX_DISCM: [u8; 8usize] = [
    119, 33, 54, 52, 194, 8, 211, 239,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DirectWithdrawStrategyIxArgs {
    pub user_args: Option<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DirectWithdrawStrategyIxData(pub DirectWithdrawStrategyIxArgs);
impl From<DirectWithdrawStrategyIxArgs> for DirectWithdrawStrategyIxData {
    fn from(args: DirectWithdrawStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl DirectWithdrawStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DIRECT_WITHDRAW_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let user_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DirectWithdrawStrategyIxArgs {
                user_args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DIRECT_WITHDRAW_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.user_args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn direct_withdraw_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: DirectWithdrawStrategyKeys,
    args: DirectWithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DirectWithdrawStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn direct_withdraw_strategy_ix(
    keys: DirectWithdrawStrategyKeys,
    args: DirectWithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    direct_withdraw_strategy_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn direct_withdraw_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DirectWithdrawStrategyAccounts<'_, '_>,
    args: DirectWithdrawStrategyIxArgs,
) -> ProgramResult {
    let keys: DirectWithdrawStrategyKeys = accounts.into();
    let ix = direct_withdraw_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn direct_withdraw_strategy_invoke(
    accounts: DirectWithdrawStrategyAccounts<'_, '_>,
    args: DirectWithdrawStrategyIxArgs,
) -> ProgramResult {
    direct_withdraw_strategy_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn direct_withdraw_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DirectWithdrawStrategyAccounts<'_, '_>,
    args: DirectWithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DirectWithdrawStrategyKeys = accounts.into();
    let ix = direct_withdraw_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn direct_withdraw_strategy_invoke_signed(
    accounts: DirectWithdrawStrategyAccounts<'_, '_>,
    args: DirectWithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    direct_withdraw_strategy_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn direct_withdraw_strategy_verify_account_keys(
    accounts: DirectWithdrawStrategyAccounts<'_, '_>,
    keys: DirectWithdrawStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.direct_withdraw_init_receipt.key, keys.direct_withdraw_init_receipt),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.request_withdraw_lp_ata.key, keys.request_withdraw_lp_ata),
        (*accounts.vault_strategy_auth.key, keys.vault_strategy_auth),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_strategy_asset_ata.key, keys.vault_strategy_asset_ata),
        (
            *accounts.request_withdraw_vault_receipt.key,
            keys.request_withdraw_vault_receipt,
        ),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn direct_withdraw_strategy_verify_writable_privileges<'me, 'info>(
    accounts: DirectWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_transfer_authority,
        accounts.vault,
        accounts.strategy_init_receipt,
        accounts.vault_asset_mint,
        accounts.vault_lp_mint,
        accounts.request_withdraw_lp_ata,
        accounts.vault_strategy_auth,
        accounts.user_asset_ata,
        accounts.vault_strategy_asset_ata,
        accounts.request_withdraw_vault_receipt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn direct_withdraw_strategy_verify_signer_privileges<'me, 'info>(
    accounts: DirectWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn direct_withdraw_strategy_verify_account_privileges<'me, 'info>(
    accounts: DirectWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    direct_withdraw_strategy_verify_writable_privileges(accounts)?;
    direct_withdraw_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct DirectWithdrawStrategyWithToleranceAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub direct_withdraw_init_receipt: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub request_withdraw_lp_ata: &'me AccountInfo<'info>,
    pub vault_strategy_auth: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_strategy_asset_ata: &'me AccountInfo<'info>,
    pub request_withdraw_vault_receipt: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DirectWithdrawStrategyWithToleranceKeys {
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub direct_withdraw_init_receipt: Pubkey,
    pub strategy: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub request_withdraw_lp_ata: Pubkey,
    pub vault_strategy_auth: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_strategy_asset_ata: Pubkey,
    pub request_withdraw_vault_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DirectWithdrawStrategyWithToleranceAccounts<'_, '_>>
for DirectWithdrawStrategyWithToleranceKeys {
    fn from(accounts: DirectWithdrawStrategyWithToleranceAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            direct_withdraw_init_receipt: *accounts.direct_withdraw_init_receipt.key,
            strategy: *accounts.strategy.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            request_withdraw_lp_ata: *accounts.request_withdraw_lp_ata.key,
            vault_strategy_auth: *accounts.vault_strategy_auth.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_strategy_asset_ata: *accounts.vault_strategy_asset_ata.key,
            request_withdraw_vault_receipt: *accounts.request_withdraw_vault_receipt.key,
            adaptor_program: *accounts.adaptor_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DirectWithdrawStrategyWithToleranceKeys>
for [AccountMeta; DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: DirectWithdrawStrategyWithToleranceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.direct_withdraw_init_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_vault_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN]>
for DirectWithdrawStrategyWithToleranceKeys {
    fn from(
        pubkeys: [Pubkey; DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            adaptor_add_receipt: pubkeys[3],
            strategy_init_receipt: pubkeys[4],
            direct_withdraw_init_receipt: pubkeys[5],
            strategy: pubkeys[6],
            vault_asset_mint: pubkeys[7],
            vault_lp_mint: pubkeys[8],
            request_withdraw_lp_ata: pubkeys[9],
            vault_strategy_auth: pubkeys[10],
            user_asset_ata: pubkeys[11],
            vault_strategy_asset_ata: pubkeys[12],
            request_withdraw_vault_receipt: pubkeys[13],
            adaptor_program: pubkeys[14],
            asset_token_program: pubkeys[15],
            lp_token_program: pubkeys[16],
            system_program: pubkeys[17],
        }
    }
}
impl<'info> From<DirectWithdrawStrategyWithToleranceAccounts<'_, 'info>>
for [AccountInfo<'info>; DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DirectWithdrawStrategyWithToleranceAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.direct_withdraw_init_receipt.clone(),
            accounts.strategy.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.request_withdraw_lp_ata.clone(),
            accounts.vault_strategy_auth.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_strategy_asset_ata.clone(),
            accounts.request_withdraw_vault_receipt.clone(),
            accounts.adaptor_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN],
> for DirectWithdrawStrategyWithToleranceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            adaptor_add_receipt: &arr[3],
            strategy_init_receipt: &arr[4],
            direct_withdraw_init_receipt: &arr[5],
            strategy: &arr[6],
            vault_asset_mint: &arr[7],
            vault_lp_mint: &arr[8],
            request_withdraw_lp_ata: &arr[9],
            vault_strategy_auth: &arr[10],
            user_asset_ata: &arr[11],
            vault_strategy_asset_ata: &arr[12],
            request_withdraw_vault_receipt: &arr[13],
            adaptor_program: &arr[14],
            asset_token_program: &arr[15],
            lp_token_program: &arr[16],
            system_program: &arr[17],
        }
    }
}
pub const DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM: [u8; 8usize] = [
    207, 51, 234, 8, 76, 164, 10, 250,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DirectWithdrawStrategyWithToleranceIxArgs {
    pub user_args: Option<Vec<u8>>,
    pub tolerance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DirectWithdrawStrategyWithToleranceIxData(
    pub DirectWithdrawStrategyWithToleranceIxArgs,
);
impl From<DirectWithdrawStrategyWithToleranceIxArgs>
for DirectWithdrawStrategyWithToleranceIxData {
    fn from(args: DirectWithdrawStrategyWithToleranceIxArgs) -> Self {
        Self(args)
    }
}
impl DirectWithdrawStrategyWithToleranceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let user_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        let tolerance: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DirectWithdrawStrategyWithToleranceIxArgs {
                user_args,
                tolerance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.user_args, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tolerance, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn direct_withdraw_strategy_with_tolerance_ix_with_program_id(
    program_id: Pubkey,
    keys: DirectWithdrawStrategyWithToleranceKeys,
    args: DirectWithdrawStrategyWithToleranceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DIRECT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DirectWithdrawStrategyWithToleranceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn direct_withdraw_strategy_with_tolerance_ix(
    keys: DirectWithdrawStrategyWithToleranceKeys,
    args: DirectWithdrawStrategyWithToleranceIxArgs,
) -> std::io::Result<Instruction> {
    direct_withdraw_strategy_with_tolerance_ix_with_program_id(
        VOLTR_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn direct_withdraw_strategy_with_tolerance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DirectWithdrawStrategyWithToleranceAccounts<'_, '_>,
    args: DirectWithdrawStrategyWithToleranceIxArgs,
) -> ProgramResult {
    let keys: DirectWithdrawStrategyWithToleranceKeys = accounts.into();
    let ix = direct_withdraw_strategy_with_tolerance_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn direct_withdraw_strategy_with_tolerance_invoke(
    accounts: DirectWithdrawStrategyWithToleranceAccounts<'_, '_>,
    args: DirectWithdrawStrategyWithToleranceIxArgs,
) -> ProgramResult {
    direct_withdraw_strategy_with_tolerance_invoke_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn direct_withdraw_strategy_with_tolerance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DirectWithdrawStrategyWithToleranceAccounts<'_, '_>,
    args: DirectWithdrawStrategyWithToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DirectWithdrawStrategyWithToleranceKeys = accounts.into();
    let ix = direct_withdraw_strategy_with_tolerance_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn direct_withdraw_strategy_with_tolerance_invoke_signed(
    accounts: DirectWithdrawStrategyWithToleranceAccounts<'_, '_>,
    args: DirectWithdrawStrategyWithToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    direct_withdraw_strategy_with_tolerance_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn direct_withdraw_strategy_with_tolerance_verify_account_keys(
    accounts: DirectWithdrawStrategyWithToleranceAccounts<'_, '_>,
    keys: DirectWithdrawStrategyWithToleranceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.direct_withdraw_init_receipt.key, keys.direct_withdraw_init_receipt),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.request_withdraw_lp_ata.key, keys.request_withdraw_lp_ata),
        (*accounts.vault_strategy_auth.key, keys.vault_strategy_auth),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_strategy_asset_ata.key, keys.vault_strategy_asset_ata),
        (
            *accounts.request_withdraw_vault_receipt.key,
            keys.request_withdraw_vault_receipt,
        ),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn direct_withdraw_strategy_with_tolerance_verify_writable_privileges<'me, 'info>(
    accounts: DirectWithdrawStrategyWithToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_transfer_authority,
        accounts.vault,
        accounts.strategy_init_receipt,
        accounts.vault_asset_mint,
        accounts.vault_lp_mint,
        accounts.request_withdraw_lp_ata,
        accounts.vault_strategy_auth,
        accounts.user_asset_ata,
        accounts.vault_strategy_asset_ata,
        accounts.request_withdraw_vault_receipt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn direct_withdraw_strategy_with_tolerance_verify_signer_privileges<'me, 'info>(
    accounts: DirectWithdrawStrategyWithToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn direct_withdraw_strategy_with_tolerance_verify_account_privileges<'me, 'info>(
    accounts: DirectWithdrawStrategyWithToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    direct_withdraw_strategy_with_tolerance_verify_writable_privileges(accounts)?;
    direct_withdraw_strategy_with_tolerance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const HARVEST_FEE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct HarvestFeeAccounts<'me, 'info> {
    pub harvester: &'me AccountInfo<'info>,
    pub vault_manager: &'me AccountInfo<'info>,
    pub vault_admin: &'me AccountInfo<'info>,
    pub protocol_treasury: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint_auth: &'me AccountInfo<'info>,
    pub vault_manager_lp_ata: &'me AccountInfo<'info>,
    pub vault_admin_lp_ata: &'me AccountInfo<'info>,
    pub protocol_treasury_lp_ata: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HarvestFeeKeys {
    pub harvester: Pubkey,
    pub vault_manager: Pubkey,
    pub vault_admin: Pubkey,
    pub protocol_treasury: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub vault_lp_mint_auth: Pubkey,
    pub vault_manager_lp_ata: Pubkey,
    pub vault_admin_lp_ata: Pubkey,
    pub protocol_treasury_lp_ata: Pubkey,
    pub lp_token_program: Pubkey,
}
impl From<HarvestFeeAccounts<'_, '_>> for HarvestFeeKeys {
    fn from(accounts: HarvestFeeAccounts) -> Self {
        Self {
            harvester: *accounts.harvester.key,
            vault_manager: *accounts.vault_manager.key,
            vault_admin: *accounts.vault_admin.key,
            protocol_treasury: *accounts.protocol_treasury.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            vault_lp_mint_auth: *accounts.vault_lp_mint_auth.key,
            vault_manager_lp_ata: *accounts.vault_manager_lp_ata.key,
            vault_admin_lp_ata: *accounts.vault_admin_lp_ata.key,
            protocol_treasury_lp_ata: *accounts.protocol_treasury_lp_ata.key,
            lp_token_program: *accounts.lp_token_program.key,
        }
    }
}
impl From<HarvestFeeKeys> for [AccountMeta; HARVEST_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: HarvestFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.harvester,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_manager,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_treasury,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_manager_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_admin_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_treasury_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; HARVEST_FEE_IX_ACCOUNTS_LEN]> for HarvestFeeKeys {
    fn from(pubkeys: [Pubkey; HARVEST_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            harvester: pubkeys[0],
            vault_manager: pubkeys[1],
            vault_admin: pubkeys[2],
            protocol_treasury: pubkeys[3],
            protocol: pubkeys[4],
            vault: pubkeys[5],
            vault_lp_mint: pubkeys[6],
            vault_lp_mint_auth: pubkeys[7],
            vault_manager_lp_ata: pubkeys[8],
            vault_admin_lp_ata: pubkeys[9],
            protocol_treasury_lp_ata: pubkeys[10],
            lp_token_program: pubkeys[11],
        }
    }
}
impl<'info> From<HarvestFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; HARVEST_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: HarvestFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.harvester.clone(),
            accounts.vault_manager.clone(),
            accounts.vault_admin.clone(),
            accounts.protocol_treasury.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.vault_lp_mint_auth.clone(),
            accounts.vault_manager_lp_ata.clone(),
            accounts.vault_admin_lp_ata.clone(),
            accounts.protocol_treasury_lp_ata.clone(),
            accounts.lp_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; HARVEST_FEE_IX_ACCOUNTS_LEN]>
for HarvestFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; HARVEST_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            harvester: &arr[0],
            vault_manager: &arr[1],
            vault_admin: &arr[2],
            protocol_treasury: &arr[3],
            protocol: &arr[4],
            vault: &arr[5],
            vault_lp_mint: &arr[6],
            vault_lp_mint_auth: &arr[7],
            vault_manager_lp_ata: &arr[8],
            vault_admin_lp_ata: &arr[9],
            protocol_treasury_lp_ata: &arr[10],
            lp_token_program: &arr[11],
        }
    }
}
pub const HARVEST_FEE_IX_DISCM: [u8; 8usize] = [32, 59, 42, 128, 246, 73, 255, 47];
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestFeeIxData;
impl HarvestFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HARVEST_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HARVEST_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn harvest_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: HarvestFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; HARVEST_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: HarvestFeeIxData.try_to_vec()?,
    })
}
pub fn harvest_fee_ix(keys: HarvestFeeKeys) -> std::io::Result<Instruction> {
    harvest_fee_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn harvest_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: HarvestFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: HarvestFeeKeys = accounts.into();
    let ix = harvest_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn harvest_fee_invoke(accounts: HarvestFeeAccounts<'_, '_>) -> ProgramResult {
    harvest_fee_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn harvest_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: HarvestFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: HarvestFeeKeys = accounts.into();
    let ix = harvest_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn harvest_fee_invoke_signed(
    accounts: HarvestFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    harvest_fee_invoke_signed_with_program_id(VOLTR_PROGRAM_ID, accounts, seeds)
}
pub fn harvest_fee_verify_account_keys(
    accounts: HarvestFeeAccounts<'_, '_>,
    keys: HarvestFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.harvester.key, keys.harvester),
        (*accounts.vault_manager.key, keys.vault_manager),
        (*accounts.vault_admin.key, keys.vault_admin),
        (*accounts.protocol_treasury.key, keys.protocol_treasury),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.vault_lp_mint_auth.key, keys.vault_lp_mint_auth),
        (*accounts.vault_manager_lp_ata.key, keys.vault_manager_lp_ata),
        (*accounts.vault_admin_lp_ata.key, keys.vault_admin_lp_ata),
        (*accounts.protocol_treasury_lp_ata.key, keys.protocol_treasury_lp_ata),
        (*accounts.lp_token_program.key, keys.lp_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn harvest_fee_verify_writable_privileges<'me, 'info>(
    accounts: HarvestFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_lp_mint,
        accounts.vault_manager_lp_ata,
        accounts.vault_admin_lp_ata,
        accounts.protocol_treasury_lp_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn harvest_fee_verify_signer_privileges<'me, 'info>(
    accounts: HarvestFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.harvester] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn harvest_fee_verify_account_privileges<'me, 'info>(
    accounts: HarvestFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    harvest_fee_verify_writable_privileges(accounts)?;
    harvest_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_PROTOCOL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitProtocolAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitProtocolKeys {
    pub payer: Pubkey,
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitProtocolAccounts<'_, '_>> for InitProtocolKeys {
    fn from(accounts: InitProtocolAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitProtocolKeys> for [AccountMeta; INIT_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitProtocolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
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
impl From<[Pubkey; INIT_PROTOCOL_IX_ACCOUNTS_LEN]> for InitProtocolKeys {
    fn from(pubkeys: [Pubkey; INIT_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            admin: pubkeys[1],
            protocol: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitProtocolAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitProtocolAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_PROTOCOL_IX_ACCOUNTS_LEN]>
for InitProtocolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            admin: &arr[1],
            protocol: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INIT_PROTOCOL_IX_DISCM: [u8; 8usize] = [3, 188, 141, 237, 225, 226, 232, 210];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitProtocolIxArgs {
    pub treasury: Pubkey,
    pub operational_state: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitProtocolIxData(pub InitProtocolIxArgs);
impl From<InitProtocolIxArgs> for InitProtocolIxData {
    fn from(args: InitProtocolIxArgs) -> Self {
        Self(args)
    }
}
impl InitProtocolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_PROTOCOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operational_state: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitProtocolIxArgs {
                treasury,
                operational_state,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_PROTOCOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.operational_state, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_protocol_ix_with_program_id(
    program_id: Pubkey,
    keys: InitProtocolKeys,
    args: InitProtocolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_PROTOCOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitProtocolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_protocol_ix(
    keys: InitProtocolKeys,
    args: InitProtocolIxArgs,
) -> std::io::Result<Instruction> {
    init_protocol_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn init_protocol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitProtocolAccounts<'_, '_>,
    args: InitProtocolIxArgs,
) -> ProgramResult {
    let keys: InitProtocolKeys = accounts.into();
    let ix = init_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_protocol_invoke(
    accounts: InitProtocolAccounts<'_, '_>,
    args: InitProtocolIxArgs,
) -> ProgramResult {
    init_protocol_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn init_protocol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitProtocolAccounts<'_, '_>,
    args: InitProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitProtocolKeys = accounts.into();
    let ix = init_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_protocol_invoke_signed(
    accounts: InitProtocolAccounts<'_, '_>,
    args: InitProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_protocol_invoke_signed_with_program_id(VOLTR_PROGRAM_ID, accounts, args, seeds)
}
pub fn init_protocol_verify_account_keys(
    accounts: InitProtocolAccounts<'_, '_>,
    keys: InitProtocolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_protocol_verify_writable_privileges<'me, 'info>(
    accounts: InitProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.protocol] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_protocol_verify_signer_privileges<'me, 'info>(
    accounts: InitProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_protocol_verify_account_privileges<'me, 'info>(
    accounts: InitProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_protocol_verify_writable_privileges(accounts)?;
    init_protocol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeDirectWithdrawStrategyAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub direct_withdraw_init_receipt: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeDirectWithdrawStrategyKeys {
    pub payer: Pubkey,
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub direct_withdraw_init_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeDirectWithdrawStrategyAccounts<'_, '_>>
for InitializeDirectWithdrawStrategyKeys {
    fn from(accounts: InitializeDirectWithdrawStrategyAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            direct_withdraw_init_receipt: *accounts.direct_withdraw_init_receipt.key,
            adaptor_program: *accounts.adaptor_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeDirectWithdrawStrategyKeys>
for [AccountMeta; INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeDirectWithdrawStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.direct_withdraw_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
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
impl From<[Pubkey; INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]>
for InitializeDirectWithdrawStrategyKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: pubkeys[0],
            admin: pubkeys[1],
            protocol: pubkeys[2],
            vault: pubkeys[3],
            strategy: pubkeys[4],
            strategy_init_receipt: pubkeys[5],
            adaptor_add_receipt: pubkeys[6],
            direct_withdraw_init_receipt: pubkeys[7],
            adaptor_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializeDirectWithdrawStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeDirectWithdrawStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.direct_withdraw_init_receipt.clone(),
            accounts.adaptor_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]>
for InitializeDirectWithdrawStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            admin: &arr[1],
            protocol: &arr[2],
            vault: &arr[3],
            strategy: &arr[4],
            strategy_init_receipt: &arr[5],
            adaptor_add_receipt: &arr[6],
            direct_withdraw_init_receipt: &arr[7],
            adaptor_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_DISCM: [u8; 8usize] = [
    248, 207, 228, 15, 13, 191, 43, 58,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeDirectWithdrawStrategyIxArgs {
    pub instruction_discriminator: Option<Vec<u8>>,
    pub additional_args: Option<Vec<u8>>,
    pub allow_user_args: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeDirectWithdrawStrategyIxData(
    pub InitializeDirectWithdrawStrategyIxArgs,
);
impl From<InitializeDirectWithdrawStrategyIxArgs>
for InitializeDirectWithdrawStrategyIxData {
    fn from(args: InitializeDirectWithdrawStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeDirectWithdrawStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let instruction_discriminator: Option<Vec<u8>> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        let allow_user_args: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeDirectWithdrawStrategyIxArgs {
                instruction_discriminator,
                additional_args,
                allow_user_args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.instruction_discriminator,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.additional_args, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.allow_user_args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_direct_withdraw_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeDirectWithdrawStrategyKeys,
    args: InitializeDirectWithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_DIRECT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeDirectWithdrawStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_direct_withdraw_strategy_ix(
    keys: InitializeDirectWithdrawStrategyKeys,
    args: InitializeDirectWithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    initialize_direct_withdraw_strategy_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn initialize_direct_withdraw_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeDirectWithdrawStrategyAccounts<'_, '_>,
    args: InitializeDirectWithdrawStrategyIxArgs,
) -> ProgramResult {
    let keys: InitializeDirectWithdrawStrategyKeys = accounts.into();
    let ix = initialize_direct_withdraw_strategy_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_direct_withdraw_strategy_invoke(
    accounts: InitializeDirectWithdrawStrategyAccounts<'_, '_>,
    args: InitializeDirectWithdrawStrategyIxArgs,
) -> ProgramResult {
    initialize_direct_withdraw_strategy_invoke_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_direct_withdraw_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeDirectWithdrawStrategyAccounts<'_, '_>,
    args: InitializeDirectWithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeDirectWithdrawStrategyKeys = accounts.into();
    let ix = initialize_direct_withdraw_strategy_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_direct_withdraw_strategy_invoke_signed(
    accounts: InitializeDirectWithdrawStrategyAccounts<'_, '_>,
    args: InitializeDirectWithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_direct_withdraw_strategy_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_direct_withdraw_strategy_verify_account_keys(
    accounts: InitializeDirectWithdrawStrategyAccounts<'_, '_>,
    keys: InitializeDirectWithdrawStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.direct_withdraw_init_receipt.key, keys.direct_withdraw_init_receipt),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_direct_withdraw_strategy_verify_writable_privileges<'me, 'info>(
    accounts: InitializeDirectWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.vault,
        accounts.strategy_init_receipt,
        accounts.direct_withdraw_init_receipt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_direct_withdraw_strategy_verify_signer_privileges<'me, 'info>(
    accounts: InitializeDirectWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_direct_withdraw_strategy_verify_account_privileges<'me, 'info>(
    accounts: InitializeDirectWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_direct_withdraw_strategy_verify_writable_privileges(accounts)?;
    initialize_direct_withdraw_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeStrategyAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub manager: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub vault_strategy_auth: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeStrategyKeys {
    pub payer: Pubkey,
    pub manager: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub vault_strategy_auth: Pubkey,
    pub adaptor_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeStrategyAccounts<'_, '_>> for InitializeStrategyKeys {
    fn from(accounts: InitializeStrategyAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            manager: *accounts.manager.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            vault_strategy_auth: *accounts.vault_strategy_auth.key,
            adaptor_program: *accounts.adaptor_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeStrategyKeys>
for [AccountMeta; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
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
impl From<[Pubkey; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]> for InitializeStrategyKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            manager: pubkeys[1],
            protocol: pubkeys[2],
            vault: pubkeys[3],
            strategy: pubkeys[4],
            adaptor_add_receipt: pubkeys[5],
            strategy_init_receipt: pubkeys[6],
            vault_strategy_auth: pubkeys[7],
            adaptor_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializeStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.manager.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.vault_strategy_auth.clone(),
            accounts.adaptor_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]>
for InitializeStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            manager: &arr[1],
            protocol: &arr[2],
            vault: &arr[3],
            strategy: &arr[4],
            adaptor_add_receipt: &arr[5],
            strategy_init_receipt: &arr[6],
            vault_strategy_auth: &arr[7],
            adaptor_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const INITIALIZE_STRATEGY_IX_DISCM: [u8; 8usize] = [
    208, 119, 144, 145, 178, 57, 105, 252,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeStrategyIxArgs {
    pub instruction_discriminator: Option<Vec<u8>>,
    pub additional_args: Option<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeStrategyIxData(pub InitializeStrategyIxArgs);
impl From<InitializeStrategyIxArgs> for InitializeStrategyIxData {
    fn from(args: InitializeStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let instruction_discriminator: Option<Vec<u8>> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeStrategyIxArgs {
                instruction_discriminator,
                additional_args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.instruction_discriminator,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.additional_args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeStrategyKeys,
    args: InitializeStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_strategy_ix(
    keys: InitializeStrategyKeys,
    args: InitializeStrategyIxArgs,
) -> std::io::Result<Instruction> {
    initialize_strategy_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn initialize_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeStrategyAccounts<'_, '_>,
    args: InitializeStrategyIxArgs,
) -> ProgramResult {
    let keys: InitializeStrategyKeys = accounts.into();
    let ix = initialize_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_strategy_invoke(
    accounts: InitializeStrategyAccounts<'_, '_>,
    args: InitializeStrategyIxArgs,
) -> ProgramResult {
    initialize_strategy_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn initialize_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeStrategyAccounts<'_, '_>,
    args: InitializeStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeStrategyKeys = accounts.into();
    let ix = initialize_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_strategy_invoke_signed(
    accounts: InitializeStrategyAccounts<'_, '_>,
    args: InitializeStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_strategy_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_strategy_verify_account_keys(
    accounts: InitializeStrategyAccounts<'_, '_>,
    keys: InitializeStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.manager.key, keys.manager),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.vault_strategy_auth.key, keys.vault_strategy_auth),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_strategy_verify_writable_privileges<'me, 'info>(
    accounts: InitializeStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.strategy_init_receipt,
        accounts.vault_strategy_auth,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_strategy_verify_signer_privileges<'me, 'info>(
    accounts: InitializeStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_strategy_verify_account_privileges<'me, 'info>(
    accounts: InitializeStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_strategy_verify_writable_privileges(accounts)?;
    initialize_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub manager: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_idle_ata: &'me AccountInfo<'info>,
    pub vault_lp_mint_auth: &'me AccountInfo<'info>,
    pub vault_asset_idle_auth: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeVaultKeys {
    pub payer: Pubkey,
    pub manager: Pubkey,
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_asset_idle_ata: Pubkey,
    pub vault_lp_mint_auth: Pubkey,
    pub vault_asset_idle_auth: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeVaultAccounts<'_, '_>> for InitializeVaultKeys {
    fn from(accounts: InitializeVaultAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            manager: *accounts.manager.key,
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_asset_idle_ata: *accounts.vault_asset_idle_ata.key,
            vault_lp_mint_auth: *accounts.vault_lp_mint_auth.key,
            vault_asset_idle_auth: *accounts.vault_asset_idle_auth.key,
            clock: *accounts.clock.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeVaultKeys> for [AccountMeta; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]> for InitializeVaultKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            manager: pubkeys[1],
            admin: pubkeys[2],
            protocol: pubkeys[3],
            vault: pubkeys[4],
            vault_lp_mint: pubkeys[5],
            vault_asset_mint: pubkeys[6],
            vault_asset_idle_ata: pubkeys[7],
            vault_lp_mint_auth: pubkeys[8],
            vault_asset_idle_auth: pubkeys[9],
            clock: pubkeys[10],
            rent: pubkeys[11],
            associated_token_program: pubkeys[12],
            asset_token_program: pubkeys[13],
            lp_token_program: pubkeys[14],
            system_program: pubkeys[15],
        }
    }
}
impl<'info> From<InitializeVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.manager.clone(),
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_asset_idle_ata.clone(),
            accounts.vault_lp_mint_auth.clone(),
            accounts.vault_asset_idle_auth.clone(),
            accounts.clock.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]>
for InitializeVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            manager: &arr[1],
            admin: &arr[2],
            protocol: &arr[3],
            vault: &arr[4],
            vault_lp_mint: &arr[5],
            vault_asset_mint: &arr[6],
            vault_asset_idle_ata: &arr[7],
            vault_lp_mint_auth: &arr[8],
            vault_asset_idle_auth: &arr[9],
            clock: &arr[10],
            rent: &arr[11],
            associated_token_program: &arr[12],
            asset_token_program: &arr[13],
            lp_token_program: &arr[14],
            system_program: &arr[15],
        }
    }
}
pub const INITIALIZE_VAULT_IX_DISCM: [u8; 8usize] = [48, 191, 163, 44, 71, 129, 63, 164];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeVaultIxArgs {
    pub config: VaultInitializationInput,
    pub name: String,
    pub description: String,
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
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <VaultInitializationInput>::deserialize(&mut reader)?
        };
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let description: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeVaultIxArgs {
                config,
                name,
                description,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.description, &mut writer)?;
        Ok(())
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
    initialize_vault_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
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
    initialize_vault_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
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
        VOLTR_PROGRAM_ID,
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
        (*accounts.payer.key, keys.payer),
        (*accounts.manager.key, keys.manager),
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_asset_idle_ata.key, keys.vault_asset_idle_ata),
        (*accounts.vault_lp_mint_auth.key, keys.vault_lp_mint_auth),
        (*accounts.vault_asset_idle_auth.key, keys.vault_asset_idle_auth),
        (*accounts.clock.key, keys.clock),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.vault,
        accounts.vault_lp_mint,
        accounts.vault_asset_idle_ata,
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
    for should_be_signer in [accounts.payer, accounts.vault] {
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
pub const INSTANT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct InstantWithdrawStrategyAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub direct_withdraw_init_receipt: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub user_lp_ata: &'me AccountInfo<'info>,
    pub vault_strategy_auth: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_strategy_asset_ata: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantWithdrawStrategyKeys {
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub direct_withdraw_init_receipt: Pubkey,
    pub strategy: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub user_lp_ata: Pubkey,
    pub vault_strategy_auth: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_strategy_asset_ata: Pubkey,
    pub adaptor_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InstantWithdrawStrategyAccounts<'_, '_>> for InstantWithdrawStrategyKeys {
    fn from(accounts: InstantWithdrawStrategyAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            direct_withdraw_init_receipt: *accounts.direct_withdraw_init_receipt.key,
            strategy: *accounts.strategy.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            user_lp_ata: *accounts.user_lp_ata.key,
            vault_strategy_auth: *accounts.vault_strategy_auth.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_strategy_asset_ata: *accounts.vault_strategy_asset_ata.key,
            adaptor_program: *accounts.adaptor_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InstantWithdrawStrategyKeys>
for [AccountMeta; INSTANT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantWithdrawStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.direct_withdraw_init_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; INSTANT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]>
for InstantWithdrawStrategyKeys {
    fn from(pubkeys: [Pubkey; INSTANT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            adaptor_add_receipt: pubkeys[3],
            strategy_init_receipt: pubkeys[4],
            direct_withdraw_init_receipt: pubkeys[5],
            strategy: pubkeys[6],
            vault_asset_mint: pubkeys[7],
            vault_lp_mint: pubkeys[8],
            user_lp_ata: pubkeys[9],
            vault_strategy_auth: pubkeys[10],
            user_asset_ata: pubkeys[11],
            vault_strategy_asset_ata: pubkeys[12],
            adaptor_program: pubkeys[13],
            asset_token_program: pubkeys[14],
            lp_token_program: pubkeys[15],
            system_program: pubkeys[16],
        }
    }
}
impl<'info> From<InstantWithdrawStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantWithdrawStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.direct_withdraw_init_receipt.clone(),
            accounts.strategy.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.user_lp_ata.clone(),
            accounts.vault_strategy_auth.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_strategy_asset_ata.clone(),
            accounts.adaptor_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INSTANT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]>
for InstantWithdrawStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            adaptor_add_receipt: &arr[3],
            strategy_init_receipt: &arr[4],
            direct_withdraw_init_receipt: &arr[5],
            strategy: &arr[6],
            vault_asset_mint: &arr[7],
            vault_lp_mint: &arr[8],
            user_lp_ata: &arr[9],
            vault_strategy_auth: &arr[10],
            user_asset_ata: &arr[11],
            vault_strategy_asset_ata: &arr[12],
            adaptor_program: &arr[13],
            asset_token_program: &arr[14],
            lp_token_program: &arr[15],
            system_program: &arr[16],
        }
    }
}
pub const INSTANT_WITHDRAW_STRATEGY_IX_DISCM: [u8; 8usize] = [
    105, 57, 166, 130, 147, 221, 250, 189,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawStrategyIxArgs {
    pub amount: u64,
    pub is_amount_in_lp: bool,
    pub is_withdraw_all: bool,
    pub user_args: Option<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawStrategyIxData(pub InstantWithdrawStrategyIxArgs);
impl From<InstantWithdrawStrategyIxArgs> for InstantWithdrawStrategyIxData {
    fn from(args: InstantWithdrawStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl InstantWithdrawStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAW_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
        let user_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InstantWithdrawStrategyIxArgs {
                amount,
                is_amount_in_lp,
                is_withdraw_all,
                user_args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAW_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_amount_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_withdraw_all, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user_args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_withdraw_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantWithdrawStrategyKeys,
    args: InstantWithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantWithdrawStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_withdraw_strategy_ix(
    keys: InstantWithdrawStrategyKeys,
    args: InstantWithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    instant_withdraw_strategy_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn instant_withdraw_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawStrategyAccounts<'_, '_>,
    args: InstantWithdrawStrategyIxArgs,
) -> ProgramResult {
    let keys: InstantWithdrawStrategyKeys = accounts.into();
    let ix = instant_withdraw_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_withdraw_strategy_invoke(
    accounts: InstantWithdrawStrategyAccounts<'_, '_>,
    args: InstantWithdrawStrategyIxArgs,
) -> ProgramResult {
    instant_withdraw_strategy_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn instant_withdraw_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawStrategyAccounts<'_, '_>,
    args: InstantWithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantWithdrawStrategyKeys = accounts.into();
    let ix = instant_withdraw_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_withdraw_strategy_invoke_signed(
    accounts: InstantWithdrawStrategyAccounts<'_, '_>,
    args: InstantWithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_withdraw_strategy_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_withdraw_strategy_verify_account_keys(
    accounts: InstantWithdrawStrategyAccounts<'_, '_>,
    keys: InstantWithdrawStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.direct_withdraw_init_receipt.key, keys.direct_withdraw_init_receipt),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.user_lp_ata.key, keys.user_lp_ata),
        (*accounts.vault_strategy_auth.key, keys.vault_strategy_auth),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_strategy_asset_ata.key, keys.vault_strategy_asset_ata),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_withdraw_strategy_verify_writable_privileges<'me, 'info>(
    accounts: InstantWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_transfer_authority,
        accounts.vault,
        accounts.strategy_init_receipt,
        accounts.vault_asset_mint,
        accounts.vault_lp_mint,
        accounts.user_lp_ata,
        accounts.vault_strategy_auth,
        accounts.user_asset_ata,
        accounts.vault_strategy_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_withdraw_strategy_verify_signer_privileges<'me, 'info>(
    accounts: InstantWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_withdraw_strategy_verify_account_privileges<'me, 'info>(
    accounts: InstantWithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_withdraw_strategy_verify_writable_privileges(accounts)?;
    instant_withdraw_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct InstantWithdrawStrategyWithToleranceAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub direct_withdraw_init_receipt: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub user_lp_ata: &'me AccountInfo<'info>,
    pub vault_strategy_auth: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_strategy_asset_ata: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantWithdrawStrategyWithToleranceKeys {
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub direct_withdraw_init_receipt: Pubkey,
    pub strategy: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub user_lp_ata: Pubkey,
    pub vault_strategy_auth: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_strategy_asset_ata: Pubkey,
    pub adaptor_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InstantWithdrawStrategyWithToleranceAccounts<'_, '_>>
for InstantWithdrawStrategyWithToleranceKeys {
    fn from(accounts: InstantWithdrawStrategyWithToleranceAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            direct_withdraw_init_receipt: *accounts.direct_withdraw_init_receipt.key,
            strategy: *accounts.strategy.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            user_lp_ata: *accounts.user_lp_ata.key,
            vault_strategy_auth: *accounts.vault_strategy_auth.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_strategy_asset_ata: *accounts.vault_strategy_asset_ata.key,
            adaptor_program: *accounts.adaptor_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InstantWithdrawStrategyWithToleranceKeys>
for [AccountMeta; INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantWithdrawStrategyWithToleranceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.direct_withdraw_init_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN]>
for InstantWithdrawStrategyWithToleranceKeys {
    fn from(
        pubkeys: [Pubkey; INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            adaptor_add_receipt: pubkeys[3],
            strategy_init_receipt: pubkeys[4],
            direct_withdraw_init_receipt: pubkeys[5],
            strategy: pubkeys[6],
            vault_asset_mint: pubkeys[7],
            vault_lp_mint: pubkeys[8],
            user_lp_ata: pubkeys[9],
            vault_strategy_auth: pubkeys[10],
            user_asset_ata: pubkeys[11],
            vault_strategy_asset_ata: pubkeys[12],
            adaptor_program: pubkeys[13],
            asset_token_program: pubkeys[14],
            lp_token_program: pubkeys[15],
            system_program: pubkeys[16],
        }
    }
}
impl<'info> From<InstantWithdrawStrategyWithToleranceAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantWithdrawStrategyWithToleranceAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.direct_withdraw_init_receipt.clone(),
            accounts.strategy.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.user_lp_ata.clone(),
            accounts.vault_strategy_auth.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_strategy_asset_ata.clone(),
            accounts.adaptor_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN],
> for InstantWithdrawStrategyWithToleranceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            adaptor_add_receipt: &arr[3],
            strategy_init_receipt: &arr[4],
            direct_withdraw_init_receipt: &arr[5],
            strategy: &arr[6],
            vault_asset_mint: &arr[7],
            vault_lp_mint: &arr[8],
            user_lp_ata: &arr[9],
            vault_strategy_auth: &arr[10],
            user_asset_ata: &arr[11],
            vault_strategy_asset_ata: &arr[12],
            adaptor_program: &arr[13],
            asset_token_program: &arr[14],
            lp_token_program: &arr[15],
            system_program: &arr[16],
        }
    }
}
pub const INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM: [u8; 8usize] = [
    91, 21, 70, 163, 84, 67, 106, 181,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawStrategyWithToleranceIxArgs {
    pub amount: u64,
    pub is_amount_in_lp: bool,
    pub is_withdraw_all: bool,
    pub user_args: Option<Vec<u8>>,
    pub tolerance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawStrategyWithToleranceIxData(
    pub InstantWithdrawStrategyWithToleranceIxArgs,
);
impl From<InstantWithdrawStrategyWithToleranceIxArgs>
for InstantWithdrawStrategyWithToleranceIxData {
    fn from(args: InstantWithdrawStrategyWithToleranceIxArgs) -> Self {
        Self(args)
    }
}
impl InstantWithdrawStrategyWithToleranceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
        let user_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        let tolerance: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InstantWithdrawStrategyWithToleranceIxArgs {
                amount,
                is_amount_in_lp,
                is_withdraw_all,
                user_args,
                tolerance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_amount_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_withdraw_all, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.user_args, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tolerance, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_withdraw_strategy_with_tolerance_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantWithdrawStrategyWithToleranceKeys,
    args: InstantWithdrawStrategyWithToleranceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_WITHDRAW_STRATEGY_WITH_TOLERANCE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InstantWithdrawStrategyWithToleranceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_withdraw_strategy_with_tolerance_ix(
    keys: InstantWithdrawStrategyWithToleranceKeys,
    args: InstantWithdrawStrategyWithToleranceIxArgs,
) -> std::io::Result<Instruction> {
    instant_withdraw_strategy_with_tolerance_ix_with_program_id(
        VOLTR_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn instant_withdraw_strategy_with_tolerance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawStrategyWithToleranceAccounts<'_, '_>,
    args: InstantWithdrawStrategyWithToleranceIxArgs,
) -> ProgramResult {
    let keys: InstantWithdrawStrategyWithToleranceKeys = accounts.into();
    let ix = instant_withdraw_strategy_with_tolerance_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_withdraw_strategy_with_tolerance_invoke(
    accounts: InstantWithdrawStrategyWithToleranceAccounts<'_, '_>,
    args: InstantWithdrawStrategyWithToleranceIxArgs,
) -> ProgramResult {
    instant_withdraw_strategy_with_tolerance_invoke_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn instant_withdraw_strategy_with_tolerance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawStrategyWithToleranceAccounts<'_, '_>,
    args: InstantWithdrawStrategyWithToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantWithdrawStrategyWithToleranceKeys = accounts.into();
    let ix = instant_withdraw_strategy_with_tolerance_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_withdraw_strategy_with_tolerance_invoke_signed(
    accounts: InstantWithdrawStrategyWithToleranceAccounts<'_, '_>,
    args: InstantWithdrawStrategyWithToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_withdraw_strategy_with_tolerance_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_withdraw_strategy_with_tolerance_verify_account_keys(
    accounts: InstantWithdrawStrategyWithToleranceAccounts<'_, '_>,
    keys: InstantWithdrawStrategyWithToleranceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.direct_withdraw_init_receipt.key, keys.direct_withdraw_init_receipt),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.user_lp_ata.key, keys.user_lp_ata),
        (*accounts.vault_strategy_auth.key, keys.vault_strategy_auth),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_strategy_asset_ata.key, keys.vault_strategy_asset_ata),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_withdraw_strategy_with_tolerance_verify_writable_privileges<'me, 'info>(
    accounts: InstantWithdrawStrategyWithToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_transfer_authority,
        accounts.vault,
        accounts.strategy_init_receipt,
        accounts.vault_asset_mint,
        accounts.vault_lp_mint,
        accounts.user_lp_ata,
        accounts.vault_strategy_auth,
        accounts.user_asset_ata,
        accounts.vault_strategy_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_withdraw_strategy_with_tolerance_verify_signer_privileges<'me, 'info>(
    accounts: InstantWithdrawStrategyWithToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_withdraw_strategy_with_tolerance_verify_account_privileges<'me, 'info>(
    accounts: InstantWithdrawStrategyWithToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_withdraw_strategy_with_tolerance_verify_writable_privileges(accounts)?;
    instant_withdraw_strategy_with_tolerance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_WITHDRAW_VAULT_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InstantWithdrawVaultAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub user_lp_ata: &'me AccountInfo<'info>,
    pub vault_asset_idle_ata: &'me AccountInfo<'info>,
    pub vault_asset_idle_auth: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantWithdrawVaultKeys {
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub user_lp_ata: Pubkey,
    pub vault_asset_idle_ata: Pubkey,
    pub vault_asset_idle_auth: Pubkey,
    pub user_asset_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InstantWithdrawVaultAccounts<'_, '_>> for InstantWithdrawVaultKeys {
    fn from(accounts: InstantWithdrawVaultAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            user_lp_ata: *accounts.user_lp_ata.key,
            vault_asset_idle_ata: *accounts.vault_asset_idle_ata.key,
            vault_asset_idle_auth: *accounts.vault_asset_idle_auth.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InstantWithdrawVaultKeys>
for [AccountMeta; INSTANT_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantWithdrawVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; INSTANT_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]>
for InstantWithdrawVaultKeys {
    fn from(pubkeys: [Pubkey; INSTANT_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            vault_asset_mint: pubkeys[3],
            vault_lp_mint: pubkeys[4],
            user_lp_ata: pubkeys[5],
            vault_asset_idle_ata: pubkeys[6],
            vault_asset_idle_auth: pubkeys[7],
            user_asset_ata: pubkeys[8],
            asset_token_program: pubkeys[9],
            lp_token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<InstantWithdrawVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantWithdrawVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.user_lp_ata.clone(),
            accounts.vault_asset_idle_ata.clone(),
            accounts.vault_asset_idle_auth.clone(),
            accounts.user_asset_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INSTANT_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]>
for InstantWithdrawVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_WITHDRAW_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            vault_asset_mint: &arr[3],
            vault_lp_mint: &arr[4],
            user_lp_ata: &arr[5],
            vault_asset_idle_ata: &arr[6],
            vault_asset_idle_auth: &arr[7],
            user_asset_ata: &arr[8],
            asset_token_program: &arr[9],
            lp_token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const INSTANT_WITHDRAW_VAULT_IX_DISCM: [u8; 8usize] = [
    221, 56, 115, 168, 128, 220, 235, 245,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawVaultIxArgs {
    pub amount: u64,
    pub is_amount_in_lp: bool,
    pub is_withdraw_all: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawVaultIxData(pub InstantWithdrawVaultIxArgs);
impl From<InstantWithdrawVaultIxArgs> for InstantWithdrawVaultIxData {
    fn from(args: InstantWithdrawVaultIxArgs) -> Self {
        Self(args)
    }
}
impl InstantWithdrawVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAW_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InstantWithdrawVaultIxArgs {
                amount,
                is_amount_in_lp,
                is_withdraw_all,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAW_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_amount_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_withdraw_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_withdraw_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantWithdrawVaultKeys,
    args: InstantWithdrawVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantWithdrawVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_withdraw_vault_ix(
    keys: InstantWithdrawVaultKeys,
    args: InstantWithdrawVaultIxArgs,
) -> std::io::Result<Instruction> {
    instant_withdraw_vault_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn instant_withdraw_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawVaultAccounts<'_, '_>,
    args: InstantWithdrawVaultIxArgs,
) -> ProgramResult {
    let keys: InstantWithdrawVaultKeys = accounts.into();
    let ix = instant_withdraw_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_withdraw_vault_invoke(
    accounts: InstantWithdrawVaultAccounts<'_, '_>,
    args: InstantWithdrawVaultIxArgs,
) -> ProgramResult {
    instant_withdraw_vault_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn instant_withdraw_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawVaultAccounts<'_, '_>,
    args: InstantWithdrawVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantWithdrawVaultKeys = accounts.into();
    let ix = instant_withdraw_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_withdraw_vault_invoke_signed(
    accounts: InstantWithdrawVaultAccounts<'_, '_>,
    args: InstantWithdrawVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_withdraw_vault_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_withdraw_vault_verify_account_keys(
    accounts: InstantWithdrawVaultAccounts<'_, '_>,
    keys: InstantWithdrawVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.user_lp_ata.key, keys.user_lp_ata),
        (*accounts.vault_asset_idle_ata.key, keys.vault_asset_idle_ata),
        (*accounts.vault_asset_idle_auth.key, keys.vault_asset_idle_auth),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_withdraw_vault_verify_writable_privileges<'me, 'info>(
    accounts: InstantWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_lp_mint,
        accounts.user_lp_ata,
        accounts.vault_asset_idle_ata,
        accounts.vault_asset_idle_auth,
        accounts.user_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_withdraw_vault_verify_signer_privileges<'me, 'info>(
    accounts: InstantWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_withdraw_vault_verify_account_privileges<'me, 'info>(
    accounts: InstantWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_withdraw_vault_verify_writable_privileges(accounts)?;
    instant_withdraw_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_ADAPTOR_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RemoveAdaptorAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAdaptorKeys {
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RemoveAdaptorAccounts<'_, '_>> for RemoveAdaptorKeys {
    fn from(accounts: RemoveAdaptorAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            adaptor_program: *accounts.adaptor_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RemoveAdaptorKeys> for [AccountMeta; REMOVE_ADAPTOR_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAdaptorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
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
impl From<[Pubkey; REMOVE_ADAPTOR_IX_ACCOUNTS_LEN]> for RemoveAdaptorKeys {
    fn from(pubkeys: [Pubkey; REMOVE_ADAPTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            adaptor_add_receipt: pubkeys[3],
            adaptor_program: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<RemoveAdaptorAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_ADAPTOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAdaptorAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.adaptor_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_ADAPTOR_IX_ACCOUNTS_LEN]>
for RemoveAdaptorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_ADAPTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            adaptor_add_receipt: &arr[3],
            adaptor_program: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const REMOVE_ADAPTOR_IX_DISCM: [u8; 8usize] = [161, 199, 99, 22, 25, 193, 61, 193];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAdaptorIxData;
impl RemoveAdaptorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ADAPTOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ADAPTOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_adaptor_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveAdaptorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ADAPTOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveAdaptorIxData.try_to_vec()?,
    })
}
pub fn remove_adaptor_ix(keys: RemoveAdaptorKeys) -> std::io::Result<Instruction> {
    remove_adaptor_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn remove_adaptor_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAdaptorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveAdaptorKeys = accounts.into();
    let ix = remove_adaptor_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_adaptor_invoke(accounts: RemoveAdaptorAccounts<'_, '_>) -> ProgramResult {
    remove_adaptor_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn remove_adaptor_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAdaptorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAdaptorKeys = accounts.into();
    let ix = remove_adaptor_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_adaptor_invoke_signed(
    accounts: RemoveAdaptorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_adaptor_invoke_signed_with_program_id(VOLTR_PROGRAM_ID, accounts, seeds)
}
pub fn remove_adaptor_verify_account_keys(
    accounts: RemoveAdaptorAccounts<'_, '_>,
    keys: RemoveAdaptorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_adaptor_verify_writable_privileges<'me, 'info>(
    accounts: RemoveAdaptorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.vault,
        accounts.adaptor_add_receipt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_adaptor_verify_signer_privileges<'me, 'info>(
    accounts: RemoveAdaptorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_adaptor_verify_account_privileges<'me, 'info>(
    accounts: RemoveAdaptorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_adaptor_verify_writable_privileges(accounts)?;
    remove_adaptor_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RequestWithdrawVaultAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub user_lp_ata: &'me AccountInfo<'info>,
    pub request_withdraw_lp_ata: &'me AccountInfo<'info>,
    pub request_withdraw_vault_receipt: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RequestWithdrawVaultKeys {
    pub payer: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub user_lp_ata: Pubkey,
    pub request_withdraw_lp_ata: Pubkey,
    pub request_withdraw_vault_receipt: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RequestWithdrawVaultAccounts<'_, '_>> for RequestWithdrawVaultKeys {
    fn from(accounts: RequestWithdrawVaultAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            user_lp_ata: *accounts.user_lp_ata.key,
            request_withdraw_lp_ata: *accounts.request_withdraw_lp_ata.key,
            request_withdraw_vault_receipt: *accounts.request_withdraw_vault_receipt.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RequestWithdrawVaultKeys>
for [AccountMeta; REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: RequestWithdrawVaultKeys) -> Self {
        [
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
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_vault_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]>
for RequestWithdrawVaultKeys {
    fn from(pubkeys: [Pubkey; REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            protocol: pubkeys[2],
            vault: pubkeys[3],
            vault_lp_mint: pubkeys[4],
            user_lp_ata: pubkeys[5],
            request_withdraw_lp_ata: pubkeys[6],
            request_withdraw_vault_receipt: pubkeys[7],
            lp_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<RequestWithdrawVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: RequestWithdrawVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.user_lp_ata.clone(),
            accounts.request_withdraw_lp_ata.clone(),
            accounts.request_withdraw_vault_receipt.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN]>
for RequestWithdrawVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            user_transfer_authority: &arr[1],
            protocol: &arr[2],
            vault: &arr[3],
            vault_lp_mint: &arr[4],
            user_lp_ata: &arr[5],
            request_withdraw_lp_ata: &arr[6],
            request_withdraw_vault_receipt: &arr[7],
            lp_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const REQUEST_WITHDRAW_VAULT_IX_DISCM: [u8; 8usize] = [
    248, 225, 47, 22, 116, 144, 23, 143,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RequestWithdrawVaultIxArgs {
    pub amount: u64,
    pub is_amount_in_lp: bool,
    pub is_withdraw_all: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestWithdrawVaultIxData(pub RequestWithdrawVaultIxArgs);
impl From<RequestWithdrawVaultIxArgs> for RequestWithdrawVaultIxData {
    fn from(args: RequestWithdrawVaultIxArgs) -> Self {
        Self(args)
    }
}
impl RequestWithdrawVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_WITHDRAW_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RequestWithdrawVaultIxArgs {
                amount,
                is_amount_in_lp,
                is_withdraw_all,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_WITHDRAW_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_amount_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_withdraw_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn request_withdraw_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: RequestWithdrawVaultKeys,
    args: RequestWithdrawVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REQUEST_WITHDRAW_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: RequestWithdrawVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_withdraw_vault_ix(
    keys: RequestWithdrawVaultKeys,
    args: RequestWithdrawVaultIxArgs,
) -> std::io::Result<Instruction> {
    request_withdraw_vault_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn request_withdraw_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RequestWithdrawVaultAccounts<'_, '_>,
    args: RequestWithdrawVaultIxArgs,
) -> ProgramResult {
    let keys: RequestWithdrawVaultKeys = accounts.into();
    let ix = request_withdraw_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn request_withdraw_vault_invoke(
    accounts: RequestWithdrawVaultAccounts<'_, '_>,
    args: RequestWithdrawVaultIxArgs,
) -> ProgramResult {
    request_withdraw_vault_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn request_withdraw_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RequestWithdrawVaultAccounts<'_, '_>,
    args: RequestWithdrawVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RequestWithdrawVaultKeys = accounts.into();
    let ix = request_withdraw_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn request_withdraw_vault_invoke_signed(
    accounts: RequestWithdrawVaultAccounts<'_, '_>,
    args: RequestWithdrawVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    request_withdraw_vault_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn request_withdraw_vault_verify_account_keys(
    accounts: RequestWithdrawVaultAccounts<'_, '_>,
    keys: RequestWithdrawVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.user_lp_ata.key, keys.user_lp_ata),
        (*accounts.request_withdraw_lp_ata.key, keys.request_withdraw_lp_ata),
        (
            *accounts.request_withdraw_vault_receipt.key,
            keys.request_withdraw_vault_receipt,
        ),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn request_withdraw_vault_verify_writable_privileges<'me, 'info>(
    accounts: RequestWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.user_lp_ata,
        accounts.request_withdraw_lp_ata,
        accounts.request_withdraw_vault_receipt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn request_withdraw_vault_verify_signer_privileges<'me, 'info>(
    accounts: RequestWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn request_withdraw_vault_verify_account_privileges<'me, 'info>(
    accounts: RequestWithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    request_withdraw_vault_verify_writable_privileges(accounts)?;
    request_withdraw_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PROTOCOL_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateProtocolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateProtocolKeys {
    pub admin: Pubkey,
    pub protocol: Pubkey,
}
impl From<UpdateProtocolAccounts<'_, '_>> for UpdateProtocolKeys {
    fn from(accounts: UpdateProtocolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
        }
    }
}
impl From<UpdateProtocolKeys> for [AccountMeta; UPDATE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateProtocolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PROTOCOL_IX_ACCOUNTS_LEN]> for UpdateProtocolKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            protocol: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateProtocolAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateProtocolAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.protocol.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PROTOCOL_IX_ACCOUNTS_LEN]>
for UpdateProtocolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            protocol: &arr[1],
        }
    }
}
pub const UPDATE_PROTOCOL_IX_DISCM: [u8; 8usize] = [206, 25, 218, 114, 109, 41, 74, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateProtocolIxArgs {
    pub field: ProtocolConfigField,
    pub data: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateProtocolIxData(pub UpdateProtocolIxArgs);
impl From<UpdateProtocolIxArgs> for UpdateProtocolIxData {
    fn from(args: UpdateProtocolIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateProtocolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PROTOCOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field: ProtocolConfigField = crate::borsh_de_or_default(&mut reader)?;
        let data: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateProtocolIxArgs {
                field,
                data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PROTOCOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_protocol_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateProtocolKeys,
    args: UpdateProtocolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PROTOCOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateProtocolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_protocol_ix(
    keys: UpdateProtocolKeys,
    args: UpdateProtocolIxArgs,
) -> std::io::Result<Instruction> {
    update_protocol_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn update_protocol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateProtocolAccounts<'_, '_>,
    args: UpdateProtocolIxArgs,
) -> ProgramResult {
    let keys: UpdateProtocolKeys = accounts.into();
    let ix = update_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_protocol_invoke(
    accounts: UpdateProtocolAccounts<'_, '_>,
    args: UpdateProtocolIxArgs,
) -> ProgramResult {
    update_protocol_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn update_protocol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateProtocolAccounts<'_, '_>,
    args: UpdateProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateProtocolKeys = accounts.into();
    let ix = update_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_protocol_invoke_signed(
    accounts: UpdateProtocolAccounts<'_, '_>,
    args: UpdateProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_protocol_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_protocol_verify_account_keys(
    accounts: UpdateProtocolAccounts<'_, '_>,
    keys: UpdateProtocolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_protocol_verify_writable_privileges<'me, 'info>(
    accounts: UpdateProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.protocol] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_protocol_verify_signer_privileges<'me, 'info>(
    accounts: UpdateProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_protocol_verify_account_privileges<'me, 'info>(
    accounts: UpdateProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_protocol_verify_writable_privileges(accounts)?;
    update_protocol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_VAULT_ADAPTOR_POLICY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateVaultAdaptorPolicyAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateVaultAdaptorPolicyKeys {
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
}
impl From<UpdateVaultAdaptorPolicyAccounts<'_, '_>> for UpdateVaultAdaptorPolicyKeys {
    fn from(accounts: UpdateVaultAdaptorPolicyAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
        }
    }
}
impl From<UpdateVaultAdaptorPolicyKeys>
for [AccountMeta; UPDATE_VAULT_ADAPTOR_POLICY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateVaultAdaptorPolicyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_VAULT_ADAPTOR_POLICY_IX_ACCOUNTS_LEN]>
for UpdateVaultAdaptorPolicyKeys {
    fn from(pubkeys: [Pubkey; UPDATE_VAULT_ADAPTOR_POLICY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateVaultAdaptorPolicyAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_VAULT_ADAPTOR_POLICY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateVaultAdaptorPolicyAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.protocol.clone(), accounts.vault.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_VAULT_ADAPTOR_POLICY_IX_ACCOUNTS_LEN]>
for UpdateVaultAdaptorPolicyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_VAULT_ADAPTOR_POLICY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
        }
    }
}
pub const UPDATE_VAULT_ADAPTOR_POLICY_IX_DISCM: [u8; 8usize] = [
    20, 101, 232, 123, 189, 18, 133, 230,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateVaultAdaptorPolicyIxArgs {
    pub allow_any_adaptor: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateVaultAdaptorPolicyIxData(pub UpdateVaultAdaptorPolicyIxArgs);
impl From<UpdateVaultAdaptorPolicyIxArgs> for UpdateVaultAdaptorPolicyIxData {
    fn from(args: UpdateVaultAdaptorPolicyIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateVaultAdaptorPolicyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_VAULT_ADAPTOR_POLICY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let allow_any_adaptor: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateVaultAdaptorPolicyIxArgs {
                allow_any_adaptor,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_VAULT_ADAPTOR_POLICY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.allow_any_adaptor, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_vault_adaptor_policy_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateVaultAdaptorPolicyKeys,
    args: UpdateVaultAdaptorPolicyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_VAULT_ADAPTOR_POLICY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateVaultAdaptorPolicyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_vault_adaptor_policy_ix(
    keys: UpdateVaultAdaptorPolicyKeys,
    args: UpdateVaultAdaptorPolicyIxArgs,
) -> std::io::Result<Instruction> {
    update_vault_adaptor_policy_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn update_vault_adaptor_policy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateVaultAdaptorPolicyAccounts<'_, '_>,
    args: UpdateVaultAdaptorPolicyIxArgs,
) -> ProgramResult {
    let keys: UpdateVaultAdaptorPolicyKeys = accounts.into();
    let ix = update_vault_adaptor_policy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_vault_adaptor_policy_invoke(
    accounts: UpdateVaultAdaptorPolicyAccounts<'_, '_>,
    args: UpdateVaultAdaptorPolicyIxArgs,
) -> ProgramResult {
    update_vault_adaptor_policy_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn update_vault_adaptor_policy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateVaultAdaptorPolicyAccounts<'_, '_>,
    args: UpdateVaultAdaptorPolicyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateVaultAdaptorPolicyKeys = accounts.into();
    let ix = update_vault_adaptor_policy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_vault_adaptor_policy_invoke_signed(
    accounts: UpdateVaultAdaptorPolicyAccounts<'_, '_>,
    args: UpdateVaultAdaptorPolicyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_vault_adaptor_policy_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_vault_adaptor_policy_verify_account_keys(
    accounts: UpdateVaultAdaptorPolicyAccounts<'_, '_>,
    keys: UpdateVaultAdaptorPolicyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_vault_adaptor_policy_verify_writable_privileges<'me, 'info>(
    accounts: UpdateVaultAdaptorPolicyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_vault_adaptor_policy_verify_signer_privileges<'me, 'info>(
    accounts: UpdateVaultAdaptorPolicyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_vault_adaptor_policy_verify_account_privileges<'me, 'info>(
    accounts: UpdateVaultAdaptorPolicyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_vault_adaptor_policy_verify_writable_privileges(accounts)?;
    update_vault_adaptor_policy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_VAULT_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateVaultConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateVaultConfigKeys {
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub rent: Pubkey,
}
impl From<UpdateVaultConfigAccounts<'_, '_>> for UpdateVaultConfigKeys {
    fn from(accounts: UpdateVaultConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<UpdateVaultConfigKeys> for [AccountMeta; UPDATE_VAULT_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateVaultConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_VAULT_CONFIG_IX_ACCOUNTS_LEN]> for UpdateVaultConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_VAULT_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            rent: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateVaultConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_VAULT_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateVaultConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_VAULT_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateVaultConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_VAULT_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            rent: &arr[3],
        }
    }
}
pub const UPDATE_VAULT_CONFIG_IX_DISCM: [u8; 8usize] = [
    122, 3, 21, 222, 158, 255, 238, 157,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateVaultConfigIxArgs {
    pub field: VaultConfigField,
    pub data: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateVaultConfigIxData(pub UpdateVaultConfigIxArgs);
impl From<UpdateVaultConfigIxArgs> for UpdateVaultConfigIxData {
    fn from(args: UpdateVaultConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateVaultConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_VAULT_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field: VaultConfigField = crate::borsh_de_or_default(&mut reader)?;
        let data: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateVaultConfigIxArgs {
                field,
                data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_VAULT_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_vault_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateVaultConfigKeys,
    args: UpdateVaultConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_VAULT_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateVaultConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_vault_config_ix(
    keys: UpdateVaultConfigKeys,
    args: UpdateVaultConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_vault_config_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn update_vault_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateVaultConfigAccounts<'_, '_>,
    args: UpdateVaultConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateVaultConfigKeys = accounts.into();
    let ix = update_vault_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_vault_config_invoke(
    accounts: UpdateVaultConfigAccounts<'_, '_>,
    args: UpdateVaultConfigIxArgs,
) -> ProgramResult {
    update_vault_config_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn update_vault_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateVaultConfigAccounts<'_, '_>,
    args: UpdateVaultConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateVaultConfigKeys = accounts.into();
    let ix = update_vault_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_vault_config_invoke_signed(
    accounts: UpdateVaultConfigAccounts<'_, '_>,
    args: UpdateVaultConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_vault_config_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_vault_config_verify_account_keys(
    accounts: UpdateVaultConfigAccounts<'_, '_>,
    keys: UpdateVaultConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_vault_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateVaultConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_vault_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateVaultConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_vault_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateVaultConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_vault_config_verify_writable_privileges(accounts)?;
    update_vault_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_VAULT_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateVaultProtocolFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateVaultProtocolFeeKeys {
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
}
impl From<UpdateVaultProtocolFeeAccounts<'_, '_>> for UpdateVaultProtocolFeeKeys {
    fn from(accounts: UpdateVaultProtocolFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
        }
    }
}
impl From<UpdateVaultProtocolFeeKeys>
for [AccountMeta; UPDATE_VAULT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateVaultProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_VAULT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for UpdateVaultProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_VAULT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            vault_lp_mint: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateVaultProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_VAULT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateVaultProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_lp_mint.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_VAULT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for UpdateVaultProtocolFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_VAULT_PROTOCOL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            vault_lp_mint: &arr[3],
        }
    }
}
pub const UPDATE_VAULT_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    62, 83, 22, 192, 137, 86, 74, 18,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateVaultProtocolFeeIxArgs {
    pub field: ProtocolFeeField,
    pub fee_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateVaultProtocolFeeIxData(pub UpdateVaultProtocolFeeIxArgs);
impl From<UpdateVaultProtocolFeeIxArgs> for UpdateVaultProtocolFeeIxData {
    fn from(args: UpdateVaultProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateVaultProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_VAULT_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let field: ProtocolFeeField = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateVaultProtocolFeeIxArgs {
                field,
                fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_VAULT_PROTOCOL_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_vault_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateVaultProtocolFeeKeys,
    args: UpdateVaultProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_VAULT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateVaultProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_vault_protocol_fee_ix(
    keys: UpdateVaultProtocolFeeKeys,
    args: UpdateVaultProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_vault_protocol_fee_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn update_vault_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateVaultProtocolFeeAccounts<'_, '_>,
    args: UpdateVaultProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateVaultProtocolFeeKeys = accounts.into();
    let ix = update_vault_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_vault_protocol_fee_invoke(
    accounts: UpdateVaultProtocolFeeAccounts<'_, '_>,
    args: UpdateVaultProtocolFeeIxArgs,
) -> ProgramResult {
    update_vault_protocol_fee_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn update_vault_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateVaultProtocolFeeAccounts<'_, '_>,
    args: UpdateVaultProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateVaultProtocolFeeKeys = accounts.into();
    let ix = update_vault_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_vault_protocol_fee_invoke_signed(
    accounts: UpdateVaultProtocolFeeAccounts<'_, '_>,
    args: UpdateVaultProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_vault_protocol_fee_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_vault_protocol_fee_verify_account_keys(
    accounts: UpdateVaultProtocolFeeAccounts<'_, '_>,
    keys: UpdateVaultProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_vault_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateVaultProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.vault_lp_mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_vault_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateVaultProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_vault_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateVaultProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_vault_protocol_fee_verify_writable_privileges(accounts)?;
    update_vault_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawStrategyAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub adaptor_add_receipt: &'me AccountInfo<'info>,
    pub strategy_init_receipt: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub adaptor_program: &'me AccountInfo<'info>,
    pub vault_asset_idle_auth: &'me AccountInfo<'info>,
    pub vault_strategy_auth: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub vault_asset_idle_ata: &'me AccountInfo<'info>,
    pub vault_strategy_asset_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawStrategyKeys {
    pub manager: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub strategy: Pubkey,
    pub adaptor_program: Pubkey,
    pub vault_asset_idle_auth: Pubkey,
    pub vault_strategy_auth: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub vault_asset_idle_ata: Pubkey,
    pub vault_strategy_asset_ata: Pubkey,
    pub asset_token_program: Pubkey,
}
impl From<WithdrawStrategyAccounts<'_, '_>> for WithdrawStrategyKeys {
    fn from(accounts: WithdrawStrategyAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            adaptor_add_receipt: *accounts.adaptor_add_receipt.key,
            strategy_init_receipt: *accounts.strategy_init_receipt.key,
            strategy: *accounts.strategy.key,
            adaptor_program: *accounts.adaptor_program.key,
            vault_asset_idle_auth: *accounts.vault_asset_idle_auth.key,
            vault_strategy_auth: *accounts.vault_strategy_auth.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            vault_asset_idle_ata: *accounts.vault_asset_idle_ata.key,
            vault_strategy_asset_ata: *accounts.vault_strategy_asset_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
        }
    }
}
impl From<WithdrawStrategyKeys> for [AccountMeta; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.adaptor_add_receipt,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_init_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.adaptor_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_strategy_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]> for WithdrawStrategyKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            adaptor_add_receipt: pubkeys[3],
            strategy_init_receipt: pubkeys[4],
            strategy: pubkeys[5],
            adaptor_program: pubkeys[6],
            vault_asset_idle_auth: pubkeys[7],
            vault_strategy_auth: pubkeys[8],
            vault_asset_mint: pubkeys[9],
            vault_lp_mint: pubkeys[10],
            vault_asset_idle_ata: pubkeys[11],
            vault_strategy_asset_ata: pubkeys[12],
            asset_token_program: pubkeys[13],
        }
    }
}
impl<'info> From<WithdrawStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.adaptor_add_receipt.clone(),
            accounts.strategy_init_receipt.clone(),
            accounts.strategy.clone(),
            accounts.adaptor_program.clone(),
            accounts.vault_asset_idle_auth.clone(),
            accounts.vault_strategy_auth.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.vault_asset_idle_ata.clone(),
            accounts.vault_strategy_asset_ata.clone(),
            accounts.asset_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]>
for WithdrawStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            adaptor_add_receipt: &arr[3],
            strategy_init_receipt: &arr[4],
            strategy: &arr[5],
            adaptor_program: &arr[6],
            vault_asset_idle_auth: &arr[7],
            vault_strategy_auth: &arr[8],
            vault_asset_mint: &arr[9],
            vault_lp_mint: &arr[10],
            vault_asset_idle_ata: &arr[11],
            vault_strategy_asset_ata: &arr[12],
            asset_token_program: &arr[13],
        }
    }
}
pub const WITHDRAW_STRATEGY_IX_DISCM: [u8; 8usize] = [
    31, 45, 162, 5, 193, 217, 134, 188,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawStrategyIxArgs {
    pub amount: u64,
    pub instruction_discriminator: Option<Vec<u8>>,
    pub additional_args: Option<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawStrategyIxData(pub WithdrawStrategyIxArgs);
impl From<WithdrawStrategyIxArgs> for WithdrawStrategyIxData {
    fn from(args: WithdrawStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_discriminator: Option<Vec<u8>> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawStrategyIxArgs {
                amount,
                instruction_discriminator,
                additional_args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.instruction_discriminator,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.additional_args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawStrategyKeys,
    args: WithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_strategy_ix(
    keys: WithdrawStrategyKeys,
    args: WithdrawStrategyIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_strategy_ix_with_program_id(VOLTR_PROGRAM_ID, keys, args)
}
pub fn withdraw_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawStrategyAccounts<'_, '_>,
    args: WithdrawStrategyIxArgs,
) -> ProgramResult {
    let keys: WithdrawStrategyKeys = accounts.into();
    let ix = withdraw_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_strategy_invoke(
    accounts: WithdrawStrategyAccounts<'_, '_>,
    args: WithdrawStrategyIxArgs,
) -> ProgramResult {
    withdraw_strategy_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts, args)
}
pub fn withdraw_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawStrategyAccounts<'_, '_>,
    args: WithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawStrategyKeys = accounts.into();
    let ix = withdraw_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_strategy_invoke_signed(
    accounts: WithdrawStrategyAccounts<'_, '_>,
    args: WithdrawStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_strategy_invoke_signed_with_program_id(
        VOLTR_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_strategy_verify_account_keys(
    accounts: WithdrawStrategyAccounts<'_, '_>,
    keys: WithdrawStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.adaptor_add_receipt.key, keys.adaptor_add_receipt),
        (*accounts.strategy_init_receipt.key, keys.strategy_init_receipt),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.adaptor_program.key, keys.adaptor_program),
        (*accounts.vault_asset_idle_auth.key, keys.vault_asset_idle_auth),
        (*accounts.vault_strategy_auth.key, keys.vault_strategy_auth),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.vault_asset_idle_ata.key, keys.vault_asset_idle_ata),
        (*accounts.vault_strategy_asset_ata.key, keys.vault_strategy_asset_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_strategy_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy_init_receipt,
        accounts.vault_asset_idle_auth,
        accounts.vault_strategy_auth,
        accounts.vault_asset_mint,
        accounts.vault_asset_idle_ata,
        accounts.vault_strategy_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_strategy_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_strategy_verify_account_privileges<'me, 'info>(
    accounts: WithdrawStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_strategy_verify_writable_privileges(accounts)?;
    withdraw_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_VAULT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawVaultAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub protocol: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_asset_mint: &'me AccountInfo<'info>,
    pub vault_lp_mint: &'me AccountInfo<'info>,
    pub request_withdraw_lp_ata: &'me AccountInfo<'info>,
    pub vault_asset_idle_ata: &'me AccountInfo<'info>,
    pub vault_asset_idle_auth: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub request_withdraw_vault_receipt: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub lp_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawVaultKeys {
    pub user_transfer_authority: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub request_withdraw_lp_ata: Pubkey,
    pub vault_asset_idle_ata: Pubkey,
    pub vault_asset_idle_auth: Pubkey,
    pub user_asset_ata: Pubkey,
    pub request_withdraw_vault_receipt: Pubkey,
    pub asset_token_program: Pubkey,
    pub lp_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<WithdrawVaultAccounts<'_, '_>> for WithdrawVaultKeys {
    fn from(accounts: WithdrawVaultAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            protocol: *accounts.protocol.key,
            vault: *accounts.vault.key,
            vault_asset_mint: *accounts.vault_asset_mint.key,
            vault_lp_mint: *accounts.vault_lp_mint.key,
            request_withdraw_lp_ata: *accounts.request_withdraw_lp_ata.key,
            vault_asset_idle_ata: *accounts.vault_asset_idle_ata.key,
            vault_asset_idle_auth: *accounts.vault_asset_idle_auth.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            request_withdraw_vault_receipt: *accounts.request_withdraw_vault_receipt.key,
            asset_token_program: *accounts.asset_token_program.key,
            lp_token_program: *accounts.lp_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<WithdrawVaultKeys> for [AccountMeta; WITHDRAW_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_lp_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_idle_auth,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_withdraw_vault_receipt,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_program,
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
impl From<[Pubkey; WITHDRAW_VAULT_IX_ACCOUNTS_LEN]> for WithdrawVaultKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            protocol: pubkeys[1],
            vault: pubkeys[2],
            vault_asset_mint: pubkeys[3],
            vault_lp_mint: pubkeys[4],
            request_withdraw_lp_ata: pubkeys[5],
            vault_asset_idle_ata: pubkeys[6],
            vault_asset_idle_auth: pubkeys[7],
            user_asset_ata: pubkeys[8],
            request_withdraw_vault_receipt: pubkeys[9],
            asset_token_program: pubkeys[10],
            lp_token_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<WithdrawVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.protocol.clone(),
            accounts.vault.clone(),
            accounts.vault_asset_mint.clone(),
            accounts.vault_lp_mint.clone(),
            accounts.request_withdraw_lp_ata.clone(),
            accounts.vault_asset_idle_ata.clone(),
            accounts.vault_asset_idle_auth.clone(),
            accounts.user_asset_ata.clone(),
            accounts.request_withdraw_vault_receipt.clone(),
            accounts.asset_token_program.clone(),
            accounts.lp_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_VAULT_IX_ACCOUNTS_LEN]>
for WithdrawVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            protocol: &arr[1],
            vault: &arr[2],
            vault_asset_mint: &arr[3],
            vault_lp_mint: &arr[4],
            request_withdraw_lp_ata: &arr[5],
            vault_asset_idle_ata: &arr[6],
            vault_asset_idle_auth: &arr[7],
            user_asset_ata: &arr[8],
            request_withdraw_vault_receipt: &arr[9],
            asset_token_program: &arr[10],
            lp_token_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const WITHDRAW_VAULT_IX_DISCM: [u8; 8usize] = [135, 7, 237, 120, 149, 94, 95, 7];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawVaultIxData;
impl WithdrawVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_VAULT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawVaultKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawVaultIxData.try_to_vec()?,
    })
}
pub fn withdraw_vault_ix(keys: WithdrawVaultKeys) -> std::io::Result<Instruction> {
    withdraw_vault_ix_with_program_id(VOLTR_PROGRAM_ID, keys)
}
pub fn withdraw_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawVaultAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawVaultKeys = accounts.into();
    let ix = withdraw_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_vault_invoke(accounts: WithdrawVaultAccounts<'_, '_>) -> ProgramResult {
    withdraw_vault_invoke_with_program_id(VOLTR_PROGRAM_ID, accounts)
}
pub fn withdraw_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawVaultKeys = accounts.into();
    let ix = withdraw_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_vault_invoke_signed(
    accounts: WithdrawVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_vault_invoke_signed_with_program_id(VOLTR_PROGRAM_ID, accounts, seeds)
}
pub fn withdraw_vault_verify_account_keys(
    accounts: WithdrawVaultAccounts<'_, '_>,
    keys: WithdrawVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.protocol.key, keys.protocol),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_asset_mint.key, keys.vault_asset_mint),
        (*accounts.vault_lp_mint.key, keys.vault_lp_mint),
        (*accounts.request_withdraw_lp_ata.key, keys.request_withdraw_lp_ata),
        (*accounts.vault_asset_idle_ata.key, keys.vault_asset_idle_ata),
        (*accounts.vault_asset_idle_auth.key, keys.vault_asset_idle_auth),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (
            *accounts.request_withdraw_vault_receipt.key,
            keys.request_withdraw_vault_receipt,
        ),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.lp_token_program.key, keys.lp_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_vault_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_transfer_authority,
        accounts.vault,
        accounts.vault_lp_mint,
        accounts.request_withdraw_lp_ata,
        accounts.vault_asset_idle_ata,
        accounts.vault_asset_idle_auth,
        accounts.user_asset_ata,
        accounts.request_withdraw_vault_receipt,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_vault_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_vault_verify_account_privileges<'me, 'info>(
    accounts: WithdrawVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_vault_verify_writable_privileges(accounts)?;
    withdraw_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
