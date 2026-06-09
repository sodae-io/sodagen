use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum FarmsProgramIx {
    InitializeGlobalConfig,
    UpdateGlobalConfig(UpdateGlobalConfigIxArgs),
    InitializeFarm,
    InitializeFarmDelegated,
    InitializeReward,
    AddRewards(AddRewardsIxArgs),
    UpdateFarmConfig(UpdateFarmConfigIxArgs),
    InitializeUser,
    TransferOwnership,
    RewardUserOnce(RewardUserOnceIxArgs),
    RefreshFarm,
    Stake(StakeIxArgs),
    SetStakeDelegated(SetStakeDelegatedIxArgs),
    HarvestReward(HarvestRewardIxArgs),
    Unstake(UnstakeIxArgs),
    RefreshUserState,
    WithdrawUnstakedDeposits,
    WithdrawTreasury(WithdrawTreasuryIxArgs),
    DepositToFarmVault(DepositToFarmVaultIxArgs),
    WithdrawFromFarmVault(WithdrawFromFarmVaultIxArgs),
    WithdrawSlashedAmount,
    UpdateFarmAdmin,
    UpdateGlobalConfigAdmin,
    WithdrawReward(WithdrawRewardIxArgs),
    UpdateSecondDelegatedAuthority,
    CloseEmptyUserState,
    IdlMissingTypes(IdlMissingTypesIxArgs),
}
impl FarmsProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM) {
            return Ok(Self::InitializeGlobalConfig);
        }
        if buf.starts_with(&UPDATE_GLOBAL_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_GLOBAL_CONFIG_IX_DISCM.len()..];
            let mode: u8 = crate::borsh_de_or_default(&mut reader)?;
            let value: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateGlobalConfig(UpdateGlobalConfigIxArgs {
                    mode,
                    value,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_FARM_IX_DISCM) {
            return Ok(Self::InitializeFarm);
        }
        if buf.starts_with(&INITIALIZE_FARM_DELEGATED_IX_DISCM) {
            return Ok(Self::InitializeFarmDelegated);
        }
        if buf.starts_with(&INITIALIZE_REWARD_IX_DISCM) {
            return Ok(Self::InitializeReward);
        }
        if buf.starts_with(&ADD_REWARDS_IX_DISCM) {
            let mut reader = &buf[ADD_REWARDS_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddRewards(AddRewardsIxArgs {
                    amount,
                    reward_index,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FARM_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_FARM_CONFIG_IX_DISCM.len()..];
            let mode: u16 = crate::borsh_de_or_default(&mut reader)?;
            let data: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateFarmConfig(UpdateFarmConfigIxArgs {
                    mode,
                    data,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_USER_IX_DISCM) {
            return Ok(Self::InitializeUser);
        }
        if buf.starts_with(&TRANSFER_OWNERSHIP_IX_DISCM) {
            return Ok(Self::TransferOwnership);
        }
        if buf.starts_with(&REWARD_USER_ONCE_IX_DISCM) {
            let mut reader = &buf[REWARD_USER_ONCE_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let expected_rewards_issued_cumulative: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let user_state_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RewardUserOnce(RewardUserOnceIxArgs {
                    reward_index,
                    amount,
                    expected_rewards_issued_cumulative,
                    user_state_id,
                }),
            );
        }
        if buf.starts_with(&REFRESH_FARM_IX_DISCM) {
            return Ok(Self::RefreshFarm);
        }
        if buf.starts_with(&STAKE_IX_DISCM) {
            let mut reader = &buf[STAKE_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Stake(StakeIxArgs { amount }));
        }
        if buf.starts_with(&SET_STAKE_DELEGATED_IX_DISCM) {
            let mut reader = &buf[SET_STAKE_DELEGATED_IX_DISCM.len()..];
            let new_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetStakeDelegated(SetStakeDelegatedIxArgs {
                    new_amount,
                }),
            );
        }
        if buf.starts_with(&HARVEST_REWARD_IX_DISCM) {
            let mut reader = &buf[HARVEST_REWARD_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::HarvestReward(HarvestRewardIxArgs {
                    reward_index,
                }),
            );
        }
        if buf.starts_with(&UNSTAKE_IX_DISCM) {
            let mut reader = &buf[UNSTAKE_IX_DISCM.len()..];
            let stake_shares_scaled: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Unstake(UnstakeIxArgs {
                    stake_shares_scaled,
                }),
            );
        }
        if buf.starts_with(&REFRESH_USER_STATE_IX_DISCM) {
            return Ok(Self::RefreshUserState);
        }
        if buf.starts_with(&WITHDRAW_UNSTAKED_DEPOSITS_IX_DISCM) {
            return Ok(Self::WithdrawUnstakedDeposits);
        }
        if buf.starts_with(&WITHDRAW_TREASURY_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_TREASURY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::WithdrawTreasury(WithdrawTreasuryIxArgs { amount }));
        }
        if buf.starts_with(&DEPOSIT_TO_FARM_VAULT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_TO_FARM_VAULT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DepositToFarmVault(DepositToFarmVaultIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_FROM_FARM_VAULT_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_FARM_VAULT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawFromFarmVault(WithdrawFromFarmVaultIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_SLASHED_AMOUNT_IX_DISCM) {
            return Ok(Self::WithdrawSlashedAmount);
        }
        if buf.starts_with(&UPDATE_FARM_ADMIN_IX_DISCM) {
            return Ok(Self::UpdateFarmAdmin);
        }
        if buf.starts_with(&UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM) {
            return Ok(Self::UpdateGlobalConfigAdmin);
        }
        if buf.starts_with(&WITHDRAW_REWARD_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_REWARD_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawReward(WithdrawRewardIxArgs {
                    amount,
                    reward_index,
                }),
            );
        }
        if buf.starts_with(&UPDATE_SECOND_DELEGATED_AUTHORITY_IX_DISCM) {
            return Ok(Self::UpdateSecondDelegatedAuthority);
        }
        if buf.starts_with(&CLOSE_EMPTY_USER_STATE_IX_DISCM) {
            return Ok(Self::CloseEmptyUserState);
        }
        if buf.starts_with(&IDL_MISSING_TYPES_IX_DISCM) {
            let mut reader = &buf[IDL_MISSING_TYPES_IX_DISCM.len()..];
            let global_config_option_kind: GlobalConfigOption = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let farm_config_option_kind: FarmConfigOption = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let time_unit: TimeUnit = crate::borsh_de_or_default(&mut reader)?;
            let locking_mode: LockingMode = crate::borsh_de_or_default(&mut reader)?;
            let reward_type: RewardType = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IdlMissingTypes(IdlMissingTypesIxArgs {
                    global_config_option_kind,
                    farm_config_option_kind,
                    time_unit,
                    locking_mode,
                    reward_type,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeGlobalConfig => {
                writer.write_all(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM)
            }
            Self::UpdateGlobalConfig(args) => {
                writer.write_all(&UPDATE_GLOBAL_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mode, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::InitializeFarm => writer.write_all(&INITIALIZE_FARM_IX_DISCM),
            Self::InitializeFarmDelegated => {
                writer.write_all(&INITIALIZE_FARM_DELEGATED_IX_DISCM)
            }
            Self::InitializeReward => writer.write_all(&INITIALIZE_REWARD_IX_DISCM),
            Self::AddRewards(args) => {
                writer.write_all(&ADD_REWARDS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                Ok(())
            }
            Self::UpdateFarmConfig(args) => {
                writer.write_all(&UPDATE_FARM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mode, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::InitializeUser => writer.write_all(&INITIALIZE_USER_IX_DISCM),
            Self::TransferOwnership => writer.write_all(&TRANSFER_OWNERSHIP_IX_DISCM),
            Self::RewardUserOnce(args) => {
                writer.write_all(&REWARD_USER_ONCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.expected_rewards_issued_cumulative,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.user_state_id, &mut writer)?;
                Ok(())
            }
            Self::RefreshFarm => writer.write_all(&REFRESH_FARM_IX_DISCM),
            Self::Stake(args) => {
                writer.write_all(&STAKE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::SetStakeDelegated(args) => {
                writer.write_all(&SET_STAKE_DELEGATED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_amount, &mut writer)?;
                Ok(())
            }
            Self::HarvestReward(args) => {
                writer.write_all(&HARVEST_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                Ok(())
            }
            Self::Unstake(args) => {
                writer.write_all(&UNSTAKE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.stake_shares_scaled,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::RefreshUserState => writer.write_all(&REFRESH_USER_STATE_IX_DISCM),
            Self::WithdrawUnstakedDeposits => {
                writer.write_all(&WITHDRAW_UNSTAKED_DEPOSITS_IX_DISCM)
            }
            Self::WithdrawTreasury(args) => {
                writer.write_all(&WITHDRAW_TREASURY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::DepositToFarmVault(args) => {
                writer.write_all(&DEPOSIT_TO_FARM_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFromFarmVault(args) => {
                writer.write_all(&WITHDRAW_FROM_FARM_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawSlashedAmount => {
                writer.write_all(&WITHDRAW_SLASHED_AMOUNT_IX_DISCM)
            }
            Self::UpdateFarmAdmin => writer.write_all(&UPDATE_FARM_ADMIN_IX_DISCM),
            Self::UpdateGlobalConfigAdmin => {
                writer.write_all(&UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM)
            }
            Self::WithdrawReward(args) => {
                writer.write_all(&WITHDRAW_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                Ok(())
            }
            Self::UpdateSecondDelegatedAuthority => {
                writer.write_all(&UPDATE_SECOND_DELEGATED_AUTHORITY_IX_DISCM)
            }
            Self::CloseEmptyUserState => {
                writer.write_all(&CLOSE_EMPTY_USER_STATE_IX_DISCM)
            }
            Self::IdlMissingTypes(args) => {
                writer.write_all(&IDL_MISSING_TYPES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.global_config_option_kind,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.farm_config_option_kind,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.time_unit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.locking_mode, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.reward_type, &mut writer)?;
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
pub const INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeGlobalConfigAccounts<'me, 'info> {
    pub global_admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub treasury_vaults_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeGlobalConfigKeys {
    pub global_admin: Pubkey,
    pub global_config: Pubkey,
    pub treasury_vaults_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeGlobalConfigAccounts<'_, '_>> for InitializeGlobalConfigKeys {
    fn from(accounts: InitializeGlobalConfigAccounts) -> Self {
        Self {
            global_admin: *accounts.global_admin.key,
            global_config: *accounts.global_config.key,
            treasury_vaults_authority: *accounts.treasury_vaults_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeGlobalConfigKeys>
for [AccountMeta; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeGlobalConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_vaults_authority,
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
impl From<[Pubkey; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeGlobalConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_admin: pubkeys[0],
            global_config: pubkeys[1],
            treasury_vaults_authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeGlobalConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeGlobalConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.global_admin.clone(),
            accounts.global_config.clone(),
            accounts.treasury_vaults_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeGlobalConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global_admin: &arr[0],
            global_config: &arr[1],
            treasury_vaults_authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_GLOBAL_CONFIG_IX_DISCM: [u8; 8usize] = [
    113, 216, 122, 131, 225, 209, 22, 55,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeGlobalConfigIxData;
impl InitializeGlobalConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_GLOBAL_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_global_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeGlobalConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeGlobalConfigIxData.try_to_vec()?,
    })
}
pub fn initialize_global_config_ix(
    keys: InitializeGlobalConfigKeys,
) -> std::io::Result<Instruction> {
    initialize_global_config_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn initialize_global_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeGlobalConfigKeys = accounts.into();
    let ix = initialize_global_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_global_config_invoke(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
) -> ProgramResult {
    initialize_global_config_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn initialize_global_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeGlobalConfigKeys = accounts.into();
    let ix = initialize_global_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_global_config_invoke_signed(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_global_config_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_global_config_verify_account_keys(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    keys: InitializeGlobalConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global_admin.key, keys.global_admin),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.treasury_vaults_authority.key, keys.treasury_vaults_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_admin, accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_global_config_verify_writable_privileges(accounts)?;
    initialize_global_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateGlobalConfigAccounts<'me, 'info> {
    pub global_admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigKeys {
    pub global_admin: Pubkey,
    pub global_config: Pubkey,
}
impl From<UpdateGlobalConfigAccounts<'_, '_>> for UpdateGlobalConfigKeys {
    fn from(accounts: UpdateGlobalConfigAccounts) -> Self {
        Self {
            global_admin: *accounts.global_admin.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<UpdateGlobalConfigKeys>
for [AccountMeta; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateGlobalConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]> for UpdateGlobalConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_admin: pubkeys[0],
            global_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateGlobalConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateGlobalConfigAccounts<'_, 'info>) -> Self {
        [accounts.global_admin.clone(), accounts.global_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateGlobalConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global_admin: &arr[0],
            global_config: &arr[1],
        }
    }
}
pub const UPDATE_GLOBAL_CONFIG_IX_DISCM: [u8; 8usize] = [
    164, 84, 130, 189, 111, 58, 250, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateGlobalConfigIxArgs {
    pub mode: u8,
    pub value: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigIxData(pub UpdateGlobalConfigIxArgs);
impl From<UpdateGlobalConfigIxArgs> for UpdateGlobalConfigIxData {
    fn from(args: UpdateGlobalConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateGlobalConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_GLOBAL_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let value: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateGlobalConfigIxArgs {
                mode,
                value,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_GLOBAL_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.value, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_global_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateGlobalConfigKeys,
    args: UpdateGlobalConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateGlobalConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_global_config_ix(
    keys: UpdateGlobalConfigKeys,
    args: UpdateGlobalConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_global_config_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn update_global_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateGlobalConfigKeys = accounts.into();
    let ix = update_global_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_global_config_invoke(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
) -> ProgramResult {
    update_global_config_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn update_global_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateGlobalConfigKeys = accounts.into();
    let ix = update_global_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_global_config_invoke_signed(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_global_config_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_global_config_verify_account_keys(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    keys: UpdateGlobalConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global_admin.key, keys.global_admin),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_global_config_verify_writable_privileges(accounts)?;
    update_global_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_FARM_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeFarmAccounts<'me, 'info> {
    pub farm_admin: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub farm_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeFarmKeys {
    pub farm_admin: Pubkey,
    pub farm_state: Pubkey,
    pub global_config: Pubkey,
    pub farm_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeFarmAccounts<'_, '_>> for InitializeFarmKeys {
    fn from(accounts: InitializeFarmAccounts) -> Self {
        Self {
            farm_admin: *accounts.farm_admin.key,
            farm_state: *accounts.farm_state.key,
            global_config: *accounts.global_config.key,
            farm_vault: *accounts.farm_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            token_mint: *accounts.token_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeFarmKeys> for [AccountMeta; INITIALIZE_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeFarmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.farm_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_FARM_IX_ACCOUNTS_LEN]> for InitializeFarmKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_admin: pubkeys[0],
            farm_state: pubkeys[1],
            global_config: pubkeys[2],
            farm_vault: pubkeys[3],
            farm_vaults_authority: pubkeys[4],
            token_mint: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeFarmAccounts<'_, 'info>) -> Self {
        [
            accounts.farm_admin.clone(),
            accounts.farm_state.clone(),
            accounts.global_config.clone(),
            accounts.farm_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.token_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_FARM_IX_ACCOUNTS_LEN]>
for InitializeFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_admin: &arr[0],
            farm_state: &arr[1],
            global_config: &arr[2],
            farm_vault: &arr[3],
            farm_vaults_authority: &arr[4],
            token_mint: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
        }
    }
}
pub const INITIALIZE_FARM_IX_DISCM: [u8; 8usize] = [
    252, 28, 185, 172, 244, 74, 117, 165,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeFarmIxData;
impl InitializeFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_FARM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeFarmKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_FARM_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeFarmIxData.try_to_vec()?,
    })
}
pub fn initialize_farm_ix(keys: InitializeFarmKeys) -> std::io::Result<Instruction> {
    initialize_farm_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn initialize_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFarmAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeFarmKeys = accounts.into();
    let ix = initialize_farm_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_farm_invoke(
    accounts: InitializeFarmAccounts<'_, '_>,
) -> ProgramResult {
    initialize_farm_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn initialize_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFarmAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeFarmKeys = accounts.into();
    let ix = initialize_farm_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_farm_invoke_signed(
    accounts: InitializeFarmAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_farm_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_farm_verify_account_keys(
    accounts: InitializeFarmAccounts<'_, '_>,
    keys: InitializeFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.farm_admin.key, keys.farm_admin),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.farm_vault.key, keys.farm_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_farm_verify_writable_privileges<'me, 'info>(
    accounts: InitializeFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm_admin,
        accounts.farm_state,
        accounts.farm_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_farm_verify_signer_privileges<'me, 'info>(
    accounts: InitializeFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.farm_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_farm_verify_account_privileges<'me, 'info>(
    accounts: InitializeFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_farm_verify_writable_privileges(accounts)?;
    initialize_farm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_FARM_DELEGATED_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeFarmDelegatedAccounts<'me, 'info> {
    pub farm_admin: &'me AccountInfo<'info>,
    pub farm_delegate: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeFarmDelegatedKeys {
    pub farm_admin: Pubkey,
    pub farm_delegate: Pubkey,
    pub farm_state: Pubkey,
    pub global_config: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeFarmDelegatedAccounts<'_, '_>> for InitializeFarmDelegatedKeys {
    fn from(accounts: InitializeFarmDelegatedAccounts) -> Self {
        Self {
            farm_admin: *accounts.farm_admin.key,
            farm_delegate: *accounts.farm_delegate.key,
            farm_state: *accounts.farm_state.key,
            global_config: *accounts.global_config.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeFarmDelegatedKeys>
for [AccountMeta; INITIALIZE_FARM_DELEGATED_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeFarmDelegatedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.farm_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_delegate,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
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
impl From<[Pubkey; INITIALIZE_FARM_DELEGATED_IX_ACCOUNTS_LEN]>
for InitializeFarmDelegatedKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_FARM_DELEGATED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_admin: pubkeys[0],
            farm_delegate: pubkeys[1],
            farm_state: pubkeys[2],
            global_config: pubkeys[3],
            farm_vaults_authority: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeFarmDelegatedAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_FARM_DELEGATED_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeFarmDelegatedAccounts<'_, 'info>) -> Self {
        [
            accounts.farm_admin.clone(),
            accounts.farm_delegate.clone(),
            accounts.farm_state.clone(),
            accounts.global_config.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_FARM_DELEGATED_IX_ACCOUNTS_LEN]>
for InitializeFarmDelegatedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_FARM_DELEGATED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            farm_admin: &arr[0],
            farm_delegate: &arr[1],
            farm_state: &arr[2],
            global_config: &arr[3],
            farm_vaults_authority: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
        }
    }
}
pub const INITIALIZE_FARM_DELEGATED_IX_DISCM: [u8; 8usize] = [
    250, 84, 101, 25, 51, 77, 204, 91,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeFarmDelegatedIxData;
impl InitializeFarmDelegatedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_FARM_DELEGATED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_FARM_DELEGATED_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_farm_delegated_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeFarmDelegatedKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_FARM_DELEGATED_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeFarmDelegatedIxData.try_to_vec()?,
    })
}
pub fn initialize_farm_delegated_ix(
    keys: InitializeFarmDelegatedKeys,
) -> std::io::Result<Instruction> {
    initialize_farm_delegated_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn initialize_farm_delegated_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFarmDelegatedAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeFarmDelegatedKeys = accounts.into();
    let ix = initialize_farm_delegated_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_farm_delegated_invoke(
    accounts: InitializeFarmDelegatedAccounts<'_, '_>,
) -> ProgramResult {
    initialize_farm_delegated_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn initialize_farm_delegated_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeFarmDelegatedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeFarmDelegatedKeys = accounts.into();
    let ix = initialize_farm_delegated_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_farm_delegated_invoke_signed(
    accounts: InitializeFarmDelegatedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_farm_delegated_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_farm_delegated_verify_account_keys(
    accounts: InitializeFarmDelegatedAccounts<'_, '_>,
    keys: InitializeFarmDelegatedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.farm_admin.key, keys.farm_admin),
        (*accounts.farm_delegate.key, keys.farm_delegate),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_farm_delegated_verify_writable_privileges<'me, 'info>(
    accounts: InitializeFarmDelegatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.farm_admin, accounts.farm_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_farm_delegated_verify_signer_privileges<'me, 'info>(
    accounts: InitializeFarmDelegatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.farm_admin, accounts.farm_delegate] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_farm_delegated_verify_account_privileges<'me, 'info>(
    accounts: InitializeFarmDelegatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_farm_delegated_verify_writable_privileges(accounts)?;
    initialize_farm_delegated_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_REWARD_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeRewardAccounts<'me, 'info> {
    pub farm_admin: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_treasury_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub treasury_vaults_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRewardKeys {
    pub farm_admin: Pubkey,
    pub farm_state: Pubkey,
    pub global_config: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_treasury_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub treasury_vaults_authority: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeRewardAccounts<'_, '_>> for InitializeRewardKeys {
    fn from(accounts: InitializeRewardAccounts) -> Self {
        Self {
            farm_admin: *accounts.farm_admin.key,
            farm_state: *accounts.farm_state.key,
            global_config: *accounts.global_config.key,
            reward_mint: *accounts.reward_mint.key,
            reward_vault: *accounts.reward_vault.key,
            reward_treasury_vault: *accounts.reward_treasury_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            treasury_vaults_authority: *accounts.treasury_vaults_authority.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeRewardKeys> for [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.farm_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_treasury_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.treasury_vaults_authority,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]> for InitializeRewardKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_admin: pubkeys[0],
            farm_state: pubkeys[1],
            global_config: pubkeys[2],
            reward_mint: pubkeys[3],
            reward_vault: pubkeys[4],
            reward_treasury_vault: pubkeys[5],
            farm_vaults_authority: pubkeys[6],
            treasury_vaults_authority: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            rent: pubkeys[10],
        }
    }
}
impl<'info> From<InitializeRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.farm_admin.clone(),
            accounts.farm_state.clone(),
            accounts.global_config.clone(),
            accounts.reward_mint.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_treasury_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.treasury_vaults_authority.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]>
for InitializeRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_admin: &arr[0],
            farm_state: &arr[1],
            global_config: &arr[2],
            reward_mint: &arr[3],
            reward_vault: &arr[4],
            reward_treasury_vault: &arr[5],
            farm_vaults_authority: &arr[6],
            treasury_vaults_authority: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            rent: &arr[10],
        }
    }
}
pub const INITIALIZE_REWARD_IX_DISCM: [u8; 8usize] = [
    95, 135, 192, 196, 242, 129, 230, 68,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRewardIxData;
impl InitializeRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REWARD_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeRewardKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeRewardIxData.try_to_vec()?,
    })
}
pub fn initialize_reward_ix(keys: InitializeRewardKeys) -> std::io::Result<Instruction> {
    initialize_reward_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn initialize_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_reward_invoke(
    accounts: InitializeRewardAccounts<'_, '_>,
) -> ProgramResult {
    initialize_reward_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn initialize_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_reward_invoke_signed(
    accounts: InitializeRewardAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_reward_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_reward_verify_account_keys(
    accounts: InitializeRewardAccounts<'_, '_>,
    keys: InitializeRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.farm_admin.key, keys.farm_admin),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_treasury_vault.key, keys.reward_treasury_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.treasury_vaults_authority.key, keys.treasury_vaults_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_writable_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm_admin,
        accounts.farm_state,
        accounts.reward_vault,
        accounts.reward_treasury_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_signer_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.farm_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_account_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_reward_verify_writable_privileges(accounts)?;
    initialize_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_REWARDS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct AddRewardsAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub payer_reward_token_ata: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddRewardsKeys {
    pub payer: Pubkey,
    pub farm_state: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub payer_reward_token_ata: Pubkey,
    pub scope_prices: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddRewardsAccounts<'_, '_>> for AddRewardsKeys {
    fn from(accounts: AddRewardsAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            farm_state: *accounts.farm_state.key,
            reward_mint: *accounts.reward_mint.key,
            reward_vault: *accounts.reward_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            payer_reward_token_ata: *accounts.payer_reward_token_ata.key,
            scope_prices: *accounts.scope_prices.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddRewardsKeys> for [AccountMeta; ADD_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: AddRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_reward_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
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
impl From<[Pubkey; ADD_REWARDS_IX_ACCOUNTS_LEN]> for AddRewardsKeys {
    fn from(pubkeys: [Pubkey; ADD_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            farm_state: pubkeys[1],
            reward_mint: pubkeys[2],
            reward_vault: pubkeys[3],
            farm_vaults_authority: pubkeys[4],
            payer_reward_token_ata: pubkeys[5],
            scope_prices: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<AddRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.farm_state.clone(),
            accounts.reward_mint.clone(),
            accounts.reward_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.payer_reward_token_ata.clone(),
            accounts.scope_prices.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_REWARDS_IX_ACCOUNTS_LEN]>
for AddRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            farm_state: &arr[1],
            reward_mint: &arr[2],
            reward_vault: &arr[3],
            farm_vaults_authority: &arr[4],
            payer_reward_token_ata: &arr[5],
            scope_prices: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const ADD_REWARDS_IX_DISCM: [u8; 8usize] = [88, 186, 25, 227, 38, 137, 81, 23];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddRewardsIxArgs {
    pub amount: u64,
    pub reward_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddRewardsIxData(pub AddRewardsIxArgs);
impl From<AddRewardsIxArgs> for AddRewardsIxData {
    fn from(args: AddRewardsIxArgs) -> Self {
        Self(args)
    }
}
impl AddRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddRewardsIxArgs {
                amount,
                reward_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_REWARDS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: AddRewardsKeys,
    args: AddRewardsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddRewardsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_rewards_ix(
    keys: AddRewardsKeys,
    args: AddRewardsIxArgs,
) -> std::io::Result<Instruction> {
    add_rewards_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn add_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddRewardsAccounts<'_, '_>,
    args: AddRewardsIxArgs,
) -> ProgramResult {
    let keys: AddRewardsKeys = accounts.into();
    let ix = add_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_rewards_invoke(
    accounts: AddRewardsAccounts<'_, '_>,
    args: AddRewardsIxArgs,
) -> ProgramResult {
    add_rewards_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn add_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddRewardsAccounts<'_, '_>,
    args: AddRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddRewardsKeys = accounts.into();
    let ix = add_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_rewards_invoke_signed(
    accounts: AddRewardsAccounts<'_, '_>,
    args: AddRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_rewards_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_rewards_verify_account_keys(
    accounts: AddRewardsAccounts<'_, '_>,
    keys: AddRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.payer_reward_token_ata.key, keys.payer_reward_token_ata),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_rewards_verify_writable_privileges<'me, 'info>(
    accounts: AddRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.farm_state,
        accounts.reward_vault,
        accounts.payer_reward_token_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_rewards_verify_signer_privileges<'me, 'info>(
    accounts: AddRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_rewards_verify_account_privileges<'me, 'info>(
    accounts: AddRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_rewards_verify_writable_privileges(accounts)?;
    add_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFarmConfigAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFarmConfigKeys {
    pub signer: Pubkey,
    pub farm_state: Pubkey,
    pub scope_prices: Pubkey,
}
impl From<UpdateFarmConfigAccounts<'_, '_>> for UpdateFarmConfigKeys {
    fn from(accounts: UpdateFarmConfigAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            farm_state: *accounts.farm_state.key,
            scope_prices: *accounts.scope_prices.key,
        }
    }
}
impl From<UpdateFarmConfigKeys> for [AccountMeta; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFarmConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN]> for UpdateFarmConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            farm_state: pubkeys[1],
            scope_prices: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateFarmConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFarmConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.farm_state.clone(),
            accounts.scope_prices.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateFarmConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            farm_state: &arr[1],
            scope_prices: &arr[2],
        }
    }
}
pub const UPDATE_FARM_CONFIG_IX_DISCM: [u8; 8usize] = [
    214, 176, 188, 244, 203, 59, 230, 207,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFarmConfigIxArgs {
    pub mode: u16,
    pub data: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFarmConfigIxData(pub UpdateFarmConfigIxArgs);
impl From<UpdateFarmConfigIxArgs> for UpdateFarmConfigIxData {
    fn from(args: UpdateFarmConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFarmConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FARM_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mode: u16 = crate::borsh_de_or_default(&mut reader)?;
        let data: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateFarmConfigIxArgs {
                mode,
                data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FARM_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_farm_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFarmConfigKeys,
    args: UpdateFarmConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFarmConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_farm_config_ix(
    keys: UpdateFarmConfigKeys,
    args: UpdateFarmConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_farm_config_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn update_farm_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    args: UpdateFarmConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateFarmConfigKeys = accounts.into();
    let ix = update_farm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_farm_config_invoke(
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    args: UpdateFarmConfigIxArgs,
) -> ProgramResult {
    update_farm_config_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn update_farm_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    args: UpdateFarmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFarmConfigKeys = accounts.into();
    let ix = update_farm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_farm_config_invoke_signed(
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    args: UpdateFarmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_farm_config_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_farm_config_verify_account_keys(
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    keys: UpdateFarmConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.scope_prices.key, keys.scope_prices),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_farm_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFarmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.farm_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_farm_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFarmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_farm_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateFarmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_farm_config_verify_writable_privileges(accounts)?;
    update_farm_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_USER_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub delegatee: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeUserKeys {
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub delegatee: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeUserAccounts<'_, '_>> for InitializeUserKeys {
    fn from(accounts: InitializeUserAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            delegatee: *accounts.delegatee.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeUserKeys> for [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeUserKeys) -> Self {
        [
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
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegatee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
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
impl From<[Pubkey; INITIALIZE_USER_IX_ACCOUNTS_LEN]> for InitializeUserKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            payer: pubkeys[1],
            owner: pubkeys[2],
            delegatee: pubkeys[3],
            user_state: pubkeys[4],
            farm_state: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeUserAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeUserAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.delegatee.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]>
for InitializeUserAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            payer: &arr[1],
            owner: &arr[2],
            delegatee: &arr[3],
            user_state: &arr[4],
            farm_state: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const INITIALIZE_USER_IX_DISCM: [u8; 8usize] = [111, 17, 185, 250, 60, 122, 38, 254];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeUserIxData;
impl InitializeUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_user_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeUserKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeUserIxData.try_to_vec()?,
    })
}
pub fn initialize_user_ix(keys: InitializeUserKeys) -> std::io::Result<Instruction> {
    initialize_user_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn initialize_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUserAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeUserKeys = accounts.into();
    let ix = initialize_user_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_user_invoke(
    accounts: InitializeUserAccounts<'_, '_>,
) -> ProgramResult {
    initialize_user_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn initialize_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUserAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeUserKeys = accounts.into();
    let ix = initialize_user_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_user_invoke_signed(
    accounts: InitializeUserAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_user_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_user_verify_account_keys(
    accounts: InitializeUserAccounts<'_, '_>,
    keys: InitializeUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.payer.key, keys.payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.delegatee.key, keys.delegatee),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_user_verify_writable_privileges<'me, 'info>(
    accounts: InitializeUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.user_state,
        accounts.farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_user_verify_signer_privileges<'me, 'info>(
    accounts: InitializeUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_user_verify_account_privileges<'me, 'info>(
    accounts: InitializeUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_user_verify_writable_privileges(accounts)?;
    initialize_user_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_OWNERSHIP_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct TransferOwnershipAccounts<'me, 'info> {
    pub old_owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub new_owner: &'me AccountInfo<'info>,
    pub old_user_state: &'me AccountInfo<'info>,
    pub new_user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferOwnershipKeys {
    pub old_owner: Pubkey,
    pub payer: Pubkey,
    pub new_owner: Pubkey,
    pub old_user_state: Pubkey,
    pub new_user_state: Pubkey,
    pub farm_state: Pubkey,
    pub scope_prices: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<TransferOwnershipAccounts<'_, '_>> for TransferOwnershipKeys {
    fn from(accounts: TransferOwnershipAccounts) -> Self {
        Self {
            old_owner: *accounts.old_owner.key,
            payer: *accounts.payer.key,
            new_owner: *accounts.new_owner.key,
            old_user_state: *accounts.old_user_state.key,
            new_user_state: *accounts.new_user_state.key,
            farm_state: *accounts.farm_state.key,
            scope_prices: *accounts.scope_prices.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<TransferOwnershipKeys> for [AccountMeta; TRANSFER_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferOwnershipKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.old_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
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
impl From<[Pubkey; TRANSFER_OWNERSHIP_IX_ACCOUNTS_LEN]> for TransferOwnershipKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_OWNERSHIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            old_owner: pubkeys[0],
            payer: pubkeys[1],
            new_owner: pubkeys[2],
            old_user_state: pubkeys[3],
            new_user_state: pubkeys[4],
            farm_state: pubkeys[5],
            scope_prices: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<TransferOwnershipAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferOwnershipAccounts<'_, 'info>) -> Self {
        [
            accounts.old_owner.clone(),
            accounts.payer.clone(),
            accounts.new_owner.clone(),
            accounts.old_user_state.clone(),
            accounts.new_user_state.clone(),
            accounts.farm_state.clone(),
            accounts.scope_prices.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_OWNERSHIP_IX_ACCOUNTS_LEN]>
for TransferOwnershipAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_OWNERSHIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            old_owner: &arr[0],
            payer: &arr[1],
            new_owner: &arr[2],
            old_user_state: &arr[3],
            new_user_state: &arr[4],
            farm_state: &arr[5],
            scope_prices: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
        }
    }
}
pub const TRANSFER_OWNERSHIP_IX_DISCM: [u8; 8usize] = [65, 177, 215, 73, 53, 45, 99, 47];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferOwnershipIxData;
impl TransferOwnershipIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_OWNERSHIP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_OWNERSHIP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_ownership_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferOwnershipKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_OWNERSHIP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferOwnershipIxData.try_to_vec()?,
    })
}
pub fn transfer_ownership_ix(
    keys: TransferOwnershipKeys,
) -> std::io::Result<Instruction> {
    transfer_ownership_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn transfer_ownership_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferOwnershipAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferOwnershipKeys = accounts.into();
    let ix = transfer_ownership_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_ownership_invoke(
    accounts: TransferOwnershipAccounts<'_, '_>,
) -> ProgramResult {
    transfer_ownership_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn transfer_ownership_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferOwnershipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferOwnershipKeys = accounts.into();
    let ix = transfer_ownership_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_ownership_invoke_signed(
    accounts: TransferOwnershipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_ownership_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, seeds)
}
pub fn transfer_ownership_verify_account_keys(
    accounts: TransferOwnershipAccounts<'_, '_>,
    keys: TransferOwnershipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.old_owner.key, keys.old_owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.new_owner.key, keys.new_owner),
        (*accounts.old_user_state.key, keys.old_user_state),
        (*accounts.new_user_state.key, keys.new_user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_ownership_verify_writable_privileges<'me, 'info>(
    accounts: TransferOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.old_user_state,
        accounts.new_user_state,
        accounts.farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_ownership_verify_signer_privileges<'me, 'info>(
    accounts: TransferOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.old_owner, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_ownership_verify_account_privileges<'me, 'info>(
    accounts: TransferOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_ownership_verify_writable_privileges(accounts)?;
    transfer_ownership_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REWARD_USER_ONCE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RewardUserOnceAccounts<'me, 'info> {
    pub delegate_authority: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RewardUserOnceKeys {
    pub delegate_authority: Pubkey,
    pub farm_state: Pubkey,
    pub user_state: Pubkey,
}
impl From<RewardUserOnceAccounts<'_, '_>> for RewardUserOnceKeys {
    fn from(accounts: RewardUserOnceAccounts) -> Self {
        Self {
            delegate_authority: *accounts.delegate_authority.key,
            farm_state: *accounts.farm_state.key,
            user_state: *accounts.user_state.key,
        }
    }
}
impl From<RewardUserOnceKeys> for [AccountMeta; REWARD_USER_ONCE_IX_ACCOUNTS_LEN] {
    fn from(keys: RewardUserOnceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.delegate_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REWARD_USER_ONCE_IX_ACCOUNTS_LEN]> for RewardUserOnceKeys {
    fn from(pubkeys: [Pubkey; REWARD_USER_ONCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            delegate_authority: pubkeys[0],
            farm_state: pubkeys[1],
            user_state: pubkeys[2],
        }
    }
}
impl<'info> From<RewardUserOnceAccounts<'_, 'info>>
for [AccountInfo<'info>; REWARD_USER_ONCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RewardUserOnceAccounts<'_, 'info>) -> Self {
        [
            accounts.delegate_authority.clone(),
            accounts.farm_state.clone(),
            accounts.user_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REWARD_USER_ONCE_IX_ACCOUNTS_LEN]>
for RewardUserOnceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REWARD_USER_ONCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            delegate_authority: &arr[0],
            farm_state: &arr[1],
            user_state: &arr[2],
        }
    }
}
pub const REWARD_USER_ONCE_IX_DISCM: [u8; 8usize] = [219, 137, 57, 22, 94, 186, 96, 114];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RewardUserOnceIxArgs {
    pub reward_index: u64,
    pub amount: u64,
    pub expected_rewards_issued_cumulative: u64,
    pub user_state_id: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RewardUserOnceIxData(pub RewardUserOnceIxArgs);
impl From<RewardUserOnceIxArgs> for RewardUserOnceIxData {
    fn from(args: RewardUserOnceIxArgs) -> Self {
        Self(args)
    }
}
impl RewardUserOnceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REWARD_USER_ONCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_rewards_issued_cumulative: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let user_state_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RewardUserOnceIxArgs {
                reward_index,
                amount,
                expected_rewards_issued_cumulative,
                user_state_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REWARD_USER_ONCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.expected_rewards_issued_cumulative,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.user_state_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reward_user_once_ix_with_program_id(
    program_id: Pubkey,
    keys: RewardUserOnceKeys,
    args: RewardUserOnceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REWARD_USER_ONCE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RewardUserOnceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reward_user_once_ix(
    keys: RewardUserOnceKeys,
    args: RewardUserOnceIxArgs,
) -> std::io::Result<Instruction> {
    reward_user_once_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn reward_user_once_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RewardUserOnceAccounts<'_, '_>,
    args: RewardUserOnceIxArgs,
) -> ProgramResult {
    let keys: RewardUserOnceKeys = accounts.into();
    let ix = reward_user_once_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn reward_user_once_invoke(
    accounts: RewardUserOnceAccounts<'_, '_>,
    args: RewardUserOnceIxArgs,
) -> ProgramResult {
    reward_user_once_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn reward_user_once_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RewardUserOnceAccounts<'_, '_>,
    args: RewardUserOnceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RewardUserOnceKeys = accounts.into();
    let ix = reward_user_once_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn reward_user_once_invoke_signed(
    accounts: RewardUserOnceAccounts<'_, '_>,
    args: RewardUserOnceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    reward_user_once_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn reward_user_once_verify_account_keys(
    accounts: RewardUserOnceAccounts<'_, '_>,
    keys: RewardUserOnceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.delegate_authority.key, keys.delegate_authority),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.user_state.key, keys.user_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn reward_user_once_verify_writable_privileges<'me, 'info>(
    accounts: RewardUserOnceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.delegate_authority,
        accounts.farm_state,
        accounts.user_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn reward_user_once_verify_signer_privileges<'me, 'info>(
    accounts: RewardUserOnceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn reward_user_once_verify_account_privileges<'me, 'info>(
    accounts: RewardUserOnceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    reward_user_once_verify_writable_privileges(accounts)?;
    reward_user_once_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_FARM_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RefreshFarmAccounts<'me, 'info> {
    pub farm_state: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshFarmKeys {
    pub farm_state: Pubkey,
    pub scope_prices: Pubkey,
}
impl From<RefreshFarmAccounts<'_, '_>> for RefreshFarmKeys {
    fn from(accounts: RefreshFarmAccounts) -> Self {
        Self {
            farm_state: *accounts.farm_state.key,
            scope_prices: *accounts.scope_prices.key,
        }
    }
}
impl From<RefreshFarmKeys> for [AccountMeta; REFRESH_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshFarmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REFRESH_FARM_IX_ACCOUNTS_LEN]> for RefreshFarmKeys {
    fn from(pubkeys: [Pubkey; REFRESH_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_state: pubkeys[0],
            scope_prices: pubkeys[1],
        }
    }
}
impl<'info> From<RefreshFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshFarmAccounts<'_, 'info>) -> Self {
        [accounts.farm_state.clone(), accounts.scope_prices.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFRESH_FARM_IX_ACCOUNTS_LEN]>
for RefreshFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REFRESH_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_state: &arr[0],
            scope_prices: &arr[1],
        }
    }
}
pub const REFRESH_FARM_IX_DISCM: [u8; 8usize] = [214, 131, 138, 183, 144, 194, 172, 42];
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshFarmIxData;
impl RefreshFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_FARM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshFarmKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_FARM_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefreshFarmIxData.try_to_vec()?,
    })
}
pub fn refresh_farm_ix(keys: RefreshFarmKeys) -> std::io::Result<Instruction> {
    refresh_farm_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn refresh_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshFarmAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshFarmKeys = accounts.into();
    let ix = refresh_farm_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_farm_invoke(accounts: RefreshFarmAccounts<'_, '_>) -> ProgramResult {
    refresh_farm_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn refresh_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshFarmAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshFarmKeys = accounts.into();
    let ix = refresh_farm_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_farm_invoke_signed(
    accounts: RefreshFarmAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_farm_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, seeds)
}
pub fn refresh_farm_verify_account_keys(
    accounts: RefreshFarmAccounts<'_, '_>,
    keys: RefreshFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.scope_prices.key, keys.scope_prices),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_farm_verify_writable_privileges<'me, 'info>(
    accounts: RefreshFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.farm_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_farm_verify_account_privileges<'me, 'info>(
    accounts: RefreshFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_farm_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const STAKE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct StakeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub farm_vault: &'me AccountInfo<'info>,
    pub user_ata: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StakeKeys {
    pub owner: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub farm_vault: Pubkey,
    pub user_ata: Pubkey,
    pub token_mint: Pubkey,
    pub scope_prices: Pubkey,
    pub token_program: Pubkey,
}
impl From<StakeAccounts<'_, '_>> for StakeKeys {
    fn from(accounts: StakeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            farm_vault: *accounts.farm_vault.key,
            user_ata: *accounts.user_ata.key,
            token_mint: *accounts.token_mint.key,
            scope_prices: *accounts.scope_prices.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<StakeKeys> for [AccountMeta; STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: StakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
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
impl From<[Pubkey; STAKE_IX_ACCOUNTS_LEN]> for StakeKeys {
    fn from(pubkeys: [Pubkey; STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            user_state: pubkeys[1],
            farm_state: pubkeys[2],
            farm_vault: pubkeys[3],
            user_ata: pubkeys[4],
            token_mint: pubkeys[5],
            scope_prices: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<StakeAccounts<'_, 'info>>
for [AccountInfo<'info>; STAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: StakeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.farm_vault.clone(),
            accounts.user_ata.clone(),
            accounts.token_mint.clone(),
            accounts.scope_prices.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STAKE_IX_ACCOUNTS_LEN]>
for StakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            user_state: &arr[1],
            farm_state: &arr[2],
            farm_vault: &arr[3],
            user_ata: &arr[4],
            token_mint: &arr[5],
            scope_prices: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const STAKE_IX_DISCM: [u8; 8usize] = [206, 176, 202, 18, 200, 209, 179, 108];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StakeIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeIxData(pub StakeIxArgs);
impl From<StakeIxArgs> for StakeIxData {
    fn from(args: StakeIxArgs) -> Self {
        Self(args)
    }
}
impl StakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(StakeIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stake_ix_with_program_id(
    program_id: Pubkey,
    keys: StakeKeys,
    args: StakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: StakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_ix(keys: StakeKeys, args: StakeIxArgs) -> std::io::Result<Instruction> {
    stake_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StakeAccounts<'_, '_>,
    args: StakeIxArgs,
) -> ProgramResult {
    let keys: StakeKeys = accounts.into();
    let ix = stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stake_invoke(
    accounts: StakeAccounts<'_, '_>,
    args: StakeIxArgs,
) -> ProgramResult {
    stake_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StakeAccounts<'_, '_>,
    args: StakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StakeKeys = accounts.into();
    let ix = stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stake_invoke_signed(
    accounts: StakeAccounts<'_, '_>,
    args: StakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stake_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, args, seeds)
}
pub fn stake_verify_account_keys(
    accounts: StakeAccounts<'_, '_>,
    keys: StakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.farm_vault.key, keys.farm_vault),
        (*accounts.user_ata.key, keys.user_ata),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stake_verify_writable_privileges<'me, 'info>(
    accounts: StakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_state,
        accounts.farm_state,
        accounts.farm_vault,
        accounts.user_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stake_verify_signer_privileges<'me, 'info>(
    accounts: StakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stake_verify_account_privileges<'me, 'info>(
    accounts: StakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stake_verify_writable_privileges(accounts)?;
    stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_STAKE_DELEGATED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetStakeDelegatedAccounts<'me, 'info> {
    pub delegate_authority: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetStakeDelegatedKeys {
    pub delegate_authority: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
}
impl From<SetStakeDelegatedAccounts<'_, '_>> for SetStakeDelegatedKeys {
    fn from(accounts: SetStakeDelegatedAccounts) -> Self {
        Self {
            delegate_authority: *accounts.delegate_authority.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
        }
    }
}
impl From<SetStakeDelegatedKeys> for [AccountMeta; SET_STAKE_DELEGATED_IX_ACCOUNTS_LEN] {
    fn from(keys: SetStakeDelegatedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.delegate_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_STAKE_DELEGATED_IX_ACCOUNTS_LEN]> for SetStakeDelegatedKeys {
    fn from(pubkeys: [Pubkey; SET_STAKE_DELEGATED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            delegate_authority: pubkeys[0],
            user_state: pubkeys[1],
            farm_state: pubkeys[2],
        }
    }
}
impl<'info> From<SetStakeDelegatedAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_STAKE_DELEGATED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetStakeDelegatedAccounts<'_, 'info>) -> Self {
        [
            accounts.delegate_authority.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_STAKE_DELEGATED_IX_ACCOUNTS_LEN]>
for SetStakeDelegatedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_STAKE_DELEGATED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            delegate_authority: &arr[0],
            user_state: &arr[1],
            farm_state: &arr[2],
        }
    }
}
pub const SET_STAKE_DELEGATED_IX_DISCM: [u8; 8usize] = [
    73, 171, 184, 75, 30, 56, 198, 223,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetStakeDelegatedIxArgs {
    pub new_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetStakeDelegatedIxData(pub SetStakeDelegatedIxArgs);
impl From<SetStakeDelegatedIxArgs> for SetStakeDelegatedIxData {
    fn from(args: SetStakeDelegatedIxArgs) -> Self {
        Self(args)
    }
}
impl SetStakeDelegatedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_STAKE_DELEGATED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetStakeDelegatedIxArgs {
                new_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_STAKE_DELEGATED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_stake_delegated_ix_with_program_id(
    program_id: Pubkey,
    keys: SetStakeDelegatedKeys,
    args: SetStakeDelegatedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_STAKE_DELEGATED_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetStakeDelegatedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_stake_delegated_ix(
    keys: SetStakeDelegatedKeys,
    args: SetStakeDelegatedIxArgs,
) -> std::io::Result<Instruction> {
    set_stake_delegated_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn set_stake_delegated_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetStakeDelegatedAccounts<'_, '_>,
    args: SetStakeDelegatedIxArgs,
) -> ProgramResult {
    let keys: SetStakeDelegatedKeys = accounts.into();
    let ix = set_stake_delegated_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_stake_delegated_invoke(
    accounts: SetStakeDelegatedAccounts<'_, '_>,
    args: SetStakeDelegatedIxArgs,
) -> ProgramResult {
    set_stake_delegated_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn set_stake_delegated_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetStakeDelegatedAccounts<'_, '_>,
    args: SetStakeDelegatedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetStakeDelegatedKeys = accounts.into();
    let ix = set_stake_delegated_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_stake_delegated_invoke_signed(
    accounts: SetStakeDelegatedAccounts<'_, '_>,
    args: SetStakeDelegatedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_stake_delegated_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_stake_delegated_verify_account_keys(
    accounts: SetStakeDelegatedAccounts<'_, '_>,
    keys: SetStakeDelegatedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.delegate_authority.key, keys.delegate_authority),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_stake_delegated_verify_writable_privileges<'me, 'info>(
    accounts: SetStakeDelegatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_state, accounts.farm_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_stake_delegated_verify_signer_privileges<'me, 'info>(
    accounts: SetStakeDelegatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_stake_delegated_verify_account_privileges<'me, 'info>(
    accounts: SetStakeDelegatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_stake_delegated_verify_writable_privileges(accounts)?;
    set_stake_delegated_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const HARVEST_REWARD_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct HarvestRewardAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub user_reward_token_account: &'me AccountInfo<'info>,
    pub rewards_vault: &'me AccountInfo<'info>,
    pub rewards_treasury_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HarvestRewardKeys {
    pub payer: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub global_config: Pubkey,
    pub reward_mint: Pubkey,
    pub user_reward_token_account: Pubkey,
    pub rewards_vault: Pubkey,
    pub rewards_treasury_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub scope_prices: Pubkey,
    pub token_program: Pubkey,
}
impl From<HarvestRewardAccounts<'_, '_>> for HarvestRewardKeys {
    fn from(accounts: HarvestRewardAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            global_config: *accounts.global_config.key,
            reward_mint: *accounts.reward_mint.key,
            user_reward_token_account: *accounts.user_reward_token_account.key,
            rewards_vault: *accounts.rewards_vault.key,
            rewards_treasury_vault: *accounts.rewards_treasury_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            scope_prices: *accounts.scope_prices.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<HarvestRewardKeys> for [AccountMeta; HARVEST_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: HarvestRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_reward_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rewards_treasury_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
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
impl From<[Pubkey; HARVEST_REWARD_IX_ACCOUNTS_LEN]> for HarvestRewardKeys {
    fn from(pubkeys: [Pubkey; HARVEST_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            user_state: pubkeys[1],
            farm_state: pubkeys[2],
            global_config: pubkeys[3],
            reward_mint: pubkeys[4],
            user_reward_token_account: pubkeys[5],
            rewards_vault: pubkeys[6],
            rewards_treasury_vault: pubkeys[7],
            farm_vaults_authority: pubkeys[8],
            scope_prices: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<HarvestRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; HARVEST_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: HarvestRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.global_config.clone(),
            accounts.reward_mint.clone(),
            accounts.user_reward_token_account.clone(),
            accounts.rewards_vault.clone(),
            accounts.rewards_treasury_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.scope_prices.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; HARVEST_REWARD_IX_ACCOUNTS_LEN]>
for HarvestRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; HARVEST_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            user_state: &arr[1],
            farm_state: &arr[2],
            global_config: &arr[3],
            reward_mint: &arr[4],
            user_reward_token_account: &arr[5],
            rewards_vault: &arr[6],
            rewards_treasury_vault: &arr[7],
            farm_vaults_authority: &arr[8],
            scope_prices: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const HARVEST_REWARD_IX_DISCM: [u8; 8usize] = [68, 200, 228, 233, 184, 32, 226, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HarvestRewardIxArgs {
    pub reward_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestRewardIxData(pub HarvestRewardIxArgs);
impl From<HarvestRewardIxArgs> for HarvestRewardIxData {
    fn from(args: HarvestRewardIxArgs) -> Self {
        Self(args)
    }
}
impl HarvestRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HARVEST_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(HarvestRewardIxArgs {
                reward_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HARVEST_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn harvest_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: HarvestRewardKeys,
    args: HarvestRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; HARVEST_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: HarvestRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn harvest_reward_ix(
    keys: HarvestRewardKeys,
    args: HarvestRewardIxArgs,
) -> std::io::Result<Instruction> {
    harvest_reward_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn harvest_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: HarvestRewardAccounts<'_, '_>,
    args: HarvestRewardIxArgs,
) -> ProgramResult {
    let keys: HarvestRewardKeys = accounts.into();
    let ix = harvest_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn harvest_reward_invoke(
    accounts: HarvestRewardAccounts<'_, '_>,
    args: HarvestRewardIxArgs,
) -> ProgramResult {
    harvest_reward_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn harvest_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: HarvestRewardAccounts<'_, '_>,
    args: HarvestRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: HarvestRewardKeys = accounts.into();
    let ix = harvest_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn harvest_reward_invoke_signed(
    accounts: HarvestRewardAccounts<'_, '_>,
    args: HarvestRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    harvest_reward_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, args, seeds)
}
pub fn harvest_reward_verify_account_keys(
    accounts: HarvestRewardAccounts<'_, '_>,
    keys: HarvestRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.user_reward_token_account.key, keys.user_reward_token_account),
        (*accounts.rewards_vault.key, keys.rewards_vault),
        (*accounts.rewards_treasury_vault.key, keys.rewards_treasury_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn harvest_reward_verify_writable_privileges<'me, 'info>(
    accounts: HarvestRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.user_state,
        accounts.farm_state,
        accounts.user_reward_token_account,
        accounts.rewards_vault,
        accounts.rewards_treasury_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn harvest_reward_verify_signer_privileges<'me, 'info>(
    accounts: HarvestRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn harvest_reward_verify_account_privileges<'me, 'info>(
    accounts: HarvestRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    harvest_reward_verify_writable_privileges(accounts)?;
    harvest_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNSTAKE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UnstakeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnstakeKeys {
    pub owner: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub scope_prices: Pubkey,
}
impl From<UnstakeAccounts<'_, '_>> for UnstakeKeys {
    fn from(accounts: UnstakeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            scope_prices: *accounts.scope_prices.key,
        }
    }
}
impl From<UnstakeKeys> for [AccountMeta; UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: UnstakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNSTAKE_IX_ACCOUNTS_LEN]> for UnstakeKeys {
    fn from(pubkeys: [Pubkey; UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            user_state: pubkeys[1],
            farm_state: pubkeys[2],
            scope_prices: pubkeys[3],
        }
    }
}
impl<'info> From<UnstakeAccounts<'_, 'info>>
for [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.scope_prices.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN]>
for UnstakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            user_state: &arr[1],
            farm_state: &arr[2],
            scope_prices: &arr[3],
        }
    }
}
pub const UNSTAKE_IX_DISCM: [u8; 8usize] = [90, 95, 107, 42, 205, 124, 50, 225];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnstakeIxArgs {
    pub stake_shares_scaled: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnstakeIxData(pub UnstakeIxArgs);
impl From<UnstakeIxArgs> for UnstakeIxData {
    fn from(args: UnstakeIxArgs) -> Self {
        Self(args)
    }
}
impl UnstakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNSTAKE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let stake_shares_scaled: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UnstakeIxArgs {
                stake_shares_scaled,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNSTAKE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.stake_shares_scaled, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unstake_ix_with_program_id(
    program_id: Pubkey,
    keys: UnstakeKeys,
    args: UnstakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNSTAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UnstakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unstake_ix(
    keys: UnstakeKeys,
    args: UnstakeIxArgs,
) -> std::io::Result<Instruction> {
    unstake_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn unstake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnstakeAccounts<'_, '_>,
    args: UnstakeIxArgs,
) -> ProgramResult {
    let keys: UnstakeKeys = accounts.into();
    let ix = unstake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn unstake_invoke(
    accounts: UnstakeAccounts<'_, '_>,
    args: UnstakeIxArgs,
) -> ProgramResult {
    unstake_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn unstake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnstakeAccounts<'_, '_>,
    args: UnstakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnstakeKeys = accounts.into();
    let ix = unstake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unstake_invoke_signed(
    accounts: UnstakeAccounts<'_, '_>,
    args: UnstakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unstake_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, args, seeds)
}
pub fn unstake_verify_account_keys(
    accounts: UnstakeAccounts<'_, '_>,
    keys: UnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.scope_prices.key, keys.scope_prices),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unstake_verify_writable_privileges<'me, 'info>(
    accounts: UnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.user_state,
        accounts.farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unstake_verify_signer_privileges<'me, 'info>(
    accounts: UnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unstake_verify_account_privileges<'me, 'info>(
    accounts: UnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unstake_verify_writable_privileges(accounts)?;
    unstake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_USER_STATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RefreshUserStateAccounts<'me, 'info> {
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshUserStateKeys {
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub scope_prices: Pubkey,
}
impl From<RefreshUserStateAccounts<'_, '_>> for RefreshUserStateKeys {
    fn from(accounts: RefreshUserStateAccounts) -> Self {
        Self {
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            scope_prices: *accounts.scope_prices.key,
        }
    }
}
impl From<RefreshUserStateKeys> for [AccountMeta; REFRESH_USER_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshUserStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REFRESH_USER_STATE_IX_ACCOUNTS_LEN]> for RefreshUserStateKeys {
    fn from(pubkeys: [Pubkey; REFRESH_USER_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_state: pubkeys[0],
            farm_state: pubkeys[1],
            scope_prices: pubkeys[2],
        }
    }
}
impl<'info> From<RefreshUserStateAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_USER_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshUserStateAccounts<'_, 'info>) -> Self {
        [
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.scope_prices.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFRESH_USER_STATE_IX_ACCOUNTS_LEN]>
for RefreshUserStateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REFRESH_USER_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_state: &arr[0],
            farm_state: &arr[1],
            scope_prices: &arr[2],
        }
    }
}
pub const REFRESH_USER_STATE_IX_DISCM: [u8; 8usize] = [
    1, 135, 12, 62, 243, 140, 77, 108,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshUserStateIxData;
impl RefreshUserStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_USER_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_USER_STATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_user_state_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshUserStateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_USER_STATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefreshUserStateIxData.try_to_vec()?,
    })
}
pub fn refresh_user_state_ix(
    keys: RefreshUserStateKeys,
) -> std::io::Result<Instruction> {
    refresh_user_state_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn refresh_user_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshUserStateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshUserStateKeys = accounts.into();
    let ix = refresh_user_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_user_state_invoke(
    accounts: RefreshUserStateAccounts<'_, '_>,
) -> ProgramResult {
    refresh_user_state_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn refresh_user_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshUserStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshUserStateKeys = accounts.into();
    let ix = refresh_user_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_user_state_invoke_signed(
    accounts: RefreshUserStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_user_state_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, seeds)
}
pub fn refresh_user_state_verify_account_keys(
    accounts: RefreshUserStateAccounts<'_, '_>,
    keys: RefreshUserStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.scope_prices.key, keys.scope_prices),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_user_state_verify_writable_privileges<'me, 'info>(
    accounts: RefreshUserStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_state, accounts.farm_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_user_state_verify_account_privileges<'me, 'info>(
    accounts: RefreshUserStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_user_state_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_UNSTAKED_DEPOSITS_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawUnstakedDepositsAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub user_ata: &'me AccountInfo<'info>,
    pub farm_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawUnstakedDepositsKeys {
    pub owner: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub user_ata: Pubkey,
    pub farm_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawUnstakedDepositsAccounts<'_, '_>> for WithdrawUnstakedDepositsKeys {
    fn from(accounts: WithdrawUnstakedDepositsAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            user_ata: *accounts.user_ata.key,
            farm_vault: *accounts.farm_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawUnstakedDepositsKeys>
for [AccountMeta; WITHDRAW_UNSTAKED_DEPOSITS_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawUnstakedDepositsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
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
impl From<[Pubkey; WITHDRAW_UNSTAKED_DEPOSITS_IX_ACCOUNTS_LEN]>
for WithdrawUnstakedDepositsKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_UNSTAKED_DEPOSITS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            user_state: pubkeys[1],
            farm_state: pubkeys[2],
            user_ata: pubkeys[3],
            farm_vault: pubkeys[4],
            farm_vaults_authority: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawUnstakedDepositsAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_UNSTAKED_DEPOSITS_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawUnstakedDepositsAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.user_ata.clone(),
            accounts.farm_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_UNSTAKED_DEPOSITS_IX_ACCOUNTS_LEN]>
for WithdrawUnstakedDepositsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_UNSTAKED_DEPOSITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            user_state: &arr[1],
            farm_state: &arr[2],
            user_ata: &arr[3],
            farm_vault: &arr[4],
            farm_vaults_authority: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_UNSTAKED_DEPOSITS_IX_DISCM: [u8; 8usize] = [
    36, 102, 187, 49, 220, 36, 132, 67,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawUnstakedDepositsIxData;
impl WithdrawUnstakedDepositsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_UNSTAKED_DEPOSITS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_UNSTAKED_DEPOSITS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_unstaked_deposits_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawUnstakedDepositsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_UNSTAKED_DEPOSITS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawUnstakedDepositsIxData.try_to_vec()?,
    })
}
pub fn withdraw_unstaked_deposits_ix(
    keys: WithdrawUnstakedDepositsKeys,
) -> std::io::Result<Instruction> {
    withdraw_unstaked_deposits_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn withdraw_unstaked_deposits_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawUnstakedDepositsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawUnstakedDepositsKeys = accounts.into();
    let ix = withdraw_unstaked_deposits_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_unstaked_deposits_invoke(
    accounts: WithdrawUnstakedDepositsAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_unstaked_deposits_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn withdraw_unstaked_deposits_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawUnstakedDepositsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawUnstakedDepositsKeys = accounts.into();
    let ix = withdraw_unstaked_deposits_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_unstaked_deposits_invoke_signed(
    accounts: WithdrawUnstakedDepositsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_unstaked_deposits_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_unstaked_deposits_verify_account_keys(
    accounts: WithdrawUnstakedDepositsAccounts<'_, '_>,
    keys: WithdrawUnstakedDepositsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.user_ata.key, keys.user_ata),
        (*accounts.farm_vault.key, keys.farm_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_unstaked_deposits_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawUnstakedDepositsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.user_state,
        accounts.farm_state,
        accounts.user_ata,
        accounts.farm_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_unstaked_deposits_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawUnstakedDepositsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_unstaked_deposits_verify_account_privileges<'me, 'info>(
    accounts: WithdrawUnstakedDepositsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_unstaked_deposits_verify_writable_privileges(accounts)?;
    withdraw_unstaked_deposits_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_TREASURY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawTreasuryAccounts<'me, 'info> {
    pub global_admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub reward_treasury_vault: &'me AccountInfo<'info>,
    pub treasury_vault_authority: &'me AccountInfo<'info>,
    pub withdraw_destination_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawTreasuryKeys {
    pub global_admin: Pubkey,
    pub global_config: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_treasury_vault: Pubkey,
    pub treasury_vault_authority: Pubkey,
    pub withdraw_destination_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawTreasuryAccounts<'_, '_>> for WithdrawTreasuryKeys {
    fn from(accounts: WithdrawTreasuryAccounts) -> Self {
        Self {
            global_admin: *accounts.global_admin.key,
            global_config: *accounts.global_config.key,
            reward_mint: *accounts.reward_mint.key,
            reward_treasury_vault: *accounts.reward_treasury_vault.key,
            treasury_vault_authority: *accounts.treasury_vault_authority.key,
            withdraw_destination_token_account: *accounts
                .withdraw_destination_token_account
                .key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawTreasuryKeys> for [AccountMeta; WITHDRAW_TREASURY_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawTreasuryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_treasury_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_destination_token_account,
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
impl From<[Pubkey; WITHDRAW_TREASURY_IX_ACCOUNTS_LEN]> for WithdrawTreasuryKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_TREASURY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_admin: pubkeys[0],
            global_config: pubkeys[1],
            reward_mint: pubkeys[2],
            reward_treasury_vault: pubkeys[3],
            treasury_vault_authority: pubkeys[4],
            withdraw_destination_token_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawTreasuryAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_TREASURY_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawTreasuryAccounts<'_, 'info>) -> Self {
        [
            accounts.global_admin.clone(),
            accounts.global_config.clone(),
            accounts.reward_mint.clone(),
            accounts.reward_treasury_vault.clone(),
            accounts.treasury_vault_authority.clone(),
            accounts.withdraw_destination_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_TREASURY_IX_ACCOUNTS_LEN]>
for WithdrawTreasuryAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_TREASURY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_admin: &arr[0],
            global_config: &arr[1],
            reward_mint: &arr[2],
            reward_treasury_vault: &arr[3],
            treasury_vault_authority: &arr[4],
            withdraw_destination_token_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_TREASURY_IX_DISCM: [u8; 8usize] = [
    40, 63, 122, 158, 144, 216, 83, 96,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawTreasuryIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawTreasuryIxData(pub WithdrawTreasuryIxArgs);
impl From<WithdrawTreasuryIxArgs> for WithdrawTreasuryIxData {
    fn from(args: WithdrawTreasuryIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawTreasuryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_TREASURY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawTreasuryIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_TREASURY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_treasury_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawTreasuryKeys,
    args: WithdrawTreasuryIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_TREASURY_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawTreasuryIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_treasury_ix(
    keys: WithdrawTreasuryKeys,
    args: WithdrawTreasuryIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_treasury_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn withdraw_treasury_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawTreasuryAccounts<'_, '_>,
    args: WithdrawTreasuryIxArgs,
) -> ProgramResult {
    let keys: WithdrawTreasuryKeys = accounts.into();
    let ix = withdraw_treasury_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_treasury_invoke(
    accounts: WithdrawTreasuryAccounts<'_, '_>,
    args: WithdrawTreasuryIxArgs,
) -> ProgramResult {
    withdraw_treasury_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn withdraw_treasury_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawTreasuryAccounts<'_, '_>,
    args: WithdrawTreasuryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawTreasuryKeys = accounts.into();
    let ix = withdraw_treasury_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_treasury_invoke_signed(
    accounts: WithdrawTreasuryAccounts<'_, '_>,
    args: WithdrawTreasuryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_treasury_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_treasury_verify_account_keys(
    accounts: WithdrawTreasuryAccounts<'_, '_>,
    keys: WithdrawTreasuryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global_admin.key, keys.global_admin),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.reward_treasury_vault.key, keys.reward_treasury_vault),
        (*accounts.treasury_vault_authority.key, keys.treasury_vault_authority),
        (
            *accounts.withdraw_destination_token_account.key,
            keys.withdraw_destination_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_treasury_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.global_admin,
        accounts.reward_treasury_vault,
        accounts.withdraw_destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_treasury_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_treasury_verify_account_privileges<'me, 'info>(
    accounts: WithdrawTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_treasury_verify_writable_privileges(accounts)?;
    withdraw_treasury_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_TO_FARM_VAULT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct DepositToFarmVaultAccounts<'me, 'info> {
    pub depositor: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub farm_vault: &'me AccountInfo<'info>,
    pub depositor_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositToFarmVaultKeys {
    pub depositor: Pubkey,
    pub farm_state: Pubkey,
    pub farm_vault: Pubkey,
    pub depositor_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositToFarmVaultAccounts<'_, '_>> for DepositToFarmVaultKeys {
    fn from(accounts: DepositToFarmVaultAccounts) -> Self {
        Self {
            depositor: *accounts.depositor.key,
            farm_state: *accounts.farm_state.key,
            farm_vault: *accounts.farm_vault.key,
            depositor_ata: *accounts.depositor_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositToFarmVaultKeys>
for [AccountMeta; DEPOSIT_TO_FARM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositToFarmVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_ata,
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
impl From<[Pubkey; DEPOSIT_TO_FARM_VAULT_IX_ACCOUNTS_LEN]> for DepositToFarmVaultKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_TO_FARM_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            depositor: pubkeys[0],
            farm_state: pubkeys[1],
            farm_vault: pubkeys[2],
            depositor_ata: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<DepositToFarmVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_TO_FARM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositToFarmVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.depositor.clone(),
            accounts.farm_state.clone(),
            accounts.farm_vault.clone(),
            accounts.depositor_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_TO_FARM_VAULT_IX_ACCOUNTS_LEN]>
for DepositToFarmVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_TO_FARM_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            depositor: &arr[0],
            farm_state: &arr[1],
            farm_vault: &arr[2],
            depositor_ata: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const DEPOSIT_TO_FARM_VAULT_IX_DISCM: [u8; 8usize] = [
    131, 166, 64, 94, 108, 213, 114, 183,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositToFarmVaultIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositToFarmVaultIxData(pub DepositToFarmVaultIxArgs);
impl From<DepositToFarmVaultIxArgs> for DepositToFarmVaultIxData {
    fn from(args: DepositToFarmVaultIxArgs) -> Self {
        Self(args)
    }
}
impl DepositToFarmVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_TO_FARM_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositToFarmVaultIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_TO_FARM_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_to_farm_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositToFarmVaultKeys,
    args: DepositToFarmVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_TO_FARM_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositToFarmVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_to_farm_vault_ix(
    keys: DepositToFarmVaultKeys,
    args: DepositToFarmVaultIxArgs,
) -> std::io::Result<Instruction> {
    deposit_to_farm_vault_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn deposit_to_farm_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositToFarmVaultAccounts<'_, '_>,
    args: DepositToFarmVaultIxArgs,
) -> ProgramResult {
    let keys: DepositToFarmVaultKeys = accounts.into();
    let ix = deposit_to_farm_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_to_farm_vault_invoke(
    accounts: DepositToFarmVaultAccounts<'_, '_>,
    args: DepositToFarmVaultIxArgs,
) -> ProgramResult {
    deposit_to_farm_vault_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn deposit_to_farm_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositToFarmVaultAccounts<'_, '_>,
    args: DepositToFarmVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositToFarmVaultKeys = accounts.into();
    let ix = deposit_to_farm_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_to_farm_vault_invoke_signed(
    accounts: DepositToFarmVaultAccounts<'_, '_>,
    args: DepositToFarmVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_to_farm_vault_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_to_farm_vault_verify_account_keys(
    accounts: DepositToFarmVaultAccounts<'_, '_>,
    keys: DepositToFarmVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.depositor.key, keys.depositor),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.farm_vault.key, keys.farm_vault),
        (*accounts.depositor_ata.key, keys.depositor_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_to_farm_vault_verify_writable_privileges<'me, 'info>(
    accounts: DepositToFarmVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm_state,
        accounts.farm_vault,
        accounts.depositor_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_to_farm_vault_verify_signer_privileges<'me, 'info>(
    accounts: DepositToFarmVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.depositor] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_to_farm_vault_verify_account_privileges<'me, 'info>(
    accounts: DepositToFarmVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_to_farm_vault_verify_writable_privileges(accounts)?;
    deposit_to_farm_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FROM_FARM_VAULT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFromFarmVaultAccounts<'me, 'info> {
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub withdrawer_token_account: &'me AccountInfo<'info>,
    pub farm_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFromFarmVaultKeys {
    pub withdraw_authority: Pubkey,
    pub farm_state: Pubkey,
    pub withdrawer_token_account: Pubkey,
    pub farm_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawFromFarmVaultAccounts<'_, '_>> for WithdrawFromFarmVaultKeys {
    fn from(accounts: WithdrawFromFarmVaultAccounts) -> Self {
        Self {
            withdraw_authority: *accounts.withdraw_authority.key,
            farm_state: *accounts.farm_state.key,
            withdrawer_token_account: *accounts.withdrawer_token_account.key,
            farm_vault: *accounts.farm_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawFromFarmVaultKeys>
for [AccountMeta; WITHDRAW_FROM_FARM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFromFarmVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdrawer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
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
impl From<[Pubkey; WITHDRAW_FROM_FARM_VAULT_IX_ACCOUNTS_LEN]>
for WithdrawFromFarmVaultKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FROM_FARM_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            withdraw_authority: pubkeys[0],
            farm_state: pubkeys[1],
            withdrawer_token_account: pubkeys[2],
            farm_vault: pubkeys[3],
            farm_vaults_authority: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<WithdrawFromFarmVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FROM_FARM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFromFarmVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.withdraw_authority.clone(),
            accounts.farm_state.clone(),
            accounts.withdrawer_token_account.clone(),
            accounts.farm_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_FROM_FARM_VAULT_IX_ACCOUNTS_LEN]>
for WithdrawFromFarmVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_FROM_FARM_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            withdraw_authority: &arr[0],
            farm_state: &arr[1],
            withdrawer_token_account: &arr[2],
            farm_vault: &arr[3],
            farm_vaults_authority: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const WITHDRAW_FROM_FARM_VAULT_IX_DISCM: [u8; 8usize] = [
    22, 82, 128, 250, 86, 79, 124, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFromFarmVaultIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFromFarmVaultIxData(pub WithdrawFromFarmVaultIxArgs);
impl From<WithdrawFromFarmVaultIxArgs> for WithdrawFromFarmVaultIxData {
    fn from(args: WithdrawFromFarmVaultIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFromFarmVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FROM_FARM_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawFromFarmVaultIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FROM_FARM_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_from_farm_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFromFarmVaultKeys,
    args: WithdrawFromFarmVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FROM_FARM_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFromFarmVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_from_farm_vault_ix(
    keys: WithdrawFromFarmVaultKeys,
    args: WithdrawFromFarmVaultIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_from_farm_vault_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn withdraw_from_farm_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromFarmVaultAccounts<'_, '_>,
    args: WithdrawFromFarmVaultIxArgs,
) -> ProgramResult {
    let keys: WithdrawFromFarmVaultKeys = accounts.into();
    let ix = withdraw_from_farm_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_from_farm_vault_invoke(
    accounts: WithdrawFromFarmVaultAccounts<'_, '_>,
    args: WithdrawFromFarmVaultIxArgs,
) -> ProgramResult {
    withdraw_from_farm_vault_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn withdraw_from_farm_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromFarmVaultAccounts<'_, '_>,
    args: WithdrawFromFarmVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFromFarmVaultKeys = accounts.into();
    let ix = withdraw_from_farm_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_from_farm_vault_invoke_signed(
    accounts: WithdrawFromFarmVaultAccounts<'_, '_>,
    args: WithdrawFromFarmVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_from_farm_vault_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_from_farm_vault_verify_account_keys(
    accounts: WithdrawFromFarmVaultAccounts<'_, '_>,
    keys: WithdrawFromFarmVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.withdrawer_token_account.key, keys.withdrawer_token_account),
        (*accounts.farm_vault.key, keys.farm_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_from_farm_vault_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFromFarmVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_authority,
        accounts.farm_state,
        accounts.withdrawer_token_account,
        accounts.farm_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_from_farm_vault_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFromFarmVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.withdraw_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_from_farm_vault_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFromFarmVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_from_farm_vault_verify_writable_privileges(accounts)?;
    withdraw_from_farm_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_SLASHED_AMOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawSlashedAmountAccounts<'me, 'info> {
    pub crank: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub slashed_amount_spill_address: &'me AccountInfo<'info>,
    pub farm_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawSlashedAmountKeys {
    pub crank: Pubkey,
    pub farm_state: Pubkey,
    pub slashed_amount_spill_address: Pubkey,
    pub farm_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawSlashedAmountAccounts<'_, '_>> for WithdrawSlashedAmountKeys {
    fn from(accounts: WithdrawSlashedAmountAccounts) -> Self {
        Self {
            crank: *accounts.crank.key,
            farm_state: *accounts.farm_state.key,
            slashed_amount_spill_address: *accounts.slashed_amount_spill_address.key,
            farm_vault: *accounts.farm_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawSlashedAmountKeys>
for [AccountMeta; WITHDRAW_SLASHED_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawSlashedAmountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.crank,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.slashed_amount_spill_address,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
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
impl From<[Pubkey; WITHDRAW_SLASHED_AMOUNT_IX_ACCOUNTS_LEN]>
for WithdrawSlashedAmountKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_SLASHED_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            crank: pubkeys[0],
            farm_state: pubkeys[1],
            slashed_amount_spill_address: pubkeys[2],
            farm_vault: pubkeys[3],
            farm_vaults_authority: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<WithdrawSlashedAmountAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_SLASHED_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawSlashedAmountAccounts<'_, 'info>) -> Self {
        [
            accounts.crank.clone(),
            accounts.farm_state.clone(),
            accounts.slashed_amount_spill_address.clone(),
            accounts.farm_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_SLASHED_AMOUNT_IX_ACCOUNTS_LEN]>
for WithdrawSlashedAmountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_SLASHED_AMOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            crank: &arr[0],
            farm_state: &arr[1],
            slashed_amount_spill_address: &arr[2],
            farm_vault: &arr[3],
            farm_vaults_authority: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const WITHDRAW_SLASHED_AMOUNT_IX_DISCM: [u8; 8usize] = [
    202, 217, 67, 74, 172, 22, 140, 216,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawSlashedAmountIxData;
impl WithdrawSlashedAmountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_SLASHED_AMOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_SLASHED_AMOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_slashed_amount_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawSlashedAmountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_SLASHED_AMOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawSlashedAmountIxData.try_to_vec()?,
    })
}
pub fn withdraw_slashed_amount_ix(
    keys: WithdrawSlashedAmountKeys,
) -> std::io::Result<Instruction> {
    withdraw_slashed_amount_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn withdraw_slashed_amount_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSlashedAmountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawSlashedAmountKeys = accounts.into();
    let ix = withdraw_slashed_amount_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_slashed_amount_invoke(
    accounts: WithdrawSlashedAmountAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_slashed_amount_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn withdraw_slashed_amount_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSlashedAmountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawSlashedAmountKeys = accounts.into();
    let ix = withdraw_slashed_amount_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_slashed_amount_invoke_signed(
    accounts: WithdrawSlashedAmountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_slashed_amount_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_slashed_amount_verify_account_keys(
    accounts: WithdrawSlashedAmountAccounts<'_, '_>,
    keys: WithdrawSlashedAmountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.crank.key, keys.crank),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.slashed_amount_spill_address.key, keys.slashed_amount_spill_address),
        (*accounts.farm_vault.key, keys.farm_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_slashed_amount_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawSlashedAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.crank,
        accounts.farm_state,
        accounts.slashed_amount_spill_address,
        accounts.farm_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_slashed_amount_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawSlashedAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.crank] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_slashed_amount_verify_account_privileges<'me, 'info>(
    accounts: WithdrawSlashedAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_slashed_amount_verify_writable_privileges(accounts)?;
    withdraw_slashed_amount_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FARM_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFarmAdminAccounts<'me, 'info> {
    pub pending_farm_admin: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFarmAdminKeys {
    pub pending_farm_admin: Pubkey,
    pub farm_state: Pubkey,
}
impl From<UpdateFarmAdminAccounts<'_, '_>> for UpdateFarmAdminKeys {
    fn from(accounts: UpdateFarmAdminAccounts) -> Self {
        Self {
            pending_farm_admin: *accounts.pending_farm_admin.key,
            farm_state: *accounts.farm_state.key,
        }
    }
}
impl From<UpdateFarmAdminKeys> for [AccountMeta; UPDATE_FARM_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFarmAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_farm_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FARM_ADMIN_IX_ACCOUNTS_LEN]> for UpdateFarmAdminKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FARM_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_farm_admin: pubkeys[0],
            farm_state: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateFarmAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FARM_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFarmAdminAccounts<'_, 'info>) -> Self {
        [accounts.pending_farm_admin.clone(), accounts.farm_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FARM_ADMIN_IX_ACCOUNTS_LEN]>
for UpdateFarmAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FARM_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_farm_admin: &arr[0],
            farm_state: &arr[1],
        }
    }
}
pub const UPDATE_FARM_ADMIN_IX_DISCM: [u8; 8usize] = [
    20, 37, 136, 19, 122, 239, 36, 130,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFarmAdminIxData;
impl UpdateFarmAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FARM_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FARM_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_farm_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFarmAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FARM_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateFarmAdminIxData.try_to_vec()?,
    })
}
pub fn update_farm_admin_ix(keys: UpdateFarmAdminKeys) -> std::io::Result<Instruction> {
    update_farm_admin_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn update_farm_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFarmAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateFarmAdminKeys = accounts.into();
    let ix = update_farm_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_farm_admin_invoke(
    accounts: UpdateFarmAdminAccounts<'_, '_>,
) -> ProgramResult {
    update_farm_admin_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn update_farm_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFarmAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFarmAdminKeys = accounts.into();
    let ix = update_farm_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_farm_admin_invoke_signed(
    accounts: UpdateFarmAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_farm_admin_invoke_signed_with_program_id(FARMS_PROGRAM_ID, accounts, seeds)
}
pub fn update_farm_admin_verify_account_keys(
    accounts: UpdateFarmAdminAccounts<'_, '_>,
    keys: UpdateFarmAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_farm_admin.key, keys.pending_farm_admin),
        (*accounts.farm_state.key, keys.farm_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_farm_admin_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFarmAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pending_farm_admin, accounts.farm_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_farm_admin_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFarmAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_farm_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_farm_admin_verify_account_privileges<'me, 'info>(
    accounts: UpdateFarmAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_farm_admin_verify_writable_privileges(accounts)?;
    update_farm_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateGlobalConfigAdminAccounts<'me, 'info> {
    pub pending_global_admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigAdminKeys {
    pub pending_global_admin: Pubkey,
    pub global_config: Pubkey,
}
impl From<UpdateGlobalConfigAdminAccounts<'_, '_>> for UpdateGlobalConfigAdminKeys {
    fn from(accounts: UpdateGlobalConfigAdminAccounts) -> Self {
        Self {
            pending_global_admin: *accounts.pending_global_admin.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<UpdateGlobalConfigAdminKeys>
for [AccountMeta; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateGlobalConfigAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_global_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN]>
for UpdateGlobalConfigAdminKeys {
    fn from(pubkeys: [Pubkey; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_global_admin: pubkeys[0],
            global_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateGlobalConfigAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateGlobalConfigAdminAccounts<'_, 'info>) -> Self {
        [accounts.pending_global_admin.clone(), accounts.global_config.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN]>
for UpdateGlobalConfigAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pending_global_admin: &arr[0],
            global_config: &arr[1],
        }
    }
}
pub const UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM: [u8; 8usize] = [
    184, 87, 23, 193, 156, 238, 175, 119,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigAdminIxData;
impl UpdateGlobalConfigAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_GLOBAL_CONFIG_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_global_config_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateGlobalConfigAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_GLOBAL_CONFIG_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateGlobalConfigAdminIxData.try_to_vec()?,
    })
}
pub fn update_global_config_admin_ix(
    keys: UpdateGlobalConfigAdminKeys,
) -> std::io::Result<Instruction> {
    update_global_config_admin_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn update_global_config_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateGlobalConfigAdminKeys = accounts.into();
    let ix = update_global_config_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_global_config_admin_invoke(
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
) -> ProgramResult {
    update_global_config_admin_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn update_global_config_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateGlobalConfigAdminKeys = accounts.into();
    let ix = update_global_config_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_global_config_admin_invoke_signed(
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_global_config_admin_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_global_config_admin_verify_account_keys(
    accounts: UpdateGlobalConfigAdminAccounts<'_, '_>,
    keys: UpdateGlobalConfigAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_global_admin.key, keys.pending_global_admin),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_global_config_admin_verify_writable_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_global_config_admin_verify_signer_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_global_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_global_config_admin_verify_account_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_global_config_admin_verify_writable_privileges(accounts)?;
    update_global_config_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_REWARD_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawRewardAccounts<'me, 'info> {
    pub farm_admin: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub admin_reward_token_ata: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawRewardKeys {
    pub farm_admin: Pubkey,
    pub farm_state: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub admin_reward_token_ata: Pubkey,
    pub scope_prices: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawRewardAccounts<'_, '_>> for WithdrawRewardKeys {
    fn from(accounts: WithdrawRewardAccounts) -> Self {
        Self {
            farm_admin: *accounts.farm_admin.key,
            farm_state: *accounts.farm_state.key,
            reward_mint: *accounts.reward_mint.key,
            reward_vault: *accounts.reward_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            admin_reward_token_ata: *accounts.admin_reward_token_ata.key,
            scope_prices: *accounts.scope_prices.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawRewardKeys> for [AccountMeta; WITHDRAW_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.farm_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_vaults_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin_reward_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
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
impl From<[Pubkey; WITHDRAW_REWARD_IX_ACCOUNTS_LEN]> for WithdrawRewardKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_admin: pubkeys[0],
            farm_state: pubkeys[1],
            reward_mint: pubkeys[2],
            reward_vault: pubkeys[3],
            farm_vaults_authority: pubkeys[4],
            admin_reward_token_ata: pubkeys[5],
            scope_prices: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<WithdrawRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.farm_admin.clone(),
            accounts.farm_state.clone(),
            accounts.reward_mint.clone(),
            accounts.reward_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.admin_reward_token_ata.clone(),
            accounts.scope_prices.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_REWARD_IX_ACCOUNTS_LEN]>
for WithdrawRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            farm_admin: &arr[0],
            farm_state: &arr[1],
            reward_mint: &arr[2],
            reward_vault: &arr[3],
            farm_vaults_authority: &arr[4],
            admin_reward_token_ata: &arr[5],
            scope_prices: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const WITHDRAW_REWARD_IX_DISCM: [u8; 8usize] = [191, 187, 176, 137, 9, 25, 187, 244];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawRewardIxArgs {
    pub amount: u64,
    pub reward_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawRewardIxData(pub WithdrawRewardIxArgs);
impl From<WithdrawRewardIxArgs> for WithdrawRewardIxData {
    fn from(args: WithdrawRewardIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawRewardIxArgs {
                amount,
                reward_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawRewardKeys,
    args: WithdrawRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_reward_ix(
    keys: WithdrawRewardKeys,
    args: WithdrawRewardIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_reward_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn withdraw_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawRewardAccounts<'_, '_>,
    args: WithdrawRewardIxArgs,
) -> ProgramResult {
    let keys: WithdrawRewardKeys = accounts.into();
    let ix = withdraw_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_reward_invoke(
    accounts: WithdrawRewardAccounts<'_, '_>,
    args: WithdrawRewardIxArgs,
) -> ProgramResult {
    withdraw_reward_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn withdraw_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawRewardAccounts<'_, '_>,
    args: WithdrawRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawRewardKeys = accounts.into();
    let ix = withdraw_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_reward_invoke_signed(
    accounts: WithdrawRewardAccounts<'_, '_>,
    args: WithdrawRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_reward_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_reward_verify_account_keys(
    accounts: WithdrawRewardAccounts<'_, '_>,
    keys: WithdrawRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.farm_admin.key, keys.farm_admin),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.admin_reward_token_ata.key, keys.admin_reward_token_ata),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_reward_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm_admin,
        accounts.farm_state,
        accounts.reward_vault,
        accounts.admin_reward_token_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_reward_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.farm_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_reward_verify_account_privileges<'me, 'info>(
    accounts: WithdrawRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_reward_verify_writable_privileges(accounts)?;
    withdraw_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_SECOND_DELEGATED_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSecondDelegatedAuthorityAccounts<'me, 'info> {
    pub global_admin: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub new_second_delegated_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSecondDelegatedAuthorityKeys {
    pub global_admin: Pubkey,
    pub farm_state: Pubkey,
    pub global_config: Pubkey,
    pub new_second_delegated_authority: Pubkey,
}
impl From<UpdateSecondDelegatedAuthorityAccounts<'_, '_>>
for UpdateSecondDelegatedAuthorityKeys {
    fn from(accounts: UpdateSecondDelegatedAuthorityAccounts) -> Self {
        Self {
            global_admin: *accounts.global_admin.key,
            farm_state: *accounts.farm_state.key,
            global_config: *accounts.global_config.key,
            new_second_delegated_authority: *accounts.new_second_delegated_authority.key,
        }
    }
}
impl From<UpdateSecondDelegatedAuthorityKeys>
for [AccountMeta; UPDATE_SECOND_DELEGATED_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateSecondDelegatedAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_second_delegated_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_SECOND_DELEGATED_AUTHORITY_IX_ACCOUNTS_LEN]>
for UpdateSecondDelegatedAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_SECOND_DELEGATED_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global_admin: pubkeys[0],
            farm_state: pubkeys[1],
            global_config: pubkeys[2],
            new_second_delegated_authority: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateSecondDelegatedAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_SECOND_DELEGATED_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateSecondDelegatedAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.global_admin.clone(),
            accounts.farm_state.clone(),
            accounts.global_config.clone(),
            accounts.new_second_delegated_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_SECOND_DELEGATED_AUTHORITY_IX_ACCOUNTS_LEN]>
for UpdateSecondDelegatedAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SECOND_DELEGATED_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global_admin: &arr[0],
            farm_state: &arr[1],
            global_config: &arr[2],
            new_second_delegated_authority: &arr[3],
        }
    }
}
pub const UPDATE_SECOND_DELEGATED_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    127, 26, 6, 181, 203, 248, 117, 64,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSecondDelegatedAuthorityIxData;
impl UpdateSecondDelegatedAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SECOND_DELEGATED_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SECOND_DELEGATED_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_second_delegated_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateSecondDelegatedAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_SECOND_DELEGATED_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateSecondDelegatedAuthorityIxData.try_to_vec()?,
    })
}
pub fn update_second_delegated_authority_ix(
    keys: UpdateSecondDelegatedAuthorityKeys,
) -> std::io::Result<Instruction> {
    update_second_delegated_authority_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn update_second_delegated_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSecondDelegatedAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateSecondDelegatedAuthorityKeys = accounts.into();
    let ix = update_second_delegated_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_second_delegated_authority_invoke(
    accounts: UpdateSecondDelegatedAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    update_second_delegated_authority_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn update_second_delegated_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSecondDelegatedAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateSecondDelegatedAuthorityKeys = accounts.into();
    let ix = update_second_delegated_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_second_delegated_authority_invoke_signed(
    accounts: UpdateSecondDelegatedAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_second_delegated_authority_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_second_delegated_authority_verify_account_keys(
    accounts: UpdateSecondDelegatedAuthorityAccounts<'_, '_>,
    keys: UpdateSecondDelegatedAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global_admin.key, keys.global_admin),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.global_config.key, keys.global_config),
        (
            *accounts.new_second_delegated_authority.key,
            keys.new_second_delegated_authority,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_second_delegated_authority_verify_writable_privileges<'me, 'info>(
    accounts: UpdateSecondDelegatedAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_admin, accounts.farm_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_second_delegated_authority_verify_signer_privileges<'me, 'info>(
    accounts: UpdateSecondDelegatedAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_second_delegated_authority_verify_account_privileges<'me, 'info>(
    accounts: UpdateSecondDelegatedAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_second_delegated_authority_verify_writable_privileges(accounts)?;
    update_second_delegated_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_EMPTY_USER_STATE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseEmptyUserStateAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseEmptyUserStateKeys {
    pub signer: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub rent_receiver: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseEmptyUserStateAccounts<'_, '_>> for CloseEmptyUserStateKeys {
    fn from(accounts: CloseEmptyUserStateAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            rent_receiver: *accounts.rent_receiver.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseEmptyUserStateKeys>
for [AccountMeta; CLOSE_EMPTY_USER_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseEmptyUserStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_EMPTY_USER_STATE_IX_ACCOUNTS_LEN]> for CloseEmptyUserStateKeys {
    fn from(pubkeys: [Pubkey; CLOSE_EMPTY_USER_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            user_state: pubkeys[1],
            farm_state: pubkeys[2],
            rent_receiver: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseEmptyUserStateAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_EMPTY_USER_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseEmptyUserStateAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.rent_receiver.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_EMPTY_USER_STATE_IX_ACCOUNTS_LEN]>
for CloseEmptyUserStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_EMPTY_USER_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            user_state: &arr[1],
            farm_state: &arr[2],
            rent_receiver: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CLOSE_EMPTY_USER_STATE_IX_DISCM: [u8; 8usize] = [
    240, 24, 9, 227, 86, 225, 199, 95,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseEmptyUserStateIxData;
impl CloseEmptyUserStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_EMPTY_USER_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_EMPTY_USER_STATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_empty_user_state_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseEmptyUserStateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_EMPTY_USER_STATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseEmptyUserStateIxData.try_to_vec()?,
    })
}
pub fn close_empty_user_state_ix(
    keys: CloseEmptyUserStateKeys,
) -> std::io::Result<Instruction> {
    close_empty_user_state_ix_with_program_id(FARMS_PROGRAM_ID, keys)
}
pub fn close_empty_user_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseEmptyUserStateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseEmptyUserStateKeys = accounts.into();
    let ix = close_empty_user_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_empty_user_state_invoke(
    accounts: CloseEmptyUserStateAccounts<'_, '_>,
) -> ProgramResult {
    close_empty_user_state_invoke_with_program_id(FARMS_PROGRAM_ID, accounts)
}
pub fn close_empty_user_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseEmptyUserStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseEmptyUserStateKeys = accounts.into();
    let ix = close_empty_user_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_empty_user_state_invoke_signed(
    accounts: CloseEmptyUserStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_empty_user_state_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_empty_user_state_verify_account_keys(
    accounts: CloseEmptyUserStateAccounts<'_, '_>,
    keys: CloseEmptyUserStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_empty_user_state_verify_writable_privileges<'me, 'info>(
    accounts: CloseEmptyUserStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_state, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_empty_user_state_verify_signer_privileges<'me, 'info>(
    accounts: CloseEmptyUserStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_empty_user_state_verify_account_privileges<'me, 'info>(
    accounts: CloseEmptyUserStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_empty_user_state_verify_writable_privileges(accounts)?;
    close_empty_user_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const IDL_MISSING_TYPES_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct IdlMissingTypesAccounts<'me, 'info> {
    pub global_admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IdlMissingTypesKeys {
    pub global_admin: Pubkey,
    pub global_config: Pubkey,
}
impl From<IdlMissingTypesAccounts<'_, '_>> for IdlMissingTypesKeys {
    fn from(accounts: IdlMissingTypesAccounts) -> Self {
        Self {
            global_admin: *accounts.global_admin.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<IdlMissingTypesKeys> for [AccountMeta; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN] {
    fn from(keys: IdlMissingTypesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN]> for IdlMissingTypesKeys {
    fn from(pubkeys: [Pubkey; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_admin: pubkeys[0],
            global_config: pubkeys[1],
        }
    }
}
impl<'info> From<IdlMissingTypesAccounts<'_, 'info>>
for [AccountInfo<'info>; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN] {
    fn from(accounts: IdlMissingTypesAccounts<'_, 'info>) -> Self {
        [accounts.global_admin.clone(), accounts.global_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN]>
for IdlMissingTypesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_admin: &arr[0],
            global_config: &arr[1],
        }
    }
}
pub const IDL_MISSING_TYPES_IX_DISCM: [u8; 8usize] = [
    130, 80, 38, 153, 80, 212, 182, 253,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IdlMissingTypesIxArgs {
    pub global_config_option_kind: GlobalConfigOption,
    pub farm_config_option_kind: FarmConfigOption,
    pub time_unit: TimeUnit,
    pub locking_mode: LockingMode,
    pub reward_type: RewardType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IdlMissingTypesIxData(pub IdlMissingTypesIxArgs);
impl From<IdlMissingTypesIxArgs> for IdlMissingTypesIxData {
    fn from(args: IdlMissingTypesIxArgs) -> Self {
        Self(args)
    }
}
impl IdlMissingTypesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != IDL_MISSING_TYPES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let global_config_option_kind: GlobalConfigOption = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let farm_config_option_kind: FarmConfigOption = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let time_unit: TimeUnit = crate::borsh_de_or_default(&mut reader)?;
        let locking_mode: LockingMode = crate::borsh_de_or_default(&mut reader)?;
        let reward_type: RewardType = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IdlMissingTypesIxArgs {
                global_config_option_kind,
                farm_config_option_kind,
                time_unit,
                locking_mode,
                reward_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&IDL_MISSING_TYPES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.global_config_option_kind,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.farm_config_option_kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.time_unit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.locking_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reward_type, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn idl_missing_types_ix_with_program_id(
    program_id: Pubkey,
    keys: IdlMissingTypesKeys,
    args: IdlMissingTypesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN] = keys.into();
    let data: IdlMissingTypesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn idl_missing_types_ix(
    keys: IdlMissingTypesKeys,
    args: IdlMissingTypesIxArgs,
) -> std::io::Result<Instruction> {
    idl_missing_types_ix_with_program_id(FARMS_PROGRAM_ID, keys, args)
}
pub fn idl_missing_types_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IdlMissingTypesAccounts<'_, '_>,
    args: IdlMissingTypesIxArgs,
) -> ProgramResult {
    let keys: IdlMissingTypesKeys = accounts.into();
    let ix = idl_missing_types_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn idl_missing_types_invoke(
    accounts: IdlMissingTypesAccounts<'_, '_>,
    args: IdlMissingTypesIxArgs,
) -> ProgramResult {
    idl_missing_types_invoke_with_program_id(FARMS_PROGRAM_ID, accounts, args)
}
pub fn idl_missing_types_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IdlMissingTypesAccounts<'_, '_>,
    args: IdlMissingTypesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IdlMissingTypesKeys = accounts.into();
    let ix = idl_missing_types_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn idl_missing_types_invoke_signed(
    accounts: IdlMissingTypesAccounts<'_, '_>,
    args: IdlMissingTypesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    idl_missing_types_invoke_signed_with_program_id(
        FARMS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn idl_missing_types_verify_account_keys(
    accounts: IdlMissingTypesAccounts<'_, '_>,
    keys: IdlMissingTypesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global_admin.key, keys.global_admin),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn idl_missing_types_verify_writable_privileges<'me, 'info>(
    accounts: IdlMissingTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn idl_missing_types_verify_signer_privileges<'me, 'info>(
    accounts: IdlMissingTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn idl_missing_types_verify_account_privileges<'me, 'info>(
    accounts: IdlMissingTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    idl_missing_types_verify_writable_privileges(accounts)?;
    idl_missing_types_verify_signer_privileges(accounts)?;
    Ok(())
}
