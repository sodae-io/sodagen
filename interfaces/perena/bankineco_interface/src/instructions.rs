use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum BankinecoProgramIx {
    BurnForYieldingGen(BurnForYieldingGenIxArgs),
    ChangeFees(ChangeFeesIxArgs),
    CreateBank(CreateBankIxArgs),
    CreateGenOracleAccount(CreateGenOracleAccountIxArgs),
    CreateGenTeamAccount(CreateGenTeamAccountIxArgs),
    CreateGenVault(CreateGenVaultIxArgs),
    CreateTrancheState(CreateTrancheStateIxArgs),
    DepositInLp(DepositInLpIxArgs),
    FulfillUnstakeJunior,
    InitMarginfiAccount(InitMarginfiAccountIxArgs),
    InstantUnstakeJunior(InstantUnstakeJuniorIxArgs),
    MintWYieldingGen(MintWYieldingGenIxArgs),
    NameBankManager(NameBankManagerIxArgs),
    NameBankRiskManager(NameBankRiskManagerIxArgs),
    RefreshAtomicLendingAccounting,
    RequestUnstakeJunior(RequestUnstakeJuniorIxArgs),
    SetVaultConfigDetails(SetVaultConfigDetailsIxArgs),
    StakeJunior(StakeJuniorIxArgs),
    TeamDepositsFromInvest(TeamDepositsFromInvestIxArgs),
    TeamWithdrawsFees(TeamWithdrawsFeesIxArgs),
    TeamWithdrawsToInvest(TeamWithdrawsToInvestIxArgs),
    TriggerBankCircuitBreaker(TriggerBankCircuitBreakerIxArgs),
    TriggerVaultCircuitBreaker(TriggerVaultCircuitBreakerIxArgs),
    UpdateOracleState(UpdateOracleStateIxArgs),
    UpdateTrancheConfig(UpdateTrancheConfigIxArgs),
    UpdateYieldingAmount(UpdateYieldingAmountIxArgs),
    UpdateYieldingInfo(UpdateYieldingInfoIxArgs),
    UpdateYieldingPriceGen(UpdateYieldingPriceGenIxArgs),
    VaultReallocator(VaultReallocatorIxArgs),
    VaultTransfer(VaultTransferIxArgs),
    WithdrawFromLp(WithdrawFromLpIxArgs),
    WithdrawTrancheFees(WithdrawTrancheFeesIxArgs),
}
impl BankinecoProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BURN_FOR_YIELDING_GEN_IX_DISCM) {
            let mut reader = &buf[BURN_FOR_YIELDING_GEN_IX_DISCM.len()..];
            let amount_to_burn: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_yielding_withdrawn: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::BurnForYieldingGen(BurnForYieldingGenIxArgs {
                    amount_to_burn,
                    minimum_yielding_withdrawn,
                }),
            );
        }
        if buf.starts_with(&CHANGE_FEES_IX_DISCM) {
            let mut reader = &buf[CHANGE_FEES_IX_DISCM.len()..];
            let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let minting_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let burning_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ChangeFees(ChangeFeesIxArgs {
                    performance_fee_bps,
                    minting_fee_bps,
                    burning_fee_bps,
                }),
            );
        }
        if buf.starts_with(&CREATE_BANK_IX_DISCM) {
            let mut reader = &buf[CREATE_BANK_IX_DISCM.len()..];
            let bank_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let max_yielding_tvl: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateBank(CreateBankIxArgs {
                    bank_index,
                    max_yielding_tvl,
                }),
            );
        }
        if buf.starts_with(&CREATE_GEN_ORACLE_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CREATE_GEN_ORACLE_ACCOUNT_IX_DISCM.len()..];
            let contract_init_price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let oracle_one: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let oracle_two: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateGenOracleAccount(CreateGenOracleAccountIxArgs {
                    contract_init_price,
                    oracle_one,
                    oracle_two,
                }),
            );
        }
        if buf.starts_with(&CREATE_GEN_TEAM_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CREATE_GEN_TEAM_ACCOUNT_IX_DISCM.len()..];
            let yield_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateGenTeamAccount(CreateGenTeamAccountIxArgs {
                    yield_manager,
                }),
            );
        }
        if buf.starts_with(&CREATE_GEN_VAULT_IX_DISCM) {
            let mut reader = &buf[CREATE_GEN_VAULT_IX_DISCM.len()..];
            let vault_type: VaultType = crate::borsh_de_or_default(&mut reader)?;
            let yielding_token_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let mint_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let burn_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let risk_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            let lending_platform: Option<LendingPlatform> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let lending_type: Option<LendingType> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::CreateGenVault(CreateGenVaultIxArgs {
                    vault_type,
                    yielding_token_index,
                    mint_fee_bps,
                    burn_fee_bps,
                    performance_fee_bps,
                    risk_manager,
                    lending_platform,
                    lending_type,
                }),
            );
        }
        if buf.starts_with(&CREATE_TRANCHE_STATE_IX_DISCM) {
            let mut reader = &buf[CREATE_TRANCHE_STATE_IX_DISCM.len()..];
            let base_leverage_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
            let target_percent_staked_bps: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let leverage_slope: u16 = crate::borsh_de_or_default(&mut reader)?;
            let early_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let standard_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let low_stake_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let low_stake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let target_lockup_duration_secs: i64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let recovery_yield_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let recovery_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let bootstrap_recovery_yield_bps: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::CreateTrancheState(CreateTrancheStateIxArgs {
                    base_leverage_factor,
                    target_percent_staked_bps,
                    leverage_slope,
                    early_unstake_fee_bps,
                    standard_unstake_fee_bps,
                    low_stake_threshold_bps,
                    low_stake_fee_bps,
                    target_lockup_duration_secs,
                    recovery_yield_bps,
                    recovery_threshold_bps,
                    bootstrap_recovery_yield_bps,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IN_LP_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IN_LP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DepositInLp(DepositInLpIxArgs { amount }));
        }
        if buf.starts_with(&FULFILL_UNSTAKE_JUNIOR_IX_DISCM) {
            return Ok(Self::FulfillUnstakeJunior);
        }
        if buf.starts_with(&INIT_MARGINFI_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[INIT_MARGINFI_ACCOUNT_IX_DISCM.len()..];
            let account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
            let third_party_id: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitMarginfiAccount(InitMarginfiAccountIxArgs {
                    account_index,
                    third_party_id,
                }),
            );
        }
        if buf.starts_with(&INSTANT_UNSTAKE_JUNIOR_IX_DISCM) {
            let mut reader = &buf[INSTANT_UNSTAKE_JUNIOR_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InstantUnstakeJunior(InstantUnstakeJuniorIxArgs {
                    shares,
                }),
            );
        }
        if buf.starts_with(&MINT_W_YIELDING_GEN_IX_DISCM) {
            let mut reader = &buf[MINT_W_YIELDING_GEN_IX_DISCM.len()..];
            let amount_yielding_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_bank_mint_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MintWYieldingGen(MintWYieldingGenIxArgs {
                    amount_yielding_deposit,
                    min_bank_mint_minted,
                }),
            );
        }
        if buf.starts_with(&NAME_BANK_MANAGER_IX_DISCM) {
            let mut reader = &buf[NAME_BANK_MANAGER_IX_DISCM.len()..];
            let new_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::NameBankManager(NameBankManagerIxArgs {
                    new_manager,
                }),
            );
        }
        if buf.starts_with(&NAME_BANK_RISK_MANAGER_IX_DISCM) {
            let mut reader = &buf[NAME_BANK_RISK_MANAGER_IX_DISCM.len()..];
            let risk_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::NameBankRiskManager(NameBankRiskManagerIxArgs {
                    risk_manager,
                }),
            );
        }
        if buf.starts_with(&REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_DISCM) {
            return Ok(Self::RefreshAtomicLendingAccounting);
        }
        if buf.starts_with(&REQUEST_UNSTAKE_JUNIOR_IX_DISCM) {
            let mut reader = &buf[REQUEST_UNSTAKE_JUNIOR_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let queue_id: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RequestUnstakeJunior(RequestUnstakeJuniorIxArgs {
                    shares,
                    queue_id,
                }),
            );
        }
        if buf.starts_with(&SET_VAULT_CONFIG_DETAILS_IX_DISCM) {
            let mut reader = &buf[SET_VAULT_CONFIG_DETAILS_IX_DISCM.len()..];
            let data = if reader.is_empty() {
                Default::default()
            } else {
                <VaultConfigData>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SetVaultConfigDetails(SetVaultConfigDetailsIxArgs {
                    data,
                }),
            );
        }
        if buf.starts_with(&STAKE_JUNIOR_IX_DISCM) {
            let mut reader = &buf[STAKE_JUNIOR_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::StakeJunior(StakeJuniorIxArgs { amount }));
        }
        if buf.starts_with(&TEAM_DEPOSITS_FROM_INVEST_IX_DISCM) {
            let mut reader = &buf[TEAM_DEPOSITS_FROM_INVEST_IX_DISCM.len()..];
            let deposited_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TeamDepositsFromInvest(TeamDepositsFromInvestIxArgs {
                    deposited_amount,
                }),
            );
        }
        if buf.starts_with(&TEAM_WITHDRAWS_FEES_IX_DISCM) {
            let mut reader = &buf[TEAM_WITHDRAWS_FEES_IX_DISCM.len()..];
            let fee_type: FeeType = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TeamWithdrawsFees(TeamWithdrawsFeesIxArgs {
                    fee_type,
                }),
            );
        }
        if buf.starts_with(&TEAM_WITHDRAWS_TO_INVEST_IX_DISCM) {
            let mut reader = &buf[TEAM_WITHDRAWS_TO_INVEST_IX_DISCM.len()..];
            let withdrawal_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TeamWithdrawsToInvest(TeamWithdrawsToInvestIxArgs {
                    withdrawal_amount,
                }),
            );
        }
        if buf.starts_with(&TRIGGER_BANK_CIRCUIT_BREAKER_IX_DISCM) {
            let mut reader = &buf[TRIGGER_BANK_CIRCUIT_BREAKER_IX_DISCM.len()..];
            let is_halted: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_halted_deposit: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_halted_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TriggerBankCircuitBreaker(TriggerBankCircuitBreakerIxArgs {
                    is_halted,
                    is_halted_deposit,
                    is_halted_withdrawal,
                }),
            );
        }
        if buf.starts_with(&TRIGGER_VAULT_CIRCUIT_BREAKER_IX_DISCM) {
            let mut reader = &buf[TRIGGER_VAULT_CIRCUIT_BREAKER_IX_DISCM.len()..];
            let is_halted: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_halted_deposit: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_halted_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
            let losses_accepted: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TriggerVaultCircuitBreaker(TriggerVaultCircuitBreakerIxArgs {
                    is_halted,
                    is_halted_deposit,
                    is_halted_withdrawal,
                    losses_accepted,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ORACLE_STATE_IX_DISCM) {
            let mut reader = &buf[UPDATE_ORACLE_STATE_IX_DISCM.len()..];
            let oracle_one: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            let oracle_two: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateOracleState(UpdateOracleStateIxArgs {
                    oracle_one,
                    oracle_two,
                }),
            );
        }
        if buf.starts_with(&UPDATE_TRANCHE_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_TRANCHE_CONFIG_IX_DISCM.len()..];
            let base_leverage_factor: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let target_percent_staked_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let leverage_slope: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
            let early_unstake_fee_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let standard_unstake_fee_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let low_stake_threshold_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let low_stake_fee_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let low_stake_fee_enabled: Option<bool> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let target_lockup_duration_secs: Option<i64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let recovery_yield_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let recovery_threshold_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let bootstrap_recovery_yield_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateTrancheConfig(UpdateTrancheConfigIxArgs {
                    base_leverage_factor,
                    target_percent_staked_bps,
                    leverage_slope,
                    early_unstake_fee_bps,
                    standard_unstake_fee_bps,
                    low_stake_threshold_bps,
                    low_stake_fee_bps,
                    low_stake_fee_enabled,
                    target_lockup_duration_secs,
                    recovery_yield_bps,
                    recovery_threshold_bps,
                    bootstrap_recovery_yield_bps,
                }),
            );
        }
        if buf.starts_with(&UPDATE_YIELDING_AMOUNT_IX_DISCM) {
            let mut reader = &buf[UPDATE_YIELDING_AMOUNT_IX_DISCM.len()..];
            let push_all_pending_yield: bool = crate::borsh_de_or_default(&mut reader)?;
            let yield_to_push: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let new_start_marker_unix_seconds: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let synthetic_yield_push: Option<bool> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateYieldingAmount(UpdateYieldingAmountIxArgs {
                    push_all_pending_yield,
                    yield_to_push,
                    new_start_marker_unix_seconds,
                    synthetic_yield_push,
                }),
            );
        }
        if buf.starts_with(&UPDATE_YIELDING_INFO_IX_DISCM) {
            let mut reader = &buf[UPDATE_YIELDING_INFO_IX_DISCM.len()..];
            let new_price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let start_marker_unix_seconds: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateYieldingInfo(UpdateYieldingInfoIxArgs {
                    new_price,
                    start_marker_unix_seconds,
                }),
            );
        }
        if buf.starts_with(&UPDATE_YIELDING_PRICE_GEN_IX_DISCM) {
            let mut reader = &buf[UPDATE_YIELDING_PRICE_GEN_IX_DISCM.len()..];
            let new_price: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateYieldingPriceGen(UpdateYieldingPriceGenIxArgs {
                    new_price,
                }),
            );
        }
        if buf.starts_with(&VAULT_REALLOCATOR_IX_DISCM) {
            let mut reader = &buf[VAULT_REALLOCATOR_IX_DISCM.len()..];
            let decrement_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let increment_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let synthetic_reallocation: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::VaultReallocator(VaultReallocatorIxArgs {
                    decrement_amount,
                    increment_amount,
                    synthetic_reallocation,
                }),
            );
        }
        if buf.starts_with(&VAULT_TRANSFER_IX_DISCM) {
            let mut reader = &buf[VAULT_TRANSFER_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::VaultTransfer(VaultTransferIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_FROM_LP_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_LP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::WithdrawFromLp(WithdrawFromLpIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_TRANCHE_FEES_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_TRANCHE_FEES_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawTrancheFees(WithdrawTrancheFeesIxArgs {
                    amount,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::BurnForYieldingGen(args) => {
                writer.write_all(&BURN_FOR_YIELDING_GEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_to_burn, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_yielding_withdrawn,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ChangeFees(args) => {
                writer.write_all(&CHANGE_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.performance_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.minting_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.burning_fee_bps, &mut writer)?;
                Ok(())
            }
            Self::CreateBank(args) => {
                writer.write_all(&CREATE_BANK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_yielding_tvl, &mut writer)?;
                Ok(())
            }
            Self::CreateGenOracleAccount(args) => {
                writer.write_all(&CREATE_GEN_ORACLE_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.contract_init_price,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.oracle_one, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.oracle_two, &mut writer)?;
                Ok(())
            }
            Self::CreateGenTeamAccount(args) => {
                writer.write_all(&CREATE_GEN_TEAM_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.yield_manager, &mut writer)?;
                Ok(())
            }
            Self::CreateGenVault(args) => {
                writer.write_all(&CREATE_GEN_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.vault_type, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.yielding_token_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.mint_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.burn_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.performance_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.risk_manager, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.lending_platform, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.lending_type, &mut writer)?;
                Ok(())
            }
            Self::CreateTrancheState(args) => {
                writer.write_all(&CREATE_TRANCHE_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.base_leverage_factor,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.target_percent_staked_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.leverage_slope, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.early_unstake_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.standard_unstake_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.low_stake_threshold_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.low_stake_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.target_lockup_duration_secs,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.recovery_yield_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.recovery_threshold_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.bootstrap_recovery_yield_bps,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::DepositInLp(args) => {
                writer.write_all(&DEPOSIT_IN_LP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::FulfillUnstakeJunior => {
                writer.write_all(&FULFILL_UNSTAKE_JUNIOR_IX_DISCM)
            }
            Self::InitMarginfiAccount(args) => {
                writer.write_all(&INIT_MARGINFI_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.account_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.third_party_id, &mut writer)?;
                Ok(())
            }
            Self::InstantUnstakeJunior(args) => {
                writer.write_all(&INSTANT_UNSTAKE_JUNIOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::MintWYieldingGen(args) => {
                writer.write_all(&MINT_W_YIELDING_GEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.amount_yielding_deposit,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.min_bank_mint_minted,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::NameBankManager(args) => {
                writer.write_all(&NAME_BANK_MANAGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_manager, &mut writer)?;
                Ok(())
            }
            Self::NameBankRiskManager(args) => {
                writer.write_all(&NAME_BANK_RISK_MANAGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.risk_manager, &mut writer)?;
                Ok(())
            }
            Self::RefreshAtomicLendingAccounting => {
                writer.write_all(&REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_DISCM)
            }
            Self::RequestUnstakeJunior(args) => {
                writer.write_all(&REQUEST_UNSTAKE_JUNIOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.queue_id, &mut writer)?;
                Ok(())
            }
            Self::SetVaultConfigDetails(args) => {
                writer.write_all(&SET_VAULT_CONFIG_DETAILS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::StakeJunior(args) => {
                writer.write_all(&STAKE_JUNIOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::TeamDepositsFromInvest(args) => {
                writer.write_all(&TEAM_DEPOSITS_FROM_INVEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.deposited_amount, &mut writer)?;
                Ok(())
            }
            Self::TeamWithdrawsFees(args) => {
                writer.write_all(&TEAM_WITHDRAWS_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_type, &mut writer)?;
                Ok(())
            }
            Self::TeamWithdrawsToInvest(args) => {
                writer.write_all(&TEAM_WITHDRAWS_TO_INVEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.withdrawal_amount, &mut writer)?;
                Ok(())
            }
            Self::TriggerBankCircuitBreaker(args) => {
                writer.write_all(&TRIGGER_BANK_CIRCUIT_BREAKER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.is_halted, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_halted_deposit, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.is_halted_withdrawal,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::TriggerVaultCircuitBreaker(args) => {
                writer.write_all(&TRIGGER_VAULT_CIRCUIT_BREAKER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.is_halted, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_halted_deposit, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.is_halted_withdrawal,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.losses_accepted, &mut writer)?;
                Ok(())
            }
            Self::UpdateOracleState(args) => {
                writer.write_all(&UPDATE_ORACLE_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.oracle_one, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.oracle_two, &mut writer)?;
                Ok(())
            }
            Self::UpdateTrancheConfig(args) => {
                writer.write_all(&UPDATE_TRANCHE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.base_leverage_factor,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.target_percent_staked_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.leverage_slope, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.early_unstake_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.standard_unstake_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.low_stake_threshold_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.low_stake_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.low_stake_fee_enabled,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.target_lockup_duration_secs,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.recovery_yield_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.recovery_threshold_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.bootstrap_recovery_yield_bps,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateYieldingAmount(args) => {
                writer.write_all(&UPDATE_YIELDING_AMOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.push_all_pending_yield,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.yield_to_push, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.new_start_marker_unix_seconds,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.synthetic_yield_push,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateYieldingInfo(args) => {
                writer.write_all(&UPDATE_YIELDING_INFO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_price, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.start_marker_unix_seconds,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateYieldingPriceGen(args) => {
                writer.write_all(&UPDATE_YIELDING_PRICE_GEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_price, &mut writer)?;
                Ok(())
            }
            Self::VaultReallocator(args) => {
                writer.write_all(&VAULT_REALLOCATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.decrement_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.increment_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.synthetic_reallocation,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::VaultTransfer(args) => {
                writer.write_all(&VAULT_TRANSFER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFromLp(args) => {
                writer.write_all(&WITHDRAW_FROM_LP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawTrancheFees(args) => {
                writer.write_all(&WITHDRAW_TRANCHE_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
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
pub const BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct BurnForYieldingGenAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub yielding_user_ta: &'me AccountInfo<'info>,
    pub bank_mint_user_ta: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub fee_team_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub yielding_mint_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BurnForYieldingGenKeys {
    pub user: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub bank_mint: Pubkey,
    pub yielding_user_ta: Pubkey,
    pub bank_mint_user_ta: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub team_state: Pubkey,
    pub fee_team_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub yielding_mint_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<BurnForYieldingGenAccounts<'_, '_>> for BurnForYieldingGenKeys {
    fn from(accounts: BurnForYieldingGenAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            bank_mint: *accounts.bank_mint.key,
            yielding_user_ta: *accounts.yielding_user_ta.key,
            bank_mint_user_ta: *accounts.bank_mint_user_ta.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            team_state: *accounts.team_state.key,
            fee_team_ata: *accounts.fee_team_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            yielding_mint_program: *accounts.yielding_mint_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<BurnForYieldingGenKeys>
for [AccountMeta; BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnForYieldingGenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_user_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint_user_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_team_ata,
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
                pubkey: keys.yielding_mint_program,
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
impl From<[Pubkey; BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN]> for BurnForYieldingGenKeys {
    fn from(pubkeys: [Pubkey; BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            yielding_mint: pubkeys[4],
            bank_mint: pubkeys[5],
            yielding_user_ta: pubkeys[6],
            bank_mint_user_ta: pubkeys[7],
            yielding_vault_ata: pubkeys[8],
            team_state: pubkeys[9],
            fee_team_ata: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
            yielding_mint_program: pubkeys[13],
            associated_token_program: pubkeys[14],
        }
    }
}
impl<'info> From<BurnForYieldingGenAccounts<'_, 'info>>
for [AccountInfo<'info>; BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BurnForYieldingGenAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.bank_mint.clone(),
            accounts.yielding_user_ta.clone(),
            accounts.bank_mint_user_ta.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.team_state.clone(),
            accounts.fee_team_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.yielding_mint_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN]>
for BurnForYieldingGenAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            yielding_mint: &arr[4],
            bank_mint: &arr[5],
            yielding_user_ta: &arr[6],
            bank_mint_user_ta: &arr[7],
            yielding_vault_ata: &arr[8],
            team_state: &arr[9],
            fee_team_ata: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
            yielding_mint_program: &arr[13],
            associated_token_program: &arr[14],
        }
    }
}
pub const BURN_FOR_YIELDING_GEN_IX_DISCM: [u8; 8usize] = [
    167, 22, 56, 95, 212, 15, 185, 218,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BurnForYieldingGenIxArgs {
    pub amount_to_burn: u64,
    pub minimum_yielding_withdrawn: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnForYieldingGenIxData(pub BurnForYieldingGenIxArgs);
impl From<BurnForYieldingGenIxArgs> for BurnForYieldingGenIxData {
    fn from(args: BurnForYieldingGenIxArgs) -> Self {
        Self(args)
    }
}
impl BurnForYieldingGenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BURN_FOR_YIELDING_GEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_to_burn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_yielding_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BurnForYieldingGenIxArgs {
                amount_to_burn,
                minimum_yielding_withdrawn,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BURN_FOR_YIELDING_GEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_to_burn, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.minimum_yielding_withdrawn,
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
pub fn burn_for_yielding_gen_ix_with_program_id(
    program_id: Pubkey,
    keys: BurnForYieldingGenKeys,
    args: BurnForYieldingGenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BurnForYieldingGenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn burn_for_yielding_gen_ix(
    keys: BurnForYieldingGenKeys,
    args: BurnForYieldingGenIxArgs,
) -> std::io::Result<Instruction> {
    burn_for_yielding_gen_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn burn_for_yielding_gen_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BurnForYieldingGenAccounts<'_, '_>,
    args: BurnForYieldingGenIxArgs,
) -> ProgramResult {
    let keys: BurnForYieldingGenKeys = accounts.into();
    let ix = burn_for_yielding_gen_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn burn_for_yielding_gen_invoke(
    accounts: BurnForYieldingGenAccounts<'_, '_>,
    args: BurnForYieldingGenIxArgs,
) -> ProgramResult {
    burn_for_yielding_gen_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn burn_for_yielding_gen_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BurnForYieldingGenAccounts<'_, '_>,
    args: BurnForYieldingGenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnForYieldingGenKeys = accounts.into();
    let ix = burn_for_yielding_gen_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn burn_for_yielding_gen_invoke_signed(
    accounts: BurnForYieldingGenAccounts<'_, '_>,
    args: BurnForYieldingGenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    burn_for_yielding_gen_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn burn_for_yielding_gen_verify_account_keys(
    accounts: BurnForYieldingGenAccounts<'_, '_>,
    keys: BurnForYieldingGenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.yielding_user_ta.key, keys.yielding_user_ta),
        (*accounts.bank_mint_user_ta.key, keys.bank_mint_user_ta),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.fee_team_ata.key, keys.fee_team_ata),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.yielding_mint_program.key, keys.yielding_mint_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn burn_for_yielding_gen_verify_writable_privileges<'me, 'info>(
    accounts: BurnForYieldingGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank_state,
        accounts.vault_state,
        accounts.bank_mint,
        accounts.yielding_user_ta,
        accounts.bank_mint_user_ta,
        accounts.yielding_vault_ata,
        accounts.team_state,
        accounts.fee_team_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_for_yielding_gen_verify_signer_privileges<'me, 'info>(
    accounts: BurnForYieldingGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_for_yielding_gen_verify_account_privileges<'me, 'info>(
    accounts: BurnForYieldingGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_for_yielding_gen_verify_writable_privileges(accounts)?;
    burn_for_yielding_gen_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_FEES_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ChangeFeesAccounts<'me, 'info> {
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub manager: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeFeesKeys {
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub manager: Pubkey,
    pub system_program: Pubkey,
}
impl From<ChangeFeesAccounts<'_, '_>> for ChangeFeesKeys {
    fn from(accounts: ChangeFeesAccounts) -> Self {
        Self {
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            manager: *accounts.manager.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ChangeFeesKeys> for [AccountMeta; CHANGE_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
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
impl From<[Pubkey; CHANGE_FEES_IX_ACCOUNTS_LEN]> for ChangeFeesKeys {
    fn from(pubkeys: [Pubkey; CHANGE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank_state: pubkeys[0],
            vault_state: pubkeys[1],
            manager: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<ChangeFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.manager.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_FEES_IX_ACCOUNTS_LEN]>
for ChangeFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank_state: &arr[0],
            vault_state: &arr[1],
            manager: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CHANGE_FEES_IX_DISCM: [u8; 8usize] = [64, 20, 248, 74, 96, 40, 95, 231];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangeFeesIxArgs {
    pub performance_fee_bps: u16,
    pub minting_fee_bps: u16,
    pub burning_fee_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeFeesIxData(pub ChangeFeesIxArgs);
impl From<ChangeFeesIxArgs> for ChangeFeesIxData {
    fn from(args: ChangeFeesIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let minting_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let burning_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ChangeFeesIxArgs {
                performance_fee_bps,
                minting_fee_bps,
                burning_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.performance_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minting_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.burning_fee_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeFeesKeys,
    args: ChangeFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_fees_ix(
    keys: ChangeFeesKeys,
    args: ChangeFeesIxArgs,
) -> std::io::Result<Instruction> {
    change_fees_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn change_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeFeesAccounts<'_, '_>,
    args: ChangeFeesIxArgs,
) -> ProgramResult {
    let keys: ChangeFeesKeys = accounts.into();
    let ix = change_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_fees_invoke(
    accounts: ChangeFeesAccounts<'_, '_>,
    args: ChangeFeesIxArgs,
) -> ProgramResult {
    change_fees_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn change_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeFeesAccounts<'_, '_>,
    args: ChangeFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeFeesKeys = accounts.into();
    let ix = change_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_fees_invoke_signed(
    accounts: ChangeFeesAccounts<'_, '_>,
    args: ChangeFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_fees_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn change_fees_verify_account_keys(
    accounts: ChangeFeesAccounts<'_, '_>,
    keys: ChangeFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.manager.key, keys.manager),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_fees_verify_writable_privileges<'me, 'info>(
    accounts: ChangeFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank_state,
        accounts.vault_state,
        accounts.manager,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_fees_verify_signer_privileges<'me, 'info>(
    accounts: ChangeFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_fees_verify_account_privileges<'me, 'info>(
    accounts: ChangeFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_fees_verify_writable_privileges(accounts)?;
    change_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_BANK_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateBankAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateBankKeys {
    pub creator: Pubkey,
    pub bank_state: Pubkey,
    pub mint: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<CreateBankAccounts<'_, '_>> for CreateBankKeys {
    fn from(accounts: CreateBankAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            bank_state: *accounts.bank_state.key,
            mint: *accounts.mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CreateBankKeys> for [AccountMeta; CREATE_BANK_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateBankKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
        ]
    }
}
impl From<[Pubkey; CREATE_BANK_IX_ACCOUNTS_LEN]> for CreateBankKeys {
    fn from(pubkeys: [Pubkey; CREATE_BANK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            bank_state: pubkeys[1],
            mint: pubkeys[2],
            system_program: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateBankAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_BANK_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateBankAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.bank_state.clone(),
            accounts.mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_BANK_IX_ACCOUNTS_LEN]>
for CreateBankAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_BANK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            bank_state: &arr[1],
            mint: &arr[2],
            system_program: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const CREATE_BANK_IX_DISCM: [u8; 8usize] = [109, 33, 246, 148, 209, 160, 169, 141];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateBankIxArgs {
    pub bank_index: u8,
    pub max_yielding_tvl: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateBankIxData(pub CreateBankIxArgs);
impl From<CreateBankIxArgs> for CreateBankIxData {
    fn from(args: CreateBankIxArgs) -> Self {
        Self(args)
    }
}
impl CreateBankIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_BANK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_yielding_tvl: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateBankIxArgs {
                bank_index,
                max_yielding_tvl,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_BANK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_yielding_tvl, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_bank_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateBankKeys,
    args: CreateBankIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_BANK_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateBankIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_bank_ix(
    keys: CreateBankKeys,
    args: CreateBankIxArgs,
) -> std::io::Result<Instruction> {
    create_bank_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn create_bank_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateBankAccounts<'_, '_>,
    args: CreateBankIxArgs,
) -> ProgramResult {
    let keys: CreateBankKeys = accounts.into();
    let ix = create_bank_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_bank_invoke(
    accounts: CreateBankAccounts<'_, '_>,
    args: CreateBankIxArgs,
) -> ProgramResult {
    create_bank_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn create_bank_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateBankAccounts<'_, '_>,
    args: CreateBankIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateBankKeys = accounts.into();
    let ix = create_bank_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_bank_invoke_signed(
    accounts: CreateBankAccounts<'_, '_>,
    args: CreateBankIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_bank_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_bank_verify_account_keys(
    accounts: CreateBankAccounts<'_, '_>,
    keys: CreateBankKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.mint.key, keys.mint),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_bank_verify_writable_privileges<'me, 'info>(
    accounts: CreateBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.creator, accounts.bank_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_bank_verify_signer_privileges<'me, 'info>(
    accounts: CreateBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_bank_verify_account_privileges<'me, 'info>(
    accounts: CreateBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_bank_verify_writable_privileges(accounts)?;
    create_bank_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_GEN_ORACLE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateGenOracleAccountAccounts<'me, 'info> {
    pub vault_creator: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateGenOracleAccountKeys {
    pub vault_creator: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub oracle_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateGenOracleAccountAccounts<'_, '_>> for CreateGenOracleAccountKeys {
    fn from(accounts: CreateGenOracleAccountAccounts) -> Self {
        Self {
            vault_creator: *accounts.vault_creator.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            oracle_state: *accounts.oracle_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateGenOracleAccountKeys>
for [AccountMeta; CREATE_GEN_ORACLE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateGenOracleAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault_creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
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
impl From<[Pubkey; CREATE_GEN_ORACLE_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateGenOracleAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_GEN_ORACLE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_creator: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            yielding_mint: pubkeys[3],
            oracle_state: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateGenOracleAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_GEN_ORACLE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateGenOracleAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.vault_creator.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.oracle_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_GEN_ORACLE_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateGenOracleAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_GEN_ORACLE_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault_creator: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            yielding_mint: &arr[3],
            oracle_state: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const CREATE_GEN_ORACLE_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    209, 115, 30, 214, 80, 154, 93, 137,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateGenOracleAccountIxArgs {
    pub contract_init_price: u64,
    pub oracle_one: Pubkey,
    pub oracle_two: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateGenOracleAccountIxData(pub CreateGenOracleAccountIxArgs);
impl From<CreateGenOracleAccountIxArgs> for CreateGenOracleAccountIxData {
    fn from(args: CreateGenOracleAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateGenOracleAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_GEN_ORACLE_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let contract_init_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_one: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_two: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateGenOracleAccountIxArgs {
                contract_init_price,
                oracle_one,
                oracle_two,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_GEN_ORACLE_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.contract_init_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_one, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_two, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_gen_oracle_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateGenOracleAccountKeys,
    args: CreateGenOracleAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_GEN_ORACLE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateGenOracleAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_gen_oracle_account_ix(
    keys: CreateGenOracleAccountKeys,
    args: CreateGenOracleAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_gen_oracle_account_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn create_gen_oracle_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateGenOracleAccountAccounts<'_, '_>,
    args: CreateGenOracleAccountIxArgs,
) -> ProgramResult {
    let keys: CreateGenOracleAccountKeys = accounts.into();
    let ix = create_gen_oracle_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_gen_oracle_account_invoke(
    accounts: CreateGenOracleAccountAccounts<'_, '_>,
    args: CreateGenOracleAccountIxArgs,
) -> ProgramResult {
    create_gen_oracle_account_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_gen_oracle_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateGenOracleAccountAccounts<'_, '_>,
    args: CreateGenOracleAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateGenOracleAccountKeys = accounts.into();
    let ix = create_gen_oracle_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_gen_oracle_account_invoke_signed(
    accounts: CreateGenOracleAccountAccounts<'_, '_>,
    args: CreateGenOracleAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_gen_oracle_account_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_gen_oracle_account_verify_account_keys(
    accounts: CreateGenOracleAccountAccounts<'_, '_>,
    keys: CreateGenOracleAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault_creator.key, keys.vault_creator),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_gen_oracle_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateGenOracleAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_creator,
        accounts.vault_state,
        accounts.oracle_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_gen_oracle_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateGenOracleAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vault_creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_gen_oracle_account_verify_account_privileges<'me, 'info>(
    accounts: CreateGenOracleAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_gen_oracle_account_verify_writable_privileges(accounts)?;
    create_gen_oracle_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_GEN_TEAM_ACCOUNT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateGenTeamAccountAccounts<'me, 'info> {
    pub vault_creator: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub fee_team_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateGenTeamAccountKeys {
    pub vault_creator: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub team_state: Pubkey,
    pub fee_team_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<CreateGenTeamAccountAccounts<'_, '_>> for CreateGenTeamAccountKeys {
    fn from(accounts: CreateGenTeamAccountAccounts) -> Self {
        Self {
            vault_creator: *accounts.vault_creator.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            team_state: *accounts.team_state.key,
            fee_team_ata: *accounts.fee_team_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<CreateGenTeamAccountKeys>
for [AccountMeta; CREATE_GEN_TEAM_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateGenTeamAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault_creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_team_ata,
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
impl From<[Pubkey; CREATE_GEN_TEAM_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateGenTeamAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_GEN_TEAM_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_creator: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            yielding_mint: pubkeys[3],
            team_state: pubkeys[4],
            fee_team_ata: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
        }
    }
}
impl<'info> From<CreateGenTeamAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_GEN_TEAM_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateGenTeamAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.vault_creator.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.team_state.clone(),
            accounts.fee_team_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_GEN_TEAM_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateGenTeamAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_GEN_TEAM_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault_creator: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            yielding_mint: &arr[3],
            team_state: &arr[4],
            fee_team_ata: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
        }
    }
}
pub const CREATE_GEN_TEAM_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    19, 232, 148, 22, 76, 27, 199, 241,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateGenTeamAccountIxArgs {
    pub yield_manager: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateGenTeamAccountIxData(pub CreateGenTeamAccountIxArgs);
impl From<CreateGenTeamAccountIxArgs> for CreateGenTeamAccountIxData {
    fn from(args: CreateGenTeamAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateGenTeamAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_GEN_TEAM_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let yield_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateGenTeamAccountIxArgs {
                yield_manager,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_GEN_TEAM_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.yield_manager, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_gen_team_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateGenTeamAccountKeys,
    args: CreateGenTeamAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_GEN_TEAM_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateGenTeamAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_gen_team_account_ix(
    keys: CreateGenTeamAccountKeys,
    args: CreateGenTeamAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_gen_team_account_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn create_gen_team_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateGenTeamAccountAccounts<'_, '_>,
    args: CreateGenTeamAccountIxArgs,
) -> ProgramResult {
    let keys: CreateGenTeamAccountKeys = accounts.into();
    let ix = create_gen_team_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_gen_team_account_invoke(
    accounts: CreateGenTeamAccountAccounts<'_, '_>,
    args: CreateGenTeamAccountIxArgs,
) -> ProgramResult {
    create_gen_team_account_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn create_gen_team_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateGenTeamAccountAccounts<'_, '_>,
    args: CreateGenTeamAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateGenTeamAccountKeys = accounts.into();
    let ix = create_gen_team_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_gen_team_account_invoke_signed(
    accounts: CreateGenTeamAccountAccounts<'_, '_>,
    args: CreateGenTeamAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_gen_team_account_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_gen_team_account_verify_account_keys(
    accounts: CreateGenTeamAccountAccounts<'_, '_>,
    keys: CreateGenTeamAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault_creator.key, keys.vault_creator),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.fee_team_ata.key, keys.fee_team_ata),
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
pub fn create_gen_team_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateGenTeamAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_creator,
        accounts.vault_state,
        accounts.team_state,
        accounts.fee_team_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_gen_team_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateGenTeamAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vault_creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_gen_team_account_verify_account_privileges<'me, 'info>(
    accounts: CreateGenTeamAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_gen_team_account_verify_writable_privileges(accounts)?;
    create_gen_team_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_GEN_VAULT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateGenVaultAccounts<'me, 'info> {
    pub vault_creator: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateGenVaultKeys {
    pub vault_creator: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<CreateGenVaultAccounts<'_, '_>> for CreateGenVaultKeys {
    fn from(accounts: CreateGenVaultAccounts) -> Self {
        Self {
            vault_creator: *accounts.vault_creator.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<CreateGenVaultKeys> for [AccountMeta; CREATE_GEN_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateGenVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault_creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
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
impl From<[Pubkey; CREATE_GEN_VAULT_IX_ACCOUNTS_LEN]> for CreateGenVaultKeys {
    fn from(pubkeys: [Pubkey; CREATE_GEN_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_creator: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            yielding_mint: pubkeys[3],
            yielding_vault_ata: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
        }
    }
}
impl<'info> From<CreateGenVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_GEN_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateGenVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.vault_creator.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_GEN_VAULT_IX_ACCOUNTS_LEN]>
for CreateGenVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_GEN_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_creator: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            yielding_mint: &arr[3],
            yielding_vault_ata: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
        }
    }
}
pub const CREATE_GEN_VAULT_IX_DISCM: [u8; 8usize] = [
    20, 31, 141, 116, 117, 123, 227, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateGenVaultIxArgs {
    pub vault_type: VaultType,
    pub yielding_token_index: u8,
    pub mint_fee_bps: u16,
    pub burn_fee_bps: u16,
    pub performance_fee_bps: u16,
    pub risk_manager: Option<Pubkey>,
    pub lending_platform: Option<LendingPlatform>,
    pub lending_type: Option<LendingType>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateGenVaultIxData(pub CreateGenVaultIxArgs);
impl From<CreateGenVaultIxArgs> for CreateGenVaultIxData {
    fn from(args: CreateGenVaultIxArgs) -> Self {
        Self(args)
    }
}
impl CreateGenVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_GEN_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let vault_type: VaultType = crate::borsh_de_or_default(&mut reader)?;
        let yielding_token_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let burn_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let risk_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let lending_platform: Option<LendingPlatform> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lending_type: Option<LendingType> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateGenVaultIxArgs {
                vault_type,
                yielding_token_index,
                mint_fee_bps,
                burn_fee_bps,
                performance_fee_bps,
                risk_manager,
                lending_platform,
                lending_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_GEN_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.vault_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.yielding_token_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.mint_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.burn_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.performance_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.risk_manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lending_platform, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lending_type, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_gen_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateGenVaultKeys,
    args: CreateGenVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_GEN_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateGenVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_gen_vault_ix(
    keys: CreateGenVaultKeys,
    args: CreateGenVaultIxArgs,
) -> std::io::Result<Instruction> {
    create_gen_vault_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn create_gen_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateGenVaultAccounts<'_, '_>,
    args: CreateGenVaultIxArgs,
) -> ProgramResult {
    let keys: CreateGenVaultKeys = accounts.into();
    let ix = create_gen_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_gen_vault_invoke(
    accounts: CreateGenVaultAccounts<'_, '_>,
    args: CreateGenVaultIxArgs,
) -> ProgramResult {
    create_gen_vault_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn create_gen_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateGenVaultAccounts<'_, '_>,
    args: CreateGenVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateGenVaultKeys = accounts.into();
    let ix = create_gen_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_gen_vault_invoke_signed(
    accounts: CreateGenVaultAccounts<'_, '_>,
    args: CreateGenVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_gen_vault_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_gen_vault_verify_account_keys(
    accounts: CreateGenVaultAccounts<'_, '_>,
    keys: CreateGenVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault_creator.key, keys.vault_creator),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
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
pub fn create_gen_vault_verify_writable_privileges<'me, 'info>(
    accounts: CreateGenVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_creator,
        accounts.bank_state,
        accounts.vault_state,
        accounts.yielding_vault_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_gen_vault_verify_signer_privileges<'me, 'info>(
    accounts: CreateGenVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vault_creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_gen_vault_verify_account_privileges<'me, 'info>(
    accounts: CreateGenVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_gen_vault_verify_writable_privileges(accounts)?;
    create_gen_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CreateTrancheStateAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub junior_mint: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub locked_junior_shares_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTrancheStateKeys {
    pub creator: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_state: Pubkey,
    pub junior_mint: Pubkey,
    pub bank_mint: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub locked_junior_shares_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<CreateTrancheStateAccounts<'_, '_>> for CreateTrancheStateKeys {
    fn from(accounts: CreateTrancheStateAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            tranche_state: *accounts.tranche_state.key,
            bank_state: *accounts.bank_state.key,
            junior_mint: *accounts.junior_mint.key,
            bank_mint: *accounts.bank_mint.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            locked_junior_shares_ata: *accounts.locked_junior_shares_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<CreateTrancheStateKeys>
for [AccountMeta; CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTrancheStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.junior_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.junior_escrow_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_junior_shares_ata,
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
impl From<[Pubkey; CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN]> for CreateTrancheStateKeys {
    fn from(pubkeys: [Pubkey; CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            tranche_state: pubkeys[1],
            bank_state: pubkeys[2],
            junior_mint: pubkeys[3],
            bank_mint: pubkeys[4],
            junior_escrow_ata: pubkeys[5],
            locked_junior_shares_ata: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<CreateTrancheStateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTrancheStateAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_state.clone(),
            accounts.junior_mint.clone(),
            accounts.bank_mint.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.locked_junior_shares_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN]>
for CreateTrancheStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            creator: &arr[0],
            tranche_state: &arr[1],
            bank_state: &arr[2],
            junior_mint: &arr[3],
            bank_mint: &arr[4],
            junior_escrow_ata: &arr[5],
            locked_junior_shares_ata: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const CREATE_TRANCHE_STATE_IX_DISCM: [u8; 8usize] = [
    204, 109, 246, 152, 92, 52, 43, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTrancheStateIxArgs {
    pub base_leverage_factor: u16,
    pub target_percent_staked_bps: u16,
    pub leverage_slope: u16,
    pub early_unstake_fee_bps: u16,
    pub standard_unstake_fee_bps: u16,
    pub low_stake_threshold_bps: u16,
    pub low_stake_fee_bps: u16,
    pub target_lockup_duration_secs: i64,
    pub recovery_yield_bps: u16,
    pub recovery_threshold_bps: u16,
    pub bootstrap_recovery_yield_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTrancheStateIxData(pub CreateTrancheStateIxArgs);
impl From<CreateTrancheStateIxArgs> for CreateTrancheStateIxData {
    fn from(args: CreateTrancheStateIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTrancheStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TRANCHE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_leverage_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let target_percent_staked_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let leverage_slope: u16 = crate::borsh_de_or_default(&mut reader)?;
        let early_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let standard_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let low_stake_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let low_stake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let target_lockup_duration_secs: i64 = crate::borsh_de_or_default(&mut reader)?;
        let recovery_yield_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let recovery_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bootstrap_recovery_yield_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateTrancheStateIxArgs {
                base_leverage_factor,
                target_percent_staked_bps,
                leverage_slope,
                early_unstake_fee_bps,
                standard_unstake_fee_bps,
                low_stake_threshold_bps,
                low_stake_fee_bps,
                target_lockup_duration_secs,
                recovery_yield_bps,
                recovery_threshold_bps,
                bootstrap_recovery_yield_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TRANCHE_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_leverage_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.target_percent_staked_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.leverage_slope, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.early_unstake_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.standard_unstake_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.low_stake_threshold_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.low_stake_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.target_lockup_duration_secs,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.recovery_yield_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.recovery_threshold_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.bootstrap_recovery_yield_bps,
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
pub fn create_tranche_state_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTrancheStateKeys,
    args: CreateTrancheStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTrancheStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_tranche_state_ix(
    keys: CreateTrancheStateKeys,
    args: CreateTrancheStateIxArgs,
) -> std::io::Result<Instruction> {
    create_tranche_state_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn create_tranche_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTrancheStateAccounts<'_, '_>,
    args: CreateTrancheStateIxArgs,
) -> ProgramResult {
    let keys: CreateTrancheStateKeys = accounts.into();
    let ix = create_tranche_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_tranche_state_invoke(
    accounts: CreateTrancheStateAccounts<'_, '_>,
    args: CreateTrancheStateIxArgs,
) -> ProgramResult {
    create_tranche_state_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn create_tranche_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTrancheStateAccounts<'_, '_>,
    args: CreateTrancheStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTrancheStateKeys = accounts.into();
    let ix = create_tranche_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_tranche_state_invoke_signed(
    accounts: CreateTrancheStateAccounts<'_, '_>,
    args: CreateTrancheStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_tranche_state_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_tranche_state_verify_account_keys(
    accounts: CreateTrancheStateAccounts<'_, '_>,
    keys: CreateTrancheStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.junior_mint.key, keys.junior_mint),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.locked_junior_shares_ata.key, keys.locked_junior_shares_ata),
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
pub fn create_tranche_state_verify_writable_privileges<'me, 'info>(
    accounts: CreateTrancheStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.tranche_state,
        accounts.junior_escrow_ata,
        accounts.locked_junior_shares_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_tranche_state_verify_signer_privileges<'me, 'info>(
    accounts: CreateTrancheStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_tranche_state_verify_account_privileges<'me, 'info>(
    accounts: CreateTrancheStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_tranche_state_verify_writable_privileges(accounts)?;
    create_tranche_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IN_LP_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DepositInLpAccounts<'me, 'info> {
    pub yield_manager: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositInLpKeys {
    pub yield_manager: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub team_state: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositInLpAccounts<'_, '_>> for DepositInLpKeys {
    fn from(accounts: DepositInLpAccounts) -> Self {
        Self {
            yield_manager: *accounts.yield_manager.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            team_state: *accounts.team_state.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositInLpKeys> for [AccountMeta; DEPOSIT_IN_LP_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositInLpKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.yield_manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
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
impl From<[Pubkey; DEPOSIT_IN_LP_IX_ACCOUNTS_LEN]> for DepositInLpKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IN_LP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            yield_manager: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            team_state: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<DepositInLpAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IN_LP_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositInLpAccounts<'_, 'info>) -> Self {
        [
            accounts.yield_manager.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.team_state.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IN_LP_IX_ACCOUNTS_LEN]>
for DepositInLpAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IN_LP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            yield_manager: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            team_state: &arr[4],
            yielding_vault_ata: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const DEPOSIT_IN_LP_IX_DISCM: [u8; 8usize] = [149, 73, 5, 160, 40, 98, 66, 112];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositInLpIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositInLpIxData(pub DepositInLpIxArgs);
impl From<DepositInLpIxArgs> for DepositInLpIxData {
    fn from(args: DepositInLpIxArgs) -> Self {
        Self(args)
    }
}
impl DepositInLpIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_IN_LP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DepositInLpIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IN_LP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_in_lp_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositInLpKeys,
    args: DepositInLpIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_IN_LP_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositInLpIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_in_lp_ix(
    keys: DepositInLpKeys,
    args: DepositInLpIxArgs,
) -> std::io::Result<Instruction> {
    deposit_in_lp_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn deposit_in_lp_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositInLpAccounts<'_, '_>,
    args: DepositInLpIxArgs,
) -> ProgramResult {
    let keys: DepositInLpKeys = accounts.into();
    let ix = deposit_in_lp_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_in_lp_invoke(
    accounts: DepositInLpAccounts<'_, '_>,
    args: DepositInLpIxArgs,
) -> ProgramResult {
    deposit_in_lp_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn deposit_in_lp_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositInLpAccounts<'_, '_>,
    args: DepositInLpIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositInLpKeys = accounts.into();
    let ix = deposit_in_lp_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_in_lp_invoke_signed(
    accounts: DepositInLpAccounts<'_, '_>,
    args: DepositInLpIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_in_lp_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_in_lp_verify_account_keys(
    accounts: DepositInLpAccounts<'_, '_>,
    keys: DepositInLpKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.yield_manager.key, keys.yield_manager),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_in_lp_verify_writable_privileges<'me, 'info>(
    accounts: DepositInLpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.yield_manager,
        accounts.bank_state,
        accounts.vault_state,
        accounts.oracle_state,
        accounts.team_state,
        accounts.yielding_vault_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_in_lp_verify_signer_privileges<'me, 'info>(
    accounts: DepositInLpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.yield_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_in_lp_verify_account_privileges<'me, 'info>(
    accounts: DepositInLpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_in_lp_verify_writable_privileges(accounts)?;
    deposit_in_lp_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FULFILL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct FulfillUnstakeJuniorAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub queue_payer: &'me AccountInfo<'info>,
    pub withdrawal_queue: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub junior_mint: &'me AccountInfo<'info>,
    pub owner_bank_mint_ata: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub locked_junior_shares_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub vault_oracle_state: &'me AccountInfo<'info>,
    pub vault_team_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub owner_yielding_ta: &'me AccountInfo<'info>,
    pub vault_fee_team_ata: &'me AccountInfo<'info>,
    pub yielding_mint_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FulfillUnstakeJuniorKeys {
    pub signer: Pubkey,
    pub queue_payer: Pubkey,
    pub withdrawal_queue: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub junior_mint: Pubkey,
    pub owner_bank_mint_ata: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub locked_junior_shares_ata: Pubkey,
    pub token_program: Pubkey,
    pub vault_state: Pubkey,
    pub vault_oracle_state: Pubkey,
    pub vault_team_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub owner_yielding_ta: Pubkey,
    pub vault_fee_team_ata: Pubkey,
    pub yielding_mint_program: Pubkey,
}
impl From<FulfillUnstakeJuniorAccounts<'_, '_>> for FulfillUnstakeJuniorKeys {
    fn from(accounts: FulfillUnstakeJuniorAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            queue_payer: *accounts.queue_payer.key,
            withdrawal_queue: *accounts.withdrawal_queue.key,
            tranche_state: *accounts.tranche_state.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            junior_mint: *accounts.junior_mint.key,
            owner_bank_mint_ata: *accounts.owner_bank_mint_ata.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            locked_junior_shares_ata: *accounts.locked_junior_shares_ata.key,
            token_program: *accounts.token_program.key,
            vault_state: *accounts.vault_state.key,
            vault_oracle_state: *accounts.vault_oracle_state.key,
            vault_team_state: *accounts.vault_team_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            owner_yielding_ta: *accounts.owner_yielding_ta.key,
            vault_fee_team_ata: *accounts.vault_fee_team_ata.key,
            yielding_mint_program: *accounts.yielding_mint_program.key,
        }
    }
}
impl From<FulfillUnstakeJuniorKeys>
for [AccountMeta; FULFILL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(keys: FulfillUnstakeJuniorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.queue_payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdrawal_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.junior_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_bank_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.junior_escrow_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_junior_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_yielding_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_fee_team_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FULFILL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]>
for FulfillUnstakeJuniorKeys {
    fn from(pubkeys: [Pubkey; FULFILL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            queue_payer: pubkeys[1],
            withdrawal_queue: pubkeys[2],
            tranche_state: pubkeys[3],
            bank_state: pubkeys[4],
            bank_mint: pubkeys[5],
            junior_mint: pubkeys[6],
            owner_bank_mint_ata: pubkeys[7],
            junior_escrow_ata: pubkeys[8],
            locked_junior_shares_ata: pubkeys[9],
            token_program: pubkeys[10],
            vault_state: pubkeys[11],
            vault_oracle_state: pubkeys[12],
            vault_team_state: pubkeys[13],
            yielding_mint: pubkeys[14],
            yielding_vault_ata: pubkeys[15],
            owner_yielding_ta: pubkeys[16],
            vault_fee_team_ata: pubkeys[17],
            yielding_mint_program: pubkeys[18],
        }
    }
}
impl<'info> From<FulfillUnstakeJuniorAccounts<'_, 'info>>
for [AccountInfo<'info>; FULFILL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: FulfillUnstakeJuniorAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.queue_payer.clone(),
            accounts.withdrawal_queue.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.junior_mint.clone(),
            accounts.owner_bank_mint_ata.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.locked_junior_shares_ata.clone(),
            accounts.token_program.clone(),
            accounts.vault_state.clone(),
            accounts.vault_oracle_state.clone(),
            accounts.vault_team_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.owner_yielding_ta.clone(),
            accounts.vault_fee_team_ata.clone(),
            accounts.yielding_mint_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FULFILL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]>
for FulfillUnstakeJuniorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FULFILL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            queue_payer: &arr[1],
            withdrawal_queue: &arr[2],
            tranche_state: &arr[3],
            bank_state: &arr[4],
            bank_mint: &arr[5],
            junior_mint: &arr[6],
            owner_bank_mint_ata: &arr[7],
            junior_escrow_ata: &arr[8],
            locked_junior_shares_ata: &arr[9],
            token_program: &arr[10],
            vault_state: &arr[11],
            vault_oracle_state: &arr[12],
            vault_team_state: &arr[13],
            yielding_mint: &arr[14],
            yielding_vault_ata: &arr[15],
            owner_yielding_ta: &arr[16],
            vault_fee_team_ata: &arr[17],
            yielding_mint_program: &arr[18],
        }
    }
}
pub const FULFILL_UNSTAKE_JUNIOR_IX_DISCM: [u8; 8usize] = [
    251, 30, 101, 10, 80, 9, 11, 164,
];
#[derive(Clone, Debug, PartialEq)]
pub struct FulfillUnstakeJuniorIxData;
impl FulfillUnstakeJuniorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FULFILL_UNSTAKE_JUNIOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FULFILL_UNSTAKE_JUNIOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fulfill_unstake_junior_ix_with_program_id(
    program_id: Pubkey,
    keys: FulfillUnstakeJuniorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FULFILL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: FulfillUnstakeJuniorIxData.try_to_vec()?,
    })
}
pub fn fulfill_unstake_junior_ix(
    keys: FulfillUnstakeJuniorKeys,
) -> std::io::Result<Instruction> {
    fulfill_unstake_junior_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn fulfill_unstake_junior_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FulfillUnstakeJuniorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: FulfillUnstakeJuniorKeys = accounts.into();
    let ix = fulfill_unstake_junior_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn fulfill_unstake_junior_invoke(
    accounts: FulfillUnstakeJuniorAccounts<'_, '_>,
) -> ProgramResult {
    fulfill_unstake_junior_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn fulfill_unstake_junior_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FulfillUnstakeJuniorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FulfillUnstakeJuniorKeys = accounts.into();
    let ix = fulfill_unstake_junior_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fulfill_unstake_junior_invoke_signed(
    accounts: FulfillUnstakeJuniorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fulfill_unstake_junior_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn fulfill_unstake_junior_verify_account_keys(
    accounts: FulfillUnstakeJuniorAccounts<'_, '_>,
    keys: FulfillUnstakeJuniorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.queue_payer.key, keys.queue_payer),
        (*accounts.withdrawal_queue.key, keys.withdrawal_queue),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.junior_mint.key, keys.junior_mint),
        (*accounts.owner_bank_mint_ata.key, keys.owner_bank_mint_ata),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.locked_junior_shares_ata.key, keys.locked_junior_shares_ata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.vault_oracle_state.key, keys.vault_oracle_state),
        (*accounts.vault_team_state.key, keys.vault_team_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.owner_yielding_ta.key, keys.owner_yielding_ta),
        (*accounts.vault_fee_team_ata.key, keys.vault_fee_team_ata),
        (*accounts.yielding_mint_program.key, keys.yielding_mint_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fulfill_unstake_junior_verify_writable_privileges<'me, 'info>(
    accounts: FulfillUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.queue_payer,
        accounts.withdrawal_queue,
        accounts.tranche_state,
        accounts.bank_state,
        accounts.bank_mint,
        accounts.junior_mint,
        accounts.owner_bank_mint_ata,
        accounts.junior_escrow_ata,
        accounts.locked_junior_shares_ata,
        accounts.vault_state,
        accounts.vault_team_state,
        accounts.yielding_vault_ata,
        accounts.owner_yielding_ta,
        accounts.vault_fee_team_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fulfill_unstake_junior_verify_signer_privileges<'me, 'info>(
    accounts: FulfillUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fulfill_unstake_junior_verify_account_privileges<'me, 'info>(
    accounts: FulfillUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fulfill_unstake_junior_verify_writable_privileges(accounts)?;
    fulfill_unstake_junior_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_MARGINFI_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitMarginfiAccountAccounts<'me, 'info> {
    pub yield_manager: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitMarginfiAccountKeys {
    pub yield_manager: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub team_state: Pubkey,
}
impl From<InitMarginfiAccountAccounts<'_, '_>> for InitMarginfiAccountKeys {
    fn from(accounts: InitMarginfiAccountAccounts) -> Self {
        Self {
            yield_manager: *accounts.yield_manager.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            team_state: *accounts.team_state.key,
        }
    }
}
impl From<InitMarginfiAccountKeys>
for [AccountMeta; INIT_MARGINFI_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitMarginfiAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.yield_manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_MARGINFI_ACCOUNT_IX_ACCOUNTS_LEN]> for InitMarginfiAccountKeys {
    fn from(pubkeys: [Pubkey; INIT_MARGINFI_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            yield_manager: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            team_state: pubkeys[3],
        }
    }
}
impl<'info> From<InitMarginfiAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_MARGINFI_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitMarginfiAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.yield_manager.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.team_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_MARGINFI_ACCOUNT_IX_ACCOUNTS_LEN]>
for InitMarginfiAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_MARGINFI_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            yield_manager: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            team_state: &arr[3],
        }
    }
}
pub const INIT_MARGINFI_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    225, 212, 68, 233, 62, 96, 242, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitMarginfiAccountIxArgs {
    pub account_index: u16,
    pub third_party_id: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitMarginfiAccountIxData(pub InitMarginfiAccountIxArgs);
impl From<InitMarginfiAccountIxArgs> for InitMarginfiAccountIxData {
    fn from(args: InitMarginfiAccountIxArgs) -> Self {
        Self(args)
    }
}
impl InitMarginfiAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_MARGINFI_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let third_party_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitMarginfiAccountIxArgs {
                account_index,
                third_party_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_MARGINFI_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.account_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.third_party_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_marginfi_account_ix_with_program_id(
    program_id: Pubkey,
    keys: InitMarginfiAccountKeys,
    args: InitMarginfiAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_MARGINFI_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitMarginfiAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_marginfi_account_ix(
    keys: InitMarginfiAccountKeys,
    args: InitMarginfiAccountIxArgs,
) -> std::io::Result<Instruction> {
    init_marginfi_account_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn init_marginfi_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitMarginfiAccountAccounts<'_, '_>,
    args: InitMarginfiAccountIxArgs,
) -> ProgramResult {
    let keys: InitMarginfiAccountKeys = accounts.into();
    let ix = init_marginfi_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_marginfi_account_invoke(
    accounts: InitMarginfiAccountAccounts<'_, '_>,
    args: InitMarginfiAccountIxArgs,
) -> ProgramResult {
    init_marginfi_account_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn init_marginfi_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitMarginfiAccountAccounts<'_, '_>,
    args: InitMarginfiAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitMarginfiAccountKeys = accounts.into();
    let ix = init_marginfi_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_marginfi_account_invoke_signed(
    accounts: InitMarginfiAccountAccounts<'_, '_>,
    args: InitMarginfiAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_marginfi_account_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_marginfi_account_verify_account_keys(
    accounts: InitMarginfiAccountAccounts<'_, '_>,
    keys: InitMarginfiAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.yield_manager.key, keys.yield_manager),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.team_state.key, keys.team_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_marginfi_account_verify_writable_privileges<'me, 'info>(
    accounts: InitMarginfiAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.yield_manager, accounts.vault_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_marginfi_account_verify_signer_privileges<'me, 'info>(
    accounts: InitMarginfiAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.yield_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_marginfi_account_verify_account_privileges<'me, 'info>(
    accounts: InitMarginfiAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_marginfi_account_verify_writable_privileges(accounts)?;
    init_marginfi_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct InstantUnstakeJuniorAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub target_mint: &'me AccountInfo<'info>,
    pub junior_mint: &'me AccountInfo<'info>,
    pub user_junior_mint_ata: &'me AccountInfo<'info>,
    pub owner_bank_mint_ata: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub locked_junior_shares_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub vault_oracle_state: &'me AccountInfo<'info>,
    pub vault_team_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub owner_yielding_ta: &'me AccountInfo<'info>,
    pub vault_fee_team_ata: &'me AccountInfo<'info>,
    pub yielding_mint_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantUnstakeJuniorKeys {
    pub user: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub target_mint: Pubkey,
    pub junior_mint: Pubkey,
    pub user_junior_mint_ata: Pubkey,
    pub owner_bank_mint_ata: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub locked_junior_shares_ata: Pubkey,
    pub token_program: Pubkey,
    pub vault_state: Pubkey,
    pub vault_oracle_state: Pubkey,
    pub vault_team_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub owner_yielding_ta: Pubkey,
    pub vault_fee_team_ata: Pubkey,
    pub yielding_mint_program: Pubkey,
}
impl From<InstantUnstakeJuniorAccounts<'_, '_>> for InstantUnstakeJuniorKeys {
    fn from(accounts: InstantUnstakeJuniorAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            tranche_state: *accounts.tranche_state.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            target_mint: *accounts.target_mint.key,
            junior_mint: *accounts.junior_mint.key,
            user_junior_mint_ata: *accounts.user_junior_mint_ata.key,
            owner_bank_mint_ata: *accounts.owner_bank_mint_ata.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            locked_junior_shares_ata: *accounts.locked_junior_shares_ata.key,
            token_program: *accounts.token_program.key,
            vault_state: *accounts.vault_state.key,
            vault_oracle_state: *accounts.vault_oracle_state.key,
            vault_team_state: *accounts.vault_team_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            owner_yielding_ta: *accounts.owner_yielding_ta.key,
            vault_fee_team_ata: *accounts.vault_fee_team_ata.key,
            yielding_mint_program: *accounts.yielding_mint_program.key,
        }
    }
}
impl From<InstantUnstakeJuniorKeys>
for [AccountMeta; INSTANT_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantUnstakeJuniorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.target_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.junior_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_junior_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_bank_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.junior_escrow_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_junior_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_yielding_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_fee_team_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]>
for InstantUnstakeJuniorKeys {
    fn from(pubkeys: [Pubkey; INSTANT_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            tranche_state: pubkeys[1],
            bank_state: pubkeys[2],
            bank_mint: pubkeys[3],
            target_mint: pubkeys[4],
            junior_mint: pubkeys[5],
            user_junior_mint_ata: pubkeys[6],
            owner_bank_mint_ata: pubkeys[7],
            junior_escrow_ata: pubkeys[8],
            locked_junior_shares_ata: pubkeys[9],
            token_program: pubkeys[10],
            vault_state: pubkeys[11],
            vault_oracle_state: pubkeys[12],
            vault_team_state: pubkeys[13],
            yielding_mint: pubkeys[14],
            yielding_vault_ata: pubkeys[15],
            owner_yielding_ta: pubkeys[16],
            vault_fee_team_ata: pubkeys[17],
            yielding_mint_program: pubkeys[18],
        }
    }
}
impl<'info> From<InstantUnstakeJuniorAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantUnstakeJuniorAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.target_mint.clone(),
            accounts.junior_mint.clone(),
            accounts.user_junior_mint_ata.clone(),
            accounts.owner_bank_mint_ata.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.locked_junior_shares_ata.clone(),
            accounts.token_program.clone(),
            accounts.vault_state.clone(),
            accounts.vault_oracle_state.clone(),
            accounts.vault_team_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.owner_yielding_ta.clone(),
            accounts.vault_fee_team_ata.clone(),
            accounts.yielding_mint_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INSTANT_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]>
for InstantUnstakeJuniorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            tranche_state: &arr[1],
            bank_state: &arr[2],
            bank_mint: &arr[3],
            target_mint: &arr[4],
            junior_mint: &arr[5],
            user_junior_mint_ata: &arr[6],
            owner_bank_mint_ata: &arr[7],
            junior_escrow_ata: &arr[8],
            locked_junior_shares_ata: &arr[9],
            token_program: &arr[10],
            vault_state: &arr[11],
            vault_oracle_state: &arr[12],
            vault_team_state: &arr[13],
            yielding_mint: &arr[14],
            yielding_vault_ata: &arr[15],
            owner_yielding_ta: &arr[16],
            vault_fee_team_ata: &arr[17],
            yielding_mint_program: &arr[18],
        }
    }
}
pub const INSTANT_UNSTAKE_JUNIOR_IX_DISCM: [u8; 8usize] = [
    187, 52, 186, 3, 166, 82, 14, 121,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantUnstakeJuniorIxArgs {
    pub shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantUnstakeJuniorIxData(pub InstantUnstakeJuniorIxArgs);
impl From<InstantUnstakeJuniorIxArgs> for InstantUnstakeJuniorIxData {
    fn from(args: InstantUnstakeJuniorIxArgs) -> Self {
        Self(args)
    }
}
impl InstantUnstakeJuniorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_UNSTAKE_JUNIOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InstantUnstakeJuniorIxArgs {
                shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_UNSTAKE_JUNIOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_unstake_junior_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantUnstakeJuniorKeys,
    args: InstantUnstakeJuniorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantUnstakeJuniorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_unstake_junior_ix(
    keys: InstantUnstakeJuniorKeys,
    args: InstantUnstakeJuniorIxArgs,
) -> std::io::Result<Instruction> {
    instant_unstake_junior_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn instant_unstake_junior_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantUnstakeJuniorAccounts<'_, '_>,
    args: InstantUnstakeJuniorIxArgs,
) -> ProgramResult {
    let keys: InstantUnstakeJuniorKeys = accounts.into();
    let ix = instant_unstake_junior_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_unstake_junior_invoke(
    accounts: InstantUnstakeJuniorAccounts<'_, '_>,
    args: InstantUnstakeJuniorIxArgs,
) -> ProgramResult {
    instant_unstake_junior_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn instant_unstake_junior_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantUnstakeJuniorAccounts<'_, '_>,
    args: InstantUnstakeJuniorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantUnstakeJuniorKeys = accounts.into();
    let ix = instant_unstake_junior_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_unstake_junior_invoke_signed(
    accounts: InstantUnstakeJuniorAccounts<'_, '_>,
    args: InstantUnstakeJuniorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_unstake_junior_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_unstake_junior_verify_account_keys(
    accounts: InstantUnstakeJuniorAccounts<'_, '_>,
    keys: InstantUnstakeJuniorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.target_mint.key, keys.target_mint),
        (*accounts.junior_mint.key, keys.junior_mint),
        (*accounts.user_junior_mint_ata.key, keys.user_junior_mint_ata),
        (*accounts.owner_bank_mint_ata.key, keys.owner_bank_mint_ata),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.locked_junior_shares_ata.key, keys.locked_junior_shares_ata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.vault_oracle_state.key, keys.vault_oracle_state),
        (*accounts.vault_team_state.key, keys.vault_team_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.owner_yielding_ta.key, keys.owner_yielding_ta),
        (*accounts.vault_fee_team_ata.key, keys.vault_fee_team_ata),
        (*accounts.yielding_mint_program.key, keys.yielding_mint_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_unstake_junior_verify_writable_privileges<'me, 'info>(
    accounts: InstantUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.tranche_state,
        accounts.bank_state,
        accounts.bank_mint,
        accounts.junior_mint,
        accounts.user_junior_mint_ata,
        accounts.owner_bank_mint_ata,
        accounts.junior_escrow_ata,
        accounts.locked_junior_shares_ata,
        accounts.vault_state,
        accounts.vault_team_state,
        accounts.yielding_vault_ata,
        accounts.owner_yielding_ta,
        accounts.vault_fee_team_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_unstake_junior_verify_signer_privileges<'me, 'info>(
    accounts: InstantUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_unstake_junior_verify_account_privileges<'me, 'info>(
    accounts: InstantUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_unstake_junior_verify_writable_privileges(accounts)?;
    instant_unstake_junior_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_W_YIELDING_GEN_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct MintWYieldingGenAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub yielding_user_ta: &'me AccountInfo<'info>,
    pub bank_mint_user_ta: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub fee_team_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub yielding_mint_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintWYieldingGenKeys {
    pub user: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub bank_mint: Pubkey,
    pub yielding_user_ta: Pubkey,
    pub bank_mint_user_ta: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub team_state: Pubkey,
    pub fee_team_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub yielding_mint_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<MintWYieldingGenAccounts<'_, '_>> for MintWYieldingGenKeys {
    fn from(accounts: MintWYieldingGenAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            bank_mint: *accounts.bank_mint.key,
            yielding_user_ta: *accounts.yielding_user_ta.key,
            bank_mint_user_ta: *accounts.bank_mint_user_ta.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            team_state: *accounts.team_state.key,
            fee_team_ata: *accounts.fee_team_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            yielding_mint_program: *accounts.yielding_mint_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<MintWYieldingGenKeys> for [AccountMeta; MINT_W_YIELDING_GEN_IX_ACCOUNTS_LEN] {
    fn from(keys: MintWYieldingGenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_user_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint_user_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_team_ata,
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
                pubkey: keys.yielding_mint_program,
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
impl From<[Pubkey; MINT_W_YIELDING_GEN_IX_ACCOUNTS_LEN]> for MintWYieldingGenKeys {
    fn from(pubkeys: [Pubkey; MINT_W_YIELDING_GEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            yielding_mint: pubkeys[4],
            bank_mint: pubkeys[5],
            yielding_user_ta: pubkeys[6],
            bank_mint_user_ta: pubkeys[7],
            yielding_vault_ata: pubkeys[8],
            team_state: pubkeys[9],
            fee_team_ata: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
            yielding_mint_program: pubkeys[13],
            associated_token_program: pubkeys[14],
        }
    }
}
impl<'info> From<MintWYieldingGenAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_W_YIELDING_GEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintWYieldingGenAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.bank_mint.clone(),
            accounts.yielding_user_ta.clone(),
            accounts.bank_mint_user_ta.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.team_state.clone(),
            accounts.fee_team_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.yielding_mint_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_W_YIELDING_GEN_IX_ACCOUNTS_LEN]>
for MintWYieldingGenAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MINT_W_YIELDING_GEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            yielding_mint: &arr[4],
            bank_mint: &arr[5],
            yielding_user_ta: &arr[6],
            bank_mint_user_ta: &arr[7],
            yielding_vault_ata: &arr[8],
            team_state: &arr[9],
            fee_team_ata: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
            yielding_mint_program: &arr[13],
            associated_token_program: &arr[14],
        }
    }
}
pub const MINT_W_YIELDING_GEN_IX_DISCM: [u8; 8usize] = [31, 100, 17, 215, 62, 12, 31, 2];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintWYieldingGenIxArgs {
    pub amount_yielding_deposit: u64,
    pub min_bank_mint_minted: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintWYieldingGenIxData(pub MintWYieldingGenIxArgs);
impl From<MintWYieldingGenIxArgs> for MintWYieldingGenIxData {
    fn from(args: MintWYieldingGenIxArgs) -> Self {
        Self(args)
    }
}
impl MintWYieldingGenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_W_YIELDING_GEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_yielding_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_bank_mint_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MintWYieldingGenIxArgs {
                amount_yielding_deposit,
                min_bank_mint_minted,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_W_YIELDING_GEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_yielding_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_bank_mint_minted, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_w_yielding_gen_ix_with_program_id(
    program_id: Pubkey,
    keys: MintWYieldingGenKeys,
    args: MintWYieldingGenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_W_YIELDING_GEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintWYieldingGenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_w_yielding_gen_ix(
    keys: MintWYieldingGenKeys,
    args: MintWYieldingGenIxArgs,
) -> std::io::Result<Instruction> {
    mint_w_yielding_gen_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn mint_w_yielding_gen_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintWYieldingGenAccounts<'_, '_>,
    args: MintWYieldingGenIxArgs,
) -> ProgramResult {
    let keys: MintWYieldingGenKeys = accounts.into();
    let ix = mint_w_yielding_gen_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_w_yielding_gen_invoke(
    accounts: MintWYieldingGenAccounts<'_, '_>,
    args: MintWYieldingGenIxArgs,
) -> ProgramResult {
    mint_w_yielding_gen_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn mint_w_yielding_gen_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintWYieldingGenAccounts<'_, '_>,
    args: MintWYieldingGenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintWYieldingGenKeys = accounts.into();
    let ix = mint_w_yielding_gen_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_w_yielding_gen_invoke_signed(
    accounts: MintWYieldingGenAccounts<'_, '_>,
    args: MintWYieldingGenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_w_yielding_gen_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_w_yielding_gen_verify_account_keys(
    accounts: MintWYieldingGenAccounts<'_, '_>,
    keys: MintWYieldingGenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.yielding_user_ta.key, keys.yielding_user_ta),
        (*accounts.bank_mint_user_ta.key, keys.bank_mint_user_ta),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.fee_team_ata.key, keys.fee_team_ata),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.yielding_mint_program.key, keys.yielding_mint_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_w_yielding_gen_verify_writable_privileges<'me, 'info>(
    accounts: MintWYieldingGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank_state,
        accounts.vault_state,
        accounts.bank_mint,
        accounts.yielding_user_ta,
        accounts.bank_mint_user_ta,
        accounts.yielding_vault_ata,
        accounts.team_state,
        accounts.fee_team_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_w_yielding_gen_verify_signer_privileges<'me, 'info>(
    accounts: MintWYieldingGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_w_yielding_gen_verify_account_privileges<'me, 'info>(
    accounts: MintWYieldingGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_w_yielding_gen_verify_writable_privileges(accounts)?;
    mint_w_yielding_gen_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NAME_BANK_MANAGER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct NameBankManagerAccounts<'me, 'info> {
    pub bank_state: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NameBankManagerKeys {
    pub bank_state: Pubkey,
    pub creator: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<NameBankManagerAccounts<'_, '_>> for NameBankManagerKeys {
    fn from(accounts: NameBankManagerAccounts) -> Self {
        Self {
            bank_state: *accounts.bank_state.key,
            creator: *accounts.creator.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<NameBankManagerKeys> for [AccountMeta; NAME_BANK_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: NameBankManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
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
impl From<[Pubkey; NAME_BANK_MANAGER_IX_ACCOUNTS_LEN]> for NameBankManagerKeys {
    fn from(pubkeys: [Pubkey; NAME_BANK_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank_state: pubkeys[0],
            creator: pubkeys[1],
            system_program: pubkeys[2],
            token_program: pubkeys[3],
        }
    }
}
impl<'info> From<NameBankManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; NAME_BANK_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: NameBankManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.bank_state.clone(),
            accounts.creator.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NAME_BANK_MANAGER_IX_ACCOUNTS_LEN]>
for NameBankManagerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NAME_BANK_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank_state: &arr[0],
            creator: &arr[1],
            system_program: &arr[2],
            token_program: &arr[3],
        }
    }
}
pub const NAME_BANK_MANAGER_IX_DISCM: [u8; 8usize] = [
    77, 204, 162, 85, 86, 181, 79, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NameBankManagerIxArgs {
    pub new_manager: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NameBankManagerIxData(pub NameBankManagerIxArgs);
impl From<NameBankManagerIxArgs> for NameBankManagerIxData {
    fn from(args: NameBankManagerIxArgs) -> Self {
        Self(args)
    }
}
impl NameBankManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NAME_BANK_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(NameBankManagerIxArgs {
                new_manager,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NAME_BANK_MANAGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_manager, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn name_bank_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: NameBankManagerKeys,
    args: NameBankManagerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NAME_BANK_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: NameBankManagerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn name_bank_manager_ix(
    keys: NameBankManagerKeys,
    args: NameBankManagerIxArgs,
) -> std::io::Result<Instruction> {
    name_bank_manager_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn name_bank_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NameBankManagerAccounts<'_, '_>,
    args: NameBankManagerIxArgs,
) -> ProgramResult {
    let keys: NameBankManagerKeys = accounts.into();
    let ix = name_bank_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn name_bank_manager_invoke(
    accounts: NameBankManagerAccounts<'_, '_>,
    args: NameBankManagerIxArgs,
) -> ProgramResult {
    name_bank_manager_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn name_bank_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NameBankManagerAccounts<'_, '_>,
    args: NameBankManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NameBankManagerKeys = accounts.into();
    let ix = name_bank_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn name_bank_manager_invoke_signed(
    accounts: NameBankManagerAccounts<'_, '_>,
    args: NameBankManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    name_bank_manager_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn name_bank_manager_verify_account_keys(
    accounts: NameBankManagerAccounts<'_, '_>,
    keys: NameBankManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.creator.key, keys.creator),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn name_bank_manager_verify_writable_privileges<'me, 'info>(
    accounts: NameBankManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank_state, accounts.creator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn name_bank_manager_verify_signer_privileges<'me, 'info>(
    accounts: NameBankManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn name_bank_manager_verify_account_privileges<'me, 'info>(
    accounts: NameBankManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    name_bank_manager_verify_writable_privileges(accounts)?;
    name_bank_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NAME_BANK_RISK_MANAGER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct NameBankRiskManagerAccounts<'me, 'info> {
    pub bank_state: &'me AccountInfo<'info>,
    pub manager: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NameBankRiskManagerKeys {
    pub bank_state: Pubkey,
    pub manager: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<NameBankRiskManagerAccounts<'_, '_>> for NameBankRiskManagerKeys {
    fn from(accounts: NameBankRiskManagerAccounts) -> Self {
        Self {
            bank_state: *accounts.bank_state.key,
            manager: *accounts.manager.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<NameBankRiskManagerKeys>
for [AccountMeta; NAME_BANK_RISK_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: NameBankRiskManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
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
impl From<[Pubkey; NAME_BANK_RISK_MANAGER_IX_ACCOUNTS_LEN]> for NameBankRiskManagerKeys {
    fn from(pubkeys: [Pubkey; NAME_BANK_RISK_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank_state: pubkeys[0],
            manager: pubkeys[1],
            system_program: pubkeys[2],
            token_program: pubkeys[3],
        }
    }
}
impl<'info> From<NameBankRiskManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; NAME_BANK_RISK_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: NameBankRiskManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.bank_state.clone(),
            accounts.manager.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NAME_BANK_RISK_MANAGER_IX_ACCOUNTS_LEN]>
for NameBankRiskManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; NAME_BANK_RISK_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bank_state: &arr[0],
            manager: &arr[1],
            system_program: &arr[2],
            token_program: &arr[3],
        }
    }
}
pub const NAME_BANK_RISK_MANAGER_IX_DISCM: [u8; 8usize] = [
    52, 210, 175, 235, 176, 133, 200, 122,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NameBankRiskManagerIxArgs {
    pub risk_manager: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NameBankRiskManagerIxData(pub NameBankRiskManagerIxArgs);
impl From<NameBankRiskManagerIxArgs> for NameBankRiskManagerIxData {
    fn from(args: NameBankRiskManagerIxArgs) -> Self {
        Self(args)
    }
}
impl NameBankRiskManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NAME_BANK_RISK_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let risk_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(NameBankRiskManagerIxArgs {
                risk_manager,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NAME_BANK_RISK_MANAGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.risk_manager, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn name_bank_risk_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: NameBankRiskManagerKeys,
    args: NameBankRiskManagerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NAME_BANK_RISK_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: NameBankRiskManagerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn name_bank_risk_manager_ix(
    keys: NameBankRiskManagerKeys,
    args: NameBankRiskManagerIxArgs,
) -> std::io::Result<Instruction> {
    name_bank_risk_manager_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn name_bank_risk_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NameBankRiskManagerAccounts<'_, '_>,
    args: NameBankRiskManagerIxArgs,
) -> ProgramResult {
    let keys: NameBankRiskManagerKeys = accounts.into();
    let ix = name_bank_risk_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn name_bank_risk_manager_invoke(
    accounts: NameBankRiskManagerAccounts<'_, '_>,
    args: NameBankRiskManagerIxArgs,
) -> ProgramResult {
    name_bank_risk_manager_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn name_bank_risk_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NameBankRiskManagerAccounts<'_, '_>,
    args: NameBankRiskManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NameBankRiskManagerKeys = accounts.into();
    let ix = name_bank_risk_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn name_bank_risk_manager_invoke_signed(
    accounts: NameBankRiskManagerAccounts<'_, '_>,
    args: NameBankRiskManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    name_bank_risk_manager_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn name_bank_risk_manager_verify_account_keys(
    accounts: NameBankRiskManagerAccounts<'_, '_>,
    keys: NameBankRiskManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.manager.key, keys.manager),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn name_bank_risk_manager_verify_writable_privileges<'me, 'info>(
    accounts: NameBankRiskManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank_state, accounts.manager] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn name_bank_risk_manager_verify_signer_privileges<'me, 'info>(
    accounts: NameBankRiskManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn name_bank_risk_manager_verify_account_privileges<'me, 'info>(
    accounts: NameBankRiskManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    name_bank_risk_manager_verify_writable_privileges(accounts)?;
    name_bank_risk_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RefreshAtomicLendingAccountingAccounts<'me, 'info> {
    pub yield_manager: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshAtomicLendingAccountingKeys {
    pub yield_manager: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub team_state: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<RefreshAtomicLendingAccountingAccounts<'_, '_>>
for RefreshAtomicLendingAccountingKeys {
    fn from(accounts: RefreshAtomicLendingAccountingAccounts) -> Self {
        Self {
            yield_manager: *accounts.yield_manager.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            team_state: *accounts.team_state.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RefreshAtomicLendingAccountingKeys>
for [AccountMeta; REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshAtomicLendingAccountingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.yield_manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
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
impl From<[Pubkey; REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_ACCOUNTS_LEN]>
for RefreshAtomicLendingAccountingKeys {
    fn from(
        pubkeys: [Pubkey; REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            yield_manager: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            team_state: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<RefreshAtomicLendingAccountingAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshAtomicLendingAccountingAccounts<'_, 'info>) -> Self {
        [
            accounts.yield_manager.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.team_state.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_ACCOUNTS_LEN]>
for RefreshAtomicLendingAccountingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            yield_manager: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            team_state: &arr[4],
            yielding_vault_ata: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_DISCM: [u8; 8usize] = [
    20, 136, 210, 216, 52, 126, 102, 176,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshAtomicLendingAccountingIxData;
impl RefreshAtomicLendingAccountingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_atomic_lending_accounting_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshAtomicLendingAccountingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefreshAtomicLendingAccountingIxData.try_to_vec()?,
    })
}
pub fn refresh_atomic_lending_accounting_ix(
    keys: RefreshAtomicLendingAccountingKeys,
) -> std::io::Result<Instruction> {
    refresh_atomic_lending_accounting_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn refresh_atomic_lending_accounting_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshAtomicLendingAccountingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshAtomicLendingAccountingKeys = accounts.into();
    let ix = refresh_atomic_lending_accounting_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_atomic_lending_accounting_invoke(
    accounts: RefreshAtomicLendingAccountingAccounts<'_, '_>,
) -> ProgramResult {
    refresh_atomic_lending_accounting_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
    )
}
pub fn refresh_atomic_lending_accounting_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshAtomicLendingAccountingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshAtomicLendingAccountingKeys = accounts.into();
    let ix = refresh_atomic_lending_accounting_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_atomic_lending_accounting_invoke_signed(
    accounts: RefreshAtomicLendingAccountingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_atomic_lending_accounting_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn refresh_atomic_lending_accounting_verify_account_keys(
    accounts: RefreshAtomicLendingAccountingAccounts<'_, '_>,
    keys: RefreshAtomicLendingAccountingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.yield_manager.key, keys.yield_manager),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_atomic_lending_accounting_verify_writable_privileges<'me, 'info>(
    accounts: RefreshAtomicLendingAccountingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.yield_manager,
        accounts.vault_state,
        accounts.team_state,
        accounts.yielding_vault_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_atomic_lending_accounting_verify_signer_privileges<'me, 'info>(
    accounts: RefreshAtomicLendingAccountingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.yield_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn refresh_atomic_lending_accounting_verify_account_privileges<'me, 'info>(
    accounts: RefreshAtomicLendingAccountingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_atomic_lending_accounting_verify_writable_privileges(accounts)?;
    refresh_atomic_lending_accounting_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REQUEST_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct RequestUnstakeJuniorAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub withdrawal_queue: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub target_mint: &'me AccountInfo<'info>,
    pub junior_mint: &'me AccountInfo<'info>,
    pub user_junior_mint_ata: &'me AccountInfo<'info>,
    pub locked_junior_shares_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RequestUnstakeJuniorKeys {
    pub user: Pubkey,
    pub payer: Pubkey,
    pub withdrawal_queue: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub target_mint: Pubkey,
    pub junior_mint: Pubkey,
    pub user_junior_mint_ata: Pubkey,
    pub locked_junior_shares_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<RequestUnstakeJuniorAccounts<'_, '_>> for RequestUnstakeJuniorKeys {
    fn from(accounts: RequestUnstakeJuniorAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            payer: *accounts.payer.key,
            withdrawal_queue: *accounts.withdrawal_queue.key,
            tranche_state: *accounts.tranche_state.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            target_mint: *accounts.target_mint.key,
            junior_mint: *accounts.junior_mint.key,
            user_junior_mint_ata: *accounts.user_junior_mint_ata.key,
            locked_junior_shares_ata: *accounts.locked_junior_shares_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RequestUnstakeJuniorKeys>
for [AccountMeta; REQUEST_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(keys: RequestUnstakeJuniorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdrawal_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.target_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.junior_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_junior_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locked_junior_shares_ata,
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
impl From<[Pubkey; REQUEST_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]>
for RequestUnstakeJuniorKeys {
    fn from(pubkeys: [Pubkey; REQUEST_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            payer: pubkeys[1],
            withdrawal_queue: pubkeys[2],
            tranche_state: pubkeys[3],
            bank_state: pubkeys[4],
            bank_mint: pubkeys[5],
            target_mint: pubkeys[6],
            junior_mint: pubkeys[7],
            user_junior_mint_ata: pubkeys[8],
            locked_junior_shares_ata: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<RequestUnstakeJuniorAccounts<'_, 'info>>
for [AccountInfo<'info>; REQUEST_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: RequestUnstakeJuniorAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.payer.clone(),
            accounts.withdrawal_queue.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.target_mint.clone(),
            accounts.junior_mint.clone(),
            accounts.user_junior_mint_ata.clone(),
            accounts.locked_junior_shares_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REQUEST_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]>
for RequestUnstakeJuniorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REQUEST_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            payer: &arr[1],
            withdrawal_queue: &arr[2],
            tranche_state: &arr[3],
            bank_state: &arr[4],
            bank_mint: &arr[5],
            target_mint: &arr[6],
            junior_mint: &arr[7],
            user_junior_mint_ata: &arr[8],
            locked_junior_shares_ata: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const REQUEST_UNSTAKE_JUNIOR_IX_DISCM: [u8; 8usize] = [
    57, 62, 24, 173, 40, 103, 48, 63,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RequestUnstakeJuniorIxArgs {
    pub shares: u64,
    pub queue_id: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestUnstakeJuniorIxData(pub RequestUnstakeJuniorIxArgs);
impl From<RequestUnstakeJuniorIxArgs> for RequestUnstakeJuniorIxData {
    fn from(args: RequestUnstakeJuniorIxArgs) -> Self {
        Self(args)
    }
}
impl RequestUnstakeJuniorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_UNSTAKE_JUNIOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let queue_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RequestUnstakeJuniorIxArgs {
                shares,
                queue_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_UNSTAKE_JUNIOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.queue_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn request_unstake_junior_ix_with_program_id(
    program_id: Pubkey,
    keys: RequestUnstakeJuniorKeys,
    args: RequestUnstakeJuniorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REQUEST_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: RequestUnstakeJuniorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_unstake_junior_ix(
    keys: RequestUnstakeJuniorKeys,
    args: RequestUnstakeJuniorIxArgs,
) -> std::io::Result<Instruction> {
    request_unstake_junior_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn request_unstake_junior_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RequestUnstakeJuniorAccounts<'_, '_>,
    args: RequestUnstakeJuniorIxArgs,
) -> ProgramResult {
    let keys: RequestUnstakeJuniorKeys = accounts.into();
    let ix = request_unstake_junior_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn request_unstake_junior_invoke(
    accounts: RequestUnstakeJuniorAccounts<'_, '_>,
    args: RequestUnstakeJuniorIxArgs,
) -> ProgramResult {
    request_unstake_junior_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn request_unstake_junior_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RequestUnstakeJuniorAccounts<'_, '_>,
    args: RequestUnstakeJuniorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RequestUnstakeJuniorKeys = accounts.into();
    let ix = request_unstake_junior_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn request_unstake_junior_invoke_signed(
    accounts: RequestUnstakeJuniorAccounts<'_, '_>,
    args: RequestUnstakeJuniorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    request_unstake_junior_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn request_unstake_junior_verify_account_keys(
    accounts: RequestUnstakeJuniorAccounts<'_, '_>,
    keys: RequestUnstakeJuniorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.payer.key, keys.payer),
        (*accounts.withdrawal_queue.key, keys.withdrawal_queue),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.target_mint.key, keys.target_mint),
        (*accounts.junior_mint.key, keys.junior_mint),
        (*accounts.user_junior_mint_ata.key, keys.user_junior_mint_ata),
        (*accounts.locked_junior_shares_ata.key, keys.locked_junior_shares_ata),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn request_unstake_junior_verify_writable_privileges<'me, 'info>(
    accounts: RequestUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.withdrawal_queue,
        accounts.user_junior_mint_ata,
        accounts.locked_junior_shares_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn request_unstake_junior_verify_signer_privileges<'me, 'info>(
    accounts: RequestUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn request_unstake_junior_verify_account_privileges<'me, 'info>(
    accounts: RequestUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    request_unstake_junior_verify_writable_privileges(accounts)?;
    request_unstake_junior_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_VAULT_CONFIG_DETAILS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetVaultConfigDetailsAccounts<'me, 'info> {
    pub vault_creator: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetVaultConfigDetailsKeys {
    pub vault_creator: Pubkey,
    pub team_state: Pubkey,
    pub vault_state: Pubkey,
}
impl From<SetVaultConfigDetailsAccounts<'_, '_>> for SetVaultConfigDetailsKeys {
    fn from(accounts: SetVaultConfigDetailsAccounts) -> Self {
        Self {
            vault_creator: *accounts.vault_creator.key,
            team_state: *accounts.team_state.key,
            vault_state: *accounts.vault_state.key,
        }
    }
}
impl From<SetVaultConfigDetailsKeys>
for [AccountMeta; SET_VAULT_CONFIG_DETAILS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetVaultConfigDetailsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault_creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_VAULT_CONFIG_DETAILS_IX_ACCOUNTS_LEN]>
for SetVaultConfigDetailsKeys {
    fn from(pubkeys: [Pubkey; SET_VAULT_CONFIG_DETAILS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_creator: pubkeys[0],
            team_state: pubkeys[1],
            vault_state: pubkeys[2],
        }
    }
}
impl<'info> From<SetVaultConfigDetailsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_VAULT_CONFIG_DETAILS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetVaultConfigDetailsAccounts<'_, 'info>) -> Self {
        [
            accounts.vault_creator.clone(),
            accounts.team_state.clone(),
            accounts.vault_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_VAULT_CONFIG_DETAILS_IX_ACCOUNTS_LEN]>
for SetVaultConfigDetailsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_VAULT_CONFIG_DETAILS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault_creator: &arr[0],
            team_state: &arr[1],
            vault_state: &arr[2],
        }
    }
}
pub const SET_VAULT_CONFIG_DETAILS_IX_DISCM: [u8; 8usize] = [
    94, 99, 5, 208, 172, 49, 86, 126,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetVaultConfigDetailsIxArgs {
    pub data: VaultConfigData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetVaultConfigDetailsIxData(pub SetVaultConfigDetailsIxArgs);
impl From<SetVaultConfigDetailsIxArgs> for SetVaultConfigDetailsIxData {
    fn from(args: SetVaultConfigDetailsIxArgs) -> Self {
        Self(args)
    }
}
impl SetVaultConfigDetailsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_VAULT_CONFIG_DETAILS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <VaultConfigData>::deserialize(&mut reader)?
        };
        Ok(
            Self(SetVaultConfigDetailsIxArgs {
                data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_VAULT_CONFIG_DETAILS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_vault_config_details_ix_with_program_id(
    program_id: Pubkey,
    keys: SetVaultConfigDetailsKeys,
    args: SetVaultConfigDetailsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_VAULT_CONFIG_DETAILS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetVaultConfigDetailsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_vault_config_details_ix(
    keys: SetVaultConfigDetailsKeys,
    args: SetVaultConfigDetailsIxArgs,
) -> std::io::Result<Instruction> {
    set_vault_config_details_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn set_vault_config_details_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetVaultConfigDetailsAccounts<'_, '_>,
    args: SetVaultConfigDetailsIxArgs,
) -> ProgramResult {
    let keys: SetVaultConfigDetailsKeys = accounts.into();
    let ix = set_vault_config_details_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_vault_config_details_invoke(
    accounts: SetVaultConfigDetailsAccounts<'_, '_>,
    args: SetVaultConfigDetailsIxArgs,
) -> ProgramResult {
    set_vault_config_details_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn set_vault_config_details_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetVaultConfigDetailsAccounts<'_, '_>,
    args: SetVaultConfigDetailsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetVaultConfigDetailsKeys = accounts.into();
    let ix = set_vault_config_details_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_vault_config_details_invoke_signed(
    accounts: SetVaultConfigDetailsAccounts<'_, '_>,
    args: SetVaultConfigDetailsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_vault_config_details_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_vault_config_details_verify_account_keys(
    accounts: SetVaultConfigDetailsAccounts<'_, '_>,
    keys: SetVaultConfigDetailsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault_creator.key, keys.vault_creator),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.vault_state.key, keys.vault_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_vault_config_details_verify_writable_privileges<'me, 'info>(
    accounts: SetVaultConfigDetailsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_creator,
        accounts.team_state,
        accounts.vault_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_vault_config_details_verify_signer_privileges<'me, 'info>(
    accounts: SetVaultConfigDetailsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vault_creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_vault_config_details_verify_account_privileges<'me, 'info>(
    accounts: SetVaultConfigDetailsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_vault_config_details_verify_writable_privileges(accounts)?;
    set_vault_config_details_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STAKE_JUNIOR_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct StakeJuniorAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub junior_mint: &'me AccountInfo<'info>,
    pub user_bank_mint_ata: &'me AccountInfo<'info>,
    pub user_junior_mint_ata: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StakeJuniorKeys {
    pub user: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub junior_mint: Pubkey,
    pub user_bank_mint_ata: Pubkey,
    pub user_junior_mint_ata: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<StakeJuniorAccounts<'_, '_>> for StakeJuniorKeys {
    fn from(accounts: StakeJuniorAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            tranche_state: *accounts.tranche_state.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            junior_mint: *accounts.junior_mint.key,
            user_bank_mint_ata: *accounts.user_bank_mint_ata.key,
            user_junior_mint_ata: *accounts.user_junior_mint_ata.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<StakeJuniorKeys> for [AccountMeta; STAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(keys: StakeJuniorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.junior_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_bank_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_junior_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.junior_escrow_ata,
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
impl From<[Pubkey; STAKE_JUNIOR_IX_ACCOUNTS_LEN]> for StakeJuniorKeys {
    fn from(pubkeys: [Pubkey; STAKE_JUNIOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            tranche_state: pubkeys[1],
            bank_state: pubkeys[2],
            bank_mint: pubkeys[3],
            junior_mint: pubkeys[4],
            user_bank_mint_ata: pubkeys[5],
            user_junior_mint_ata: pubkeys[6],
            junior_escrow_ata: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
        }
    }
}
impl<'info> From<StakeJuniorAccounts<'_, 'info>>
for [AccountInfo<'info>; STAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: StakeJuniorAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.junior_mint.clone(),
            accounts.user_bank_mint_ata.clone(),
            accounts.user_junior_mint_ata.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STAKE_JUNIOR_IX_ACCOUNTS_LEN]>
for StakeJuniorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; STAKE_JUNIOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            tranche_state: &arr[1],
            bank_state: &arr[2],
            bank_mint: &arr[3],
            junior_mint: &arr[4],
            user_bank_mint_ata: &arr[5],
            user_junior_mint_ata: &arr[6],
            junior_escrow_ata: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
        }
    }
}
pub const STAKE_JUNIOR_IX_DISCM: [u8; 8usize] = [6, 116, 147, 101, 38, 154, 179, 246];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StakeJuniorIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeJuniorIxData(pub StakeJuniorIxArgs);
impl From<StakeJuniorIxArgs> for StakeJuniorIxData {
    fn from(args: StakeJuniorIxArgs) -> Self {
        Self(args)
    }
}
impl StakeJuniorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_JUNIOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(StakeJuniorIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_JUNIOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stake_junior_ix_with_program_id(
    program_id: Pubkey,
    keys: StakeJuniorKeys,
    args: StakeJuniorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STAKE_JUNIOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: StakeJuniorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_junior_ix(
    keys: StakeJuniorKeys,
    args: StakeJuniorIxArgs,
) -> std::io::Result<Instruction> {
    stake_junior_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn stake_junior_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StakeJuniorAccounts<'_, '_>,
    args: StakeJuniorIxArgs,
) -> ProgramResult {
    let keys: StakeJuniorKeys = accounts.into();
    let ix = stake_junior_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stake_junior_invoke(
    accounts: StakeJuniorAccounts<'_, '_>,
    args: StakeJuniorIxArgs,
) -> ProgramResult {
    stake_junior_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn stake_junior_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StakeJuniorAccounts<'_, '_>,
    args: StakeJuniorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StakeJuniorKeys = accounts.into();
    let ix = stake_junior_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stake_junior_invoke_signed(
    accounts: StakeJuniorAccounts<'_, '_>,
    args: StakeJuniorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stake_junior_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn stake_junior_verify_account_keys(
    accounts: StakeJuniorAccounts<'_, '_>,
    keys: StakeJuniorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.junior_mint.key, keys.junior_mint),
        (*accounts.user_bank_mint_ata.key, keys.user_bank_mint_ata),
        (*accounts.user_junior_mint_ata.key, keys.user_junior_mint_ata),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
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
pub fn stake_junior_verify_writable_privileges<'me, 'info>(
    accounts: StakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.tranche_state,
        accounts.junior_mint,
        accounts.user_bank_mint_ata,
        accounts.user_junior_mint_ata,
        accounts.junior_escrow_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stake_junior_verify_signer_privileges<'me, 'info>(
    accounts: StakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stake_junior_verify_account_privileges<'me, 'info>(
    accounts: StakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stake_junior_verify_writable_privileges(accounts)?;
    stake_junior_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TEAM_DEPOSITS_FROM_INVEST_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct TeamDepositsFromInvestAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub signer_ta: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TeamDepositsFromInvestKeys {
    pub signer: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub team_state: Pubkey,
    pub signer_ta: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<TeamDepositsFromInvestAccounts<'_, '_>> for TeamDepositsFromInvestKeys {
    fn from(accounts: TeamDepositsFromInvestAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            team_state: *accounts.team_state.key,
            signer_ta: *accounts.signer_ta.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<TeamDepositsFromInvestKeys>
for [AccountMeta; TEAM_DEPOSITS_FROM_INVEST_IX_ACCOUNTS_LEN] {
    fn from(keys: TeamDepositsFromInvestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_ta,
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
impl From<[Pubkey; TEAM_DEPOSITS_FROM_INVEST_IX_ACCOUNTS_LEN]>
for TeamDepositsFromInvestKeys {
    fn from(pubkeys: [Pubkey; TEAM_DEPOSITS_FROM_INVEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            yielding_mint: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            team_state: pubkeys[6],
            signer_ta: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<TeamDepositsFromInvestAccounts<'_, 'info>>
for [AccountInfo<'info>; TEAM_DEPOSITS_FROM_INVEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: TeamDepositsFromInvestAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.team_state.clone(),
            accounts.signer_ta.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TEAM_DEPOSITS_FROM_INVEST_IX_ACCOUNTS_LEN]>
for TeamDepositsFromInvestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TEAM_DEPOSITS_FROM_INVEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            yielding_mint: &arr[4],
            yielding_vault_ata: &arr[5],
            team_state: &arr[6],
            signer_ta: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const TEAM_DEPOSITS_FROM_INVEST_IX_DISCM: [u8; 8usize] = [
    237, 41, 29, 215, 187, 56, 237, 185,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TeamDepositsFromInvestIxArgs {
    pub deposited_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TeamDepositsFromInvestIxData(pub TeamDepositsFromInvestIxArgs);
impl From<TeamDepositsFromInvestIxArgs> for TeamDepositsFromInvestIxData {
    fn from(args: TeamDepositsFromInvestIxArgs) -> Self {
        Self(args)
    }
}
impl TeamDepositsFromInvestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TEAM_DEPOSITS_FROM_INVEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let deposited_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TeamDepositsFromInvestIxArgs {
                deposited_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TEAM_DEPOSITS_FROM_INVEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.deposited_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn team_deposits_from_invest_ix_with_program_id(
    program_id: Pubkey,
    keys: TeamDepositsFromInvestKeys,
    args: TeamDepositsFromInvestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TEAM_DEPOSITS_FROM_INVEST_IX_ACCOUNTS_LEN] = keys.into();
    let data: TeamDepositsFromInvestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn team_deposits_from_invest_ix(
    keys: TeamDepositsFromInvestKeys,
    args: TeamDepositsFromInvestIxArgs,
) -> std::io::Result<Instruction> {
    team_deposits_from_invest_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn team_deposits_from_invest_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TeamDepositsFromInvestAccounts<'_, '_>,
    args: TeamDepositsFromInvestIxArgs,
) -> ProgramResult {
    let keys: TeamDepositsFromInvestKeys = accounts.into();
    let ix = team_deposits_from_invest_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn team_deposits_from_invest_invoke(
    accounts: TeamDepositsFromInvestAccounts<'_, '_>,
    args: TeamDepositsFromInvestIxArgs,
) -> ProgramResult {
    team_deposits_from_invest_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn team_deposits_from_invest_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TeamDepositsFromInvestAccounts<'_, '_>,
    args: TeamDepositsFromInvestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TeamDepositsFromInvestKeys = accounts.into();
    let ix = team_deposits_from_invest_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn team_deposits_from_invest_invoke_signed(
    accounts: TeamDepositsFromInvestAccounts<'_, '_>,
    args: TeamDepositsFromInvestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    team_deposits_from_invest_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn team_deposits_from_invest_verify_account_keys(
    accounts: TeamDepositsFromInvestAccounts<'_, '_>,
    keys: TeamDepositsFromInvestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.signer_ta.key, keys.signer_ta),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn team_deposits_from_invest_verify_writable_privileges<'me, 'info>(
    accounts: TeamDepositsFromInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank_state,
        accounts.vault_state,
        accounts.oracle_state,
        accounts.yielding_vault_ata,
        accounts.team_state,
        accounts.signer_ta,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn team_deposits_from_invest_verify_signer_privileges<'me, 'info>(
    accounts: TeamDepositsFromInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn team_deposits_from_invest_verify_account_privileges<'me, 'info>(
    accounts: TeamDepositsFromInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    team_deposits_from_invest_verify_writable_privileges(accounts)?;
    team_deposits_from_invest_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TEAM_WITHDRAWS_FEES_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct TeamWithdrawsFeesAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub team_ata: &'me AccountInfo<'info>,
    pub destination_ta: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TeamWithdrawsFeesKeys {
    pub signer: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub team_state: Pubkey,
    pub team_ata: Pubkey,
    pub destination_ta: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<TeamWithdrawsFeesAccounts<'_, '_>> for TeamWithdrawsFeesKeys {
    fn from(accounts: TeamWithdrawsFeesAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            team_state: *accounts.team_state.key,
            team_ata: *accounts.team_ata.key,
            destination_ta: *accounts.destination_ta.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<TeamWithdrawsFeesKeys> for [AccountMeta; TEAM_WITHDRAWS_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: TeamWithdrawsFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_ta,
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
impl From<[Pubkey; TEAM_WITHDRAWS_FEES_IX_ACCOUNTS_LEN]> for TeamWithdrawsFeesKeys {
    fn from(pubkeys: [Pubkey; TEAM_WITHDRAWS_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            yielding_mint: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            team_state: pubkeys[6],
            team_ata: pubkeys[7],
            destination_ta: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<TeamWithdrawsFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; TEAM_WITHDRAWS_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: TeamWithdrawsFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.team_state.clone(),
            accounts.team_ata.clone(),
            accounts.destination_ta.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TEAM_WITHDRAWS_FEES_IX_ACCOUNTS_LEN]>
for TeamWithdrawsFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TEAM_WITHDRAWS_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            yielding_mint: &arr[4],
            yielding_vault_ata: &arr[5],
            team_state: &arr[6],
            team_ata: &arr[7],
            destination_ta: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const TEAM_WITHDRAWS_FEES_IX_DISCM: [u8; 8usize] = [
    189, 55, 198, 43, 155, 47, 96, 10,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TeamWithdrawsFeesIxArgs {
    pub fee_type: FeeType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TeamWithdrawsFeesIxData(pub TeamWithdrawsFeesIxArgs);
impl From<TeamWithdrawsFeesIxArgs> for TeamWithdrawsFeesIxData {
    fn from(args: TeamWithdrawsFeesIxArgs) -> Self {
        Self(args)
    }
}
impl TeamWithdrawsFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TEAM_WITHDRAWS_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_type: FeeType = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TeamWithdrawsFeesIxArgs {
                fee_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TEAM_WITHDRAWS_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_type, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn team_withdraws_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: TeamWithdrawsFeesKeys,
    args: TeamWithdrawsFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TEAM_WITHDRAWS_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: TeamWithdrawsFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn team_withdraws_fees_ix(
    keys: TeamWithdrawsFeesKeys,
    args: TeamWithdrawsFeesIxArgs,
) -> std::io::Result<Instruction> {
    team_withdraws_fees_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn team_withdraws_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TeamWithdrawsFeesAccounts<'_, '_>,
    args: TeamWithdrawsFeesIxArgs,
) -> ProgramResult {
    let keys: TeamWithdrawsFeesKeys = accounts.into();
    let ix = team_withdraws_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn team_withdraws_fees_invoke(
    accounts: TeamWithdrawsFeesAccounts<'_, '_>,
    args: TeamWithdrawsFeesIxArgs,
) -> ProgramResult {
    team_withdraws_fees_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn team_withdraws_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TeamWithdrawsFeesAccounts<'_, '_>,
    args: TeamWithdrawsFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TeamWithdrawsFeesKeys = accounts.into();
    let ix = team_withdraws_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn team_withdraws_fees_invoke_signed(
    accounts: TeamWithdrawsFeesAccounts<'_, '_>,
    args: TeamWithdrawsFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    team_withdraws_fees_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn team_withdraws_fees_verify_account_keys(
    accounts: TeamWithdrawsFeesAccounts<'_, '_>,
    keys: TeamWithdrawsFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.team_ata.key, keys.team_ata),
        (*accounts.destination_ta.key, keys.destination_ta),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn team_withdraws_fees_verify_writable_privileges<'me, 'info>(
    accounts: TeamWithdrawsFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.bank_state,
        accounts.vault_state,
        accounts.oracle_state,
        accounts.yielding_vault_ata,
        accounts.team_state,
        accounts.team_ata,
        accounts.destination_ta,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn team_withdraws_fees_verify_signer_privileges<'me, 'info>(
    accounts: TeamWithdrawsFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn team_withdraws_fees_verify_account_privileges<'me, 'info>(
    accounts: TeamWithdrawsFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    team_withdraws_fees_verify_writable_privileges(accounts)?;
    team_withdraws_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TEAM_WITHDRAWS_TO_INVEST_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct TeamWithdrawsToInvestAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub signer_ta: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TeamWithdrawsToInvestKeys {
    pub signer: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub team_state: Pubkey,
    pub signer_ta: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<TeamWithdrawsToInvestAccounts<'_, '_>> for TeamWithdrawsToInvestKeys {
    fn from(accounts: TeamWithdrawsToInvestAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            team_state: *accounts.team_state.key,
            signer_ta: *accounts.signer_ta.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<TeamWithdrawsToInvestKeys>
for [AccountMeta; TEAM_WITHDRAWS_TO_INVEST_IX_ACCOUNTS_LEN] {
    fn from(keys: TeamWithdrawsToInvestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_ta,
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
impl From<[Pubkey; TEAM_WITHDRAWS_TO_INVEST_IX_ACCOUNTS_LEN]>
for TeamWithdrawsToInvestKeys {
    fn from(pubkeys: [Pubkey; TEAM_WITHDRAWS_TO_INVEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            yielding_mint: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            team_state: pubkeys[6],
            signer_ta: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<TeamWithdrawsToInvestAccounts<'_, 'info>>
for [AccountInfo<'info>; TEAM_WITHDRAWS_TO_INVEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: TeamWithdrawsToInvestAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.team_state.clone(),
            accounts.signer_ta.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TEAM_WITHDRAWS_TO_INVEST_IX_ACCOUNTS_LEN]>
for TeamWithdrawsToInvestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TEAM_WITHDRAWS_TO_INVEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            yielding_mint: &arr[4],
            yielding_vault_ata: &arr[5],
            team_state: &arr[6],
            signer_ta: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const TEAM_WITHDRAWS_TO_INVEST_IX_DISCM: [u8; 8usize] = [
    10, 151, 218, 67, 15, 149, 126, 251,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TeamWithdrawsToInvestIxArgs {
    pub withdrawal_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TeamWithdrawsToInvestIxData(pub TeamWithdrawsToInvestIxArgs);
impl From<TeamWithdrawsToInvestIxArgs> for TeamWithdrawsToInvestIxData {
    fn from(args: TeamWithdrawsToInvestIxArgs) -> Self {
        Self(args)
    }
}
impl TeamWithdrawsToInvestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TEAM_WITHDRAWS_TO_INVEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let withdrawal_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TeamWithdrawsToInvestIxArgs {
                withdrawal_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TEAM_WITHDRAWS_TO_INVEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.withdrawal_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn team_withdraws_to_invest_ix_with_program_id(
    program_id: Pubkey,
    keys: TeamWithdrawsToInvestKeys,
    args: TeamWithdrawsToInvestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TEAM_WITHDRAWS_TO_INVEST_IX_ACCOUNTS_LEN] = keys.into();
    let data: TeamWithdrawsToInvestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn team_withdraws_to_invest_ix(
    keys: TeamWithdrawsToInvestKeys,
    args: TeamWithdrawsToInvestIxArgs,
) -> std::io::Result<Instruction> {
    team_withdraws_to_invest_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn team_withdraws_to_invest_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TeamWithdrawsToInvestAccounts<'_, '_>,
    args: TeamWithdrawsToInvestIxArgs,
) -> ProgramResult {
    let keys: TeamWithdrawsToInvestKeys = accounts.into();
    let ix = team_withdraws_to_invest_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn team_withdraws_to_invest_invoke(
    accounts: TeamWithdrawsToInvestAccounts<'_, '_>,
    args: TeamWithdrawsToInvestIxArgs,
) -> ProgramResult {
    team_withdraws_to_invest_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn team_withdraws_to_invest_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TeamWithdrawsToInvestAccounts<'_, '_>,
    args: TeamWithdrawsToInvestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TeamWithdrawsToInvestKeys = accounts.into();
    let ix = team_withdraws_to_invest_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn team_withdraws_to_invest_invoke_signed(
    accounts: TeamWithdrawsToInvestAccounts<'_, '_>,
    args: TeamWithdrawsToInvestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    team_withdraws_to_invest_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn team_withdraws_to_invest_verify_account_keys(
    accounts: TeamWithdrawsToInvestAccounts<'_, '_>,
    keys: TeamWithdrawsToInvestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.signer_ta.key, keys.signer_ta),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn team_withdraws_to_invest_verify_writable_privileges<'me, 'info>(
    accounts: TeamWithdrawsToInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.bank_state,
        accounts.vault_state,
        accounts.oracle_state,
        accounts.yielding_vault_ata,
        accounts.team_state,
        accounts.signer_ta,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn team_withdraws_to_invest_verify_signer_privileges<'me, 'info>(
    accounts: TeamWithdrawsToInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn team_withdraws_to_invest_verify_account_privileges<'me, 'info>(
    accounts: TeamWithdrawsToInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    team_withdraws_to_invest_verify_writable_privileges(accounts)?;
    team_withdraws_to_invest_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRIGGER_BANK_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct TriggerBankCircuitBreakerAccounts<'me, 'info> {
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_manager: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TriggerBankCircuitBreakerKeys {
    pub bank_state: Pubkey,
    pub bank_manager: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<TriggerBankCircuitBreakerAccounts<'_, '_>> for TriggerBankCircuitBreakerKeys {
    fn from(accounts: TriggerBankCircuitBreakerAccounts) -> Self {
        Self {
            bank_state: *accounts.bank_state.key,
            bank_manager: *accounts.bank_manager.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<TriggerBankCircuitBreakerKeys>
for [AccountMeta; TRIGGER_BANK_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] {
    fn from(keys: TriggerBankCircuitBreakerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_manager,
                is_signer: true,
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
impl From<[Pubkey; TRIGGER_BANK_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]>
for TriggerBankCircuitBreakerKeys {
    fn from(pubkeys: [Pubkey; TRIGGER_BANK_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank_state: pubkeys[0],
            bank_manager: pubkeys[1],
            system_program: pubkeys[2],
            token_program: pubkeys[3],
        }
    }
}
impl<'info> From<TriggerBankCircuitBreakerAccounts<'_, 'info>>
for [AccountInfo<'info>; TRIGGER_BANK_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TriggerBankCircuitBreakerAccounts<'_, 'info>) -> Self {
        [
            accounts.bank_state.clone(),
            accounts.bank_manager.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRIGGER_BANK_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]>
for TriggerBankCircuitBreakerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRIGGER_BANK_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bank_state: &arr[0],
            bank_manager: &arr[1],
            system_program: &arr[2],
            token_program: &arr[3],
        }
    }
}
pub const TRIGGER_BANK_CIRCUIT_BREAKER_IX_DISCM: [u8; 8usize] = [
    250, 236, 165, 211, 181, 241, 74, 84,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TriggerBankCircuitBreakerIxArgs {
    pub is_halted: bool,
    pub is_halted_deposit: bool,
    pub is_halted_withdrawal: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TriggerBankCircuitBreakerIxData(pub TriggerBankCircuitBreakerIxArgs);
impl From<TriggerBankCircuitBreakerIxArgs> for TriggerBankCircuitBreakerIxData {
    fn from(args: TriggerBankCircuitBreakerIxArgs) -> Self {
        Self(args)
    }
}
impl TriggerBankCircuitBreakerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRIGGER_BANK_CIRCUIT_BREAKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let is_halted: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_halted_deposit: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_halted_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TriggerBankCircuitBreakerIxArgs {
                is_halted,
                is_halted_deposit,
                is_halted_withdrawal,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRIGGER_BANK_CIRCUIT_BREAKER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.is_halted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_halted_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_halted_withdrawal, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn trigger_bank_circuit_breaker_ix_with_program_id(
    program_id: Pubkey,
    keys: TriggerBankCircuitBreakerKeys,
    args: TriggerBankCircuitBreakerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRIGGER_BANK_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] = keys.into();
    let data: TriggerBankCircuitBreakerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn trigger_bank_circuit_breaker_ix(
    keys: TriggerBankCircuitBreakerKeys,
    args: TriggerBankCircuitBreakerIxArgs,
) -> std::io::Result<Instruction> {
    trigger_bank_circuit_breaker_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn trigger_bank_circuit_breaker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TriggerBankCircuitBreakerAccounts<'_, '_>,
    args: TriggerBankCircuitBreakerIxArgs,
) -> ProgramResult {
    let keys: TriggerBankCircuitBreakerKeys = accounts.into();
    let ix = trigger_bank_circuit_breaker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn trigger_bank_circuit_breaker_invoke(
    accounts: TriggerBankCircuitBreakerAccounts<'_, '_>,
    args: TriggerBankCircuitBreakerIxArgs,
) -> ProgramResult {
    trigger_bank_circuit_breaker_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn trigger_bank_circuit_breaker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TriggerBankCircuitBreakerAccounts<'_, '_>,
    args: TriggerBankCircuitBreakerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TriggerBankCircuitBreakerKeys = accounts.into();
    let ix = trigger_bank_circuit_breaker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn trigger_bank_circuit_breaker_invoke_signed(
    accounts: TriggerBankCircuitBreakerAccounts<'_, '_>,
    args: TriggerBankCircuitBreakerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    trigger_bank_circuit_breaker_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn trigger_bank_circuit_breaker_verify_account_keys(
    accounts: TriggerBankCircuitBreakerAccounts<'_, '_>,
    keys: TriggerBankCircuitBreakerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_manager.key, keys.bank_manager),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn trigger_bank_circuit_breaker_verify_writable_privileges<'me, 'info>(
    accounts: TriggerBankCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank_state, accounts.bank_manager] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn trigger_bank_circuit_breaker_verify_signer_privileges<'me, 'info>(
    accounts: TriggerBankCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.bank_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn trigger_bank_circuit_breaker_verify_account_privileges<'me, 'info>(
    accounts: TriggerBankCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    trigger_bank_circuit_breaker_verify_writable_privileges(accounts)?;
    trigger_bank_circuit_breaker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRIGGER_VAULT_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct TriggerVaultCircuitBreakerAccounts<'me, 'info> {
    pub bank_state: &'me AccountInfo<'info>,
    pub manager: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TriggerVaultCircuitBreakerKeys {
    pub bank_state: Pubkey,
    pub manager: Pubkey,
    pub vault_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<TriggerVaultCircuitBreakerAccounts<'_, '_>>
for TriggerVaultCircuitBreakerKeys {
    fn from(accounts: TriggerVaultCircuitBreakerAccounts) -> Self {
        Self {
            bank_state: *accounts.bank_state.key,
            manager: *accounts.manager.key,
            vault_state: *accounts.vault_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TriggerVaultCircuitBreakerKeys>
for [AccountMeta; TRIGGER_VAULT_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] {
    fn from(keys: TriggerVaultCircuitBreakerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
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
impl From<[Pubkey; TRIGGER_VAULT_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]>
for TriggerVaultCircuitBreakerKeys {
    fn from(pubkeys: [Pubkey; TRIGGER_VAULT_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank_state: pubkeys[0],
            manager: pubkeys[1],
            vault_state: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<TriggerVaultCircuitBreakerAccounts<'_, 'info>>
for [AccountInfo<'info>; TRIGGER_VAULT_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TriggerVaultCircuitBreakerAccounts<'_, 'info>) -> Self {
        [
            accounts.bank_state.clone(),
            accounts.manager.clone(),
            accounts.vault_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRIGGER_VAULT_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]>
for TriggerVaultCircuitBreakerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRIGGER_VAULT_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bank_state: &arr[0],
            manager: &arr[1],
            vault_state: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const TRIGGER_VAULT_CIRCUIT_BREAKER_IX_DISCM: [u8; 8usize] = [
    29, 3, 224, 104, 247, 145, 14, 188,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TriggerVaultCircuitBreakerIxArgs {
    pub is_halted: bool,
    pub is_halted_deposit: bool,
    pub is_halted_withdrawal: bool,
    pub losses_accepted: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TriggerVaultCircuitBreakerIxData(pub TriggerVaultCircuitBreakerIxArgs);
impl From<TriggerVaultCircuitBreakerIxArgs> for TriggerVaultCircuitBreakerIxData {
    fn from(args: TriggerVaultCircuitBreakerIxArgs) -> Self {
        Self(args)
    }
}
impl TriggerVaultCircuitBreakerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRIGGER_VAULT_CIRCUIT_BREAKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let is_halted: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_halted_deposit: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_halted_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
        let losses_accepted: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TriggerVaultCircuitBreakerIxArgs {
                is_halted,
                is_halted_deposit,
                is_halted_withdrawal,
                losses_accepted,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRIGGER_VAULT_CIRCUIT_BREAKER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.is_halted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_halted_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_halted_withdrawal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.losses_accepted, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn trigger_vault_circuit_breaker_ix_with_program_id(
    program_id: Pubkey,
    keys: TriggerVaultCircuitBreakerKeys,
    args: TriggerVaultCircuitBreakerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRIGGER_VAULT_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: TriggerVaultCircuitBreakerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn trigger_vault_circuit_breaker_ix(
    keys: TriggerVaultCircuitBreakerKeys,
    args: TriggerVaultCircuitBreakerIxArgs,
) -> std::io::Result<Instruction> {
    trigger_vault_circuit_breaker_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn trigger_vault_circuit_breaker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TriggerVaultCircuitBreakerAccounts<'_, '_>,
    args: TriggerVaultCircuitBreakerIxArgs,
) -> ProgramResult {
    let keys: TriggerVaultCircuitBreakerKeys = accounts.into();
    let ix = trigger_vault_circuit_breaker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn trigger_vault_circuit_breaker_invoke(
    accounts: TriggerVaultCircuitBreakerAccounts<'_, '_>,
    args: TriggerVaultCircuitBreakerIxArgs,
) -> ProgramResult {
    trigger_vault_circuit_breaker_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn trigger_vault_circuit_breaker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TriggerVaultCircuitBreakerAccounts<'_, '_>,
    args: TriggerVaultCircuitBreakerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TriggerVaultCircuitBreakerKeys = accounts.into();
    let ix = trigger_vault_circuit_breaker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn trigger_vault_circuit_breaker_invoke_signed(
    accounts: TriggerVaultCircuitBreakerAccounts<'_, '_>,
    args: TriggerVaultCircuitBreakerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    trigger_vault_circuit_breaker_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn trigger_vault_circuit_breaker_verify_account_keys(
    accounts: TriggerVaultCircuitBreakerAccounts<'_, '_>,
    keys: TriggerVaultCircuitBreakerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.manager.key, keys.manager),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn trigger_vault_circuit_breaker_verify_writable_privileges<'me, 'info>(
    accounts: TriggerVaultCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank_state,
        accounts.manager,
        accounts.vault_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn trigger_vault_circuit_breaker_verify_signer_privileges<'me, 'info>(
    accounts: TriggerVaultCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn trigger_vault_circuit_breaker_verify_account_privileges<'me, 'info>(
    accounts: TriggerVaultCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    trigger_vault_circuit_breaker_verify_writable_privileges(accounts)?;
    trigger_vault_circuit_breaker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ORACLE_STATE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOracleStateAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOracleStateKeys {
    pub admin: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
}
impl From<UpdateOracleStateAccounts<'_, '_>> for UpdateOracleStateKeys {
    fn from(accounts: UpdateOracleStateAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
        }
    }
}
impl From<UpdateOracleStateKeys> for [AccountMeta; UPDATE_ORACLE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOracleStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ORACLE_STATE_IX_ACCOUNTS_LEN]> for UpdateOracleStateKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ORACLE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateOracleStateAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ORACLE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOracleStateAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ORACLE_STATE_IX_ACCOUNTS_LEN]>
for UpdateOracleStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_ORACLE_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
        }
    }
}
pub const UPDATE_ORACLE_STATE_IX_DISCM: [u8; 8usize] = [
    180, 77, 182, 123, 139, 150, 244, 12,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOracleStateIxArgs {
    pub oracle_one: Option<Pubkey>,
    pub oracle_two: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOracleStateIxData(pub UpdateOracleStateIxArgs);
impl From<UpdateOracleStateIxArgs> for UpdateOracleStateIxData {
    fn from(args: UpdateOracleStateIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOracleStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let oracle_one: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let oracle_two: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateOracleStateIxArgs {
                oracle_one,
                oracle_two,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_one, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_two, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_oracle_state_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOracleStateKeys,
    args: UpdateOracleStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ORACLE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOracleStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_oracle_state_ix(
    keys: UpdateOracleStateKeys,
    args: UpdateOracleStateIxArgs,
) -> std::io::Result<Instruction> {
    update_oracle_state_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn update_oracle_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOracleStateAccounts<'_, '_>,
    args: UpdateOracleStateIxArgs,
) -> ProgramResult {
    let keys: UpdateOracleStateKeys = accounts.into();
    let ix = update_oracle_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_oracle_state_invoke(
    accounts: UpdateOracleStateAccounts<'_, '_>,
    args: UpdateOracleStateIxArgs,
) -> ProgramResult {
    update_oracle_state_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn update_oracle_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOracleStateAccounts<'_, '_>,
    args: UpdateOracleStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOracleStateKeys = accounts.into();
    let ix = update_oracle_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_oracle_state_invoke_signed(
    accounts: UpdateOracleStateAccounts<'_, '_>,
    args: UpdateOracleStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_oracle_state_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_oracle_state_verify_account_keys(
    accounts: UpdateOracleStateAccounts<'_, '_>,
    keys: UpdateOracleStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_oracle_state_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOracleStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.oracle_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_oracle_state_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOracleStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_oracle_state_verify_account_privileges<'me, 'info>(
    accounts: UpdateOracleStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_oracle_state_verify_writable_privileges(accounts)?;
    update_oracle_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTrancheConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTrancheConfigKeys {
    pub admin: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_state: Pubkey,
}
impl From<UpdateTrancheConfigAccounts<'_, '_>> for UpdateTrancheConfigKeys {
    fn from(accounts: UpdateTrancheConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            tranche_state: *accounts.tranche_state.key,
            bank_state: *accounts.bank_state.key,
        }
    }
}
impl From<UpdateTrancheConfigKeys>
for [AccountMeta; UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTrancheConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN]> for UpdateTrancheConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            tranche_state: pubkeys[1],
            bank_state: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateTrancheConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTrancheConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateTrancheConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            tranche_state: &arr[1],
            bank_state: &arr[2],
        }
    }
}
pub const UPDATE_TRANCHE_CONFIG_IX_DISCM: [u8; 8usize] = [
    17, 66, 9, 47, 165, 108, 26, 255,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateTrancheConfigIxArgs {
    pub base_leverage_factor: Option<u16>,
    pub target_percent_staked_bps: Option<u16>,
    pub leverage_slope: Option<u16>,
    pub early_unstake_fee_bps: Option<u16>,
    pub standard_unstake_fee_bps: Option<u16>,
    pub low_stake_threshold_bps: Option<u16>,
    pub low_stake_fee_bps: Option<u16>,
    pub low_stake_fee_enabled: Option<bool>,
    pub target_lockup_duration_secs: Option<i64>,
    pub recovery_yield_bps: Option<u16>,
    pub recovery_threshold_bps: Option<u16>,
    pub bootstrap_recovery_yield_bps: Option<u16>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTrancheConfigIxData(pub UpdateTrancheConfigIxArgs);
impl From<UpdateTrancheConfigIxArgs> for UpdateTrancheConfigIxData {
    fn from(args: UpdateTrancheConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTrancheConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TRANCHE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_leverage_factor: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let target_percent_staked_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let leverage_slope: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let early_unstake_fee_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let standard_unstake_fee_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let low_stake_threshold_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let low_stake_fee_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let low_stake_fee_enabled: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_lockup_duration_secs: Option<i64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let recovery_yield_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let recovery_threshold_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let bootstrap_recovery_yield_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateTrancheConfigIxArgs {
                base_leverage_factor,
                target_percent_staked_bps,
                leverage_slope,
                early_unstake_fee_bps,
                standard_unstake_fee_bps,
                low_stake_threshold_bps,
                low_stake_fee_bps,
                low_stake_fee_enabled,
                target_lockup_duration_secs,
                recovery_yield_bps,
                recovery_threshold_bps,
                bootstrap_recovery_yield_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TRANCHE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_leverage_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.target_percent_staked_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.leverage_slope, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.early_unstake_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.standard_unstake_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.low_stake_threshold_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.low_stake_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.low_stake_fee_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.target_lockup_duration_secs,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.recovery_yield_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.recovery_threshold_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.bootstrap_recovery_yield_bps,
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
pub fn update_tranche_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTrancheConfigKeys,
    args: UpdateTrancheConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTrancheConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_tranche_config_ix(
    keys: UpdateTrancheConfigKeys,
    args: UpdateTrancheConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_tranche_config_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn update_tranche_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTrancheConfigAccounts<'_, '_>,
    args: UpdateTrancheConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateTrancheConfigKeys = accounts.into();
    let ix = update_tranche_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_tranche_config_invoke(
    accounts: UpdateTrancheConfigAccounts<'_, '_>,
    args: UpdateTrancheConfigIxArgs,
) -> ProgramResult {
    update_tranche_config_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn update_tranche_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTrancheConfigAccounts<'_, '_>,
    args: UpdateTrancheConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTrancheConfigKeys = accounts.into();
    let ix = update_tranche_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_tranche_config_invoke_signed(
    accounts: UpdateTrancheConfigAccounts<'_, '_>,
    args: UpdateTrancheConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_tranche_config_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_tranche_config_verify_account_keys(
    accounts: UpdateTrancheConfigAccounts<'_, '_>,
    keys: UpdateTrancheConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_state.key, keys.bank_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_tranche_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTrancheConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.tranche_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_tranche_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTrancheConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_tranche_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateTrancheConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_tranche_config_verify_writable_privileges(accounts)?;
    update_tranche_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_YIELDING_AMOUNT_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct UpdateYieldingAmountAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub signer_ata: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub team_state_ata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateYieldingAmountKeys {
    pub signer: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub yielding_mint: Pubkey,
    pub signer_ata: Pubkey,
    pub team_state: Pubkey,
    pub team_state_ata: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<UpdateYieldingAmountAccounts<'_, '_>> for UpdateYieldingAmountKeys {
    fn from(accounts: UpdateYieldingAmountAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            yielding_mint: *accounts.yielding_mint.key,
            signer_ata: *accounts.signer_ata.key,
            team_state: *accounts.team_state.key,
            team_state_ata: *accounts.team_state_ata.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<UpdateYieldingAmountKeys>
for [AccountMeta; UPDATE_YIELDING_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateYieldingAmountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state_ata,
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
impl From<[Pubkey; UPDATE_YIELDING_AMOUNT_IX_ACCOUNTS_LEN]>
for UpdateYieldingAmountKeys {
    fn from(pubkeys: [Pubkey; UPDATE_YIELDING_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            yielding_vault_ata: pubkeys[4],
            yielding_mint: pubkeys[5],
            signer_ata: pubkeys[6],
            team_state: pubkeys[7],
            team_state_ata: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<UpdateYieldingAmountAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_YIELDING_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateYieldingAmountAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.yielding_mint.clone(),
            accounts.signer_ata.clone(),
            accounts.team_state.clone(),
            accounts.team_state_ata.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_YIELDING_AMOUNT_IX_ACCOUNTS_LEN]>
for UpdateYieldingAmountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_YIELDING_AMOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            yielding_vault_ata: &arr[4],
            yielding_mint: &arr[5],
            signer_ata: &arr[6],
            team_state: &arr[7],
            team_state_ata: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const UPDATE_YIELDING_AMOUNT_IX_DISCM: [u8; 8usize] = [
    148, 50, 173, 118, 153, 71, 25, 188,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateYieldingAmountIxArgs {
    pub push_all_pending_yield: bool,
    pub yield_to_push: Option<u64>,
    pub new_start_marker_unix_seconds: Option<u64>,
    pub synthetic_yield_push: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateYieldingAmountIxData(pub UpdateYieldingAmountIxArgs);
impl From<UpdateYieldingAmountIxArgs> for UpdateYieldingAmountIxData {
    fn from(args: UpdateYieldingAmountIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateYieldingAmountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_YIELDING_AMOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let push_all_pending_yield: bool = crate::borsh_de_or_default(&mut reader)?;
        let yield_to_push: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let new_start_marker_unix_seconds: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let synthetic_yield_push: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateYieldingAmountIxArgs {
                push_all_pending_yield,
                yield_to_push,
                new_start_marker_unix_seconds,
                synthetic_yield_push,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_YIELDING_AMOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.push_all_pending_yield, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.yield_to_push, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_start_marker_unix_seconds,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.synthetic_yield_push, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_yielding_amount_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateYieldingAmountKeys,
    args: UpdateYieldingAmountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_YIELDING_AMOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateYieldingAmountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_yielding_amount_ix(
    keys: UpdateYieldingAmountKeys,
    args: UpdateYieldingAmountIxArgs,
) -> std::io::Result<Instruction> {
    update_yielding_amount_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn update_yielding_amount_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateYieldingAmountAccounts<'_, '_>,
    args: UpdateYieldingAmountIxArgs,
) -> ProgramResult {
    let keys: UpdateYieldingAmountKeys = accounts.into();
    let ix = update_yielding_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_yielding_amount_invoke(
    accounts: UpdateYieldingAmountAccounts<'_, '_>,
    args: UpdateYieldingAmountIxArgs,
) -> ProgramResult {
    update_yielding_amount_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn update_yielding_amount_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateYieldingAmountAccounts<'_, '_>,
    args: UpdateYieldingAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateYieldingAmountKeys = accounts.into();
    let ix = update_yielding_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_yielding_amount_invoke_signed(
    accounts: UpdateYieldingAmountAccounts<'_, '_>,
    args: UpdateYieldingAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_yielding_amount_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_yielding_amount_verify_account_keys(
    accounts: UpdateYieldingAmountAccounts<'_, '_>,
    keys: UpdateYieldingAmountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.signer_ata.key, keys.signer_ata),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.team_state_ata.key, keys.team_state_ata),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_yielding_amount_verify_writable_privileges<'me, 'info>(
    accounts: UpdateYieldingAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.bank_state,
        accounts.vault_state,
        accounts.oracle_state,
        accounts.yielding_vault_ata,
        accounts.yielding_mint,
        accounts.signer_ata,
        accounts.team_state,
        accounts.team_state_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_yielding_amount_verify_signer_privileges<'me, 'info>(
    accounts: UpdateYieldingAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_yielding_amount_verify_account_privileges<'me, 'info>(
    accounts: UpdateYieldingAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_yielding_amount_verify_writable_privileges(accounts)?;
    update_yielding_amount_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_YIELDING_INFO_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct UpdateYieldingInfoAccounts<'me, 'info> {
    pub oracle: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateYieldingInfoKeys {
    pub oracle: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub yielding_mint: Pubkey,
    pub team_state: Pubkey,
    pub tranche_state: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateYieldingInfoAccounts<'_, '_>> for UpdateYieldingInfoKeys {
    fn from(accounts: UpdateYieldingInfoAccounts) -> Self {
        Self {
            oracle: *accounts.oracle.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            yielding_mint: *accounts.yielding_mint.key,
            team_state: *accounts.team_state.key,
            tranche_state: *accounts.tranche_state.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateYieldingInfoKeys>
for [AccountMeta; UPDATE_YIELDING_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateYieldingInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.junior_escrow_ata,
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
        ]
    }
}
impl From<[Pubkey; UPDATE_YIELDING_INFO_IX_ACCOUNTS_LEN]> for UpdateYieldingInfoKeys {
    fn from(pubkeys: [Pubkey; UPDATE_YIELDING_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            oracle: pubkeys[0],
            bank_state: pubkeys[1],
            bank_mint: pubkeys[2],
            vault_state: pubkeys[3],
            oracle_state: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            yielding_mint: pubkeys[6],
            team_state: pubkeys[7],
            tranche_state: pubkeys[8],
            junior_escrow_ata: pubkeys[9],
            token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<UpdateYieldingInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_YIELDING_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateYieldingInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.oracle.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.yielding_mint.clone(),
            accounts.team_state.clone(),
            accounts.tranche_state.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_YIELDING_INFO_IX_ACCOUNTS_LEN]>
for UpdateYieldingInfoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_YIELDING_INFO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            oracle: &arr[0],
            bank_state: &arr[1],
            bank_mint: &arr[2],
            vault_state: &arr[3],
            oracle_state: &arr[4],
            yielding_vault_ata: &arr[5],
            yielding_mint: &arr[6],
            team_state: &arr[7],
            tranche_state: &arr[8],
            junior_escrow_ata: &arr[9],
            token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const UPDATE_YIELDING_INFO_IX_DISCM: [u8; 8usize] = [
    22, 206, 207, 204, 134, 135, 45, 183,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateYieldingInfoIxArgs {
    pub new_price: u64,
    pub start_marker_unix_seconds: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateYieldingInfoIxData(pub UpdateYieldingInfoIxArgs);
impl From<UpdateYieldingInfoIxArgs> for UpdateYieldingInfoIxData {
    fn from(args: UpdateYieldingInfoIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateYieldingInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_YIELDING_INFO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_marker_unix_seconds: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateYieldingInfoIxArgs {
                new_price,
                start_marker_unix_seconds,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_YIELDING_INFO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_price, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.start_marker_unix_seconds,
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
pub fn update_yielding_info_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateYieldingInfoKeys,
    args: UpdateYieldingInfoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_YIELDING_INFO_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateYieldingInfoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_yielding_info_ix(
    keys: UpdateYieldingInfoKeys,
    args: UpdateYieldingInfoIxArgs,
) -> std::io::Result<Instruction> {
    update_yielding_info_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn update_yielding_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateYieldingInfoAccounts<'_, '_>,
    args: UpdateYieldingInfoIxArgs,
) -> ProgramResult {
    let keys: UpdateYieldingInfoKeys = accounts.into();
    let ix = update_yielding_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_yielding_info_invoke(
    accounts: UpdateYieldingInfoAccounts<'_, '_>,
    args: UpdateYieldingInfoIxArgs,
) -> ProgramResult {
    update_yielding_info_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn update_yielding_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateYieldingInfoAccounts<'_, '_>,
    args: UpdateYieldingInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateYieldingInfoKeys = accounts.into();
    let ix = update_yielding_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_yielding_info_invoke_signed(
    accounts: UpdateYieldingInfoAccounts<'_, '_>,
    args: UpdateYieldingInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_yielding_info_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_yielding_info_verify_account_keys(
    accounts: UpdateYieldingInfoAccounts<'_, '_>,
    keys: UpdateYieldingInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.oracle.key, keys.oracle),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_yielding_info_verify_writable_privileges<'me, 'info>(
    accounts: UpdateYieldingInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.oracle,
        accounts.bank_state,
        accounts.bank_mint,
        accounts.vault_state,
        accounts.oracle_state,
        accounts.yielding_vault_ata,
        accounts.yielding_mint,
        accounts.team_state,
        accounts.tranche_state,
        accounts.junior_escrow_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_yielding_info_verify_signer_privileges<'me, 'info>(
    accounts: UpdateYieldingInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.oracle] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_yielding_info_verify_account_privileges<'me, 'info>(
    accounts: UpdateYieldingInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_yielding_info_verify_writable_privileges(accounts)?;
    update_yielding_info_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_YIELDING_PRICE_GEN_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct UpdateYieldingPriceGenAccounts<'me, 'info> {
    pub oracle: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub team_state_ata: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub yielding_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateYieldingPriceGenKeys {
    pub oracle: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub yielding_mint: Pubkey,
    pub team_state: Pubkey,
    pub team_state_ata: Pubkey,
    pub tranche_state: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub token_program: Pubkey,
    pub yielding_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateYieldingPriceGenAccounts<'_, '_>> for UpdateYieldingPriceGenKeys {
    fn from(accounts: UpdateYieldingPriceGenAccounts) -> Self {
        Self {
            oracle: *accounts.oracle.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            yielding_mint: *accounts.yielding_mint.key,
            team_state: *accounts.team_state.key,
            team_state_ata: *accounts.team_state_ata.key,
            tranche_state: *accounts.tranche_state.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            token_program: *accounts.token_program.key,
            yielding_token_program: *accounts.yielding_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateYieldingPriceGenKeys>
for [AccountMeta; UPDATE_YIELDING_PRICE_GEN_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateYieldingPriceGenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.junior_escrow_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_token_program,
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
impl From<[Pubkey; UPDATE_YIELDING_PRICE_GEN_IX_ACCOUNTS_LEN]>
for UpdateYieldingPriceGenKeys {
    fn from(pubkeys: [Pubkey; UPDATE_YIELDING_PRICE_GEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            oracle: pubkeys[0],
            bank_state: pubkeys[1],
            bank_mint: pubkeys[2],
            vault_state: pubkeys[3],
            oracle_state: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            yielding_mint: pubkeys[6],
            team_state: pubkeys[7],
            team_state_ata: pubkeys[8],
            tranche_state: pubkeys[9],
            junior_escrow_ata: pubkeys[10],
            token_program: pubkeys[11],
            yielding_token_program: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<UpdateYieldingPriceGenAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_YIELDING_PRICE_GEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateYieldingPriceGenAccounts<'_, 'info>) -> Self {
        [
            accounts.oracle.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.yielding_mint.clone(),
            accounts.team_state.clone(),
            accounts.team_state_ata.clone(),
            accounts.tranche_state.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.token_program.clone(),
            accounts.yielding_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_YIELDING_PRICE_GEN_IX_ACCOUNTS_LEN]>
for UpdateYieldingPriceGenAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_YIELDING_PRICE_GEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            oracle: &arr[0],
            bank_state: &arr[1],
            bank_mint: &arr[2],
            vault_state: &arr[3],
            oracle_state: &arr[4],
            yielding_vault_ata: &arr[5],
            yielding_mint: &arr[6],
            team_state: &arr[7],
            team_state_ata: &arr[8],
            tranche_state: &arr[9],
            junior_escrow_ata: &arr[10],
            token_program: &arr[11],
            yielding_token_program: &arr[12],
            system_program: &arr[13],
        }
    }
}
pub const UPDATE_YIELDING_PRICE_GEN_IX_DISCM: [u8; 8usize] = [
    229, 137, 196, 95, 164, 248, 251, 139,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateYieldingPriceGenIxArgs {
    pub new_price: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateYieldingPriceGenIxData(pub UpdateYieldingPriceGenIxArgs);
impl From<UpdateYieldingPriceGenIxArgs> for UpdateYieldingPriceGenIxData {
    fn from(args: UpdateYieldingPriceGenIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateYieldingPriceGenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_YIELDING_PRICE_GEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateYieldingPriceGenIxArgs {
                new_price,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_YIELDING_PRICE_GEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_yielding_price_gen_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateYieldingPriceGenKeys,
    args: UpdateYieldingPriceGenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_YIELDING_PRICE_GEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateYieldingPriceGenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_yielding_price_gen_ix(
    keys: UpdateYieldingPriceGenKeys,
    args: UpdateYieldingPriceGenIxArgs,
) -> std::io::Result<Instruction> {
    update_yielding_price_gen_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn update_yielding_price_gen_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateYieldingPriceGenAccounts<'_, '_>,
    args: UpdateYieldingPriceGenIxArgs,
) -> ProgramResult {
    let keys: UpdateYieldingPriceGenKeys = accounts.into();
    let ix = update_yielding_price_gen_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_yielding_price_gen_invoke(
    accounts: UpdateYieldingPriceGenAccounts<'_, '_>,
    args: UpdateYieldingPriceGenIxArgs,
) -> ProgramResult {
    update_yielding_price_gen_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_yielding_price_gen_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateYieldingPriceGenAccounts<'_, '_>,
    args: UpdateYieldingPriceGenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateYieldingPriceGenKeys = accounts.into();
    let ix = update_yielding_price_gen_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_yielding_price_gen_invoke_signed(
    accounts: UpdateYieldingPriceGenAccounts<'_, '_>,
    args: UpdateYieldingPriceGenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_yielding_price_gen_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_yielding_price_gen_verify_account_keys(
    accounts: UpdateYieldingPriceGenAccounts<'_, '_>,
    keys: UpdateYieldingPriceGenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.oracle.key, keys.oracle),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.team_state_ata.key, keys.team_state_ata),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.yielding_token_program.key, keys.yielding_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_yielding_price_gen_verify_writable_privileges<'me, 'info>(
    accounts: UpdateYieldingPriceGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.oracle,
        accounts.bank_state,
        accounts.bank_mint,
        accounts.vault_state,
        accounts.oracle_state,
        accounts.yielding_vault_ata,
        accounts.yielding_mint,
        accounts.team_state,
        accounts.team_state_ata,
        accounts.tranche_state,
        accounts.junior_escrow_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_yielding_price_gen_verify_signer_privileges<'me, 'info>(
    accounts: UpdateYieldingPriceGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.oracle] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_yielding_price_gen_verify_account_privileges<'me, 'info>(
    accounts: UpdateYieldingPriceGenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_yielding_price_gen_verify_writable_privileges(accounts)?;
    update_yielding_price_gen_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const VAULT_REALLOCATOR_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct VaultReallocatorAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub source_vault_state: &'me AccountInfo<'info>,
    pub source_oracle_state: &'me AccountInfo<'info>,
    pub source_team_state: &'me AccountInfo<'info>,
    pub source_yielding_mint: &'me AccountInfo<'info>,
    pub source_vault_ata: &'me AccountInfo<'info>,
    pub destination_vault_state: &'me AccountInfo<'info>,
    pub destination_oracle_state: &'me AccountInfo<'info>,
    pub destination_team_state: &'me AccountInfo<'info>,
    pub destination_yielding_mint: &'me AccountInfo<'info>,
    pub destination_vault_ata: &'me AccountInfo<'info>,
    pub manager_ta: &'me AccountInfo<'info>,
    pub source_token_program: &'me AccountInfo<'info>,
    pub dest_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VaultReallocatorKeys {
    pub manager: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub source_vault_state: Pubkey,
    pub source_oracle_state: Pubkey,
    pub source_team_state: Pubkey,
    pub source_yielding_mint: Pubkey,
    pub source_vault_ata: Pubkey,
    pub destination_vault_state: Pubkey,
    pub destination_oracle_state: Pubkey,
    pub destination_team_state: Pubkey,
    pub destination_yielding_mint: Pubkey,
    pub destination_vault_ata: Pubkey,
    pub manager_ta: Pubkey,
    pub source_token_program: Pubkey,
    pub dest_token_program: Pubkey,
}
impl From<VaultReallocatorAccounts<'_, '_>> for VaultReallocatorKeys {
    fn from(accounts: VaultReallocatorAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            source_vault_state: *accounts.source_vault_state.key,
            source_oracle_state: *accounts.source_oracle_state.key,
            source_team_state: *accounts.source_team_state.key,
            source_yielding_mint: *accounts.source_yielding_mint.key,
            source_vault_ata: *accounts.source_vault_ata.key,
            destination_vault_state: *accounts.destination_vault_state.key,
            destination_oracle_state: *accounts.destination_oracle_state.key,
            destination_team_state: *accounts.destination_team_state.key,
            destination_yielding_mint: *accounts.destination_yielding_mint.key,
            destination_vault_ata: *accounts.destination_vault_ata.key,
            manager_ta: *accounts.manager_ta.key,
            source_token_program: *accounts.source_token_program.key,
            dest_token_program: *accounts.dest_token_program.key,
        }
    }
}
impl From<VaultReallocatorKeys> for [AccountMeta; VAULT_REALLOCATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: VaultReallocatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dest_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; VAULT_REALLOCATOR_IX_ACCOUNTS_LEN]> for VaultReallocatorKeys {
    fn from(pubkeys: [Pubkey; VAULT_REALLOCATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            bank_state: pubkeys[1],
            bank_mint: pubkeys[2],
            source_vault_state: pubkeys[3],
            source_oracle_state: pubkeys[4],
            source_team_state: pubkeys[5],
            source_yielding_mint: pubkeys[6],
            source_vault_ata: pubkeys[7],
            destination_vault_state: pubkeys[8],
            destination_oracle_state: pubkeys[9],
            destination_team_state: pubkeys[10],
            destination_yielding_mint: pubkeys[11],
            destination_vault_ata: pubkeys[12],
            manager_ta: pubkeys[13],
            source_token_program: pubkeys[14],
            dest_token_program: pubkeys[15],
        }
    }
}
impl<'info> From<VaultReallocatorAccounts<'_, 'info>>
for [AccountInfo<'info>; VAULT_REALLOCATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: VaultReallocatorAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.source_vault_state.clone(),
            accounts.source_oracle_state.clone(),
            accounts.source_team_state.clone(),
            accounts.source_yielding_mint.clone(),
            accounts.source_vault_ata.clone(),
            accounts.destination_vault_state.clone(),
            accounts.destination_oracle_state.clone(),
            accounts.destination_team_state.clone(),
            accounts.destination_yielding_mint.clone(),
            accounts.destination_vault_ata.clone(),
            accounts.manager_ta.clone(),
            accounts.source_token_program.clone(),
            accounts.dest_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; VAULT_REALLOCATOR_IX_ACCOUNTS_LEN]>
for VaultReallocatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; VAULT_REALLOCATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            bank_state: &arr[1],
            bank_mint: &arr[2],
            source_vault_state: &arr[3],
            source_oracle_state: &arr[4],
            source_team_state: &arr[5],
            source_yielding_mint: &arr[6],
            source_vault_ata: &arr[7],
            destination_vault_state: &arr[8],
            destination_oracle_state: &arr[9],
            destination_team_state: &arr[10],
            destination_yielding_mint: &arr[11],
            destination_vault_ata: &arr[12],
            manager_ta: &arr[13],
            source_token_program: &arr[14],
            dest_token_program: &arr[15],
        }
    }
}
pub const VAULT_REALLOCATOR_IX_DISCM: [u8; 8usize] = [
    227, 170, 141, 7, 227, 237, 150, 16,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VaultReallocatorIxArgs {
    pub decrement_amount: u64,
    pub increment_amount: u64,
    pub synthetic_reallocation: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultReallocatorIxData(pub VaultReallocatorIxArgs);
impl From<VaultReallocatorIxArgs> for VaultReallocatorIxData {
    fn from(args: VaultReallocatorIxArgs) -> Self {
        Self(args)
    }
}
impl VaultReallocatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_REALLOCATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let decrement_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let increment_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let synthetic_reallocation: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(VaultReallocatorIxArgs {
                decrement_amount,
                increment_amount,
                synthetic_reallocation,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_REALLOCATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.decrement_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.increment_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.synthetic_reallocation, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn vault_reallocator_ix_with_program_id(
    program_id: Pubkey,
    keys: VaultReallocatorKeys,
    args: VaultReallocatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VAULT_REALLOCATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: VaultReallocatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn vault_reallocator_ix(
    keys: VaultReallocatorKeys,
    args: VaultReallocatorIxArgs,
) -> std::io::Result<Instruction> {
    vault_reallocator_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn vault_reallocator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: VaultReallocatorAccounts<'_, '_>,
    args: VaultReallocatorIxArgs,
) -> ProgramResult {
    let keys: VaultReallocatorKeys = accounts.into();
    let ix = vault_reallocator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn vault_reallocator_invoke(
    accounts: VaultReallocatorAccounts<'_, '_>,
    args: VaultReallocatorIxArgs,
) -> ProgramResult {
    vault_reallocator_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn vault_reallocator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: VaultReallocatorAccounts<'_, '_>,
    args: VaultReallocatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: VaultReallocatorKeys = accounts.into();
    let ix = vault_reallocator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn vault_reallocator_invoke_signed(
    accounts: VaultReallocatorAccounts<'_, '_>,
    args: VaultReallocatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    vault_reallocator_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn vault_reallocator_verify_account_keys(
    accounts: VaultReallocatorAccounts<'_, '_>,
    keys: VaultReallocatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.source_vault_state.key, keys.source_vault_state),
        (*accounts.source_oracle_state.key, keys.source_oracle_state),
        (*accounts.source_team_state.key, keys.source_team_state),
        (*accounts.source_yielding_mint.key, keys.source_yielding_mint),
        (*accounts.source_vault_ata.key, keys.source_vault_ata),
        (*accounts.destination_vault_state.key, keys.destination_vault_state),
        (*accounts.destination_oracle_state.key, keys.destination_oracle_state),
        (*accounts.destination_team_state.key, keys.destination_team_state),
        (*accounts.destination_yielding_mint.key, keys.destination_yielding_mint),
        (*accounts.destination_vault_ata.key, keys.destination_vault_ata),
        (*accounts.manager_ta.key, keys.manager_ta),
        (*accounts.source_token_program.key, keys.source_token_program),
        (*accounts.dest_token_program.key, keys.dest_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn vault_reallocator_verify_writable_privileges<'me, 'info>(
    accounts: VaultReallocatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.bank_state,
        accounts.bank_mint,
        accounts.source_vault_state,
        accounts.source_team_state,
        accounts.source_vault_ata,
        accounts.destination_vault_state,
        accounts.destination_team_state,
        accounts.destination_vault_ata,
        accounts.manager_ta,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn vault_reallocator_verify_signer_privileges<'me, 'info>(
    accounts: VaultReallocatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn vault_reallocator_verify_account_privileges<'me, 'info>(
    accounts: VaultReallocatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    vault_reallocator_verify_writable_privileges(accounts)?;
    vault_reallocator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const VAULT_TRANSFER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct VaultTransferAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub source_vault_state: &'me AccountInfo<'info>,
    pub source_oracle_state: &'me AccountInfo<'info>,
    pub destination_vault_state: &'me AccountInfo<'info>,
    pub destination_oracle_state: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub source_vault_ata: &'me AccountInfo<'info>,
    pub destination_vault_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VaultTransferKeys {
    pub manager: Pubkey,
    pub bank_state: Pubkey,
    pub source_vault_state: Pubkey,
    pub source_oracle_state: Pubkey,
    pub destination_vault_state: Pubkey,
    pub destination_oracle_state: Pubkey,
    pub yielding_mint: Pubkey,
    pub source_vault_ata: Pubkey,
    pub destination_vault_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<VaultTransferAccounts<'_, '_>> for VaultTransferKeys {
    fn from(accounts: VaultTransferAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            bank_state: *accounts.bank_state.key,
            source_vault_state: *accounts.source_vault_state.key,
            source_oracle_state: *accounts.source_oracle_state.key,
            destination_vault_state: *accounts.destination_vault_state.key,
            destination_oracle_state: *accounts.destination_oracle_state.key,
            yielding_mint: *accounts.yielding_mint.key,
            source_vault_ata: *accounts.source_vault_ata.key,
            destination_vault_ata: *accounts.destination_vault_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<VaultTransferKeys> for [AccountMeta; VAULT_TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(keys: VaultTransferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_oracle_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault_ata,
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
impl From<[Pubkey; VAULT_TRANSFER_IX_ACCOUNTS_LEN]> for VaultTransferKeys {
    fn from(pubkeys: [Pubkey; VAULT_TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            bank_state: pubkeys[1],
            source_vault_state: pubkeys[2],
            source_oracle_state: pubkeys[3],
            destination_vault_state: pubkeys[4],
            destination_oracle_state: pubkeys[5],
            yielding_mint: pubkeys[6],
            source_vault_ata: pubkeys[7],
            destination_vault_ata: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<VaultTransferAccounts<'_, 'info>>
for [AccountInfo<'info>; VAULT_TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: VaultTransferAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.bank_state.clone(),
            accounts.source_vault_state.clone(),
            accounts.source_oracle_state.clone(),
            accounts.destination_vault_state.clone(),
            accounts.destination_oracle_state.clone(),
            accounts.yielding_mint.clone(),
            accounts.source_vault_ata.clone(),
            accounts.destination_vault_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; VAULT_TRANSFER_IX_ACCOUNTS_LEN]>
for VaultTransferAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; VAULT_TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            bank_state: &arr[1],
            source_vault_state: &arr[2],
            source_oracle_state: &arr[3],
            destination_vault_state: &arr[4],
            destination_oracle_state: &arr[5],
            yielding_mint: &arr[6],
            source_vault_ata: &arr[7],
            destination_vault_ata: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const VAULT_TRANSFER_IX_DISCM: [u8; 8usize] = [211, 125, 3, 105, 45, 33, 227, 214];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VaultTransferIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultTransferIxData(pub VaultTransferIxArgs);
impl From<VaultTransferIxArgs> for VaultTransferIxData {
    fn from(args: VaultTransferIxArgs) -> Self {
        Self(args)
    }
}
impl VaultTransferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_TRANSFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(VaultTransferIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_TRANSFER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn vault_transfer_ix_with_program_id(
    program_id: Pubkey,
    keys: VaultTransferKeys,
    args: VaultTransferIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VAULT_TRANSFER_IX_ACCOUNTS_LEN] = keys.into();
    let data: VaultTransferIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn vault_transfer_ix(
    keys: VaultTransferKeys,
    args: VaultTransferIxArgs,
) -> std::io::Result<Instruction> {
    vault_transfer_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn vault_transfer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: VaultTransferAccounts<'_, '_>,
    args: VaultTransferIxArgs,
) -> ProgramResult {
    let keys: VaultTransferKeys = accounts.into();
    let ix = vault_transfer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn vault_transfer_invoke(
    accounts: VaultTransferAccounts<'_, '_>,
    args: VaultTransferIxArgs,
) -> ProgramResult {
    vault_transfer_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn vault_transfer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: VaultTransferAccounts<'_, '_>,
    args: VaultTransferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: VaultTransferKeys = accounts.into();
    let ix = vault_transfer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn vault_transfer_invoke_signed(
    accounts: VaultTransferAccounts<'_, '_>,
    args: VaultTransferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    vault_transfer_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn vault_transfer_verify_account_keys(
    accounts: VaultTransferAccounts<'_, '_>,
    keys: VaultTransferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.source_vault_state.key, keys.source_vault_state),
        (*accounts.source_oracle_state.key, keys.source_oracle_state),
        (*accounts.destination_vault_state.key, keys.destination_vault_state),
        (*accounts.destination_oracle_state.key, keys.destination_oracle_state),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.source_vault_ata.key, keys.source_vault_ata),
        (*accounts.destination_vault_ata.key, keys.destination_vault_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn vault_transfer_verify_writable_privileges<'me, 'info>(
    accounts: VaultTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.bank_state,
        accounts.source_vault_state,
        accounts.destination_vault_state,
        accounts.yielding_mint,
        accounts.source_vault_ata,
        accounts.destination_vault_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn vault_transfer_verify_signer_privileges<'me, 'info>(
    accounts: VaultTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn vault_transfer_verify_account_privileges<'me, 'info>(
    accounts: VaultTransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    vault_transfer_verify_writable_privileges(accounts)?;
    vault_transfer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FROM_LP_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFromLpAccounts<'me, 'info> {
    pub yield_manager: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub team_state: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFromLpKeys {
    pub yield_manager: Pubkey,
    pub bank_state: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub team_state: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawFromLpAccounts<'_, '_>> for WithdrawFromLpKeys {
    fn from(accounts: WithdrawFromLpAccounts) -> Self {
        Self {
            yield_manager: *accounts.yield_manager.key,
            bank_state: *accounts.bank_state.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            team_state: *accounts.team_state.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawFromLpKeys> for [AccountMeta; WITHDRAW_FROM_LP_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFromLpKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.yield_manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.team_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
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
impl From<[Pubkey; WITHDRAW_FROM_LP_IX_ACCOUNTS_LEN]> for WithdrawFromLpKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FROM_LP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            yield_manager: pubkeys[0],
            bank_state: pubkeys[1],
            vault_state: pubkeys[2],
            oracle_state: pubkeys[3],
            team_state: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawFromLpAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FROM_LP_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFromLpAccounts<'_, 'info>) -> Self {
        [
            accounts.yield_manager.clone(),
            accounts.bank_state.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.team_state.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FROM_LP_IX_ACCOUNTS_LEN]>
for WithdrawFromLpAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_FROM_LP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            yield_manager: &arr[0],
            bank_state: &arr[1],
            vault_state: &arr[2],
            oracle_state: &arr[3],
            team_state: &arr[4],
            yielding_vault_ata: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_FROM_LP_IX_DISCM: [u8; 8usize] = [182, 255, 237, 26, 53, 53, 20, 167];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFromLpIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFromLpIxData(pub WithdrawFromLpIxArgs);
impl From<WithdrawFromLpIxArgs> for WithdrawFromLpIxData {
    fn from(args: WithdrawFromLpIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFromLpIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FROM_LP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawFromLpIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FROM_LP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_from_lp_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFromLpKeys,
    args: WithdrawFromLpIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FROM_LP_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFromLpIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_from_lp_ix(
    keys: WithdrawFromLpKeys,
    args: WithdrawFromLpIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_from_lp_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn withdraw_from_lp_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromLpAccounts<'_, '_>,
    args: WithdrawFromLpIxArgs,
) -> ProgramResult {
    let keys: WithdrawFromLpKeys = accounts.into();
    let ix = withdraw_from_lp_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_from_lp_invoke(
    accounts: WithdrawFromLpAccounts<'_, '_>,
    args: WithdrawFromLpIxArgs,
) -> ProgramResult {
    withdraw_from_lp_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn withdraw_from_lp_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromLpAccounts<'_, '_>,
    args: WithdrawFromLpIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFromLpKeys = accounts.into();
    let ix = withdraw_from_lp_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_from_lp_invoke_signed(
    accounts: WithdrawFromLpAccounts<'_, '_>,
    args: WithdrawFromLpIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_from_lp_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_from_lp_verify_account_keys(
    accounts: WithdrawFromLpAccounts<'_, '_>,
    keys: WithdrawFromLpKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.yield_manager.key, keys.yield_manager),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.team_state.key, keys.team_state),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_from_lp_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFromLpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.yield_manager,
        accounts.bank_state,
        accounts.vault_state,
        accounts.oracle_state,
        accounts.team_state,
        accounts.yielding_vault_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_from_lp_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFromLpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.yield_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_from_lp_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFromLpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_from_lp_verify_writable_privileges(accounts)?;
    withdraw_from_lp_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawTrancheFeesAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub destination_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawTrancheFeesKeys {
    pub signer: Pubkey,
    pub bank_state: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_mint: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub destination_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawTrancheFeesAccounts<'_, '_>> for WithdrawTrancheFeesKeys {
    fn from(accounts: WithdrawTrancheFeesAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            bank_state: *accounts.bank_state.key,
            tranche_state: *accounts.tranche_state.key,
            bank_mint: *accounts.bank_mint.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            destination_ata: *accounts.destination_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawTrancheFeesKeys>
for [AccountMeta; WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawTrancheFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.junior_escrow_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_ata,
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
impl From<[Pubkey; WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN]> for WithdrawTrancheFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            bank_state: pubkeys[1],
            tranche_state: pubkeys[2],
            bank_mint: pubkeys[3],
            junior_escrow_ata: pubkeys[4],
            destination_ata: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawTrancheFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawTrancheFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.bank_state.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_mint.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.destination_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN]>
for WithdrawTrancheFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            bank_state: &arr[1],
            tranche_state: &arr[2],
            bank_mint: &arr[3],
            junior_escrow_ata: &arr[4],
            destination_ata: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_TRANCHE_FEES_IX_DISCM: [u8; 8usize] = [
    152, 176, 238, 59, 106, 133, 152, 21,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawTrancheFeesIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawTrancheFeesIxData(pub WithdrawTrancheFeesIxArgs);
impl From<WithdrawTrancheFeesIxArgs> for WithdrawTrancheFeesIxData {
    fn from(args: WithdrawTrancheFeesIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawTrancheFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_TRANCHE_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawTrancheFeesIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_TRANCHE_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_tranche_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawTrancheFeesKeys,
    args: WithdrawTrancheFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawTrancheFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_tranche_fees_ix(
    keys: WithdrawTrancheFeesKeys,
    args: WithdrawTrancheFeesIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_tranche_fees_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn withdraw_tranche_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawTrancheFeesAccounts<'_, '_>,
    args: WithdrawTrancheFeesIxArgs,
) -> ProgramResult {
    let keys: WithdrawTrancheFeesKeys = accounts.into();
    let ix = withdraw_tranche_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_tranche_fees_invoke(
    accounts: WithdrawTrancheFeesAccounts<'_, '_>,
    args: WithdrawTrancheFeesIxArgs,
) -> ProgramResult {
    withdraw_tranche_fees_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn withdraw_tranche_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawTrancheFeesAccounts<'_, '_>,
    args: WithdrawTrancheFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawTrancheFeesKeys = accounts.into();
    let ix = withdraw_tranche_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_tranche_fees_invoke_signed(
    accounts: WithdrawTrancheFeesAccounts<'_, '_>,
    args: WithdrawTrancheFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_tranche_fees_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_tranche_fees_verify_account_keys(
    accounts: WithdrawTrancheFeesAccounts<'_, '_>,
    keys: WithdrawTrancheFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.destination_ata.key, keys.destination_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_tranche_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawTrancheFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.tranche_state,
        accounts.junior_escrow_ata,
        accounts.destination_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_tranche_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawTrancheFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_tranche_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawTrancheFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_tranche_fees_verify_writable_privileges(accounts)?;
    withdraw_tranche_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
