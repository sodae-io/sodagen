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
    ActivateCircuitBreaker,
    ApplyCapitalLoss(ApplyCapitalLossIxArgs),
    BurnForYieldingGen(BurnForYieldingGenIxArgs),
    CancelJuniorTrancheWithdraw,
    CancelUnstakeJunior,
    ChangeFees(ChangeFeesIxArgs),
    CreateAssetHolding(CreateAssetHoldingIxArgs),
    CreateBank(CreateBankIxArgs),
    CreateGenOracleAccount(CreateGenOracleAccountIxArgs),
    CreateGenTeamAccount(CreateGenTeamAccountIxArgs),
    CreateGenVault(CreateGenVaultIxArgs),
    CreateTrancheState(CreateTrancheStateIxArgs),
    CreateVault(CreateVaultIxArgs),
    DepositInLp(DepositInLpIxArgs),
    DisableCircuitBreaker,
    ExecuteDeposit(ExecuteDepositIxArgs),
    ExecuteTrancheDeposit(ExecuteTrancheDepositIxArgs),
    ExecuteTrancheWithdraw(ExecuteTrancheWithdrawIxArgs),
    ExecuteWithdraw(ExecuteWithdrawIxArgs),
    FulfillJuniorTrancheWithdraw,
    FulfillUnstakeJunior,
    GetNav,
    GetTrancheNav(GetTrancheNavIxArgs),
    InitMarginfiAccount(InitMarginfiAccountIxArgs),
    InstantUnstakeJunior(InstantUnstakeJuniorIxArgs),
    JupiterSwap(JupiterSwapIxArgs),
    ManagerRedepositAsset(ManagerRedepositAssetIxArgs),
    ManagerWithdrawAsset(ManagerWithdrawAssetIxArgs),
    MigrateLegacyVault(MigrateLegacyVaultIxArgs),
    MintWYieldingGen(MintWYieldingGenIxArgs),
    NameBankManager(NameBankManagerIxArgs),
    NameBankRiskManager(NameBankRiskManagerIxArgs),
    ProtocolInteraction(ProtocolInteractionIxArgs),
    RefreshAtomicLendingAccounting,
    RemoveAssetHolding,
    RequestJuniorTrancheWithdraw(RequestJuniorTrancheWithdrawIxArgs),
    RequestUnstakeJunior(RequestUnstakeJuniorIxArgs),
    SetAssetPriceOracle(SetAssetPriceOracleIxArgs),
    SetExternalLiquidity(SetExternalLiquidityIxArgs),
    SetVaultConfig(SetVaultConfigIxArgs),
    SetVaultConfigDetails(SetVaultConfigDetailsIxArgs),
    StakeJunior(StakeJuniorIxArgs),
    TeamDepositsFromInvest(TeamDepositsFromInvestIxArgs),
    TeamWithdrawsFees(TeamWithdrawsFeesIxArgs),
    TeamWithdrawsToInvest(TeamWithdrawsToInvestIxArgs),
    TransferMigratedMintAuthority(TransferMigratedMintAuthorityIxArgs),
    TriggerBankCircuitBreaker(TriggerBankCircuitBreakerIxArgs),
    TriggerVaultCircuitBreaker(TriggerVaultCircuitBreakerIxArgs),
    UpdateAssetPrice(UpdateAssetPriceIxArgs),
    UpdateConsensusOracle(UpdateConsensusOracleIxArgs),
    UpdateConsensusSigners(UpdateConsensusSignersIxArgs),
    UpdateOracleState(UpdateOracleStateIxArgs),
    UpdateTrancheConfig(UpdateTrancheConfigIxArgs),
    UpdateYieldingAmount(UpdateYieldingAmountIxArgs),
    UpdateYieldingInfo(UpdateYieldingInfoIxArgs),
    UpdateYieldingPriceGen(UpdateYieldingPriceGenIxArgs),
    VaultCreateTrancheState(VaultCreateTrancheStateIxArgs),
    VaultReallocation(VaultReallocationIxArgs),
    VaultReallocator(VaultReallocatorIxArgs),
    VaultTransfer(VaultTransferIxArgs),
    VaultUpdateTrancheConfig(VaultUpdateTrancheConfigIxArgs),
    VaultWithdrawTrancheFees(VaultWithdrawTrancheFeesIxArgs),
    WithdrawFromLp(WithdrawFromLpIxArgs),
    WithdrawProtocolFees(WithdrawProtocolFeesIxArgs),
    WithdrawTrancheFees(WithdrawTrancheFeesIxArgs),
    CrankNav,
    CrankPerformanceFees,
    MigrateIdleBankMintVault,
    SetProtocolFee(SetProtocolFeeIxArgs),
    ExecuteDepositFeeExempt(ExecuteDepositFeeExemptIxArgs),
    ExecuteShareSwap(ExecuteShareSwapIxArgs),
    ExecuteTrancheDepositFeeExempt(ExecuteTrancheDepositFeeExemptIxArgs),
    ExecuteTrancheWithdrawFeeExempt(ExecuteTrancheWithdrawFeeExemptIxArgs),
    ExecuteWithdrawFeeExempt(ExecuteWithdrawFeeExemptIxArgs),
    ExecuteWithdrawFromExternal(ExecuteWithdrawFromExternalIxArgs),
    ExecuteWithdrawFromExternalFeeExempt(ExecuteWithdrawFromExternalFeeExemptIxArgs),
    FulfillJuniorTrancheWithdrawFeeExempt(FulfillJuniorTrancheWithdrawFeeExemptIxArgs),
    InitializeVaultRoles(InitializeVaultRolesIxArgs),
    SweepLegacyJuniorEscrowExcess,
}
impl BankinecoProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ACTIVATE_CIRCUIT_BREAKER_IX_DISCM) {
            return Ok(Self::ActivateCircuitBreaker);
        }
        if buf.starts_with(&APPLY_CAPITAL_LOSS_IX_DISCM) {
            let mut reader = &buf[APPLY_CAPITAL_LOSS_IX_DISCM.len()..];
            let loss_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ApplyCapitalLoss(ApplyCapitalLossIxArgs {
                    loss_amount,
                }),
            );
        }
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
        if buf.starts_with(&CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM) {
            return Ok(Self::CancelJuniorTrancheWithdraw);
        }
        if buf.starts_with(&CANCEL_UNSTAKE_JUNIOR_IX_DISCM) {
            return Ok(Self::CancelUnstakeJunior);
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
        if buf.starts_with(&CREATE_ASSET_HOLDING_IX_DISCM) {
            let mut reader = &buf[CREATE_ASSET_HOLDING_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <CreateAssetHoldingArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::CreateAssetHolding(CreateAssetHoldingIxArgs { args }));
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
        if buf.starts_with(&CREATE_VAULT_IX_DISCM) {
            let mut reader = &buf[CREATE_VAULT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <VaultCreateVaultArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::CreateVault(CreateVaultIxArgs { args }));
        }
        if buf.starts_with(&DEPOSIT_IN_LP_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IN_LP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DepositInLp(DepositInLpIxArgs { amount }));
        }
        if buf.starts_with(&DISABLE_CIRCUIT_BREAKER_IX_DISCM) {
            return Ok(Self::DisableCircuitBreaker);
        }
        if buf.starts_with(&EXECUTE_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[EXECUTE_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ExecuteDeposit(ExecuteDepositIxArgs { amount }));
        }
        if buf.starts_with(&EXECUTE_TRANCHE_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[EXECUTE_TRANCHE_DEPOSIT_IX_DISCM.len()..];
            let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteTrancheDeposit(ExecuteTrancheDepositIxArgs {
                    kind,
                    amount,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_TRANCHE_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[EXECUTE_TRANCHE_WITHDRAW_IX_DISCM.len()..];
            let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteTrancheWithdraw(ExecuteTrancheWithdrawIxArgs {
                    kind,
                    share_amount,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[EXECUTE_WITHDRAW_IX_DISCM.len()..];
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteWithdraw(ExecuteWithdrawIxArgs {
                    share_amount,
                }),
            );
        }
        if buf.starts_with(&FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM) {
            return Ok(Self::FulfillJuniorTrancheWithdraw);
        }
        if buf.starts_with(&FULFILL_UNSTAKE_JUNIOR_IX_DISCM) {
            return Ok(Self::FulfillUnstakeJunior);
        }
        if buf.starts_with(&GET_NAV_IX_DISCM) {
            return Ok(Self::GetNav);
        }
        if buf.starts_with(&GET_TRANCHE_NAV_IX_DISCM) {
            let mut reader = &buf[GET_TRANCHE_NAV_IX_DISCM.len()..];
            let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::GetTrancheNav(GetTrancheNavIxArgs { kind }));
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
        if buf.starts_with(&JUPITER_SWAP_IX_DISCM) {
            let mut reader = &buf[JUPITER_SWAP_IX_DISCM.len()..];
            let refs = if reader.is_empty() {
                Default::default()
            } else {
                <InstructionRefs>::deserialize(&mut reader)?
            };
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <JupiterSwapArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::JupiterSwap(JupiterSwapIxArgs { refs, args }));
        }
        if buf.starts_with(&MANAGER_REDEPOSIT_ASSET_IX_DISCM) {
            let mut reader = &buf[MANAGER_REDEPOSIT_ASSET_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ManagerRedepositAssetArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ManagerRedepositAsset(ManagerRedepositAssetIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&MANAGER_WITHDRAW_ASSET_IX_DISCM) {
            let mut reader = &buf[MANAGER_WITHDRAW_ASSET_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ManagerWithdrawAsset(ManagerWithdrawAssetIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&MIGRATE_LEGACY_VAULT_IX_DISCM) {
            let mut reader = &buf[MIGRATE_LEGACY_VAULT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <MigrateLegacyVaultArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::MigrateLegacyVault(MigrateLegacyVaultIxArgs { args }));
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
        if buf.starts_with(&PROTOCOL_INTERACTION_IX_DISCM) {
            let mut reader = &buf[PROTOCOL_INTERACTION_IX_DISCM.len()..];
            let refs = if reader.is_empty() {
                Default::default()
            } else {
                <InstructionRefs>::deserialize(&mut reader)?
            };
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ProtocolInteractionArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ProtocolInteraction(ProtocolInteractionIxArgs {
                    refs,
                    args,
                }),
            );
        }
        if buf.starts_with(&REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_DISCM) {
            return Ok(Self::RefreshAtomicLendingAccounting);
        }
        if buf.starts_with(&REMOVE_ASSET_HOLDING_IX_DISCM) {
            return Ok(Self::RemoveAssetHolding);
        }
        if buf.starts_with(&REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM.len()..];
            let queue_id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RequestJuniorTrancheWithdraw(RequestJuniorTrancheWithdrawIxArgs {
                    queue_id,
                    share_amount,
                }),
            );
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
        if buf.starts_with(&SET_ASSET_PRICE_ORACLE_IX_DISCM) {
            let mut reader = &buf[SET_ASSET_PRICE_ORACLE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SetAssetPriceOracleArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::SetAssetPriceOracle(SetAssetPriceOracleIxArgs { args }));
        }
        if buf.starts_with(&SET_EXTERNAL_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[SET_EXTERNAL_LIQUIDITY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SetExternalLiquidityArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::SetExternalLiquidity(SetExternalLiquidityIxArgs { args }));
        }
        if buf.starts_with(&SET_VAULT_CONFIG_IX_DISCM) {
            let mut reader = &buf[SET_VAULT_CONFIG_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SetVaultConfigArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::SetVaultConfig(SetVaultConfigIxArgs { args }));
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
        if buf.starts_with(&TRANSFER_MIGRATED_MINT_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[TRANSFER_MIGRATED_MINT_AUTHORITY_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <TransferMigratedMintAuthorityArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::TransferMigratedMintAuthority(TransferMigratedMintAuthorityIxArgs {
                    args,
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
        if buf.starts_with(&UPDATE_ASSET_PRICE_IX_DISCM) {
            let mut reader = &buf[UPDATE_ASSET_PRICE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateAssetPriceArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::UpdateAssetPrice(UpdateAssetPriceIxArgs { args }));
        }
        if buf.starts_with(&UPDATE_CONSENSUS_ORACLE_IX_DISCM) {
            let mut reader = &buf[UPDATE_CONSENSUS_ORACLE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateConsensusOracleArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateConsensusOracle(UpdateConsensusOracleIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CONSENSUS_SIGNERS_IX_DISCM) {
            let mut reader = &buf[UPDATE_CONSENSUS_SIGNERS_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateConsensusSignersArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateConsensusSigners(UpdateConsensusSignersIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ORACLE_STATE_IX_DISCM) {
            let mut reader = &buf[UPDATE_ORACLE_STATE_IX_DISCM.len()..];
            let oracles: Option<Vec<Pubkey>> = crate::borsh_de_or_default(&mut reader)?;
            let price_max_gap_bps: Option<u16> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let price_max_ts_gap: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateOracleState(UpdateOracleStateIxArgs {
                    oracles,
                    price_max_gap_bps,
                    price_max_ts_gap,
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
            let pending_yield: u64 = crate::borsh_de_or_default(&mut reader)?;
            let start_marker_unix_seconds: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateYieldingInfo(UpdateYieldingInfoIxArgs {
                    pending_yield,
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
        if buf.starts_with(&VAULT_CREATE_TRANCHE_STATE_IX_DISCM) {
            let mut reader = &buf[VAULT_CREATE_TRANCHE_STATE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <VaultCreateTrancheStateArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::VaultCreateTrancheState(VaultCreateTrancheStateIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&VAULT_REALLOCATION_IX_DISCM) {
            let mut reader = &buf[VAULT_REALLOCATION_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <VaultReallocationArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::VaultReallocation(VaultReallocationIxArgs { args }));
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
        if buf.starts_with(&VAULT_UPDATE_TRANCHE_CONFIG_IX_DISCM) {
            let mut reader = &buf[VAULT_UPDATE_TRANCHE_CONFIG_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <VaultUpdateTrancheConfigArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::VaultUpdateTrancheConfig(VaultUpdateTrancheConfigIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&VAULT_WITHDRAW_TRANCHE_FEES_IX_DISCM) {
            let mut reader = &buf[VAULT_WITHDRAW_TRANCHE_FEES_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::VaultWithdrawTrancheFees(VaultWithdrawTrancheFeesIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_FROM_LP_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_LP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::WithdrawFromLp(WithdrawFromLpIxArgs { amount }));
        }
        if buf.starts_with(&WITHDRAW_PROTOCOL_FEES_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_PROTOCOL_FEES_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawProtocolFees(WithdrawProtocolFeesIxArgs {
                    amount,
                }),
            );
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
        if buf.starts_with(&CRANK_NAV_IX_DISCM) {
            return Ok(Self::CrankNav);
        }
        if buf.starts_with(&CRANK_PERFORMANCE_FEES_IX_DISCM) {
            return Ok(Self::CrankPerformanceFees);
        }
        if buf.starts_with(&MIGRATE_IDLE_BANK_MINT_VAULT_IX_DISCM) {
            return Ok(Self::MigrateIdleBankMintVault);
        }
        if buf.starts_with(&SET_PROTOCOL_FEE_IX_DISCM) {
            let mut reader = &buf[SET_PROTOCOL_FEE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SetProtocolFeeArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::SetProtocolFee(SetProtocolFeeIxArgs { args }));
        }
        if buf.starts_with(&EXECUTE_DEPOSIT_FEE_EXEMPT_IX_DISCM) {
            let mut reader = &buf[EXECUTE_DEPOSIT_FEE_EXEMPT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteDepositFeeExempt(ExecuteDepositFeeExemptIxArgs {
                    amount,
                    signer_seeds,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_SHARE_SWAP_IX_DISCM) {
            let mut reader = &buf[EXECUTE_SHARE_SWAP_IX_DISCM.len()..];
            let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteShareSwap(ExecuteShareSwapIxArgs {
                    kind,
                    share_amount,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_DISCM) {
            let mut reader = &buf[EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_DISCM.len()..];
            let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteTrancheDepositFeeExempt(ExecuteTrancheDepositFeeExemptIxArgs {
                    kind,
                    amount,
                    signer_seeds,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM) {
            let mut reader = &buf[EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM.len()..];
            let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteTrancheWithdrawFeeExempt(ExecuteTrancheWithdrawFeeExemptIxArgs {
                    kind,
                    share_amount,
                    signer_seeds,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_WITHDRAW_FEE_EXEMPT_IX_DISCM) {
            let mut reader = &buf[EXECUTE_WITHDRAW_FEE_EXEMPT_IX_DISCM.len()..];
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteWithdrawFeeExempt(ExecuteWithdrawFeeExemptIxArgs {
                    share_amount,
                    signer_seeds,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_DISCM) {
            let mut reader = &buf[EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_DISCM.len()..];
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let external_withdraw_ix_refs: Option<InstructionRefs> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let external_liquidity_source: Option<u8> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ExecuteWithdrawFromExternal(ExecuteWithdrawFromExternalIxArgs {
                    share_amount,
                    external_withdraw_ix_refs,
                    external_liquidity_source,
                }),
            );
        }
        if buf.starts_with(&EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_DISCM) {
            let mut reader = &buf[EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_DISCM
                .len()..];
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let external_withdraw_ix_refs: Option<InstructionRefs> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let external_liquidity_source: Option<u8> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ExecuteWithdrawFromExternalFeeExempt(ExecuteWithdrawFromExternalFeeExemptIxArgs {
                    share_amount,
                    external_withdraw_ix_refs,
                    external_liquidity_source,
                    signer_seeds,
                }),
            );
        }
        if buf.starts_with(&FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM) {
            let mut reader = &buf[FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM
                .len()..];
            let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FulfillJuniorTrancheWithdrawFeeExempt(FulfillJuniorTrancheWithdrawFeeExemptIxArgs {
                    signer_seeds,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_VAULT_ROLES_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_VAULT_ROLES_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeVaultRolesArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::InitializeVaultRoles(InitializeVaultRolesIxArgs { args }));
        }
        if buf.starts_with(&SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_DISCM) {
            return Ok(Self::SweepLegacyJuniorEscrowExcess);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::ActivateCircuitBreaker => {
                writer.write_all(&ACTIVATE_CIRCUIT_BREAKER_IX_DISCM)
            }
            Self::ApplyCapitalLoss(args) => {
                writer.write_all(&APPLY_CAPITAL_LOSS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.loss_amount, &mut writer)?;
                Ok(())
            }
            Self::BurnForYieldingGen(args) => {
                writer.write_all(&BURN_FOR_YIELDING_GEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_to_burn, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_yielding_withdrawn,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CancelJuniorTrancheWithdraw => {
                writer.write_all(&CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM)
            }
            Self::CancelUnstakeJunior => {
                writer.write_all(&CANCEL_UNSTAKE_JUNIOR_IX_DISCM)
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
            Self::CreateAssetHolding(args) => {
                writer.write_all(&CREATE_ASSET_HOLDING_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
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
            Self::CreateVault(args) => {
                writer.write_all(&CREATE_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::DepositInLp(args) => {
                writer.write_all(&DEPOSIT_IN_LP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::DisableCircuitBreaker => {
                writer.write_all(&DISABLE_CIRCUIT_BREAKER_IX_DISCM)
            }
            Self::ExecuteDeposit(args) => {
                writer.write_all(&EXECUTE_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::ExecuteTrancheDeposit(args) => {
                writer.write_all(&EXECUTE_TRANCHE_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.kind, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::ExecuteTrancheWithdraw(args) => {
                writer.write_all(&EXECUTE_TRANCHE_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.kind, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                Ok(())
            }
            Self::ExecuteWithdraw(args) => {
                writer.write_all(&EXECUTE_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                Ok(())
            }
            Self::FulfillJuniorTrancheWithdraw => {
                writer.write_all(&FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM)
            }
            Self::FulfillUnstakeJunior => {
                writer.write_all(&FULFILL_UNSTAKE_JUNIOR_IX_DISCM)
            }
            Self::GetNav => writer.write_all(&GET_NAV_IX_DISCM),
            Self::GetTrancheNav(args) => {
                writer.write_all(&GET_TRANCHE_NAV_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.kind, &mut writer)?;
                Ok(())
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
            Self::JupiterSwap(args) => {
                writer.write_all(&JUPITER_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.refs, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ManagerRedepositAsset(args) => {
                writer.write_all(&MANAGER_REDEPOSIT_ASSET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ManagerWithdrawAsset(args) => {
                writer.write_all(&MANAGER_WITHDRAW_ASSET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::MigrateLegacyVault(args) => {
                writer.write_all(&MIGRATE_LEGACY_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
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
            Self::ProtocolInteraction(args) => {
                writer.write_all(&PROTOCOL_INTERACTION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.refs, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::RefreshAtomicLendingAccounting => {
                writer.write_all(&REFRESH_ATOMIC_LENDING_ACCOUNTING_IX_DISCM)
            }
            Self::RemoveAssetHolding => writer.write_all(&REMOVE_ASSET_HOLDING_IX_DISCM),
            Self::RequestJuniorTrancheWithdraw(args) => {
                writer.write_all(&REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.queue_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                Ok(())
            }
            Self::RequestUnstakeJunior(args) => {
                writer.write_all(&REQUEST_UNSTAKE_JUNIOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.queue_id, &mut writer)?;
                Ok(())
            }
            Self::SetAssetPriceOracle(args) => {
                writer.write_all(&SET_ASSET_PRICE_ORACLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::SetExternalLiquidity(args) => {
                writer.write_all(&SET_EXTERNAL_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::SetVaultConfig(args) => {
                writer.write_all(&SET_VAULT_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
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
            Self::TransferMigratedMintAuthority(args) => {
                writer.write_all(&TRANSFER_MIGRATED_MINT_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
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
            Self::UpdateAssetPrice(args) => {
                writer.write_all(&UPDATE_ASSET_PRICE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdateConsensusOracle(args) => {
                writer.write_all(&UPDATE_CONSENSUS_ORACLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdateConsensusSigners(args) => {
                writer.write_all(&UPDATE_CONSENSUS_SIGNERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdateOracleState(args) => {
                writer.write_all(&UPDATE_ORACLE_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.oracles, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price_max_gap_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price_max_ts_gap, &mut writer)?;
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
                borsh::BorshSerialize::serialize(&args.pending_yield, &mut writer)?;
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
            Self::VaultCreateTrancheState(args) => {
                writer.write_all(&VAULT_CREATE_TRANCHE_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::VaultReallocation(args) => {
                writer.write_all(&VAULT_REALLOCATION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
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
            Self::VaultUpdateTrancheConfig(args) => {
                writer.write_all(&VAULT_UPDATE_TRANCHE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::VaultWithdrawTrancheFees(args) => {
                writer.write_all(&VAULT_WITHDRAW_TRANCHE_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFromLp(args) => {
                writer.write_all(&WITHDRAW_FROM_LP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawProtocolFees(args) => {
                writer.write_all(&WITHDRAW_PROTOCOL_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawTrancheFees(args) => {
                writer.write_all(&WITHDRAW_TRANCHE_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::CrankNav => writer.write_all(&CRANK_NAV_IX_DISCM),
            Self::CrankPerformanceFees => {
                writer.write_all(&CRANK_PERFORMANCE_FEES_IX_DISCM)
            }
            Self::MigrateIdleBankMintVault => {
                writer.write_all(&MIGRATE_IDLE_BANK_MINT_VAULT_IX_DISCM)
            }
            Self::SetProtocolFee(args) => {
                writer.write_all(&SET_PROTOCOL_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ExecuteDepositFeeExempt(args) => {
                writer.write_all(&EXECUTE_DEPOSIT_FEE_EXEMPT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.signer_seeds, &mut writer)?;
                Ok(())
            }
            Self::ExecuteShareSwap(args) => {
                writer.write_all(&EXECUTE_SHARE_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.kind, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                Ok(())
            }
            Self::ExecuteTrancheDepositFeeExempt(args) => {
                writer.write_all(&EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.kind, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.signer_seeds, &mut writer)?;
                Ok(())
            }
            Self::ExecuteTrancheWithdrawFeeExempt(args) => {
                writer.write_all(&EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.kind, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.signer_seeds, &mut writer)?;
                Ok(())
            }
            Self::ExecuteWithdrawFeeExempt(args) => {
                writer.write_all(&EXECUTE_WITHDRAW_FEE_EXEMPT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.signer_seeds, &mut writer)?;
                Ok(())
            }
            Self::ExecuteWithdrawFromExternal(args) => {
                writer.write_all(&EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.external_withdraw_ix_refs,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.external_liquidity_source,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ExecuteWithdrawFromExternalFeeExempt(args) => {
                writer.write_all(&EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.external_withdraw_ix_refs,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.external_liquidity_source,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.signer_seeds, &mut writer)?;
                Ok(())
            }
            Self::FulfillJuniorTrancheWithdrawFeeExempt(args) => {
                writer.write_all(&FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.signer_seeds, &mut writer)?;
                Ok(())
            }
            Self::InitializeVaultRoles(args) => {
                writer.write_all(&INITIALIZE_VAULT_ROLES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::SweepLegacyJuniorEscrowExcess => {
                writer.write_all(&SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_DISCM)
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
pub const ACTIVATE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ActivateCircuitBreakerAccounts<'me, 'info> {
    pub cb_trigger: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ActivateCircuitBreakerKeys {
    pub cb_trigger: Pubkey,
    pub vault: Pubkey,
}
impl From<ActivateCircuitBreakerAccounts<'_, '_>> for ActivateCircuitBreakerKeys {
    fn from(accounts: ActivateCircuitBreakerAccounts) -> Self {
        Self {
            cb_trigger: *accounts.cb_trigger.key,
            vault: *accounts.vault.key,
        }
    }
}
impl From<ActivateCircuitBreakerKeys>
for [AccountMeta; ACTIVATE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] {
    fn from(keys: ActivateCircuitBreakerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.cb_trigger,
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
impl From<[Pubkey; ACTIVATE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]>
for ActivateCircuitBreakerKeys {
    fn from(pubkeys: [Pubkey; ACTIVATE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            cb_trigger: pubkeys[0],
            vault: pubkeys[1],
        }
    }
}
impl<'info> From<ActivateCircuitBreakerAccounts<'_, 'info>>
for [AccountInfo<'info>; ACTIVATE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ActivateCircuitBreakerAccounts<'_, 'info>) -> Self {
        [accounts.cb_trigger.clone(), accounts.vault.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ACTIVATE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]>
for ActivateCircuitBreakerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ACTIVATE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            cb_trigger: &arr[0],
            vault: &arr[1],
        }
    }
}
pub const ACTIVATE_CIRCUIT_BREAKER_IX_DISCM: [u8; 8usize] = [
    67, 240, 159, 113, 7, 138, 247, 174,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ActivateCircuitBreakerIxData;
impl ActivateCircuitBreakerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACTIVATE_CIRCUIT_BREAKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACTIVATE_CIRCUIT_BREAKER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn activate_circuit_breaker_ix_with_program_id(
    program_id: Pubkey,
    keys: ActivateCircuitBreakerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACTIVATE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ActivateCircuitBreakerIxData.try_to_vec()?,
    })
}
pub fn activate_circuit_breaker_ix(
    keys: ActivateCircuitBreakerKeys,
) -> std::io::Result<Instruction> {
    activate_circuit_breaker_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn activate_circuit_breaker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ActivateCircuitBreakerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ActivateCircuitBreakerKeys = accounts.into();
    let ix = activate_circuit_breaker_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn activate_circuit_breaker_invoke(
    accounts: ActivateCircuitBreakerAccounts<'_, '_>,
) -> ProgramResult {
    activate_circuit_breaker_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn activate_circuit_breaker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ActivateCircuitBreakerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ActivateCircuitBreakerKeys = accounts.into();
    let ix = activate_circuit_breaker_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn activate_circuit_breaker_invoke_signed(
    accounts: ActivateCircuitBreakerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    activate_circuit_breaker_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn activate_circuit_breaker_verify_account_keys(
    accounts: ActivateCircuitBreakerAccounts<'_, '_>,
    keys: ActivateCircuitBreakerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.cb_trigger.key, keys.cb_trigger),
        (*accounts.vault.key, keys.vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn activate_circuit_breaker_verify_writable_privileges<'me, 'info>(
    accounts: ActivateCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn activate_circuit_breaker_verify_signer_privileges<'me, 'info>(
    accounts: ActivateCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.cb_trigger] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn activate_circuit_breaker_verify_account_privileges<'me, 'info>(
    accounts: ActivateCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    activate_circuit_breaker_verify_writable_privileges(accounts)?;
    activate_circuit_breaker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const APPLY_CAPITAL_LOSS_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ApplyCapitalLossAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub vault_state: &'me AccountInfo<'info>,
    pub oracle_state: &'me AccountInfo<'info>,
    pub yielding_vault_ata: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ApplyCapitalLossKeys {
    pub admin: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub vault_state: Pubkey,
    pub oracle_state: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub tranche_state: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ApplyCapitalLossAccounts<'_, '_>> for ApplyCapitalLossKeys {
    fn from(accounts: ApplyCapitalLossAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            vault_state: *accounts.vault_state.key,
            oracle_state: *accounts.oracle_state.key,
            yielding_vault_ata: *accounts.yielding_vault_ata.key,
            tranche_state: *accounts.tranche_state.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ApplyCapitalLossKeys> for [AccountMeta; APPLY_CAPITAL_LOSS_IX_ACCOUNTS_LEN] {
    fn from(keys: ApplyCapitalLossKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_vault_ata,
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
impl From<[Pubkey; APPLY_CAPITAL_LOSS_IX_ACCOUNTS_LEN]> for ApplyCapitalLossKeys {
    fn from(pubkeys: [Pubkey; APPLY_CAPITAL_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            bank_state: pubkeys[1],
            bank_mint: pubkeys[2],
            vault_state: pubkeys[3],
            oracle_state: pubkeys[4],
            yielding_vault_ata: pubkeys[5],
            tranche_state: pubkeys[6],
            junior_escrow_ata: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<ApplyCapitalLossAccounts<'_, 'info>>
for [AccountInfo<'info>; APPLY_CAPITAL_LOSS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ApplyCapitalLossAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.vault_state.clone(),
            accounts.oracle_state.clone(),
            accounts.yielding_vault_ata.clone(),
            accounts.tranche_state.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; APPLY_CAPITAL_LOSS_IX_ACCOUNTS_LEN]>
for ApplyCapitalLossAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; APPLY_CAPITAL_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            bank_state: &arr[1],
            bank_mint: &arr[2],
            vault_state: &arr[3],
            oracle_state: &arr[4],
            yielding_vault_ata: &arr[5],
            tranche_state: &arr[6],
            junior_escrow_ata: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const APPLY_CAPITAL_LOSS_IX_DISCM: [u8; 8usize] = [
    249, 217, 29, 182, 55, 204, 74, 138,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplyCapitalLossIxArgs {
    pub loss_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApplyCapitalLossIxData(pub ApplyCapitalLossIxArgs);
impl From<ApplyCapitalLossIxArgs> for ApplyCapitalLossIxData {
    fn from(args: ApplyCapitalLossIxArgs) -> Self {
        Self(args)
    }
}
impl ApplyCapitalLossIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != APPLY_CAPITAL_LOSS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let loss_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ApplyCapitalLossIxArgs {
                loss_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&APPLY_CAPITAL_LOSS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.loss_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn apply_capital_loss_ix_with_program_id(
    program_id: Pubkey,
    keys: ApplyCapitalLossKeys,
    args: ApplyCapitalLossIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; APPLY_CAPITAL_LOSS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ApplyCapitalLossIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn apply_capital_loss_ix(
    keys: ApplyCapitalLossKeys,
    args: ApplyCapitalLossIxArgs,
) -> std::io::Result<Instruction> {
    apply_capital_loss_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn apply_capital_loss_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ApplyCapitalLossAccounts<'_, '_>,
    args: ApplyCapitalLossIxArgs,
) -> ProgramResult {
    let keys: ApplyCapitalLossKeys = accounts.into();
    let ix = apply_capital_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn apply_capital_loss_invoke(
    accounts: ApplyCapitalLossAccounts<'_, '_>,
    args: ApplyCapitalLossIxArgs,
) -> ProgramResult {
    apply_capital_loss_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn apply_capital_loss_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ApplyCapitalLossAccounts<'_, '_>,
    args: ApplyCapitalLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ApplyCapitalLossKeys = accounts.into();
    let ix = apply_capital_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn apply_capital_loss_invoke_signed(
    accounts: ApplyCapitalLossAccounts<'_, '_>,
    args: ApplyCapitalLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    apply_capital_loss_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn apply_capital_loss_verify_account_keys(
    accounts: ApplyCapitalLossAccounts<'_, '_>,
    keys: ApplyCapitalLossKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.vault_state.key, keys.vault_state),
        (*accounts.oracle_state.key, keys.oracle_state),
        (*accounts.yielding_vault_ata.key, keys.yielding_vault_ata),
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
pub fn apply_capital_loss_verify_writable_privileges<'me, 'info>(
    accounts: ApplyCapitalLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.bank_state,
        accounts.bank_mint,
        accounts.vault_state,
        accounts.yielding_vault_ata,
        accounts.tranche_state,
        accounts.junior_escrow_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn apply_capital_loss_verify_signer_privileges<'me, 'info>(
    accounts: ApplyCapitalLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn apply_capital_loss_verify_account_privileges<'me, 'info>(
    accounts: ApplyCapitalLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    apply_capital_loss_verify_writable_privileges(accounts)?;
    apply_capital_loss_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_FOR_YIELDING_GEN_IX_ACCOUNTS_LEN: usize = 16;
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
    pub tranche_state: &'me AccountInfo<'info>,
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
    pub tranche_state: Pubkey,
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
            tranche_state: *accounts.tranche_state.key,
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
            AccountMeta {
                pubkey: keys.tranche_state,
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
            tranche_state: pubkeys[15],
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
            accounts.tranche_state.clone(),
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
            tranche_state: &arr[15],
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
        (*accounts.tranche_state.key, keys.tranche_state),
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
pub const CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CancelJuniorTrancheWithdrawAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub withdrawal_queue: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub user_input_ata: &'me AccountInfo<'info>,
    pub queue_input_ata: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelJuniorTrancheWithdrawKeys {
    pub user: Pubkey,
    pub payer: Pubkey,
    pub vault: Pubkey,
    pub withdrawal_queue: Pubkey,
    pub input_mint: Pubkey,
    pub user_input_ata: Pubkey,
    pub queue_input_ata: Pubkey,
    pub share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CancelJuniorTrancheWithdrawAccounts<'_, '_>>
for CancelJuniorTrancheWithdrawKeys {
    fn from(accounts: CancelJuniorTrancheWithdrawAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            payer: *accounts.payer.key,
            vault: *accounts.vault.key,
            withdrawal_queue: *accounts.withdrawal_queue.key,
            input_mint: *accounts.input_mint.key,
            user_input_ata: *accounts.user_input_ata.key,
            queue_input_ata: *accounts.queue_input_ata.key,
            share_token_program: *accounts.share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CancelJuniorTrancheWithdrawKeys>
for [AccountMeta; CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelJuniorTrancheWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdrawal_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.queue_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]>
for CancelJuniorTrancheWithdrawKeys {
    fn from(pubkeys: [Pubkey; CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            payer: pubkeys[1],
            vault: pubkeys[2],
            withdrawal_queue: pubkeys[3],
            input_mint: pubkeys[4],
            user_input_ata: pubkeys[5],
            queue_input_ata: pubkeys[6],
            share_token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<CancelJuniorTrancheWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelJuniorTrancheWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.payer.clone(),
            accounts.vault.clone(),
            accounts.withdrawal_queue.clone(),
            accounts.input_mint.clone(),
            accounts.user_input_ata.clone(),
            accounts.queue_input_ata.clone(),
            accounts.share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]>
for CancelJuniorTrancheWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            payer: &arr[1],
            vault: &arr[2],
            withdrawal_queue: &arr[3],
            input_mint: &arr[4],
            user_input_ata: &arr[5],
            queue_input_ata: &arr[6],
            share_token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    170, 64, 224, 22, 113, 208, 41, 14,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelJuniorTrancheWithdrawIxData;
impl CancelJuniorTrancheWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_junior_tranche_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelJuniorTrancheWithdrawKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelJuniorTrancheWithdrawIxData.try_to_vec()?,
    })
}
pub fn cancel_junior_tranche_withdraw_ix(
    keys: CancelJuniorTrancheWithdrawKeys,
) -> std::io::Result<Instruction> {
    cancel_junior_tranche_withdraw_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn cancel_junior_tranche_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelJuniorTrancheWithdrawAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelJuniorTrancheWithdrawKeys = accounts.into();
    let ix = cancel_junior_tranche_withdraw_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_junior_tranche_withdraw_invoke(
    accounts: CancelJuniorTrancheWithdrawAccounts<'_, '_>,
) -> ProgramResult {
    cancel_junior_tranche_withdraw_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn cancel_junior_tranche_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelJuniorTrancheWithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelJuniorTrancheWithdrawKeys = accounts.into();
    let ix = cancel_junior_tranche_withdraw_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_junior_tranche_withdraw_invoke_signed(
    accounts: CancelJuniorTrancheWithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_junior_tranche_withdraw_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cancel_junior_tranche_withdraw_verify_account_keys(
    accounts: CancelJuniorTrancheWithdrawAccounts<'_, '_>,
    keys: CancelJuniorTrancheWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.payer.key, keys.payer),
        (*accounts.vault.key, keys.vault),
        (*accounts.withdrawal_queue.key, keys.withdrawal_queue),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.user_input_ata.key, keys.user_input_ata),
        (*accounts.queue_input_ata.key, keys.queue_input_ata),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_junior_tranche_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: CancelJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.payer,
        accounts.withdrawal_queue,
        accounts.user_input_ata,
        accounts.queue_input_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_junior_tranche_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: CancelJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_junior_tranche_withdraw_verify_account_privileges<'me, 'info>(
    accounts: CancelJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_junior_tranche_withdraw_verify_writable_privileges(accounts)?;
    cancel_junior_tranche_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CancelUnstakeJuniorAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub queue_payer: &'me AccountInfo<'info>,
    pub withdrawal_queue: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub junior_mint: &'me AccountInfo<'info>,
    pub user_junior_mint_ata: &'me AccountInfo<'info>,
    pub locked_junior_shares_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelUnstakeJuniorKeys {
    pub user: Pubkey,
    pub queue_payer: Pubkey,
    pub withdrawal_queue: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_state: Pubkey,
    pub junior_mint: Pubkey,
    pub user_junior_mint_ata: Pubkey,
    pub locked_junior_shares_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<CancelUnstakeJuniorAccounts<'_, '_>> for CancelUnstakeJuniorKeys {
    fn from(accounts: CancelUnstakeJuniorAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            queue_payer: *accounts.queue_payer.key,
            withdrawal_queue: *accounts.withdrawal_queue.key,
            tranche_state: *accounts.tranche_state.key,
            bank_state: *accounts.bank_state.key,
            junior_mint: *accounts.junior_mint.key,
            user_junior_mint_ata: *accounts.user_junior_mint_ata.key,
            locked_junior_shares_ata: *accounts.locked_junior_shares_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CancelUnstakeJuniorKeys>
for [AccountMeta; CANCEL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelUnstakeJuniorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
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
                is_writable: false,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]> for CancelUnstakeJuniorKeys {
    fn from(pubkeys: [Pubkey; CANCEL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            queue_payer: pubkeys[1],
            withdrawal_queue: pubkeys[2],
            tranche_state: pubkeys[3],
            bank_state: pubkeys[4],
            junior_mint: pubkeys[5],
            user_junior_mint_ata: pubkeys[6],
            locked_junior_shares_ata: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<CancelUnstakeJuniorAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelUnstakeJuniorAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.queue_payer.clone(),
            accounts.withdrawal_queue.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_state.clone(),
            accounts.junior_mint.clone(),
            accounts.user_junior_mint_ata.clone(),
            accounts.locked_junior_shares_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN]>
for CancelUnstakeJuniorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            queue_payer: &arr[1],
            withdrawal_queue: &arr[2],
            tranche_state: &arr[3],
            bank_state: &arr[4],
            junior_mint: &arr[5],
            user_junior_mint_ata: &arr[6],
            locked_junior_shares_ata: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const CANCEL_UNSTAKE_JUNIOR_IX_DISCM: [u8; 8usize] = [
    216, 22, 37, 137, 37, 53, 196, 252,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelUnstakeJuniorIxData;
impl CancelUnstakeJuniorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_UNSTAKE_JUNIOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_UNSTAKE_JUNIOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_unstake_junior_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelUnstakeJuniorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_UNSTAKE_JUNIOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelUnstakeJuniorIxData.try_to_vec()?,
    })
}
pub fn cancel_unstake_junior_ix(
    keys: CancelUnstakeJuniorKeys,
) -> std::io::Result<Instruction> {
    cancel_unstake_junior_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn cancel_unstake_junior_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelUnstakeJuniorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelUnstakeJuniorKeys = accounts.into();
    let ix = cancel_unstake_junior_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_unstake_junior_invoke(
    accounts: CancelUnstakeJuniorAccounts<'_, '_>,
) -> ProgramResult {
    cancel_unstake_junior_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn cancel_unstake_junior_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelUnstakeJuniorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelUnstakeJuniorKeys = accounts.into();
    let ix = cancel_unstake_junior_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_unstake_junior_invoke_signed(
    accounts: CancelUnstakeJuniorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_unstake_junior_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cancel_unstake_junior_verify_account_keys(
    accounts: CancelUnstakeJuniorAccounts<'_, '_>,
    keys: CancelUnstakeJuniorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.queue_payer.key, keys.queue_payer),
        (*accounts.withdrawal_queue.key, keys.withdrawal_queue),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.junior_mint.key, keys.junior_mint),
        (*accounts.user_junior_mint_ata.key, keys.user_junior_mint_ata),
        (*accounts.locked_junior_shares_ata.key, keys.locked_junior_shares_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_unstake_junior_verify_writable_privileges<'me, 'info>(
    accounts: CancelUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.queue_payer,
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
pub fn cancel_unstake_junior_verify_signer_privileges<'me, 'info>(
    accounts: CancelUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_unstake_junior_verify_account_privileges<'me, 'info>(
    accounts: CancelUnstakeJuniorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_unstake_junior_verify_writable_privileges(accounts)?;
    cancel_unstake_junior_verify_signer_privileges(accounts)?;
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
pub const CREATE_ASSET_HOLDING_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateAssetHoldingAccounts<'me, 'info> {
    pub hw_manager: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateAssetHoldingKeys {
    pub hw_manager: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}
impl From<CreateAssetHoldingAccounts<'_, '_>> for CreateAssetHoldingKeys {
    fn from(accounts: CreateAssetHoldingAccounts) -> Self {
        Self {
            hw_manager: *accounts.hw_manager.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CreateAssetHoldingKeys>
for [AccountMeta; CREATE_ASSET_HOLDING_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateAssetHoldingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hw_manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
impl From<[Pubkey; CREATE_ASSET_HOLDING_IX_ACCOUNTS_LEN]> for CreateAssetHoldingKeys {
    fn from(pubkeys: [Pubkey; CREATE_ASSET_HOLDING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hw_manager: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            mint: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateAssetHoldingAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_ASSET_HOLDING_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAssetHoldingAccounts<'_, 'info>) -> Self {
        [
            accounts.hw_manager.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_ASSET_HOLDING_IX_ACCOUNTS_LEN]>
for CreateAssetHoldingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_ASSET_HOLDING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            hw_manager: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            mint: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const CREATE_ASSET_HOLDING_IX_DISCM: [u8; 8usize] = [
    140, 181, 66, 196, 87, 166, 93, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateAssetHoldingIxArgs {
    pub args: CreateAssetHoldingArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateAssetHoldingIxData(pub CreateAssetHoldingIxArgs);
impl From<CreateAssetHoldingIxArgs> for CreateAssetHoldingIxData {
    fn from(args: CreateAssetHoldingIxArgs) -> Self {
        Self(args)
    }
}
impl CreateAssetHoldingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_ASSET_HOLDING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <CreateAssetHoldingArgs>::deserialize(&mut reader)?
        };
        Ok(Self(CreateAssetHoldingIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_ASSET_HOLDING_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_asset_holding_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateAssetHoldingKeys,
    args: CreateAssetHoldingIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_ASSET_HOLDING_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateAssetHoldingIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_asset_holding_ix(
    keys: CreateAssetHoldingKeys,
    args: CreateAssetHoldingIxArgs,
) -> std::io::Result<Instruction> {
    create_asset_holding_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn create_asset_holding_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAssetHoldingAccounts<'_, '_>,
    args: CreateAssetHoldingIxArgs,
) -> ProgramResult {
    let keys: CreateAssetHoldingKeys = accounts.into();
    let ix = create_asset_holding_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_asset_holding_invoke(
    accounts: CreateAssetHoldingAccounts<'_, '_>,
    args: CreateAssetHoldingIxArgs,
) -> ProgramResult {
    create_asset_holding_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn create_asset_holding_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAssetHoldingAccounts<'_, '_>,
    args: CreateAssetHoldingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateAssetHoldingKeys = accounts.into();
    let ix = create_asset_holding_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_asset_holding_invoke_signed(
    accounts: CreateAssetHoldingAccounts<'_, '_>,
    args: CreateAssetHoldingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_asset_holding_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_asset_holding_verify_account_keys(
    accounts: CreateAssetHoldingAccounts<'_, '_>,
    keys: CreateAssetHoldingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hw_manager.key, keys.hw_manager),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_asset_holding_verify_writable_privileges<'me, 'info>(
    accounts: CreateAssetHoldingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.vault_oracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_asset_holding_verify_signer_privileges<'me, 'info>(
    accounts: CreateAssetHoldingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.hw_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_asset_holding_verify_account_privileges<'me, 'info>(
    accounts: CreateAssetHoldingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_asset_holding_verify_writable_privileges(accounts)?;
    create_asset_holding_verify_signer_privileges(accounts)?;
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
pub const CREATE_VAULT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateVaultAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateVaultKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub fee_vault: Pubkey,
    pub share_mint: Pubkey,
    pub asset_mint: Pubkey,
    pub asset_token_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateVaultAccounts<'_, '_>> for CreateVaultKeys {
    fn from(accounts: CreateVaultAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            fee_vault: *accounts.fee_vault.key,
            share_mint: *accounts.share_mint.key,
            asset_mint: *accounts.asset_mint.key,
            asset_token_program: *accounts.asset_token_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateVaultKeys> for [AccountMeta; CREATE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
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
        ]
    }
}
impl From<[Pubkey; CREATE_VAULT_IX_ACCOUNTS_LEN]> for CreateVaultKeys {
    fn from(pubkeys: [Pubkey; CREATE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            fee_vault: pubkeys[3],
            share_mint: pubkeys[4],
            asset_mint: pubkeys[5],
            asset_token_program: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<CreateVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.curator.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.fee_vault.clone(),
            accounts.share_mint.clone(),
            accounts.asset_mint.clone(),
            accounts.asset_token_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_VAULT_IX_ACCOUNTS_LEN]>
for CreateVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            fee_vault: &arr[3],
            share_mint: &arr[4],
            asset_mint: &arr[5],
            asset_token_program: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const CREATE_VAULT_IX_DISCM: [u8; 8usize] = [29, 237, 247, 208, 193, 82, 54, 135];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateVaultIxArgs {
    pub args: VaultCreateVaultArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateVaultIxData(pub CreateVaultIxArgs);
impl From<CreateVaultIxArgs> for CreateVaultIxData {
    fn from(args: CreateVaultIxArgs) -> Self {
        Self(args)
    }
}
impl CreateVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <VaultCreateVaultArgs>::deserialize(&mut reader)?
        };
        Ok(Self(CreateVaultIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateVaultKeys,
    args: CreateVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_vault_ix(
    keys: CreateVaultKeys,
    args: CreateVaultIxArgs,
) -> std::io::Result<Instruction> {
    create_vault_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn create_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateVaultAccounts<'_, '_>,
    args: CreateVaultIxArgs,
) -> ProgramResult {
    let keys: CreateVaultKeys = accounts.into();
    let ix = create_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_vault_invoke(
    accounts: CreateVaultAccounts<'_, '_>,
    args: CreateVaultIxArgs,
) -> ProgramResult {
    create_vault_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn create_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateVaultAccounts<'_, '_>,
    args: CreateVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateVaultKeys = accounts.into();
    let ix = create_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_vault_invoke_signed(
    accounts: CreateVaultAccounts<'_, '_>,
    args: CreateVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_vault_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_vault_verify_account_keys(
    accounts: CreateVaultAccounts<'_, '_>,
    keys: CreateVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_vault_verify_writable_privileges<'me, 'info>(
    accounts: CreateVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curator,
        accounts.vault,
        accounts.vault_oracle,
        accounts.fee_vault,
        accounts.share_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_vault_verify_signer_privileges<'me, 'info>(
    accounts: CreateVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_vault_verify_account_privileges<'me, 'info>(
    accounts: CreateVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_vault_verify_writable_privileges(accounts)?;
    create_vault_verify_signer_privileges(accounts)?;
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
pub const DISABLE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct DisableCircuitBreakerAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DisableCircuitBreakerKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
}
impl From<DisableCircuitBreakerAccounts<'_, '_>> for DisableCircuitBreakerKeys {
    fn from(accounts: DisableCircuitBreakerAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
        }
    }
}
impl From<DisableCircuitBreakerKeys>
for [AccountMeta; DISABLE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] {
    fn from(keys: DisableCircuitBreakerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
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
impl From<[Pubkey; DISABLE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]>
for DisableCircuitBreakerKeys {
    fn from(pubkeys: [Pubkey; DISABLE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
        }
    }
}
impl<'info> From<DisableCircuitBreakerAccounts<'_, 'info>>
for [AccountInfo<'info>; DISABLE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: DisableCircuitBreakerAccounts<'_, 'info>) -> Self {
        [accounts.curator.clone(), accounts.vault.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISABLE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN]>
for DisableCircuitBreakerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DISABLE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
        }
    }
}
pub const DISABLE_CIRCUIT_BREAKER_IX_DISCM: [u8; 8usize] = [
    200, 37, 34, 37, 19, 156, 178, 253,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DisableCircuitBreakerIxData;
impl DisableCircuitBreakerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISABLE_CIRCUIT_BREAKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISABLE_CIRCUIT_BREAKER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn disable_circuit_breaker_ix_with_program_id(
    program_id: Pubkey,
    keys: DisableCircuitBreakerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISABLE_CIRCUIT_BREAKER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DisableCircuitBreakerIxData.try_to_vec()?,
    })
}
pub fn disable_circuit_breaker_ix(
    keys: DisableCircuitBreakerKeys,
) -> std::io::Result<Instruction> {
    disable_circuit_breaker_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn disable_circuit_breaker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DisableCircuitBreakerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DisableCircuitBreakerKeys = accounts.into();
    let ix = disable_circuit_breaker_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn disable_circuit_breaker_invoke(
    accounts: DisableCircuitBreakerAccounts<'_, '_>,
) -> ProgramResult {
    disable_circuit_breaker_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn disable_circuit_breaker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DisableCircuitBreakerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DisableCircuitBreakerKeys = accounts.into();
    let ix = disable_circuit_breaker_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn disable_circuit_breaker_invoke_signed(
    accounts: DisableCircuitBreakerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    disable_circuit_breaker_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn disable_circuit_breaker_verify_account_keys(
    accounts: DisableCircuitBreakerAccounts<'_, '_>,
    keys: DisableCircuitBreakerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn disable_circuit_breaker_verify_writable_privileges<'me, 'info>(
    accounts: DisableCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn disable_circuit_breaker_verify_signer_privileges<'me, 'info>(
    accounts: DisableCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn disable_circuit_breaker_verify_account_privileges<'me, 'info>(
    accounts: DisableCircuitBreakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    disable_circuit_breaker_verify_writable_privileges(accounts)?;
    disable_circuit_breaker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteDepositAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub user_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteDepositKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub user_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
}
impl From<ExecuteDepositAccounts<'_, '_>> for ExecuteDepositKeys {
    fn from(accounts: ExecuteDepositAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            share_mint: *accounts.share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            user_share_ata: *accounts.user_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
        }
    }
}
impl From<ExecuteDepositKeys> for [AccountMeta; EXECUTE_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_DEPOSIT_IX_ACCOUNTS_LEN]> for ExecuteDepositKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            fee_vault: pubkeys[8],
            fee_vault_ata: pubkeys[9],
            user_share_ata: pubkeys[10],
            asset_token_program: pubkeys[11],
            share_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<ExecuteDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.user_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXECUTE_DEPOSIT_IX_ACCOUNTS_LEN]>
for ExecuteDepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXECUTE_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            fee_vault: &arr[8],
            fee_vault_ata: &arr[9],
            user_share_ata: &arr[10],
            asset_token_program: &arr[11],
            share_token_program: &arr[12],
        }
    }
}
pub const EXECUTE_DEPOSIT_IX_DISCM: [u8; 8usize] = [247, 103, 46, 184, 88, 188, 56, 46];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteDepositIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteDepositIxData(pub ExecuteDepositIxArgs);
impl From<ExecuteDepositIxArgs> for ExecuteDepositIxData {
    fn from(args: ExecuteDepositIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ExecuteDepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteDepositKeys,
    args: ExecuteDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_deposit_ix(
    keys: ExecuteDepositKeys,
    args: ExecuteDepositIxArgs,
) -> std::io::Result<Instruction> {
    execute_deposit_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn execute_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteDepositAccounts<'_, '_>,
    args: ExecuteDepositIxArgs,
) -> ProgramResult {
    let keys: ExecuteDepositKeys = accounts.into();
    let ix = execute_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_deposit_invoke(
    accounts: ExecuteDepositAccounts<'_, '_>,
    args: ExecuteDepositIxArgs,
) -> ProgramResult {
    execute_deposit_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn execute_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteDepositAccounts<'_, '_>,
    args: ExecuteDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteDepositKeys = accounts.into();
    let ix = execute_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_deposit_invoke_signed(
    accounts: ExecuteDepositAccounts<'_, '_>,
    args: ExecuteDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_deposit_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_deposit_verify_account_keys(
    accounts: ExecuteDepositAccounts<'_, '_>,
    keys: ExecuteDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.user_share_ata.key, keys.user_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_deposit_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.user_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_deposit_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_deposit_verify_account_privileges<'me, 'info>(
    accounts: ExecuteDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_deposit_verify_writable_privileges(accounts)?;
    execute_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_TRANCHE_DEPOSIT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteTrancheDepositAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub tranche_share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub user_tranche_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteTrancheDepositKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub tranche_share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub user_tranche_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ExecuteTrancheDepositAccounts<'_, '_>> for ExecuteTrancheDepositKeys {
    fn from(accounts: ExecuteTrancheDepositAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            tranche_share_mint: *accounts.tranche_share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            user_tranche_share_ata: *accounts.user_tranche_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ExecuteTrancheDepositKeys>
for [AccountMeta; EXECUTE_TRANCHE_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteTrancheDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tranche_share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_tranche_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; EXECUTE_TRANCHE_DEPOSIT_IX_ACCOUNTS_LEN]>
for ExecuteTrancheDepositKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_TRANCHE_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            tranche_share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            fee_vault: pubkeys[8],
            fee_vault_ata: pubkeys[9],
            user_tranche_share_ata: pubkeys[10],
            asset_token_program: pubkeys[11],
            share_token_program: pubkeys[12],
            associated_token_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<ExecuteTrancheDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_TRANCHE_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteTrancheDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.tranche_share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.user_tranche_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXECUTE_TRANCHE_DEPOSIT_IX_ACCOUNTS_LEN]>
for ExecuteTrancheDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXECUTE_TRANCHE_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            tranche_share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            fee_vault: &arr[8],
            fee_vault_ata: &arr[9],
            user_tranche_share_ata: &arr[10],
            asset_token_program: &arr[11],
            share_token_program: &arr[12],
            associated_token_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const EXECUTE_TRANCHE_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    231, 224, 46, 207, 175, 45, 77, 148,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteTrancheDepositIxArgs {
    pub kind: TrancheKind,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteTrancheDepositIxData(pub ExecuteTrancheDepositIxArgs);
impl From<ExecuteTrancheDepositIxArgs> for ExecuteTrancheDepositIxData {
    fn from(args: ExecuteTrancheDepositIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteTrancheDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_TRANCHE_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteTrancheDepositIxArgs {
                kind,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_TRANCHE_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_tranche_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteTrancheDepositKeys,
    args: ExecuteTrancheDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_TRANCHE_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteTrancheDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_tranche_deposit_ix(
    keys: ExecuteTrancheDepositKeys,
    args: ExecuteTrancheDepositIxArgs,
) -> std::io::Result<Instruction> {
    execute_tranche_deposit_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn execute_tranche_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTrancheDepositAccounts<'_, '_>,
    args: ExecuteTrancheDepositIxArgs,
) -> ProgramResult {
    let keys: ExecuteTrancheDepositKeys = accounts.into();
    let ix = execute_tranche_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_tranche_deposit_invoke(
    accounts: ExecuteTrancheDepositAccounts<'_, '_>,
    args: ExecuteTrancheDepositIxArgs,
) -> ProgramResult {
    execute_tranche_deposit_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn execute_tranche_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTrancheDepositAccounts<'_, '_>,
    args: ExecuteTrancheDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteTrancheDepositKeys = accounts.into();
    let ix = execute_tranche_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_tranche_deposit_invoke_signed(
    accounts: ExecuteTrancheDepositAccounts<'_, '_>,
    args: ExecuteTrancheDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_tranche_deposit_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_tranche_deposit_verify_account_keys(
    accounts: ExecuteTrancheDepositAccounts<'_, '_>,
    keys: ExecuteTrancheDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.tranche_share_mint.key, keys.tranche_share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.user_tranche_share_ata.key, keys.user_tranche_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_tranche_deposit_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteTrancheDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.tranche_share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.user_tranche_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_tranche_deposit_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteTrancheDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_tranche_deposit_verify_account_privileges<'me, 'info>(
    accounts: ExecuteTrancheDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_tranche_deposit_verify_writable_privileges(accounts)?;
    execute_tranche_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteTrancheWithdrawAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub tranche_share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub user_tranche_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteTrancheWithdrawKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub tranche_share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub user_tranche_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ExecuteTrancheWithdrawAccounts<'_, '_>> for ExecuteTrancheWithdrawKeys {
    fn from(accounts: ExecuteTrancheWithdrawAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            tranche_share_mint: *accounts.tranche_share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            user_tranche_share_ata: *accounts.user_tranche_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ExecuteTrancheWithdrawKeys>
for [AccountMeta; EXECUTE_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteTrancheWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tranche_share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_tranche_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; EXECUTE_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]>
for ExecuteTrancheWithdrawKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            tranche_share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            user_tranche_share_ata: pubkeys[8],
            asset_token_program: pubkeys[9],
            share_token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<ExecuteTrancheWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteTrancheWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.tranche_share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.user_tranche_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]>
for ExecuteTrancheWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXECUTE_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            tranche_share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            user_tranche_share_ata: &arr[8],
            asset_token_program: &arr[9],
            share_token_program: &arr[10],
            associated_token_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const EXECUTE_TRANCHE_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    240, 82, 254, 246, 177, 29, 57, 126,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteTrancheWithdrawIxArgs {
    pub kind: TrancheKind,
    pub share_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteTrancheWithdrawIxData(pub ExecuteTrancheWithdrawIxArgs);
impl From<ExecuteTrancheWithdrawIxArgs> for ExecuteTrancheWithdrawIxData {
    fn from(args: ExecuteTrancheWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteTrancheWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_TRANCHE_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteTrancheWithdrawIxArgs {
                kind,
                share_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_TRANCHE_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_tranche_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteTrancheWithdrawKeys,
    args: ExecuteTrancheWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteTrancheWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_tranche_withdraw_ix(
    keys: ExecuteTrancheWithdrawKeys,
    args: ExecuteTrancheWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    execute_tranche_withdraw_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn execute_tranche_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTrancheWithdrawAccounts<'_, '_>,
    args: ExecuteTrancheWithdrawIxArgs,
) -> ProgramResult {
    let keys: ExecuteTrancheWithdrawKeys = accounts.into();
    let ix = execute_tranche_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_tranche_withdraw_invoke(
    accounts: ExecuteTrancheWithdrawAccounts<'_, '_>,
    args: ExecuteTrancheWithdrawIxArgs,
) -> ProgramResult {
    execute_tranche_withdraw_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn execute_tranche_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTrancheWithdrawAccounts<'_, '_>,
    args: ExecuteTrancheWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteTrancheWithdrawKeys = accounts.into();
    let ix = execute_tranche_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_tranche_withdraw_invoke_signed(
    accounts: ExecuteTrancheWithdrawAccounts<'_, '_>,
    args: ExecuteTrancheWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_tranche_withdraw_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_tranche_withdraw_verify_account_keys(
    accounts: ExecuteTrancheWithdrawAccounts<'_, '_>,
    keys: ExecuteTrancheWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.tranche_share_mint.key, keys.tranche_share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.user_tranche_share_ata.key, keys.user_tranche_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_tranche_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.tranche_share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.user_tranche_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_tranche_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_tranche_withdraw_verify_account_privileges<'me, 'info>(
    accounts: ExecuteTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_tranche_withdraw_verify_writable_privileges(accounts)?;
    execute_tranche_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_WITHDRAW_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteWithdrawAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub user_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteWithdrawKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub user_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
}
impl From<ExecuteWithdrawAccounts<'_, '_>> for ExecuteWithdrawKeys {
    fn from(accounts: ExecuteWithdrawAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            share_mint: *accounts.share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            user_share_ata: *accounts.user_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
        }
    }
}
impl From<ExecuteWithdrawKeys> for [AccountMeta; EXECUTE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_WITHDRAW_IX_ACCOUNTS_LEN]> for ExecuteWithdrawKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            fee_vault: pubkeys[8],
            fee_vault_ata: pubkeys[9],
            user_share_ata: pubkeys[10],
            asset_token_program: pubkeys[11],
            share_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<ExecuteWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.user_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXECUTE_WITHDRAW_IX_ACCOUNTS_LEN]>
for ExecuteWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXECUTE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            fee_vault: &arr[8],
            fee_vault_ata: &arr[9],
            user_share_ata: &arr[10],
            asset_token_program: &arr[11],
            share_token_program: &arr[12],
        }
    }
}
pub const EXECUTE_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    255, 93, 15, 141, 187, 94, 246, 162,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteWithdrawIxArgs {
    pub share_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteWithdrawIxData(pub ExecuteWithdrawIxArgs);
impl From<ExecuteWithdrawIxArgs> for ExecuteWithdrawIxData {
    fn from(args: ExecuteWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteWithdrawIxArgs {
                share_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteWithdrawKeys,
    args: ExecuteWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_withdraw_ix(
    keys: ExecuteWithdrawKeys,
    args: ExecuteWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    execute_withdraw_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn execute_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteWithdrawAccounts<'_, '_>,
    args: ExecuteWithdrawIxArgs,
) -> ProgramResult {
    let keys: ExecuteWithdrawKeys = accounts.into();
    let ix = execute_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_withdraw_invoke(
    accounts: ExecuteWithdrawAccounts<'_, '_>,
    args: ExecuteWithdrawIxArgs,
) -> ProgramResult {
    execute_withdraw_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn execute_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteWithdrawAccounts<'_, '_>,
    args: ExecuteWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteWithdrawKeys = accounts.into();
    let ix = execute_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_withdraw_invoke_signed(
    accounts: ExecuteWithdrawAccounts<'_, '_>,
    args: ExecuteWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_withdraw_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_withdraw_verify_account_keys(
    accounts: ExecuteWithdrawAccounts<'_, '_>,
    keys: ExecuteWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.user_share_ata.key, keys.user_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.user_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_withdraw_verify_account_privileges<'me, 'info>(
    accounts: ExecuteWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_withdraw_verify_writable_privileges(accounts)?;
    execute_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct FulfillJuniorTrancheWithdrawAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub queue_payer: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub withdrawal_queue: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub owner_input_ata: &'me AccountInfo<'info>,
    pub owner_output_ata: &'me AccountInfo<'info>,
    pub vault_output_ata: &'me AccountInfo<'info>,
    pub fee_collector: &'me AccountInfo<'info>,
    pub fee_collector_output_ata: &'me AccountInfo<'info>,
    pub queue_input_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FulfillJuniorTrancheWithdrawKeys {
    pub signer: Pubkey,
    pub owner: Pubkey,
    pub queue_payer: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub withdrawal_queue: Pubkey,
    pub output_mint: Pubkey,
    pub input_mint: Pubkey,
    pub owner_input_ata: Pubkey,
    pub owner_output_ata: Pubkey,
    pub vault_output_ata: Pubkey,
    pub fee_collector: Pubkey,
    pub fee_collector_output_ata: Pubkey,
    pub queue_input_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<FulfillJuniorTrancheWithdrawAccounts<'_, '_>>
for FulfillJuniorTrancheWithdrawKeys {
    fn from(accounts: FulfillJuniorTrancheWithdrawAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            owner: *accounts.owner.key,
            queue_payer: *accounts.queue_payer.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            withdrawal_queue: *accounts.withdrawal_queue.key,
            output_mint: *accounts.output_mint.key,
            input_mint: *accounts.input_mint.key,
            owner_input_ata: *accounts.owner_input_ata.key,
            owner_output_ata: *accounts.owner_output_ata.key,
            vault_output_ata: *accounts.vault_output_ata.key,
            fee_collector: *accounts.fee_collector.key,
            fee_collector_output_ata: *accounts.fee_collector_output_ata.key,
            queue_input_ata: *accounts.queue_input_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<FulfillJuniorTrancheWithdrawKeys>
for [AccountMeta; FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: FulfillJuniorTrancheWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.queue_payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdrawal_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_collector,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_collector_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.queue_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]>
for FulfillJuniorTrancheWithdrawKeys {
    fn from(pubkeys: [Pubkey; FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            owner: pubkeys[1],
            queue_payer: pubkeys[2],
            vault: pubkeys[3],
            vault_oracle: pubkeys[4],
            vault_tranche_state: pubkeys[5],
            withdrawal_queue: pubkeys[6],
            output_mint: pubkeys[7],
            input_mint: pubkeys[8],
            owner_input_ata: pubkeys[9],
            owner_output_ata: pubkeys[10],
            vault_output_ata: pubkeys[11],
            fee_collector: pubkeys[12],
            fee_collector_output_ata: pubkeys[13],
            queue_input_ata: pubkeys[14],
            asset_token_program: pubkeys[15],
            share_token_program: pubkeys[16],
            associated_token_program: pubkeys[17],
            system_program: pubkeys[18],
        }
    }
}
impl<'info> From<FulfillJuniorTrancheWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: FulfillJuniorTrancheWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.owner.clone(),
            accounts.queue_payer.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.withdrawal_queue.clone(),
            accounts.output_mint.clone(),
            accounts.input_mint.clone(),
            accounts.owner_input_ata.clone(),
            accounts.owner_output_ata.clone(),
            accounts.vault_output_ata.clone(),
            accounts.fee_collector.clone(),
            accounts.fee_collector_output_ata.clone(),
            accounts.queue_input_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]>
for FulfillJuniorTrancheWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            owner: &arr[1],
            queue_payer: &arr[2],
            vault: &arr[3],
            vault_oracle: &arr[4],
            vault_tranche_state: &arr[5],
            withdrawal_queue: &arr[6],
            output_mint: &arr[7],
            input_mint: &arr[8],
            owner_input_ata: &arr[9],
            owner_output_ata: &arr[10],
            vault_output_ata: &arr[11],
            fee_collector: &arr[12],
            fee_collector_output_ata: &arr[13],
            queue_input_ata: &arr[14],
            asset_token_program: &arr[15],
            share_token_program: &arr[16],
            associated_token_program: &arr[17],
            system_program: &arr[18],
        }
    }
}
pub const FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    127, 229, 153, 168, 116, 50, 248, 103,
];
#[derive(Clone, Debug, PartialEq)]
pub struct FulfillJuniorTrancheWithdrawIxData;
impl FulfillJuniorTrancheWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fulfill_junior_tranche_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: FulfillJuniorTrancheWithdrawKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FULFILL_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: FulfillJuniorTrancheWithdrawIxData.try_to_vec()?,
    })
}
pub fn fulfill_junior_tranche_withdraw_ix(
    keys: FulfillJuniorTrancheWithdrawKeys,
) -> std::io::Result<Instruction> {
    fulfill_junior_tranche_withdraw_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn fulfill_junior_tranche_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FulfillJuniorTrancheWithdrawAccounts<'_, '_>,
) -> ProgramResult {
    let keys: FulfillJuniorTrancheWithdrawKeys = accounts.into();
    let ix = fulfill_junior_tranche_withdraw_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn fulfill_junior_tranche_withdraw_invoke(
    accounts: FulfillJuniorTrancheWithdrawAccounts<'_, '_>,
) -> ProgramResult {
    fulfill_junior_tranche_withdraw_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
    )
}
pub fn fulfill_junior_tranche_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FulfillJuniorTrancheWithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FulfillJuniorTrancheWithdrawKeys = accounts.into();
    let ix = fulfill_junior_tranche_withdraw_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fulfill_junior_tranche_withdraw_invoke_signed(
    accounts: FulfillJuniorTrancheWithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fulfill_junior_tranche_withdraw_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn fulfill_junior_tranche_withdraw_verify_account_keys(
    accounts: FulfillJuniorTrancheWithdrawAccounts<'_, '_>,
    keys: FulfillJuniorTrancheWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.owner.key, keys.owner),
        (*accounts.queue_payer.key, keys.queue_payer),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.withdrawal_queue.key, keys.withdrawal_queue),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.owner_input_ata.key, keys.owner_input_ata),
        (*accounts.owner_output_ata.key, keys.owner_output_ata),
        (*accounts.vault_output_ata.key, keys.vault_output_ata),
        (*accounts.fee_collector.key, keys.fee_collector),
        (*accounts.fee_collector_output_ata.key, keys.fee_collector_output_ata),
        (*accounts.queue_input_ata.key, keys.queue_input_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fulfill_junior_tranche_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: FulfillJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.queue_payer,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.withdrawal_queue,
        accounts.input_mint,
        accounts.owner_input_ata,
        accounts.owner_output_ata,
        accounts.vault_output_ata,
        accounts.fee_collector_output_ata,
        accounts.queue_input_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fulfill_junior_tranche_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: FulfillJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fulfill_junior_tranche_withdraw_verify_account_privileges<'me, 'info>(
    accounts: FulfillJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fulfill_junior_tranche_withdraw_verify_writable_privileges(accounts)?;
    fulfill_junior_tranche_withdraw_verify_signer_privileges(accounts)?;
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
pub const GET_NAV_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct GetNavAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub user_share_ata: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetNavKeys {
    pub vault: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub share_mint: Pubkey,
    pub user_share_ata: Pubkey,
}
impl From<GetNavAccounts<'_, '_>> for GetNavKeys {
    fn from(accounts: GetNavAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            share_mint: *accounts.share_mint.key,
            user_share_ata: *accounts.user_share_ata.key,
        }
    }
}
impl From<GetNavKeys> for [AccountMeta; GET_NAV_IX_ACCOUNTS_LEN] {
    fn from(keys: GetNavKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_share_ata,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_NAV_IX_ACCOUNTS_LEN]> for GetNavKeys {
    fn from(pubkeys: [Pubkey; GET_NAV_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            vault_tranche_state: pubkeys[1],
            share_mint: pubkeys[2],
            user_share_ata: pubkeys[3],
        }
    }
}
impl<'info> From<GetNavAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_NAV_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetNavAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.share_mint.clone(),
            accounts.user_share_ata.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_NAV_IX_ACCOUNTS_LEN]>
for GetNavAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_NAV_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            vault_tranche_state: &arr[1],
            share_mint: &arr[2],
            user_share_ata: &arr[3],
        }
    }
}
pub const GET_NAV_IX_DISCM: [u8; 8usize] = [200, 89, 76, 53, 215, 218, 63, 21];
#[derive(Clone, Debug, PartialEq)]
pub struct GetNavIxData;
impl GetNavIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_NAV_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_NAV_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_nav_ix_with_program_id(
    program_id: Pubkey,
    keys: GetNavKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_NAV_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetNavIxData.try_to_vec()?,
    })
}
pub fn get_nav_ix(keys: GetNavKeys) -> std::io::Result<Instruction> {
    get_nav_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn get_nav_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetNavAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetNavKeys = accounts.into();
    let ix = get_nav_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_nav_invoke(accounts: GetNavAccounts<'_, '_>) -> ProgramResult {
    get_nav_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn get_nav_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetNavAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetNavKeys = accounts.into();
    let ix = get_nav_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_nav_invoke_signed(
    accounts: GetNavAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_nav_invoke_signed_with_program_id(BANKINECO_PROGRAM_ID, accounts, seeds)
}
pub fn get_nav_verify_account_keys(
    accounts: GetNavAccounts<'_, '_>,
    keys: GetNavKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.user_share_ata.key, keys.user_share_ata),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_TRANCHE_NAV_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct GetTrancheNavAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub tranche_share_mint: &'me AccountInfo<'info>,
    pub user_tranche_share_ata: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetTrancheNavKeys {
    pub vault: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub tranche_share_mint: Pubkey,
    pub user_tranche_share_ata: Pubkey,
}
impl From<GetTrancheNavAccounts<'_, '_>> for GetTrancheNavKeys {
    fn from(accounts: GetTrancheNavAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            tranche_share_mint: *accounts.tranche_share_mint.key,
            user_tranche_share_ata: *accounts.user_tranche_share_ata.key,
        }
    }
}
impl From<GetTrancheNavKeys> for [AccountMeta; GET_TRANCHE_NAV_IX_ACCOUNTS_LEN] {
    fn from(keys: GetTrancheNavKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tranche_share_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_tranche_share_ata,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_TRANCHE_NAV_IX_ACCOUNTS_LEN]> for GetTrancheNavKeys {
    fn from(pubkeys: [Pubkey; GET_TRANCHE_NAV_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            vault_tranche_state: pubkeys[1],
            tranche_share_mint: pubkeys[2],
            user_tranche_share_ata: pubkeys[3],
        }
    }
}
impl<'info> From<GetTrancheNavAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_TRANCHE_NAV_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetTrancheNavAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.tranche_share_mint.clone(),
            accounts.user_tranche_share_ata.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_TRANCHE_NAV_IX_ACCOUNTS_LEN]>
for GetTrancheNavAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_TRANCHE_NAV_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            vault_tranche_state: &arr[1],
            tranche_share_mint: &arr[2],
            user_tranche_share_ata: &arr[3],
        }
    }
}
pub const GET_TRANCHE_NAV_IX_DISCM: [u8; 8usize] = [173, 139, 168, 42, 15, 25, 206, 236];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetTrancheNavIxArgs {
    pub kind: TrancheKind,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetTrancheNavIxData(pub GetTrancheNavIxArgs);
impl From<GetTrancheNavIxArgs> for GetTrancheNavIxData {
    fn from(args: GetTrancheNavIxArgs) -> Self {
        Self(args)
    }
}
impl GetTrancheNavIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_TRANCHE_NAV_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(GetTrancheNavIxArgs { kind }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_TRANCHE_NAV_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kind, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_tranche_nav_ix_with_program_id(
    program_id: Pubkey,
    keys: GetTrancheNavKeys,
    args: GetTrancheNavIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_TRANCHE_NAV_IX_ACCOUNTS_LEN] = keys.into();
    let data: GetTrancheNavIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn get_tranche_nav_ix(
    keys: GetTrancheNavKeys,
    args: GetTrancheNavIxArgs,
) -> std::io::Result<Instruction> {
    get_tranche_nav_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn get_tranche_nav_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetTrancheNavAccounts<'_, '_>,
    args: GetTrancheNavIxArgs,
) -> ProgramResult {
    let keys: GetTrancheNavKeys = accounts.into();
    let ix = get_tranche_nav_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_tranche_nav_invoke(
    accounts: GetTrancheNavAccounts<'_, '_>,
    args: GetTrancheNavIxArgs,
) -> ProgramResult {
    get_tranche_nav_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn get_tranche_nav_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetTrancheNavAccounts<'_, '_>,
    args: GetTrancheNavIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetTrancheNavKeys = accounts.into();
    let ix = get_tranche_nav_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_tranche_nav_invoke_signed(
    accounts: GetTrancheNavAccounts<'_, '_>,
    args: GetTrancheNavIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_tranche_nav_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn get_tranche_nav_verify_account_keys(
    accounts: GetTrancheNavAccounts<'_, '_>,
    keys: GetTrancheNavKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.tranche_share_mint.key, keys.tranche_share_mint),
        (*accounts.user_tranche_share_ata.key, keys.user_tranche_share_ata),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
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
pub const JUPITER_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct JupiterSwapAccounts<'me, 'info> {
    pub hw_manager: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub vault_source_ata: &'me AccountInfo<'info>,
    pub vault_destination_ata: &'me AccountInfo<'info>,
    pub source_token_program: &'me AccountInfo<'info>,
    pub destination_token_program: &'me AccountInfo<'info>,
    pub destination_oracle_account: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JupiterSwapKeys {
    pub hw_manager: Pubkey,
    pub vault: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub vault_source_ata: Pubkey,
    pub vault_destination_ata: Pubkey,
    pub source_token_program: Pubkey,
    pub destination_token_program: Pubkey,
    pub destination_oracle_account: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
}
impl From<JupiterSwapAccounts<'_, '_>> for JupiterSwapKeys {
    fn from(accounts: JupiterSwapAccounts) -> Self {
        Self {
            hw_manager: *accounts.hw_manager.key,
            vault: *accounts.vault.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            vault_source_ata: *accounts.vault_source_ata.key,
            vault_destination_ata: *accounts.vault_destination_ata.key,
            source_token_program: *accounts.source_token_program.key,
            destination_token_program: *accounts.destination_token_program.key,
            destination_oracle_account: *accounts.destination_oracle_account.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
        }
    }
}
impl From<JupiterSwapKeys> for [AccountMeta; JUPITER_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: JupiterSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hw_manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
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
                pubkey: keys.vault_source_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_destination_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_oracle_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; JUPITER_SWAP_IX_ACCOUNTS_LEN]> for JupiterSwapKeys {
    fn from(pubkeys: [Pubkey; JUPITER_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hw_manager: pubkeys[0],
            vault: pubkeys[1],
            source_mint: pubkeys[2],
            destination_mint: pubkeys[3],
            vault_source_ata: pubkeys[4],
            vault_destination_ata: pubkeys[5],
            source_token_program: pubkeys[6],
            destination_token_program: pubkeys[7],
            destination_oracle_account: pubkeys[8],
            vault_oracle: pubkeys[9],
            vault_tranche_state: pubkeys[10],
        }
    }
}
impl<'info> From<JupiterSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; JUPITER_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: JupiterSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.hw_manager.clone(),
            accounts.vault.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.vault_source_ata.clone(),
            accounts.vault_destination_ata.clone(),
            accounts.source_token_program.clone(),
            accounts.destination_token_program.clone(),
            accounts.destination_oracle_account.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; JUPITER_SWAP_IX_ACCOUNTS_LEN]>
for JupiterSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; JUPITER_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hw_manager: &arr[0],
            vault: &arr[1],
            source_mint: &arr[2],
            destination_mint: &arr[3],
            vault_source_ata: &arr[4],
            vault_destination_ata: &arr[5],
            source_token_program: &arr[6],
            destination_token_program: &arr[7],
            destination_oracle_account: &arr[8],
            vault_oracle: &arr[9],
            vault_tranche_state: &arr[10],
        }
    }
}
pub const JUPITER_SWAP_IX_DISCM: [u8; 8usize] = [116, 207, 0, 196, 252, 120, 243, 18];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JupiterSwapIxArgs {
    pub refs: InstructionRefs,
    pub args: JupiterSwapArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct JupiterSwapIxData(pub JupiterSwapIxArgs);
impl From<JupiterSwapIxArgs> for JupiterSwapIxData {
    fn from(args: JupiterSwapIxArgs) -> Self {
        Self(args)
    }
}
impl JupiterSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != JUPITER_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let refs = if reader.is_empty() {
            Default::default()
        } else {
            <InstructionRefs>::deserialize(&mut reader)?
        };
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <JupiterSwapArgs>::deserialize(&mut reader)?
        };
        Ok(Self(JupiterSwapIxArgs { refs, args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&JUPITER_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.refs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn jupiter_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: JupiterSwapKeys,
    args: JupiterSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; JUPITER_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: JupiterSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn jupiter_swap_ix(
    keys: JupiterSwapKeys,
    args: JupiterSwapIxArgs,
) -> std::io::Result<Instruction> {
    jupiter_swap_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn jupiter_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: JupiterSwapAccounts<'_, '_>,
    args: JupiterSwapIxArgs,
) -> ProgramResult {
    let keys: JupiterSwapKeys = accounts.into();
    let ix = jupiter_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn jupiter_swap_invoke(
    accounts: JupiterSwapAccounts<'_, '_>,
    args: JupiterSwapIxArgs,
) -> ProgramResult {
    jupiter_swap_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn jupiter_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: JupiterSwapAccounts<'_, '_>,
    args: JupiterSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: JupiterSwapKeys = accounts.into();
    let ix = jupiter_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn jupiter_swap_invoke_signed(
    accounts: JupiterSwapAccounts<'_, '_>,
    args: JupiterSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    jupiter_swap_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn jupiter_swap_verify_account_keys(
    accounts: JupiterSwapAccounts<'_, '_>,
    keys: JupiterSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hw_manager.key, keys.hw_manager),
        (*accounts.vault.key, keys.vault),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (*accounts.vault_source_ata.key, keys.vault_source_ata),
        (*accounts.vault_destination_ata.key, keys.vault_destination_ata),
        (*accounts.source_token_program.key, keys.source_token_program),
        (*accounts.destination_token_program.key, keys.destination_token_program),
        (*accounts.destination_oracle_account.key, keys.destination_oracle_account),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn jupiter_swap_verify_writable_privileges<'me, 'info>(
    accounts: JupiterSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_source_ata,
        accounts.vault_destination_ata,
        accounts.vault_oracle,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn jupiter_swap_verify_signer_privileges<'me, 'info>(
    accounts: JupiterSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.hw_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn jupiter_swap_verify_account_privileges<'me, 'info>(
    accounts: JupiterSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    jupiter_swap_verify_writable_privileges(accounts)?;
    jupiter_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MANAGER_REDEPOSIT_ASSET_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ManagerRedepositAssetAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub manager_asset_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ManagerRedepositAssetKeys {
    pub manager: Pubkey,
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub manager_asset_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ManagerRedepositAssetAccounts<'_, '_>> for ManagerRedepositAssetKeys {
    fn from(accounts: ManagerRedepositAssetAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            manager_asset_ata: *accounts.manager_asset_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ManagerRedepositAssetKeys>
for [AccountMeta; MANAGER_REDEPOSIT_ASSET_IX_ACCOUNTS_LEN] {
    fn from(keys: ManagerRedepositAssetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; MANAGER_REDEPOSIT_ASSET_IX_ACCOUNTS_LEN]>
for ManagerRedepositAssetKeys {
    fn from(pubkeys: [Pubkey; MANAGER_REDEPOSIT_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            vault: pubkeys[1],
            asset_mint: pubkeys[2],
            vault_asset_ata: pubkeys[3],
            manager_asset_ata: pubkeys[4],
            asset_token_program: pubkeys[5],
            associated_token_program: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<ManagerRedepositAssetAccounts<'_, 'info>>
for [AccountInfo<'info>; MANAGER_REDEPOSIT_ASSET_IX_ACCOUNTS_LEN] {
    fn from(accounts: ManagerRedepositAssetAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.manager_asset_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MANAGER_REDEPOSIT_ASSET_IX_ACCOUNTS_LEN]>
for ManagerRedepositAssetAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MANAGER_REDEPOSIT_ASSET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: &arr[0],
            vault: &arr[1],
            asset_mint: &arr[2],
            vault_asset_ata: &arr[3],
            manager_asset_ata: &arr[4],
            asset_token_program: &arr[5],
            associated_token_program: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const MANAGER_REDEPOSIT_ASSET_IX_DISCM: [u8; 8usize] = [
    239, 229, 175, 30, 229, 100, 81, 238,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ManagerRedepositAssetIxArgs {
    pub args: ManagerRedepositAssetArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManagerRedepositAssetIxData(pub ManagerRedepositAssetIxArgs);
impl From<ManagerRedepositAssetIxArgs> for ManagerRedepositAssetIxData {
    fn from(args: ManagerRedepositAssetIxArgs) -> Self {
        Self(args)
    }
}
impl ManagerRedepositAssetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANAGER_REDEPOSIT_ASSET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ManagerRedepositAssetArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(ManagerRedepositAssetIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANAGER_REDEPOSIT_ASSET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn manager_redeposit_asset_ix_with_program_id(
    program_id: Pubkey,
    keys: ManagerRedepositAssetKeys,
    args: ManagerRedepositAssetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MANAGER_REDEPOSIT_ASSET_IX_ACCOUNTS_LEN] = keys.into();
    let data: ManagerRedepositAssetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn manager_redeposit_asset_ix(
    keys: ManagerRedepositAssetKeys,
    args: ManagerRedepositAssetIxArgs,
) -> std::io::Result<Instruction> {
    manager_redeposit_asset_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn manager_redeposit_asset_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ManagerRedepositAssetAccounts<'_, '_>,
    args: ManagerRedepositAssetIxArgs,
) -> ProgramResult {
    let keys: ManagerRedepositAssetKeys = accounts.into();
    let ix = manager_redeposit_asset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn manager_redeposit_asset_invoke(
    accounts: ManagerRedepositAssetAccounts<'_, '_>,
    args: ManagerRedepositAssetIxArgs,
) -> ProgramResult {
    manager_redeposit_asset_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn manager_redeposit_asset_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ManagerRedepositAssetAccounts<'_, '_>,
    args: ManagerRedepositAssetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ManagerRedepositAssetKeys = accounts.into();
    let ix = manager_redeposit_asset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn manager_redeposit_asset_invoke_signed(
    accounts: ManagerRedepositAssetAccounts<'_, '_>,
    args: ManagerRedepositAssetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    manager_redeposit_asset_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn manager_redeposit_asset_verify_account_keys(
    accounts: ManagerRedepositAssetAccounts<'_, '_>,
    keys: ManagerRedepositAssetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.manager_asset_ata.key, keys.manager_asset_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn manager_redeposit_asset_verify_writable_privileges<'me, 'info>(
    accounts: ManagerRedepositAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.manager_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn manager_redeposit_asset_verify_signer_privileges<'me, 'info>(
    accounts: ManagerRedepositAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn manager_redeposit_asset_verify_account_privileges<'me, 'info>(
    accounts: ManagerRedepositAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    manager_redeposit_asset_verify_writable_privileges(accounts)?;
    manager_redeposit_asset_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MANAGER_WITHDRAW_ASSET_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ManagerWithdrawAssetAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub manager_asset_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ManagerWithdrawAssetKeys {
    pub manager: Pubkey,
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub manager_asset_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ManagerWithdrawAssetAccounts<'_, '_>> for ManagerWithdrawAssetKeys {
    fn from(accounts: ManagerWithdrawAssetAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            manager_asset_ata: *accounts.manager_asset_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ManagerWithdrawAssetKeys>
for [AccountMeta; MANAGER_WITHDRAW_ASSET_IX_ACCOUNTS_LEN] {
    fn from(keys: ManagerWithdrawAssetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; MANAGER_WITHDRAW_ASSET_IX_ACCOUNTS_LEN]>
for ManagerWithdrawAssetKeys {
    fn from(pubkeys: [Pubkey; MANAGER_WITHDRAW_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            vault: pubkeys[1],
            asset_mint: pubkeys[2],
            vault_asset_ata: pubkeys[3],
            manager_asset_ata: pubkeys[4],
            asset_token_program: pubkeys[5],
            associated_token_program: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<ManagerWithdrawAssetAccounts<'_, 'info>>
for [AccountInfo<'info>; MANAGER_WITHDRAW_ASSET_IX_ACCOUNTS_LEN] {
    fn from(accounts: ManagerWithdrawAssetAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.manager_asset_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MANAGER_WITHDRAW_ASSET_IX_ACCOUNTS_LEN]>
for ManagerWithdrawAssetAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MANAGER_WITHDRAW_ASSET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: &arr[0],
            vault: &arr[1],
            asset_mint: &arr[2],
            vault_asset_ata: &arr[3],
            manager_asset_ata: &arr[4],
            asset_token_program: &arr[5],
            associated_token_program: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const MANAGER_WITHDRAW_ASSET_IX_DISCM: [u8; 8usize] = [
    161, 215, 111, 102, 152, 235, 167, 146,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ManagerWithdrawAssetIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManagerWithdrawAssetIxData(pub ManagerWithdrawAssetIxArgs);
impl From<ManagerWithdrawAssetIxArgs> for ManagerWithdrawAssetIxData {
    fn from(args: ManagerWithdrawAssetIxArgs) -> Self {
        Self(args)
    }
}
impl ManagerWithdrawAssetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANAGER_WITHDRAW_ASSET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ManagerWithdrawAssetIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANAGER_WITHDRAW_ASSET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn manager_withdraw_asset_ix_with_program_id(
    program_id: Pubkey,
    keys: ManagerWithdrawAssetKeys,
    args: ManagerWithdrawAssetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MANAGER_WITHDRAW_ASSET_IX_ACCOUNTS_LEN] = keys.into();
    let data: ManagerWithdrawAssetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn manager_withdraw_asset_ix(
    keys: ManagerWithdrawAssetKeys,
    args: ManagerWithdrawAssetIxArgs,
) -> std::io::Result<Instruction> {
    manager_withdraw_asset_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn manager_withdraw_asset_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ManagerWithdrawAssetAccounts<'_, '_>,
    args: ManagerWithdrawAssetIxArgs,
) -> ProgramResult {
    let keys: ManagerWithdrawAssetKeys = accounts.into();
    let ix = manager_withdraw_asset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn manager_withdraw_asset_invoke(
    accounts: ManagerWithdrawAssetAccounts<'_, '_>,
    args: ManagerWithdrawAssetIxArgs,
) -> ProgramResult {
    manager_withdraw_asset_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn manager_withdraw_asset_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ManagerWithdrawAssetAccounts<'_, '_>,
    args: ManagerWithdrawAssetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ManagerWithdrawAssetKeys = accounts.into();
    let ix = manager_withdraw_asset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn manager_withdraw_asset_invoke_signed(
    accounts: ManagerWithdrawAssetAccounts<'_, '_>,
    args: ManagerWithdrawAssetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    manager_withdraw_asset_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn manager_withdraw_asset_verify_account_keys(
    accounts: ManagerWithdrawAssetAccounts<'_, '_>,
    keys: ManagerWithdrawAssetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.manager_asset_ata.key, keys.manager_asset_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn manager_withdraw_asset_verify_writable_privileges<'me, 'info>(
    accounts: ManagerWithdrawAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.manager_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn manager_withdraw_asset_verify_signer_privileges<'me, 'info>(
    accounts: ManagerWithdrawAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn manager_withdraw_asset_verify_account_privileges<'me, 'info>(
    accounts: ManagerWithdrawAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    manager_withdraw_asset_verify_writable_privileges(accounts)?;
    manager_withdraw_asset_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_LEGACY_VAULT_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct MigrateLegacyVaultAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub legacy_vault: &'me AccountInfo<'info>,
    pub legacy_oracle: &'me AccountInfo<'info>,
    pub legacy_team: &'me AccountInfo<'info>,
    pub yielding_mint: &'me AccountInfo<'info>,
    pub legacy_vault_ata: &'me AccountInfo<'info>,
    pub destination_vault: &'me AccountInfo<'info>,
    pub destination_vault_oracle: &'me AccountInfo<'info>,
    pub destination_vault_asset_ata: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub junior_share_mint: &'me AccountInfo<'info>,
    pub locked_junior_shares_ata: &'me AccountInfo<'info>,
    pub yielding_token_program: &'me AccountInfo<'info>,
    pub bank_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateLegacyVaultKeys {
    pub manager: Pubkey,
    pub bank_state: Pubkey,
    pub bank_mint: Pubkey,
    pub legacy_vault: Pubkey,
    pub legacy_oracle: Pubkey,
    pub legacy_team: Pubkey,
    pub yielding_mint: Pubkey,
    pub legacy_vault_ata: Pubkey,
    pub destination_vault: Pubkey,
    pub destination_vault_oracle: Pubkey,
    pub destination_vault_asset_ata: Pubkey,
    pub tranche_state: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub junior_share_mint: Pubkey,
    pub locked_junior_shares_ata: Pubkey,
    pub yielding_token_program: Pubkey,
    pub bank_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<MigrateLegacyVaultAccounts<'_, '_>> for MigrateLegacyVaultKeys {
    fn from(accounts: MigrateLegacyVaultAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            bank_state: *accounts.bank_state.key,
            bank_mint: *accounts.bank_mint.key,
            legacy_vault: *accounts.legacy_vault.key,
            legacy_oracle: *accounts.legacy_oracle.key,
            legacy_team: *accounts.legacy_team.key,
            yielding_mint: *accounts.yielding_mint.key,
            legacy_vault_ata: *accounts.legacy_vault_ata.key,
            destination_vault: *accounts.destination_vault.key,
            destination_vault_oracle: *accounts.destination_vault_oracle.key,
            destination_vault_asset_ata: *accounts.destination_vault_asset_ata.key,
            tranche_state: *accounts.tranche_state.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            junior_share_mint: *accounts.junior_share_mint.key,
            locked_junior_shares_ata: *accounts.locked_junior_shares_ata.key,
            yielding_token_program: *accounts.yielding_token_program.key,
            bank_token_program: *accounts.bank_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MigrateLegacyVaultKeys>
for [AccountMeta; MIGRATE_LEGACY_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateLegacyVaultKeys) -> Self {
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
                pubkey: keys.legacy_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.legacy_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.legacy_team,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.legacy_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_vault_asset_ata,
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
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.junior_share_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locked_junior_shares_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.yielding_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; MIGRATE_LEGACY_VAULT_IX_ACCOUNTS_LEN]> for MigrateLegacyVaultKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_LEGACY_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            bank_state: pubkeys[1],
            bank_mint: pubkeys[2],
            legacy_vault: pubkeys[3],
            legacy_oracle: pubkeys[4],
            legacy_team: pubkeys[5],
            yielding_mint: pubkeys[6],
            legacy_vault_ata: pubkeys[7],
            destination_vault: pubkeys[8],
            destination_vault_oracle: pubkeys[9],
            destination_vault_asset_ata: pubkeys[10],
            tranche_state: pubkeys[11],
            junior_escrow_ata: pubkeys[12],
            vault_tranche_state: pubkeys[13],
            junior_share_mint: pubkeys[14],
            locked_junior_shares_ata: pubkeys[15],
            yielding_token_program: pubkeys[16],
            bank_token_program: pubkeys[17],
            associated_token_program: pubkeys[18],
            system_program: pubkeys[19],
        }
    }
}
impl<'info> From<MigrateLegacyVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_LEGACY_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateLegacyVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.bank_state.clone(),
            accounts.bank_mint.clone(),
            accounts.legacy_vault.clone(),
            accounts.legacy_oracle.clone(),
            accounts.legacy_team.clone(),
            accounts.yielding_mint.clone(),
            accounts.legacy_vault_ata.clone(),
            accounts.destination_vault.clone(),
            accounts.destination_vault_oracle.clone(),
            accounts.destination_vault_asset_ata.clone(),
            accounts.tranche_state.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.junior_share_mint.clone(),
            accounts.locked_junior_shares_ata.clone(),
            accounts.yielding_token_program.clone(),
            accounts.bank_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_LEGACY_VAULT_IX_ACCOUNTS_LEN]>
for MigrateLegacyVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MIGRATE_LEGACY_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: &arr[0],
            bank_state: &arr[1],
            bank_mint: &arr[2],
            legacy_vault: &arr[3],
            legacy_oracle: &arr[4],
            legacy_team: &arr[5],
            yielding_mint: &arr[6],
            legacy_vault_ata: &arr[7],
            destination_vault: &arr[8],
            destination_vault_oracle: &arr[9],
            destination_vault_asset_ata: &arr[10],
            tranche_state: &arr[11],
            junior_escrow_ata: &arr[12],
            vault_tranche_state: &arr[13],
            junior_share_mint: &arr[14],
            locked_junior_shares_ata: &arr[15],
            yielding_token_program: &arr[16],
            bank_token_program: &arr[17],
            associated_token_program: &arr[18],
            system_program: &arr[19],
        }
    }
}
pub const MIGRATE_LEGACY_VAULT_IX_DISCM: [u8; 8usize] = [
    38, 235, 213, 175, 164, 254, 27, 85,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrateLegacyVaultIxArgs {
    pub args: MigrateLegacyVaultArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateLegacyVaultIxData(pub MigrateLegacyVaultIxArgs);
impl From<MigrateLegacyVaultIxArgs> for MigrateLegacyVaultIxData {
    fn from(args: MigrateLegacyVaultIxArgs) -> Self {
        Self(args)
    }
}
impl MigrateLegacyVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_LEGACY_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <MigrateLegacyVaultArgs>::deserialize(&mut reader)?
        };
        Ok(Self(MigrateLegacyVaultIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_LEGACY_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_legacy_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateLegacyVaultKeys,
    args: MigrateLegacyVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_LEGACY_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrateLegacyVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn migrate_legacy_vault_ix(
    keys: MigrateLegacyVaultKeys,
    args: MigrateLegacyVaultIxArgs,
) -> std::io::Result<Instruction> {
    migrate_legacy_vault_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn migrate_legacy_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateLegacyVaultAccounts<'_, '_>,
    args: MigrateLegacyVaultIxArgs,
) -> ProgramResult {
    let keys: MigrateLegacyVaultKeys = accounts.into();
    let ix = migrate_legacy_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_legacy_vault_invoke(
    accounts: MigrateLegacyVaultAccounts<'_, '_>,
    args: MigrateLegacyVaultIxArgs,
) -> ProgramResult {
    migrate_legacy_vault_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn migrate_legacy_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateLegacyVaultAccounts<'_, '_>,
    args: MigrateLegacyVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateLegacyVaultKeys = accounts.into();
    let ix = migrate_legacy_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_legacy_vault_invoke_signed(
    accounts: MigrateLegacyVaultAccounts<'_, '_>,
    args: MigrateLegacyVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_legacy_vault_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn migrate_legacy_vault_verify_account_keys(
    accounts: MigrateLegacyVaultAccounts<'_, '_>,
    keys: MigrateLegacyVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.legacy_vault.key, keys.legacy_vault),
        (*accounts.legacy_oracle.key, keys.legacy_oracle),
        (*accounts.legacy_team.key, keys.legacy_team),
        (*accounts.yielding_mint.key, keys.yielding_mint),
        (*accounts.legacy_vault_ata.key, keys.legacy_vault_ata),
        (*accounts.destination_vault.key, keys.destination_vault),
        (*accounts.destination_vault_oracle.key, keys.destination_vault_oracle),
        (*accounts.destination_vault_asset_ata.key, keys.destination_vault_asset_ata),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.junior_share_mint.key, keys.junior_share_mint),
        (*accounts.locked_junior_shares_ata.key, keys.locked_junior_shares_ata),
        (*accounts.yielding_token_program.key, keys.yielding_token_program),
        (*accounts.bank_token_program.key, keys.bank_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_legacy_vault_verify_writable_privileges<'me, 'info>(
    accounts: MigrateLegacyVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.bank_state,
        accounts.bank_mint,
        accounts.legacy_vault,
        accounts.legacy_team,
        accounts.legacy_vault_ata,
        accounts.destination_vault,
        accounts.destination_vault_asset_ata,
        accounts.tranche_state,
        accounts.junior_escrow_ata,
        accounts.vault_tranche_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_legacy_vault_verify_signer_privileges<'me, 'info>(
    accounts: MigrateLegacyVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_legacy_vault_verify_account_privileges<'me, 'info>(
    accounts: MigrateLegacyVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_legacy_vault_verify_writable_privileges(accounts)?;
    migrate_legacy_vault_verify_signer_privileges(accounts)?;
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
pub const PROTOCOL_INTERACTION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ProtocolInteractionAccounts<'me, 'info> {
    pub hw_manager: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub affected_asset_mint: &'me AccountInfo<'info>,
    pub affected_vault_ata: &'me AccountInfo<'info>,
    pub affected_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProtocolInteractionKeys {
    pub hw_manager: Pubkey,
    pub vault: Pubkey,
    pub affected_asset_mint: Pubkey,
    pub affected_vault_ata: Pubkey,
    pub affected_token_program: Pubkey,
}
impl From<ProtocolInteractionAccounts<'_, '_>> for ProtocolInteractionKeys {
    fn from(accounts: ProtocolInteractionAccounts) -> Self {
        Self {
            hw_manager: *accounts.hw_manager.key,
            vault: *accounts.vault.key,
            affected_asset_mint: *accounts.affected_asset_mint.key,
            affected_vault_ata: *accounts.affected_vault_ata.key,
            affected_token_program: *accounts.affected_token_program.key,
        }
    }
}
impl From<ProtocolInteractionKeys>
for [AccountMeta; PROTOCOL_INTERACTION_IX_ACCOUNTS_LEN] {
    fn from(keys: ProtocolInteractionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hw_manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.affected_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.affected_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.affected_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PROTOCOL_INTERACTION_IX_ACCOUNTS_LEN]> for ProtocolInteractionKeys {
    fn from(pubkeys: [Pubkey; PROTOCOL_INTERACTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hw_manager: pubkeys[0],
            vault: pubkeys[1],
            affected_asset_mint: pubkeys[2],
            affected_vault_ata: pubkeys[3],
            affected_token_program: pubkeys[4],
        }
    }
}
impl<'info> From<ProtocolInteractionAccounts<'_, 'info>>
for [AccountInfo<'info>; PROTOCOL_INTERACTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ProtocolInteractionAccounts<'_, 'info>) -> Self {
        [
            accounts.hw_manager.clone(),
            accounts.vault.clone(),
            accounts.affected_asset_mint.clone(),
            accounts.affected_vault_ata.clone(),
            accounts.affected_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PROTOCOL_INTERACTION_IX_ACCOUNTS_LEN]>
for ProtocolInteractionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PROTOCOL_INTERACTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            hw_manager: &arr[0],
            vault: &arr[1],
            affected_asset_mint: &arr[2],
            affected_vault_ata: &arr[3],
            affected_token_program: &arr[4],
        }
    }
}
pub const PROTOCOL_INTERACTION_IX_DISCM: [u8; 8usize] = [
    8, 91, 184, 153, 199, 179, 240, 248,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProtocolInteractionIxArgs {
    pub refs: InstructionRefs,
    pub args: ProtocolInteractionArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolInteractionIxData(pub ProtocolInteractionIxArgs);
impl From<ProtocolInteractionIxArgs> for ProtocolInteractionIxData {
    fn from(args: ProtocolInteractionIxArgs) -> Self {
        Self(args)
    }
}
impl ProtocolInteractionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_INTERACTION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let refs = if reader.is_empty() {
            Default::default()
        } else {
            <InstructionRefs>::deserialize(&mut reader)?
        };
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ProtocolInteractionArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(ProtocolInteractionIxArgs {
                refs,
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_INTERACTION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.refs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn protocol_interaction_ix_with_program_id(
    program_id: Pubkey,
    keys: ProtocolInteractionKeys,
    args: ProtocolInteractionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROTOCOL_INTERACTION_IX_ACCOUNTS_LEN] = keys.into();
    let data: ProtocolInteractionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn protocol_interaction_ix(
    keys: ProtocolInteractionKeys,
    args: ProtocolInteractionIxArgs,
) -> std::io::Result<Instruction> {
    protocol_interaction_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn protocol_interaction_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ProtocolInteractionAccounts<'_, '_>,
    args: ProtocolInteractionIxArgs,
) -> ProgramResult {
    let keys: ProtocolInteractionKeys = accounts.into();
    let ix = protocol_interaction_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn protocol_interaction_invoke(
    accounts: ProtocolInteractionAccounts<'_, '_>,
    args: ProtocolInteractionIxArgs,
) -> ProgramResult {
    protocol_interaction_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn protocol_interaction_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ProtocolInteractionAccounts<'_, '_>,
    args: ProtocolInteractionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ProtocolInteractionKeys = accounts.into();
    let ix = protocol_interaction_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn protocol_interaction_invoke_signed(
    accounts: ProtocolInteractionAccounts<'_, '_>,
    args: ProtocolInteractionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    protocol_interaction_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn protocol_interaction_verify_account_keys(
    accounts: ProtocolInteractionAccounts<'_, '_>,
    keys: ProtocolInteractionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hw_manager.key, keys.hw_manager),
        (*accounts.vault.key, keys.vault),
        (*accounts.affected_asset_mint.key, keys.affected_asset_mint),
        (*accounts.affected_vault_ata.key, keys.affected_vault_ata),
        (*accounts.affected_token_program.key, keys.affected_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn protocol_interaction_verify_writable_privileges<'me, 'info>(
    accounts: ProtocolInteractionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.affected_vault_ata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn protocol_interaction_verify_signer_privileges<'me, 'info>(
    accounts: ProtocolInteractionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.hw_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn protocol_interaction_verify_account_privileges<'me, 'info>(
    accounts: ProtocolInteractionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    protocol_interaction_verify_writable_privileges(accounts)?;
    protocol_interaction_verify_signer_privileges(accounts)?;
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
pub const REMOVE_ASSET_HOLDING_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RemoveAssetHoldingAccounts<'me, 'info> {
    pub hw_manager: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAssetHoldingKeys {
    pub hw_manager: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub mint: Pubkey,
}
impl From<RemoveAssetHoldingAccounts<'_, '_>> for RemoveAssetHoldingKeys {
    fn from(accounts: RemoveAssetHoldingAccounts) -> Self {
        Self {
            hw_manager: *accounts.hw_manager.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            mint: *accounts.mint.key,
        }
    }
}
impl From<RemoveAssetHoldingKeys>
for [AccountMeta; REMOVE_ASSET_HOLDING_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAssetHoldingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hw_manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_ASSET_HOLDING_IX_ACCOUNTS_LEN]> for RemoveAssetHoldingKeys {
    fn from(pubkeys: [Pubkey; REMOVE_ASSET_HOLDING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hw_manager: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            mint: pubkeys[3],
        }
    }
}
impl<'info> From<RemoveAssetHoldingAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_ASSET_HOLDING_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAssetHoldingAccounts<'_, 'info>) -> Self {
        [
            accounts.hw_manager.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_ASSET_HOLDING_IX_ACCOUNTS_LEN]>
for RemoveAssetHoldingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_ASSET_HOLDING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            hw_manager: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            mint: &arr[3],
        }
    }
}
pub const REMOVE_ASSET_HOLDING_IX_DISCM: [u8; 8usize] = [
    117, 152, 209, 107, 174, 205, 69, 145,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAssetHoldingIxData;
impl RemoveAssetHoldingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ASSET_HOLDING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ASSET_HOLDING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_asset_holding_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveAssetHoldingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ASSET_HOLDING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveAssetHoldingIxData.try_to_vec()?,
    })
}
pub fn remove_asset_holding_ix(
    keys: RemoveAssetHoldingKeys,
) -> std::io::Result<Instruction> {
    remove_asset_holding_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn remove_asset_holding_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAssetHoldingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveAssetHoldingKeys = accounts.into();
    let ix = remove_asset_holding_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_asset_holding_invoke(
    accounts: RemoveAssetHoldingAccounts<'_, '_>,
) -> ProgramResult {
    remove_asset_holding_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn remove_asset_holding_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAssetHoldingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAssetHoldingKeys = accounts.into();
    let ix = remove_asset_holding_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_asset_holding_invoke_signed(
    accounts: RemoveAssetHoldingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_asset_holding_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_asset_holding_verify_account_keys(
    accounts: RemoveAssetHoldingAccounts<'_, '_>,
    keys: RemoveAssetHoldingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hw_manager.key, keys.hw_manager),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.mint.key, keys.mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_asset_holding_verify_writable_privileges<'me, 'info>(
    accounts: RemoveAssetHoldingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.vault_oracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_asset_holding_verify_signer_privileges<'me, 'info>(
    accounts: RemoveAssetHoldingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.hw_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_asset_holding_verify_account_privileges<'me, 'info>(
    accounts: RemoveAssetHoldingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_asset_holding_verify_writable_privileges(accounts)?;
    remove_asset_holding_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct RequestJuniorTrancheWithdrawAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub withdrawal_queue: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub user_input_ata: &'me AccountInfo<'info>,
    pub queue_input_ata: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RequestJuniorTrancheWithdrawKeys {
    pub user: Pubkey,
    pub payer: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub withdrawal_queue: Pubkey,
    pub output_mint: Pubkey,
    pub input_mint: Pubkey,
    pub user_input_ata: Pubkey,
    pub queue_input_ata: Pubkey,
    pub share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RequestJuniorTrancheWithdrawAccounts<'_, '_>>
for RequestJuniorTrancheWithdrawKeys {
    fn from(accounts: RequestJuniorTrancheWithdrawAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            payer: *accounts.payer.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            withdrawal_queue: *accounts.withdrawal_queue.key,
            output_mint: *accounts.output_mint.key,
            input_mint: *accounts.input_mint.key,
            user_input_ata: *accounts.user_input_ata.key,
            queue_input_ata: *accounts.queue_input_ata.key,
            share_token_program: *accounts.share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RequestJuniorTrancheWithdrawKeys>
for [AccountMeta; REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: RequestJuniorTrancheWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdrawal_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.queue_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]>
for RequestJuniorTrancheWithdrawKeys {
    fn from(pubkeys: [Pubkey; REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            payer: pubkeys[1],
            vault: pubkeys[2],
            vault_oracle: pubkeys[3],
            vault_tranche_state: pubkeys[4],
            withdrawal_queue: pubkeys[5],
            output_mint: pubkeys[6],
            input_mint: pubkeys[7],
            user_input_ata: pubkeys[8],
            queue_input_ata: pubkeys[9],
            share_token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<RequestJuniorTrancheWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: RequestJuniorTrancheWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.payer.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.withdrawal_queue.clone(),
            accounts.output_mint.clone(),
            accounts.input_mint.clone(),
            accounts.user_input_ata.clone(),
            accounts.queue_input_ata.clone(),
            accounts.share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN]>
for RequestJuniorTrancheWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            payer: &arr[1],
            vault: &arr[2],
            vault_oracle: &arr[3],
            vault_tranche_state: &arr[4],
            withdrawal_queue: &arr[5],
            output_mint: &arr[6],
            input_mint: &arr[7],
            user_input_ata: &arr[8],
            queue_input_ata: &arr[9],
            share_token_program: &arr[10],
            associated_token_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    163, 114, 93, 2, 82, 47, 70, 253,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RequestJuniorTrancheWithdrawIxArgs {
    pub queue_id: u8,
    pub share_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestJuniorTrancheWithdrawIxData(pub RequestJuniorTrancheWithdrawIxArgs);
impl From<RequestJuniorTrancheWithdrawIxArgs> for RequestJuniorTrancheWithdrawIxData {
    fn from(args: RequestJuniorTrancheWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl RequestJuniorTrancheWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let queue_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RequestJuniorTrancheWithdrawIxArgs {
                queue_id,
                share_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.queue_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn request_junior_tranche_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: RequestJuniorTrancheWithdrawKeys,
    args: RequestJuniorTrancheWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REQUEST_JUNIOR_TRANCHE_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: RequestJuniorTrancheWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_junior_tranche_withdraw_ix(
    keys: RequestJuniorTrancheWithdrawKeys,
    args: RequestJuniorTrancheWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    request_junior_tranche_withdraw_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn request_junior_tranche_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RequestJuniorTrancheWithdrawAccounts<'_, '_>,
    args: RequestJuniorTrancheWithdrawIxArgs,
) -> ProgramResult {
    let keys: RequestJuniorTrancheWithdrawKeys = accounts.into();
    let ix = request_junior_tranche_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn request_junior_tranche_withdraw_invoke(
    accounts: RequestJuniorTrancheWithdrawAccounts<'_, '_>,
    args: RequestJuniorTrancheWithdrawIxArgs,
) -> ProgramResult {
    request_junior_tranche_withdraw_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn request_junior_tranche_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RequestJuniorTrancheWithdrawAccounts<'_, '_>,
    args: RequestJuniorTrancheWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RequestJuniorTrancheWithdrawKeys = accounts.into();
    let ix = request_junior_tranche_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn request_junior_tranche_withdraw_invoke_signed(
    accounts: RequestJuniorTrancheWithdrawAccounts<'_, '_>,
    args: RequestJuniorTrancheWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    request_junior_tranche_withdraw_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn request_junior_tranche_withdraw_verify_account_keys(
    accounts: RequestJuniorTrancheWithdrawAccounts<'_, '_>,
    keys: RequestJuniorTrancheWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.payer.key, keys.payer),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.withdrawal_queue.key, keys.withdrawal_queue),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.user_input_ata.key, keys.user_input_ata),
        (*accounts.queue_input_ata.key, keys.queue_input_ata),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn request_junior_tranche_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: RequestJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.payer,
        accounts.withdrawal_queue,
        accounts.user_input_ata,
        accounts.queue_input_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn request_junior_tranche_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: RequestJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn request_junior_tranche_withdraw_verify_account_privileges<'me, 'info>(
    accounts: RequestJuniorTrancheWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    request_junior_tranche_withdraw_verify_writable_privileges(accounts)?;
    request_junior_tranche_withdraw_verify_signer_privileges(accounts)?;
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
pub const SET_ASSET_PRICE_ORACLE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetAssetPriceOracleAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetAssetPriceOracleKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
}
impl From<SetAssetPriceOracleAccounts<'_, '_>> for SetAssetPriceOracleKeys {
    fn from(accounts: SetAssetPriceOracleAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
        }
    }
}
impl From<SetAssetPriceOracleKeys>
for [AccountMeta; SET_ASSET_PRICE_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetAssetPriceOracleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
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
impl From<[Pubkey; SET_ASSET_PRICE_ORACLE_IX_ACCOUNTS_LEN]> for SetAssetPriceOracleKeys {
    fn from(pubkeys: [Pubkey; SET_ASSET_PRICE_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
        }
    }
}
impl<'info> From<SetAssetPriceOracleAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ASSET_PRICE_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetAssetPriceOracleAccounts<'_, 'info>) -> Self {
        [accounts.curator.clone(), accounts.vault.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ASSET_PRICE_ORACLE_IX_ACCOUNTS_LEN]>
for SetAssetPriceOracleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ASSET_PRICE_ORACLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
        }
    }
}
pub const SET_ASSET_PRICE_ORACLE_IX_DISCM: [u8; 8usize] = [
    225, 57, 235, 110, 233, 222, 35, 11,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetAssetPriceOracleIxArgs {
    pub args: SetAssetPriceOracleArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetAssetPriceOracleIxData(pub SetAssetPriceOracleIxArgs);
impl From<SetAssetPriceOracleIxArgs> for SetAssetPriceOracleIxData {
    fn from(args: SetAssetPriceOracleIxArgs) -> Self {
        Self(args)
    }
}
impl SetAssetPriceOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ASSET_PRICE_ORACLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SetAssetPriceOracleArgs>::deserialize(&mut reader)?
        };
        Ok(Self(SetAssetPriceOracleIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ASSET_PRICE_ORACLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_asset_price_oracle_ix_with_program_id(
    program_id: Pubkey,
    keys: SetAssetPriceOracleKeys,
    args: SetAssetPriceOracleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ASSET_PRICE_ORACLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetAssetPriceOracleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_asset_price_oracle_ix(
    keys: SetAssetPriceOracleKeys,
    args: SetAssetPriceOracleIxArgs,
) -> std::io::Result<Instruction> {
    set_asset_price_oracle_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn set_asset_price_oracle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetAssetPriceOracleAccounts<'_, '_>,
    args: SetAssetPriceOracleIxArgs,
) -> ProgramResult {
    let keys: SetAssetPriceOracleKeys = accounts.into();
    let ix = set_asset_price_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_asset_price_oracle_invoke(
    accounts: SetAssetPriceOracleAccounts<'_, '_>,
    args: SetAssetPriceOracleIxArgs,
) -> ProgramResult {
    set_asset_price_oracle_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn set_asset_price_oracle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetAssetPriceOracleAccounts<'_, '_>,
    args: SetAssetPriceOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetAssetPriceOracleKeys = accounts.into();
    let ix = set_asset_price_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_asset_price_oracle_invoke_signed(
    accounts: SetAssetPriceOracleAccounts<'_, '_>,
    args: SetAssetPriceOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_asset_price_oracle_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_asset_price_oracle_verify_account_keys(
    accounts: SetAssetPriceOracleAccounts<'_, '_>,
    keys: SetAssetPriceOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_asset_price_oracle_verify_writable_privileges<'me, 'info>(
    accounts: SetAssetPriceOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_asset_price_oracle_verify_signer_privileges<'me, 'info>(
    accounts: SetAssetPriceOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_asset_price_oracle_verify_account_privileges<'me, 'info>(
    accounts: SetAssetPriceOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_asset_price_oracle_verify_writable_privileges(accounts)?;
    set_asset_price_oracle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_EXTERNAL_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetExternalLiquidityAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetExternalLiquidityKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
}
impl From<SetExternalLiquidityAccounts<'_, '_>> for SetExternalLiquidityKeys {
    fn from(accounts: SetExternalLiquidityAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
        }
    }
}
impl From<SetExternalLiquidityKeys>
for [AccountMeta; SET_EXTERNAL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetExternalLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
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
impl From<[Pubkey; SET_EXTERNAL_LIQUIDITY_IX_ACCOUNTS_LEN]>
for SetExternalLiquidityKeys {
    fn from(pubkeys: [Pubkey; SET_EXTERNAL_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
        }
    }
}
impl<'info> From<SetExternalLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_EXTERNAL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetExternalLiquidityAccounts<'_, 'info>) -> Self {
        [accounts.curator.clone(), accounts.vault.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_EXTERNAL_LIQUIDITY_IX_ACCOUNTS_LEN]>
for SetExternalLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_EXTERNAL_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
        }
    }
}
pub const SET_EXTERNAL_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    219, 140, 68, 229, 236, 35, 176, 79,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetExternalLiquidityIxArgs {
    pub args: SetExternalLiquidityArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetExternalLiquidityIxData(pub SetExternalLiquidityIxArgs);
impl From<SetExternalLiquidityIxArgs> for SetExternalLiquidityIxData {
    fn from(args: SetExternalLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl SetExternalLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_EXTERNAL_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SetExternalLiquidityArgs>::deserialize(&mut reader)?
        };
        Ok(Self(SetExternalLiquidityIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_EXTERNAL_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_external_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: SetExternalLiquidityKeys,
    args: SetExternalLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_EXTERNAL_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetExternalLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_external_liquidity_ix(
    keys: SetExternalLiquidityKeys,
    args: SetExternalLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    set_external_liquidity_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn set_external_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetExternalLiquidityAccounts<'_, '_>,
    args: SetExternalLiquidityIxArgs,
) -> ProgramResult {
    let keys: SetExternalLiquidityKeys = accounts.into();
    let ix = set_external_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_external_liquidity_invoke(
    accounts: SetExternalLiquidityAccounts<'_, '_>,
    args: SetExternalLiquidityIxArgs,
) -> ProgramResult {
    set_external_liquidity_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn set_external_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetExternalLiquidityAccounts<'_, '_>,
    args: SetExternalLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetExternalLiquidityKeys = accounts.into();
    let ix = set_external_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_external_liquidity_invoke_signed(
    accounts: SetExternalLiquidityAccounts<'_, '_>,
    args: SetExternalLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_external_liquidity_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_external_liquidity_verify_account_keys(
    accounts: SetExternalLiquidityAccounts<'_, '_>,
    keys: SetExternalLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_external_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: SetExternalLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_external_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: SetExternalLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_external_liquidity_verify_account_privileges<'me, 'info>(
    accounts: SetExternalLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_external_liquidity_verify_writable_privileges(accounts)?;
    set_external_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_VAULT_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetVaultConfigAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetVaultConfigKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
}
impl From<SetVaultConfigAccounts<'_, '_>> for SetVaultConfigKeys {
    fn from(accounts: SetVaultConfigAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
        }
    }
}
impl From<SetVaultConfigKeys> for [AccountMeta; SET_VAULT_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: SetVaultConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_VAULT_CONFIG_IX_ACCOUNTS_LEN]> for SetVaultConfigKeys {
    fn from(pubkeys: [Pubkey; SET_VAULT_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
        }
    }
}
impl<'info> From<SetVaultConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_VAULT_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetVaultConfigAccounts<'_, 'info>) -> Self {
        [accounts.curator.clone(), accounts.vault.clone(), accounts.vault_oracle.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_VAULT_CONFIG_IX_ACCOUNTS_LEN]>
for SetVaultConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_VAULT_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
        }
    }
}
pub const SET_VAULT_CONFIG_IX_DISCM: [u8; 8usize] = [65, 5, 248, 136, 48, 58, 235, 231];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetVaultConfigIxArgs {
    pub args: SetVaultConfigArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetVaultConfigIxData(pub SetVaultConfigIxArgs);
impl From<SetVaultConfigIxArgs> for SetVaultConfigIxData {
    fn from(args: SetVaultConfigIxArgs) -> Self {
        Self(args)
    }
}
impl SetVaultConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_VAULT_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SetVaultConfigArgs>::deserialize(&mut reader)?
        };
        Ok(Self(SetVaultConfigIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_VAULT_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_vault_config_ix_with_program_id(
    program_id: Pubkey,
    keys: SetVaultConfigKeys,
    args: SetVaultConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_VAULT_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetVaultConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_vault_config_ix(
    keys: SetVaultConfigKeys,
    args: SetVaultConfigIxArgs,
) -> std::io::Result<Instruction> {
    set_vault_config_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn set_vault_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetVaultConfigAccounts<'_, '_>,
    args: SetVaultConfigIxArgs,
) -> ProgramResult {
    let keys: SetVaultConfigKeys = accounts.into();
    let ix = set_vault_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_vault_config_invoke(
    accounts: SetVaultConfigAccounts<'_, '_>,
    args: SetVaultConfigIxArgs,
) -> ProgramResult {
    set_vault_config_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn set_vault_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetVaultConfigAccounts<'_, '_>,
    args: SetVaultConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetVaultConfigKeys = accounts.into();
    let ix = set_vault_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_vault_config_invoke_signed(
    accounts: SetVaultConfigAccounts<'_, '_>,
    args: SetVaultConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_vault_config_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_vault_config_verify_account_keys(
    accounts: SetVaultConfigAccounts<'_, '_>,
    keys: SetVaultConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_vault_config_verify_writable_privileges<'me, 'info>(
    accounts: SetVaultConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_vault_config_verify_signer_privileges<'me, 'info>(
    accounts: SetVaultConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_vault_config_verify_account_privileges<'me, 'info>(
    accounts: SetVaultConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_vault_config_verify_writable_privileges(accounts)?;
    set_vault_config_verify_signer_privileges(accounts)?;
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
pub const TRANSFER_MIGRATED_MINT_AUTHORITY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct TransferMigratedMintAuthorityAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub destination_vault: &'me AccountInfo<'info>,
    pub source_bank_state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub locked_junior_shares_ata: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub bank_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferMigratedMintAuthorityKeys {
    pub manager: Pubkey,
    pub destination_vault: Pubkey,
    pub source_bank_state: Pubkey,
    pub mint: Pubkey,
    pub tranche_state: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub locked_junior_shares_ata: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub token_program: Pubkey,
    pub bank_token_program: Pubkey,
}
impl From<TransferMigratedMintAuthorityAccounts<'_, '_>>
for TransferMigratedMintAuthorityKeys {
    fn from(accounts: TransferMigratedMintAuthorityAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            destination_vault: *accounts.destination_vault.key,
            source_bank_state: *accounts.source_bank_state.key,
            mint: *accounts.mint.key,
            tranche_state: *accounts.tranche_state.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            locked_junior_shares_ata: *accounts.locked_junior_shares_ata.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            token_program: *accounts.token_program.key,
            bank_token_program: *accounts.bank_token_program.key,
        }
    }
}
impl From<TransferMigratedMintAuthorityKeys>
for [AccountMeta; TRANSFER_MIGRATED_MINT_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferMigratedMintAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.junior_escrow_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locked_junior_shares_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_MIGRATED_MINT_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferMigratedMintAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; TRANSFER_MIGRATED_MINT_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: pubkeys[0],
            destination_vault: pubkeys[1],
            source_bank_state: pubkeys[2],
            mint: pubkeys[3],
            tranche_state: pubkeys[4],
            junior_escrow_ata: pubkeys[5],
            locked_junior_shares_ata: pubkeys[6],
            vault_tranche_state: pubkeys[7],
            token_program: pubkeys[8],
            bank_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<TransferMigratedMintAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_MIGRATED_MINT_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferMigratedMintAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.destination_vault.clone(),
            accounts.source_bank_state.clone(),
            accounts.mint.clone(),
            accounts.tranche_state.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.locked_junior_shares_ata.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.token_program.clone(),
            accounts.bank_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_MIGRATED_MINT_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferMigratedMintAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_MIGRATED_MINT_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: &arr[0],
            destination_vault: &arr[1],
            source_bank_state: &arr[2],
            mint: &arr[3],
            tranche_state: &arr[4],
            junior_escrow_ata: &arr[5],
            locked_junior_shares_ata: &arr[6],
            vault_tranche_state: &arr[7],
            token_program: &arr[8],
            bank_token_program: &arr[9],
        }
    }
}
pub const TRANSFER_MIGRATED_MINT_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    52, 140, 117, 92, 56, 124, 192, 233,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferMigratedMintAuthorityIxArgs {
    pub args: TransferMigratedMintAuthorityArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferMigratedMintAuthorityIxData(pub TransferMigratedMintAuthorityIxArgs);
impl From<TransferMigratedMintAuthorityIxArgs> for TransferMigratedMintAuthorityIxData {
    fn from(args: TransferMigratedMintAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl TransferMigratedMintAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_MIGRATED_MINT_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <TransferMigratedMintAuthorityArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(TransferMigratedMintAuthorityIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_MIGRATED_MINT_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_migrated_mint_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferMigratedMintAuthorityKeys,
    args: TransferMigratedMintAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_MIGRATED_MINT_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: TransferMigratedMintAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_migrated_mint_authority_ix(
    keys: TransferMigratedMintAuthorityKeys,
    args: TransferMigratedMintAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    transfer_migrated_mint_authority_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn transfer_migrated_mint_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferMigratedMintAuthorityAccounts<'_, '_>,
    args: TransferMigratedMintAuthorityIxArgs,
) -> ProgramResult {
    let keys: TransferMigratedMintAuthorityKeys = accounts.into();
    let ix = transfer_migrated_mint_authority_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_migrated_mint_authority_invoke(
    accounts: TransferMigratedMintAuthorityAccounts<'_, '_>,
    args: TransferMigratedMintAuthorityIxArgs,
) -> ProgramResult {
    transfer_migrated_mint_authority_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn transfer_migrated_mint_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferMigratedMintAuthorityAccounts<'_, '_>,
    args: TransferMigratedMintAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferMigratedMintAuthorityKeys = accounts.into();
    let ix = transfer_migrated_mint_authority_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_migrated_mint_authority_invoke_signed(
    accounts: TransferMigratedMintAuthorityAccounts<'_, '_>,
    args: TransferMigratedMintAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_migrated_mint_authority_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_migrated_mint_authority_verify_account_keys(
    accounts: TransferMigratedMintAuthorityAccounts<'_, '_>,
    keys: TransferMigratedMintAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.destination_vault.key, keys.destination_vault),
        (*accounts.source_bank_state.key, keys.source_bank_state),
        (*accounts.mint.key, keys.mint),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.locked_junior_shares_ata.key, keys.locked_junior_shares_ata),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.bank_token_program.key, keys.bank_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_migrated_mint_authority_verify_writable_privileges<'me, 'info>(
    accounts: TransferMigratedMintAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.destination_vault,
        accounts.source_bank_state,
        accounts.mint,
        accounts.vault_tranche_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_migrated_mint_authority_verify_signer_privileges<'me, 'info>(
    accounts: TransferMigratedMintAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_migrated_mint_authority_verify_account_privileges<'me, 'info>(
    accounts: TransferMigratedMintAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_migrated_mint_authority_verify_writable_privileges(accounts)?;
    transfer_migrated_mint_authority_verify_signer_privileges(accounts)?;
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
pub const UPDATE_ASSET_PRICE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAssetPriceAccounts<'me, 'info> {
    pub updater: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub oracle_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAssetPriceKeys {
    pub updater: Pubkey,
    pub vault: Pubkey,
    pub oracle_account: Pubkey,
}
impl From<UpdateAssetPriceAccounts<'_, '_>> for UpdateAssetPriceKeys {
    fn from(accounts: UpdateAssetPriceAccounts) -> Self {
        Self {
            updater: *accounts.updater.key,
            vault: *accounts.vault.key,
            oracle_account: *accounts.oracle_account.key,
        }
    }
}
impl From<UpdateAssetPriceKeys> for [AccountMeta; UPDATE_ASSET_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAssetPriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.updater,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ASSET_PRICE_IX_ACCOUNTS_LEN]> for UpdateAssetPriceKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ASSET_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            updater: pubkeys[0],
            vault: pubkeys[1],
            oracle_account: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateAssetPriceAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ASSET_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAssetPriceAccounts<'_, 'info>) -> Self {
        [
            accounts.updater.clone(),
            accounts.vault.clone(),
            accounts.oracle_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ASSET_PRICE_IX_ACCOUNTS_LEN]>
for UpdateAssetPriceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ASSET_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            updater: &arr[0],
            vault: &arr[1],
            oracle_account: &arr[2],
        }
    }
}
pub const UPDATE_ASSET_PRICE_IX_DISCM: [u8; 8usize] = [152, 55, 6, 120, 250, 34, 204, 1];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateAssetPriceIxArgs {
    pub args: UpdateAssetPriceArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAssetPriceIxData(pub UpdateAssetPriceIxArgs);
impl From<UpdateAssetPriceIxArgs> for UpdateAssetPriceIxData {
    fn from(args: UpdateAssetPriceIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAssetPriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ASSET_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateAssetPriceArgs>::deserialize(&mut reader)?
        };
        Ok(Self(UpdateAssetPriceIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ASSET_PRICE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_asset_price_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateAssetPriceKeys,
    args: UpdateAssetPriceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ASSET_PRICE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateAssetPriceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_asset_price_ix(
    keys: UpdateAssetPriceKeys,
    args: UpdateAssetPriceIxArgs,
) -> std::io::Result<Instruction> {
    update_asset_price_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn update_asset_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAssetPriceAccounts<'_, '_>,
    args: UpdateAssetPriceIxArgs,
) -> ProgramResult {
    let keys: UpdateAssetPriceKeys = accounts.into();
    let ix = update_asset_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_asset_price_invoke(
    accounts: UpdateAssetPriceAccounts<'_, '_>,
    args: UpdateAssetPriceIxArgs,
) -> ProgramResult {
    update_asset_price_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn update_asset_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAssetPriceAccounts<'_, '_>,
    args: UpdateAssetPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateAssetPriceKeys = accounts.into();
    let ix = update_asset_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_asset_price_invoke_signed(
    accounts: UpdateAssetPriceAccounts<'_, '_>,
    args: UpdateAssetPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_asset_price_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_asset_price_verify_account_keys(
    accounts: UpdateAssetPriceAccounts<'_, '_>,
    keys: UpdateAssetPriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.updater.key, keys.updater),
        (*accounts.vault.key, keys.vault),
        (*accounts.oracle_account.key, keys.oracle_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_asset_price_verify_writable_privileges<'me, 'info>(
    accounts: UpdateAssetPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.updater, accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_asset_price_verify_signer_privileges<'me, 'info>(
    accounts: UpdateAssetPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.updater] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_asset_price_verify_account_privileges<'me, 'info>(
    accounts: UpdateAssetPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_asset_price_verify_writable_privileges(accounts)?;
    update_asset_price_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CONSENSUS_ORACLE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateConsensusOracleAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateConsensusOracleKeys {
    pub signer: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub asset_mint: Pubkey,
    pub asset_token_program: Pubkey,
}
impl From<UpdateConsensusOracleAccounts<'_, '_>> for UpdateConsensusOracleKeys {
    fn from(accounts: UpdateConsensusOracleAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            asset_mint: *accounts.asset_mint.key,
            asset_token_program: *accounts.asset_token_program.key,
        }
    }
}
impl From<UpdateConsensusOracleKeys>
for [AccountMeta; UPDATE_CONSENSUS_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateConsensusOracleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CONSENSUS_ORACLE_IX_ACCOUNTS_LEN]>
for UpdateConsensusOracleKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CONSENSUS_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            asset_mint: pubkeys[3],
            asset_token_program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateConsensusOracleAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CONSENSUS_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateConsensusOracleAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.asset_mint.clone(),
            accounts.asset_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CONSENSUS_ORACLE_IX_ACCOUNTS_LEN]>
for UpdateConsensusOracleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CONSENSUS_ORACLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            asset_mint: &arr[3],
            asset_token_program: &arr[4],
        }
    }
}
pub const UPDATE_CONSENSUS_ORACLE_IX_DISCM: [u8; 8usize] = [
    102, 39, 159, 62, 96, 58, 42, 242,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateConsensusOracleIxArgs {
    pub args: UpdateConsensusOracleArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConsensusOracleIxData(pub UpdateConsensusOracleIxArgs);
impl From<UpdateConsensusOracleIxArgs> for UpdateConsensusOracleIxData {
    fn from(args: UpdateConsensusOracleIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateConsensusOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CONSENSUS_ORACLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateConsensusOracleArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateConsensusOracleIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CONSENSUS_ORACLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_consensus_oracle_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateConsensusOracleKeys,
    args: UpdateConsensusOracleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CONSENSUS_ORACLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateConsensusOracleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_consensus_oracle_ix(
    keys: UpdateConsensusOracleKeys,
    args: UpdateConsensusOracleIxArgs,
) -> std::io::Result<Instruction> {
    update_consensus_oracle_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn update_consensus_oracle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConsensusOracleAccounts<'_, '_>,
    args: UpdateConsensusOracleIxArgs,
) -> ProgramResult {
    let keys: UpdateConsensusOracleKeys = accounts.into();
    let ix = update_consensus_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_consensus_oracle_invoke(
    accounts: UpdateConsensusOracleAccounts<'_, '_>,
    args: UpdateConsensusOracleIxArgs,
) -> ProgramResult {
    update_consensus_oracle_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn update_consensus_oracle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConsensusOracleAccounts<'_, '_>,
    args: UpdateConsensusOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateConsensusOracleKeys = accounts.into();
    let ix = update_consensus_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_consensus_oracle_invoke_signed(
    accounts: UpdateConsensusOracleAccounts<'_, '_>,
    args: UpdateConsensusOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_consensus_oracle_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_consensus_oracle_verify_account_keys(
    accounts: UpdateConsensusOracleAccounts<'_, '_>,
    keys: UpdateConsensusOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.asset_token_program.key, keys.asset_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_consensus_oracle_verify_writable_privileges<'me, 'info>(
    accounts: UpdateConsensusOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.vault, accounts.vault_oracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_consensus_oracle_verify_signer_privileges<'me, 'info>(
    accounts: UpdateConsensusOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_consensus_oracle_verify_account_privileges<'me, 'info>(
    accounts: UpdateConsensusOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_consensus_oracle_verify_writable_privileges(accounts)?;
    update_consensus_oracle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CONSENSUS_SIGNERS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateConsensusSignersAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateConsensusSignersKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
}
impl From<UpdateConsensusSignersAccounts<'_, '_>> for UpdateConsensusSignersKeys {
    fn from(accounts: UpdateConsensusSignersAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
        }
    }
}
impl From<UpdateConsensusSignersKeys>
for [AccountMeta; UPDATE_CONSENSUS_SIGNERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateConsensusSignersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CONSENSUS_SIGNERS_IX_ACCOUNTS_LEN]>
for UpdateConsensusSignersKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CONSENSUS_SIGNERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateConsensusSignersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CONSENSUS_SIGNERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateConsensusSignersAccounts<'_, 'info>) -> Self {
        [accounts.curator.clone(), accounts.vault.clone(), accounts.vault_oracle.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_CONSENSUS_SIGNERS_IX_ACCOUNTS_LEN]>
for UpdateConsensusSignersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CONSENSUS_SIGNERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
        }
    }
}
pub const UPDATE_CONSENSUS_SIGNERS_IX_DISCM: [u8; 8usize] = [
    195, 100, 154, 131, 99, 27, 243, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateConsensusSignersIxArgs {
    pub args: UpdateConsensusSignersArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConsensusSignersIxData(pub UpdateConsensusSignersIxArgs);
impl From<UpdateConsensusSignersIxArgs> for UpdateConsensusSignersIxData {
    fn from(args: UpdateConsensusSignersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateConsensusSignersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CONSENSUS_SIGNERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateConsensusSignersArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateConsensusSignersIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CONSENSUS_SIGNERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_consensus_signers_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateConsensusSignersKeys,
    args: UpdateConsensusSignersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CONSENSUS_SIGNERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateConsensusSignersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_consensus_signers_ix(
    keys: UpdateConsensusSignersKeys,
    args: UpdateConsensusSignersIxArgs,
) -> std::io::Result<Instruction> {
    update_consensus_signers_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn update_consensus_signers_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConsensusSignersAccounts<'_, '_>,
    args: UpdateConsensusSignersIxArgs,
) -> ProgramResult {
    let keys: UpdateConsensusSignersKeys = accounts.into();
    let ix = update_consensus_signers_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_consensus_signers_invoke(
    accounts: UpdateConsensusSignersAccounts<'_, '_>,
    args: UpdateConsensusSignersIxArgs,
) -> ProgramResult {
    update_consensus_signers_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn update_consensus_signers_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConsensusSignersAccounts<'_, '_>,
    args: UpdateConsensusSignersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateConsensusSignersKeys = accounts.into();
    let ix = update_consensus_signers_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_consensus_signers_invoke_signed(
    accounts: UpdateConsensusSignersAccounts<'_, '_>,
    args: UpdateConsensusSignersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_consensus_signers_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_consensus_signers_verify_account_keys(
    accounts: UpdateConsensusSignersAccounts<'_, '_>,
    keys: UpdateConsensusSignersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_consensus_signers_verify_writable_privileges<'me, 'info>(
    accounts: UpdateConsensusSignersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault_oracle] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_consensus_signers_verify_signer_privileges<'me, 'info>(
    accounts: UpdateConsensusSignersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_consensus_signers_verify_account_privileges<'me, 'info>(
    accounts: UpdateConsensusSignersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_consensus_signers_verify_writable_privileges(accounts)?;
    update_consensus_signers_verify_signer_privileges(accounts)?;
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
    pub oracles: Option<Vec<Pubkey>>,
    pub price_max_gap_bps: Option<u16>,
    pub price_max_ts_gap: Option<u64>,
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
        let oracles: Option<Vec<Pubkey>> = crate::borsh_de_or_default(&mut reader)?;
        let price_max_gap_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let price_max_ts_gap: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateOracleStateIxArgs {
                oracles,
                price_max_gap_bps,
                price_max_ts_gap,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.oracles, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price_max_gap_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price_max_ts_gap, &mut writer)?;
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
    pub pending_yield: u64,
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
        let pending_yield: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_marker_unix_seconds: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateYieldingInfoIxArgs {
                pending_yield,
                start_marker_unix_seconds,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_YIELDING_INFO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pending_yield, &mut writer)?;
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
pub const VAULT_CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct VaultCreateTrancheStateAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub junior_share_mint: &'me AccountInfo<'info>,
    pub senior_share_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VaultCreateTrancheStateKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub junior_share_mint: Pubkey,
    pub senior_share_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub vault_oracle: Pubkey,
}
impl From<VaultCreateTrancheStateAccounts<'_, '_>> for VaultCreateTrancheStateKeys {
    fn from(accounts: VaultCreateTrancheStateAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            junior_share_mint: *accounts.junior_share_mint.key,
            senior_share_mint: *accounts.senior_share_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            vault_oracle: *accounts.vault_oracle.key,
        }
    }
}
impl From<VaultCreateTrancheStateKeys>
for [AccountMeta; VAULT_CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: VaultCreateTrancheStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.junior_share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.senior_share_mint,
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
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; VAULT_CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN]>
for VaultCreateTrancheStateKeys {
    fn from(pubkeys: [Pubkey; VAULT_CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
            vault_tranche_state: pubkeys[2],
            junior_share_mint: pubkeys[3],
            senior_share_mint: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            vault_oracle: pubkeys[7],
        }
    }
}
impl<'info> From<VaultCreateTrancheStateAccounts<'_, 'info>>
for [AccountInfo<'info>; VAULT_CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: VaultCreateTrancheStateAccounts<'_, 'info>) -> Self {
        [
            accounts.curator.clone(),
            accounts.vault.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.junior_share_mint.clone(),
            accounts.senior_share_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.vault_oracle.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; VAULT_CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN]>
for VaultCreateTrancheStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; VAULT_CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
            vault_tranche_state: &arr[2],
            junior_share_mint: &arr[3],
            senior_share_mint: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            vault_oracle: &arr[7],
        }
    }
}
pub const VAULT_CREATE_TRANCHE_STATE_IX_DISCM: [u8; 8usize] = [
    48, 93, 209, 143, 250, 99, 157, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VaultCreateTrancheStateIxArgs {
    pub args: VaultCreateTrancheStateArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultCreateTrancheStateIxData(pub VaultCreateTrancheStateIxArgs);
impl From<VaultCreateTrancheStateIxArgs> for VaultCreateTrancheStateIxData {
    fn from(args: VaultCreateTrancheStateIxArgs) -> Self {
        Self(args)
    }
}
impl VaultCreateTrancheStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_CREATE_TRANCHE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <VaultCreateTrancheStateArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(VaultCreateTrancheStateIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_CREATE_TRANCHE_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn vault_create_tranche_state_ix_with_program_id(
    program_id: Pubkey,
    keys: VaultCreateTrancheStateKeys,
    args: VaultCreateTrancheStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VAULT_CREATE_TRANCHE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: VaultCreateTrancheStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn vault_create_tranche_state_ix(
    keys: VaultCreateTrancheStateKeys,
    args: VaultCreateTrancheStateIxArgs,
) -> std::io::Result<Instruction> {
    vault_create_tranche_state_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn vault_create_tranche_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: VaultCreateTrancheStateAccounts<'_, '_>,
    args: VaultCreateTrancheStateIxArgs,
) -> ProgramResult {
    let keys: VaultCreateTrancheStateKeys = accounts.into();
    let ix = vault_create_tranche_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn vault_create_tranche_state_invoke(
    accounts: VaultCreateTrancheStateAccounts<'_, '_>,
    args: VaultCreateTrancheStateIxArgs,
) -> ProgramResult {
    vault_create_tranche_state_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn vault_create_tranche_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: VaultCreateTrancheStateAccounts<'_, '_>,
    args: VaultCreateTrancheStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: VaultCreateTrancheStateKeys = accounts.into();
    let ix = vault_create_tranche_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn vault_create_tranche_state_invoke_signed(
    accounts: VaultCreateTrancheStateAccounts<'_, '_>,
    args: VaultCreateTrancheStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    vault_create_tranche_state_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn vault_create_tranche_state_verify_account_keys(
    accounts: VaultCreateTrancheStateAccounts<'_, '_>,
    keys: VaultCreateTrancheStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.junior_share_mint.key, keys.junior_share_mint),
        (*accounts.senior_share_mint.key, keys.senior_share_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.vault_oracle.key, keys.vault_oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn vault_create_tranche_state_verify_writable_privileges<'me, 'info>(
    accounts: VaultCreateTrancheStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curator,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.junior_share_mint,
        accounts.senior_share_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn vault_create_tranche_state_verify_signer_privileges<'me, 'info>(
    accounts: VaultCreateTrancheStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn vault_create_tranche_state_verify_account_privileges<'me, 'info>(
    accounts: VaultCreateTrancheStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    vault_create_tranche_state_verify_writable_privileges(accounts)?;
    vault_create_tranche_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const VAULT_REALLOCATION_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct VaultReallocationAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub allocation_vault: &'me AccountInfo<'info>,
    pub source_vault: &'me AccountInfo<'info>,
    pub source_vault_oracle: &'me AccountInfo<'info>,
    pub destination_vault: &'me AccountInfo<'info>,
    pub destination_vault_oracle: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub source_share_mint: &'me AccountInfo<'info>,
    pub destination_share_mint: &'me AccountInfo<'info>,
    pub allocation_mint_ata: &'me AccountInfo<'info>,
    pub source_vault_mint_ata: &'me AccountInfo<'info>,
    pub destination_vault_mint_ata: &'me AccountInfo<'info>,
    pub destination_fee_vault: &'me AccountInfo<'info>,
    pub destination_fee_vault_mint_ata: &'me AccountInfo<'info>,
    pub allocation_source_share_ata: &'me AccountInfo<'info>,
    pub allocation_destination_share_ata: &'me AccountInfo<'info>,
    pub asset_oracle_account: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub source_share_token_program: &'me AccountInfo<'info>,
    pub destination_share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub source_vault_tranche_state: &'me AccountInfo<'info>,
    pub destination_vault_tranche_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VaultReallocationKeys {
    pub manager: Pubkey,
    pub allocation_vault: Pubkey,
    pub source_vault: Pubkey,
    pub source_vault_oracle: Pubkey,
    pub destination_vault: Pubkey,
    pub destination_vault_oracle: Pubkey,
    pub mint: Pubkey,
    pub source_share_mint: Pubkey,
    pub destination_share_mint: Pubkey,
    pub allocation_mint_ata: Pubkey,
    pub source_vault_mint_ata: Pubkey,
    pub destination_vault_mint_ata: Pubkey,
    pub destination_fee_vault: Pubkey,
    pub destination_fee_vault_mint_ata: Pubkey,
    pub allocation_source_share_ata: Pubkey,
    pub allocation_destination_share_ata: Pubkey,
    pub asset_oracle_account: Pubkey,
    pub asset_token_program: Pubkey,
    pub source_share_token_program: Pubkey,
    pub destination_share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub source_vault_tranche_state: Pubkey,
    pub destination_vault_tranche_state: Pubkey,
}
impl From<VaultReallocationAccounts<'_, '_>> for VaultReallocationKeys {
    fn from(accounts: VaultReallocationAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            allocation_vault: *accounts.allocation_vault.key,
            source_vault: *accounts.source_vault.key,
            source_vault_oracle: *accounts.source_vault_oracle.key,
            destination_vault: *accounts.destination_vault.key,
            destination_vault_oracle: *accounts.destination_vault_oracle.key,
            mint: *accounts.mint.key,
            source_share_mint: *accounts.source_share_mint.key,
            destination_share_mint: *accounts.destination_share_mint.key,
            allocation_mint_ata: *accounts.allocation_mint_ata.key,
            source_vault_mint_ata: *accounts.source_vault_mint_ata.key,
            destination_vault_mint_ata: *accounts.destination_vault_mint_ata.key,
            destination_fee_vault: *accounts.destination_fee_vault.key,
            destination_fee_vault_mint_ata: *accounts.destination_fee_vault_mint_ata.key,
            allocation_source_share_ata: *accounts.allocation_source_share_ata.key,
            allocation_destination_share_ata: *accounts
                .allocation_destination_share_ata
                .key,
            asset_oracle_account: *accounts.asset_oracle_account.key,
            asset_token_program: *accounts.asset_token_program.key,
            source_share_token_program: *accounts.source_share_token_program.key,
            destination_share_token_program: *accounts
                .destination_share_token_program
                .key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            source_vault_tranche_state: *accounts.source_vault_tranche_state.key,
            destination_vault_tranche_state: *accounts
                .destination_vault_tranche_state
                .key,
        }
    }
}
impl From<VaultReallocationKeys> for [AccountMeta; VAULT_REALLOCATION_IX_ACCOUNTS_LEN] {
    fn from(keys: VaultReallocationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.allocation_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.allocation_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_vault_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_fee_vault_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.allocation_source_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.allocation_destination_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_oracle_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; VAULT_REALLOCATION_IX_ACCOUNTS_LEN]> for VaultReallocationKeys {
    fn from(pubkeys: [Pubkey; VAULT_REALLOCATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            allocation_vault: pubkeys[1],
            source_vault: pubkeys[2],
            source_vault_oracle: pubkeys[3],
            destination_vault: pubkeys[4],
            destination_vault_oracle: pubkeys[5],
            mint: pubkeys[6],
            source_share_mint: pubkeys[7],
            destination_share_mint: pubkeys[8],
            allocation_mint_ata: pubkeys[9],
            source_vault_mint_ata: pubkeys[10],
            destination_vault_mint_ata: pubkeys[11],
            destination_fee_vault: pubkeys[12],
            destination_fee_vault_mint_ata: pubkeys[13],
            allocation_source_share_ata: pubkeys[14],
            allocation_destination_share_ata: pubkeys[15],
            asset_oracle_account: pubkeys[16],
            asset_token_program: pubkeys[17],
            source_share_token_program: pubkeys[18],
            destination_share_token_program: pubkeys[19],
            associated_token_program: pubkeys[20],
            system_program: pubkeys[21],
            source_vault_tranche_state: pubkeys[22],
            destination_vault_tranche_state: pubkeys[23],
        }
    }
}
impl<'info> From<VaultReallocationAccounts<'_, 'info>>
for [AccountInfo<'info>; VAULT_REALLOCATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: VaultReallocationAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.allocation_vault.clone(),
            accounts.source_vault.clone(),
            accounts.source_vault_oracle.clone(),
            accounts.destination_vault.clone(),
            accounts.destination_vault_oracle.clone(),
            accounts.mint.clone(),
            accounts.source_share_mint.clone(),
            accounts.destination_share_mint.clone(),
            accounts.allocation_mint_ata.clone(),
            accounts.source_vault_mint_ata.clone(),
            accounts.destination_vault_mint_ata.clone(),
            accounts.destination_fee_vault.clone(),
            accounts.destination_fee_vault_mint_ata.clone(),
            accounts.allocation_source_share_ata.clone(),
            accounts.allocation_destination_share_ata.clone(),
            accounts.asset_oracle_account.clone(),
            accounts.asset_token_program.clone(),
            accounts.source_share_token_program.clone(),
            accounts.destination_share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.source_vault_tranche_state.clone(),
            accounts.destination_vault_tranche_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; VAULT_REALLOCATION_IX_ACCOUNTS_LEN]>
for VaultReallocationAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; VAULT_REALLOCATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            allocation_vault: &arr[1],
            source_vault: &arr[2],
            source_vault_oracle: &arr[3],
            destination_vault: &arr[4],
            destination_vault_oracle: &arr[5],
            mint: &arr[6],
            source_share_mint: &arr[7],
            destination_share_mint: &arr[8],
            allocation_mint_ata: &arr[9],
            source_vault_mint_ata: &arr[10],
            destination_vault_mint_ata: &arr[11],
            destination_fee_vault: &arr[12],
            destination_fee_vault_mint_ata: &arr[13],
            allocation_source_share_ata: &arr[14],
            allocation_destination_share_ata: &arr[15],
            asset_oracle_account: &arr[16],
            asset_token_program: &arr[17],
            source_share_token_program: &arr[18],
            destination_share_token_program: &arr[19],
            associated_token_program: &arr[20],
            system_program: &arr[21],
            source_vault_tranche_state: &arr[22],
            destination_vault_tranche_state: &arr[23],
        }
    }
}
pub const VAULT_REALLOCATION_IX_DISCM: [u8; 8usize] = [
    172, 165, 110, 227, 79, 143, 53, 117,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VaultReallocationIxArgs {
    pub args: VaultReallocationArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultReallocationIxData(pub VaultReallocationIxArgs);
impl From<VaultReallocationIxArgs> for VaultReallocationIxData {
    fn from(args: VaultReallocationIxArgs) -> Self {
        Self(args)
    }
}
impl VaultReallocationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_REALLOCATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <VaultReallocationArgs>::deserialize(&mut reader)?
        };
        Ok(Self(VaultReallocationIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_REALLOCATION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn vault_reallocation_ix_with_program_id(
    program_id: Pubkey,
    keys: VaultReallocationKeys,
    args: VaultReallocationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VAULT_REALLOCATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: VaultReallocationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn vault_reallocation_ix(
    keys: VaultReallocationKeys,
    args: VaultReallocationIxArgs,
) -> std::io::Result<Instruction> {
    vault_reallocation_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn vault_reallocation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: VaultReallocationAccounts<'_, '_>,
    args: VaultReallocationIxArgs,
) -> ProgramResult {
    let keys: VaultReallocationKeys = accounts.into();
    let ix = vault_reallocation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn vault_reallocation_invoke(
    accounts: VaultReallocationAccounts<'_, '_>,
    args: VaultReallocationIxArgs,
) -> ProgramResult {
    vault_reallocation_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn vault_reallocation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: VaultReallocationAccounts<'_, '_>,
    args: VaultReallocationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: VaultReallocationKeys = accounts.into();
    let ix = vault_reallocation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn vault_reallocation_invoke_signed(
    accounts: VaultReallocationAccounts<'_, '_>,
    args: VaultReallocationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    vault_reallocation_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn vault_reallocation_verify_account_keys(
    accounts: VaultReallocationAccounts<'_, '_>,
    keys: VaultReallocationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.allocation_vault.key, keys.allocation_vault),
        (*accounts.source_vault.key, keys.source_vault),
        (*accounts.source_vault_oracle.key, keys.source_vault_oracle),
        (*accounts.destination_vault.key, keys.destination_vault),
        (*accounts.destination_vault_oracle.key, keys.destination_vault_oracle),
        (*accounts.mint.key, keys.mint),
        (*accounts.source_share_mint.key, keys.source_share_mint),
        (*accounts.destination_share_mint.key, keys.destination_share_mint),
        (*accounts.allocation_mint_ata.key, keys.allocation_mint_ata),
        (*accounts.source_vault_mint_ata.key, keys.source_vault_mint_ata),
        (*accounts.destination_vault_mint_ata.key, keys.destination_vault_mint_ata),
        (*accounts.destination_fee_vault.key, keys.destination_fee_vault),
        (
            *accounts.destination_fee_vault_mint_ata.key,
            keys.destination_fee_vault_mint_ata,
        ),
        (*accounts.allocation_source_share_ata.key, keys.allocation_source_share_ata),
        (
            *accounts.allocation_destination_share_ata.key,
            keys.allocation_destination_share_ata,
        ),
        (*accounts.asset_oracle_account.key, keys.asset_oracle_account),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.source_share_token_program.key, keys.source_share_token_program),
        (
            *accounts.destination_share_token_program.key,
            keys.destination_share_token_program,
        ),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.source_vault_tranche_state.key, keys.source_vault_tranche_state),
        (
            *accounts.destination_vault_tranche_state.key,
            keys.destination_vault_tranche_state,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn vault_reallocation_verify_writable_privileges<'me, 'info>(
    accounts: VaultReallocationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.allocation_vault,
        accounts.source_vault,
        accounts.destination_vault,
        accounts.source_share_mint,
        accounts.destination_share_mint,
        accounts.allocation_mint_ata,
        accounts.source_vault_mint_ata,
        accounts.destination_vault_mint_ata,
        accounts.destination_fee_vault,
        accounts.destination_fee_vault_mint_ata,
        accounts.allocation_source_share_ata,
        accounts.allocation_destination_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn vault_reallocation_verify_signer_privileges<'me, 'info>(
    accounts: VaultReallocationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn vault_reallocation_verify_account_privileges<'me, 'info>(
    accounts: VaultReallocationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    vault_reallocation_verify_writable_privileges(accounts)?;
    vault_reallocation_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const VAULT_REALLOCATOR_IX_ACCOUNTS_LEN: usize = 19;
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
    pub tranche_state: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub bank_token_program: &'me AccountInfo<'info>,
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
    pub tranche_state: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub bank_token_program: Pubkey,
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
            tranche_state: *accounts.tranche_state.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            bank_token_program: *accounts.bank_token_program.key,
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
                pubkey: keys.bank_token_program,
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
            tranche_state: pubkeys[16],
            junior_escrow_ata: pubkeys[17],
            bank_token_program: pubkeys[18],
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
            accounts.tranche_state.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.bank_token_program.clone(),
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
            tranche_state: &arr[16],
            junior_escrow_ata: &arr[17],
            bank_token_program: &arr[18],
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
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.bank_token_program.key, keys.bank_token_program),
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
        accounts.tranche_state,
        accounts.junior_escrow_ata,
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
pub const VAULT_UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct VaultUpdateTrancheConfigAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VaultUpdateTrancheConfigKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub vault_oracle: Pubkey,
}
impl From<VaultUpdateTrancheConfigAccounts<'_, '_>> for VaultUpdateTrancheConfigKeys {
    fn from(accounts: VaultUpdateTrancheConfigAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            vault_oracle: *accounts.vault_oracle.key,
        }
    }
}
impl From<VaultUpdateTrancheConfigKeys>
for [AccountMeta; VAULT_UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: VaultUpdateTrancheConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; VAULT_UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN]>
for VaultUpdateTrancheConfigKeys {
    fn from(pubkeys: [Pubkey; VAULT_UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
            vault_tranche_state: pubkeys[2],
            vault_oracle: pubkeys[3],
        }
    }
}
impl<'info> From<VaultUpdateTrancheConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; VAULT_UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: VaultUpdateTrancheConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.curator.clone(),
            accounts.vault.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.vault_oracle.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; VAULT_UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN]>
for VaultUpdateTrancheConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; VAULT_UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
            vault_tranche_state: &arr[2],
            vault_oracle: &arr[3],
        }
    }
}
pub const VAULT_UPDATE_TRANCHE_CONFIG_IX_DISCM: [u8; 8usize] = [
    113, 72, 5, 24, 177, 252, 221, 58,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VaultUpdateTrancheConfigIxArgs {
    pub args: VaultUpdateTrancheConfigArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultUpdateTrancheConfigIxData(pub VaultUpdateTrancheConfigIxArgs);
impl From<VaultUpdateTrancheConfigIxArgs> for VaultUpdateTrancheConfigIxData {
    fn from(args: VaultUpdateTrancheConfigIxArgs) -> Self {
        Self(args)
    }
}
impl VaultUpdateTrancheConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_UPDATE_TRANCHE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <VaultUpdateTrancheConfigArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(VaultUpdateTrancheConfigIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_UPDATE_TRANCHE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn vault_update_tranche_config_ix_with_program_id(
    program_id: Pubkey,
    keys: VaultUpdateTrancheConfigKeys,
    args: VaultUpdateTrancheConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VAULT_UPDATE_TRANCHE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: VaultUpdateTrancheConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn vault_update_tranche_config_ix(
    keys: VaultUpdateTrancheConfigKeys,
    args: VaultUpdateTrancheConfigIxArgs,
) -> std::io::Result<Instruction> {
    vault_update_tranche_config_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn vault_update_tranche_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: VaultUpdateTrancheConfigAccounts<'_, '_>,
    args: VaultUpdateTrancheConfigIxArgs,
) -> ProgramResult {
    let keys: VaultUpdateTrancheConfigKeys = accounts.into();
    let ix = vault_update_tranche_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn vault_update_tranche_config_invoke(
    accounts: VaultUpdateTrancheConfigAccounts<'_, '_>,
    args: VaultUpdateTrancheConfigIxArgs,
) -> ProgramResult {
    vault_update_tranche_config_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn vault_update_tranche_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: VaultUpdateTrancheConfigAccounts<'_, '_>,
    args: VaultUpdateTrancheConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: VaultUpdateTrancheConfigKeys = accounts.into();
    let ix = vault_update_tranche_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn vault_update_tranche_config_invoke_signed(
    accounts: VaultUpdateTrancheConfigAccounts<'_, '_>,
    args: VaultUpdateTrancheConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    vault_update_tranche_config_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn vault_update_tranche_config_verify_account_keys(
    accounts: VaultUpdateTrancheConfigAccounts<'_, '_>,
    keys: VaultUpdateTrancheConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.vault_oracle.key, keys.vault_oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn vault_update_tranche_config_verify_writable_privileges<'me, 'info>(
    accounts: VaultUpdateTrancheConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault_tranche_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn vault_update_tranche_config_verify_signer_privileges<'me, 'info>(
    accounts: VaultUpdateTrancheConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn vault_update_tranche_config_verify_account_privileges<'me, 'info>(
    accounts: VaultUpdateTrancheConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    vault_update_tranche_config_verify_writable_privileges(accounts)?;
    vault_update_tranche_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const VAULT_WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct VaultWithdrawTrancheFeesAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub curator_asset_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VaultWithdrawTrancheFeesKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub curator_asset_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<VaultWithdrawTrancheFeesAccounts<'_, '_>> for VaultWithdrawTrancheFeesKeys {
    fn from(accounts: VaultWithdrawTrancheFeesAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            curator_asset_ata: *accounts.curator_asset_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<VaultWithdrawTrancheFeesKeys>
for [AccountMeta; VAULT_WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: VaultWithdrawTrancheFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curator_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; VAULT_WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN]>
for VaultWithdrawTrancheFeesKeys {
    fn from(pubkeys: [Pubkey; VAULT_WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
            vault_tranche_state: pubkeys[2],
            asset_mint: pubkeys[3],
            vault_asset_ata: pubkeys[4],
            curator_asset_ata: pubkeys[5],
            asset_token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<VaultWithdrawTrancheFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; VAULT_WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: VaultWithdrawTrancheFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.curator.clone(),
            accounts.vault.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.curator_asset_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; VAULT_WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN]>
for VaultWithdrawTrancheFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; VAULT_WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
            vault_tranche_state: &arr[2],
            asset_mint: &arr[3],
            vault_asset_ata: &arr[4],
            curator_asset_ata: &arr[5],
            asset_token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const VAULT_WITHDRAW_TRANCHE_FEES_IX_DISCM: [u8; 8usize] = [
    25, 25, 207, 236, 25, 9, 246, 155,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VaultWithdrawTrancheFeesIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultWithdrawTrancheFeesIxData(pub VaultWithdrawTrancheFeesIxArgs);
impl From<VaultWithdrawTrancheFeesIxArgs> for VaultWithdrawTrancheFeesIxData {
    fn from(args: VaultWithdrawTrancheFeesIxArgs) -> Self {
        Self(args)
    }
}
impl VaultWithdrawTrancheFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_WITHDRAW_TRANCHE_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(VaultWithdrawTrancheFeesIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_WITHDRAW_TRANCHE_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn vault_withdraw_tranche_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: VaultWithdrawTrancheFeesKeys,
    args: VaultWithdrawTrancheFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VAULT_WITHDRAW_TRANCHE_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: VaultWithdrawTrancheFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn vault_withdraw_tranche_fees_ix(
    keys: VaultWithdrawTrancheFeesKeys,
    args: VaultWithdrawTrancheFeesIxArgs,
) -> std::io::Result<Instruction> {
    vault_withdraw_tranche_fees_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn vault_withdraw_tranche_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: VaultWithdrawTrancheFeesAccounts<'_, '_>,
    args: VaultWithdrawTrancheFeesIxArgs,
) -> ProgramResult {
    let keys: VaultWithdrawTrancheFeesKeys = accounts.into();
    let ix = vault_withdraw_tranche_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn vault_withdraw_tranche_fees_invoke(
    accounts: VaultWithdrawTrancheFeesAccounts<'_, '_>,
    args: VaultWithdrawTrancheFeesIxArgs,
) -> ProgramResult {
    vault_withdraw_tranche_fees_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn vault_withdraw_tranche_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: VaultWithdrawTrancheFeesAccounts<'_, '_>,
    args: VaultWithdrawTrancheFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: VaultWithdrawTrancheFeesKeys = accounts.into();
    let ix = vault_withdraw_tranche_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn vault_withdraw_tranche_fees_invoke_signed(
    accounts: VaultWithdrawTrancheFeesAccounts<'_, '_>,
    args: VaultWithdrawTrancheFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    vault_withdraw_tranche_fees_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn vault_withdraw_tranche_fees_verify_account_keys(
    accounts: VaultWithdrawTrancheFeesAccounts<'_, '_>,
    keys: VaultWithdrawTrancheFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.curator_asset_ata.key, keys.curator_asset_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn vault_withdraw_tranche_fees_verify_writable_privileges<'me, 'info>(
    accounts: VaultWithdrawTrancheFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curator,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.vault_asset_ata,
        accounts.curator_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn vault_withdraw_tranche_fees_verify_signer_privileges<'me, 'info>(
    accounts: VaultWithdrawTrancheFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn vault_withdraw_tranche_fees_verify_account_privileges<'me, 'info>(
    accounts: VaultWithdrawTrancheFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    vault_withdraw_tranche_fees_verify_writable_privileges(accounts)?;
    vault_withdraw_tranche_fees_verify_signer_privileges(accounts)?;
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
pub const WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeesAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub curator_asset_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub curator_asset_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<WithdrawProtocolFeesAccounts<'_, '_>> for WithdrawProtocolFeesKeys {
    fn from(accounts: WithdrawProtocolFeesAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            curator_asset_ata: *accounts.curator_asset_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<WithdrawProtocolFeesKeys>
for [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProtocolFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curator_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
            asset_mint: pubkeys[2],
            vault_asset_ata: pubkeys[3],
            curator_asset_ata: pubkeys[4],
            asset_token_program: pubkeys[5],
            associated_token_program: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<WithdrawProtocolFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProtocolFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.curator.clone(),
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.curator_asset_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
            asset_mint: &arr[2],
            vault_asset_ata: &arr[3],
            curator_asset_ata: &arr[4],
            asset_token_program: &arr[5],
            associated_token_program: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const WITHDRAW_PROTOCOL_FEES_IX_DISCM: [u8; 8usize] = [
    11, 68, 165, 98, 18, 208, 134, 73,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawProtocolFeesIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesIxData(pub WithdrawProtocolFeesIxArgs);
impl From<WithdrawProtocolFeesIxArgs> for WithdrawProtocolFeesIxData {
    fn from(args: WithdrawProtocolFeesIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawProtocolFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawProtocolFeesIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PROTOCOL_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_protocol_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProtocolFeesKeys,
    args: WithdrawProtocolFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawProtocolFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fees_ix(
    keys: WithdrawProtocolFeesKeys,
    args: WithdrawProtocolFeesIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_protocol_fees_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn withdraw_protocol_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    args: WithdrawProtocolFeesIxArgs,
) -> ProgramResult {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let ix = withdraw_protocol_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_protocol_fees_invoke(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    args: WithdrawProtocolFeesIxArgs,
) -> ProgramResult {
    withdraw_protocol_fees_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn withdraw_protocol_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    args: WithdrawProtocolFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let ix = withdraw_protocol_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_protocol_fees_invoke_signed(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    args: WithdrawProtocolFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_protocol_fees_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_protocol_fees_verify_account_keys(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    keys: WithdrawProtocolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.curator_asset_ata.key, keys.curator_asset_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curator,
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.curator_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_protocol_fees_verify_writable_privileges(accounts)?;
    withdraw_protocol_fees_verify_signer_privileges(accounts)?;
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
pub const CRANK_NAV_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CrankNavAccounts<'me, 'info> {
    pub cranker: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub junior_tranche_share_mint: &'me AccountInfo<'info>,
    pub senior_tranche_share_mint: &'me AccountInfo<'info>,
    pub fee_asset_mint: &'me AccountInfo<'info>,
    pub vault_fee_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub fee_asset_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CrankNavKeys {
    pub cranker: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub share_mint: Pubkey,
    pub junior_tranche_share_mint: Pubkey,
    pub senior_tranche_share_mint: Pubkey,
    pub fee_asset_mint: Pubkey,
    pub vault_fee_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub fee_asset_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CrankNavAccounts<'_, '_>> for CrankNavKeys {
    fn from(accounts: CrankNavAccounts) -> Self {
        Self {
            cranker: *accounts.cranker.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            share_mint: *accounts.share_mint.key,
            junior_tranche_share_mint: *accounts.junior_tranche_share_mint.key,
            senior_tranche_share_mint: *accounts.senior_tranche_share_mint.key,
            fee_asset_mint: *accounts.fee_asset_mint.key,
            vault_fee_asset_ata: *accounts.vault_fee_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            fee_asset_token_program: *accounts.fee_asset_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CrankNavKeys> for [AccountMeta; CRANK_NAV_IX_ACCOUNTS_LEN] {
    fn from(keys: CrankNavKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.cranker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.junior_tranche_share_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.senior_tranche_share_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_fee_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; CRANK_NAV_IX_ACCOUNTS_LEN]> for CrankNavKeys {
    fn from(pubkeys: [Pubkey; CRANK_NAV_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            cranker: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            share_mint: pubkeys[4],
            junior_tranche_share_mint: pubkeys[5],
            senior_tranche_share_mint: pubkeys[6],
            fee_asset_mint: pubkeys[7],
            vault_fee_asset_ata: pubkeys[8],
            fee_vault: pubkeys[9],
            fee_vault_ata: pubkeys[10],
            fee_asset_token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<CrankNavAccounts<'_, 'info>>
for [AccountInfo<'info>; CRANK_NAV_IX_ACCOUNTS_LEN] {
    fn from(accounts: CrankNavAccounts<'_, 'info>) -> Self {
        [
            accounts.cranker.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.share_mint.clone(),
            accounts.junior_tranche_share_mint.clone(),
            accounts.senior_tranche_share_mint.clone(),
            accounts.fee_asset_mint.clone(),
            accounts.vault_fee_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.fee_asset_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CRANK_NAV_IX_ACCOUNTS_LEN]>
for CrankNavAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CRANK_NAV_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            cranker: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            share_mint: &arr[4],
            junior_tranche_share_mint: &arr[5],
            senior_tranche_share_mint: &arr[6],
            fee_asset_mint: &arr[7],
            vault_fee_asset_ata: &arr[8],
            fee_vault: &arr[9],
            fee_vault_ata: &arr[10],
            fee_asset_token_program: &arr[11],
            associated_token_program: &arr[12],
            system_program: &arr[13],
        }
    }
}
pub const CRANK_NAV_IX_DISCM: [u8; 8usize] = [39, 40, 95, 96, 225, 149, 78, 141];
#[derive(Clone, Debug, PartialEq)]
pub struct CrankNavIxData;
impl CrankNavIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CRANK_NAV_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CRANK_NAV_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn crank_nav_ix_with_program_id(
    program_id: Pubkey,
    keys: CrankNavKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CRANK_NAV_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CrankNavIxData.try_to_vec()?,
    })
}
pub fn crank_nav_ix(keys: CrankNavKeys) -> std::io::Result<Instruction> {
    crank_nav_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn crank_nav_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CrankNavAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CrankNavKeys = accounts.into();
    let ix = crank_nav_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn crank_nav_invoke(accounts: CrankNavAccounts<'_, '_>) -> ProgramResult {
    crank_nav_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn crank_nav_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CrankNavAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CrankNavKeys = accounts.into();
    let ix = crank_nav_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn crank_nav_invoke_signed(
    accounts: CrankNavAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    crank_nav_invoke_signed_with_program_id(BANKINECO_PROGRAM_ID, accounts, seeds)
}
pub fn crank_nav_verify_account_keys(
    accounts: CrankNavAccounts<'_, '_>,
    keys: CrankNavKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.cranker.key, keys.cranker),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.junior_tranche_share_mint.key, keys.junior_tranche_share_mint),
        (*accounts.senior_tranche_share_mint.key, keys.senior_tranche_share_mint),
        (*accounts.fee_asset_mint.key, keys.fee_asset_mint),
        (*accounts.vault_fee_asset_ata.key, keys.vault_fee_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.fee_asset_token_program.key, keys.fee_asset_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn crank_nav_verify_writable_privileges<'me, 'info>(
    accounts: CrankNavAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.cranker,
        accounts.vault,
        accounts.vault_oracle,
        accounts.vault_tranche_state,
        accounts.vault_fee_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn crank_nav_verify_signer_privileges<'me, 'info>(
    accounts: CrankNavAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.cranker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn crank_nav_verify_account_privileges<'me, 'info>(
    accounts: CrankNavAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    crank_nav_verify_writable_privileges(accounts)?;
    crank_nav_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CRANK_PERFORMANCE_FEES_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CrankPerformanceFeesAccounts<'me, 'info> {
    pub cranker: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub fee_asset_mint: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub fee_collector: &'me AccountInfo<'info>,
    pub fee_collector_fee_asset_ata: &'me AccountInfo<'info>,
    pub protocol_fee_collector: &'me AccountInfo<'info>,
    pub protocol_fee_collector_fee_asset_ata: &'me AccountInfo<'info>,
    pub fee_asset_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CrankPerformanceFeesKeys {
    pub cranker: Pubkey,
    pub vault: Pubkey,
    pub fee_asset_mint: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub fee_collector: Pubkey,
    pub fee_collector_fee_asset_ata: Pubkey,
    pub protocol_fee_collector: Pubkey,
    pub protocol_fee_collector_fee_asset_ata: Pubkey,
    pub fee_asset_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CrankPerformanceFeesAccounts<'_, '_>> for CrankPerformanceFeesKeys {
    fn from(accounts: CrankPerformanceFeesAccounts) -> Self {
        Self {
            cranker: *accounts.cranker.key,
            vault: *accounts.vault.key,
            fee_asset_mint: *accounts.fee_asset_mint.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            fee_collector: *accounts.fee_collector.key,
            fee_collector_fee_asset_ata: *accounts.fee_collector_fee_asset_ata.key,
            protocol_fee_collector: *accounts.protocol_fee_collector.key,
            protocol_fee_collector_fee_asset_ata: *accounts
                .protocol_fee_collector_fee_asset_ata
                .key,
            fee_asset_token_program: *accounts.fee_asset_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CrankPerformanceFeesKeys>
for [AccountMeta; CRANK_PERFORMANCE_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CrankPerformanceFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.cranker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_collector,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_collector_fee_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_collector,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_collector_fee_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; CRANK_PERFORMANCE_FEES_IX_ACCOUNTS_LEN]>
for CrankPerformanceFeesKeys {
    fn from(pubkeys: [Pubkey; CRANK_PERFORMANCE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            cranker: pubkeys[0],
            vault: pubkeys[1],
            fee_asset_mint: pubkeys[2],
            fee_vault: pubkeys[3],
            fee_vault_ata: pubkeys[4],
            fee_collector: pubkeys[5],
            fee_collector_fee_asset_ata: pubkeys[6],
            protocol_fee_collector: pubkeys[7],
            protocol_fee_collector_fee_asset_ata: pubkeys[8],
            fee_asset_token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<CrankPerformanceFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; CRANK_PERFORMANCE_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CrankPerformanceFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.cranker.clone(),
            accounts.vault.clone(),
            accounts.fee_asset_mint.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.fee_collector.clone(),
            accounts.fee_collector_fee_asset_ata.clone(),
            accounts.protocol_fee_collector.clone(),
            accounts.protocol_fee_collector_fee_asset_ata.clone(),
            accounts.fee_asset_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CRANK_PERFORMANCE_FEES_IX_ACCOUNTS_LEN]>
for CrankPerformanceFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CRANK_PERFORMANCE_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            cranker: &arr[0],
            vault: &arr[1],
            fee_asset_mint: &arr[2],
            fee_vault: &arr[3],
            fee_vault_ata: &arr[4],
            fee_collector: &arr[5],
            fee_collector_fee_asset_ata: &arr[6],
            protocol_fee_collector: &arr[7],
            protocol_fee_collector_fee_asset_ata: &arr[8],
            fee_asset_token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const CRANK_PERFORMANCE_FEES_IX_DISCM: [u8; 8usize] = [
    192, 232, 106, 211, 23, 69, 35, 3,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CrankPerformanceFeesIxData;
impl CrankPerformanceFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CRANK_PERFORMANCE_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CRANK_PERFORMANCE_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn crank_performance_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CrankPerformanceFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CRANK_PERFORMANCE_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CrankPerformanceFeesIxData.try_to_vec()?,
    })
}
pub fn crank_performance_fees_ix(
    keys: CrankPerformanceFeesKeys,
) -> std::io::Result<Instruction> {
    crank_performance_fees_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn crank_performance_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CrankPerformanceFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CrankPerformanceFeesKeys = accounts.into();
    let ix = crank_performance_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn crank_performance_fees_invoke(
    accounts: CrankPerformanceFeesAccounts<'_, '_>,
) -> ProgramResult {
    crank_performance_fees_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn crank_performance_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CrankPerformanceFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CrankPerformanceFeesKeys = accounts.into();
    let ix = crank_performance_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn crank_performance_fees_invoke_signed(
    accounts: CrankPerformanceFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    crank_performance_fees_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn crank_performance_fees_verify_account_keys(
    accounts: CrankPerformanceFeesAccounts<'_, '_>,
    keys: CrankPerformanceFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.cranker.key, keys.cranker),
        (*accounts.vault.key, keys.vault),
        (*accounts.fee_asset_mint.key, keys.fee_asset_mint),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.fee_collector.key, keys.fee_collector),
        (*accounts.fee_collector_fee_asset_ata.key, keys.fee_collector_fee_asset_ata),
        (*accounts.protocol_fee_collector.key, keys.protocol_fee_collector),
        (
            *accounts.protocol_fee_collector_fee_asset_ata.key,
            keys.protocol_fee_collector_fee_asset_ata,
        ),
        (*accounts.fee_asset_token_program.key, keys.fee_asset_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn crank_performance_fees_verify_writable_privileges<'me, 'info>(
    accounts: CrankPerformanceFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.cranker,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.fee_collector_fee_asset_ata,
        accounts.protocol_fee_collector_fee_asset_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn crank_performance_fees_verify_signer_privileges<'me, 'info>(
    accounts: CrankPerformanceFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.cranker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn crank_performance_fees_verify_account_privileges<'me, 'info>(
    accounts: CrankPerformanceFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    crank_performance_fees_verify_writable_privileges(accounts)?;
    crank_performance_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_IDLE_BANK_MINT_VAULT_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct MigrateIdleBankMintVaultAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub idle_bank_state: &'me AccountInfo<'info>,
    pub idle_bank_mint: &'me AccountInfo<'info>,
    pub idle_bank_tranche: &'me AccountInfo<'info>,
    pub idle_vault: &'me AccountInfo<'info>,
    pub idle_vault_oracle: &'me AccountInfo<'info>,
    pub idle_vault_team: &'me AccountInfo<'info>,
    pub idle_vault_bank_mint_ata: &'me AccountInfo<'info>,
    pub yielding_bank_state: &'me AccountInfo<'info>,
    pub yielding_bank_mint: &'me AccountInfo<'info>,
    pub yielding_bank_tranche: &'me AccountInfo<'info>,
    pub usdc_vault: &'me AccountInfo<'info>,
    pub usdc_vault_oracle: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub usdc_vault_ata: &'me AccountInfo<'info>,
    pub destination_vault: &'me AccountInfo<'info>,
    pub destination_vault_oracle: &'me AccountInfo<'info>,
    pub destination_vault_asset_ata: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub yielding_token_program: &'me AccountInfo<'info>,
    pub idle_bank_mint_token_program: &'me AccountInfo<'info>,
    pub bank_mint_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateIdleBankMintVaultKeys {
    pub manager: Pubkey,
    pub idle_bank_state: Pubkey,
    pub idle_bank_mint: Pubkey,
    pub idle_bank_tranche: Pubkey,
    pub idle_vault: Pubkey,
    pub idle_vault_oracle: Pubkey,
    pub idle_vault_team: Pubkey,
    pub idle_vault_bank_mint_ata: Pubkey,
    pub yielding_bank_state: Pubkey,
    pub yielding_bank_mint: Pubkey,
    pub yielding_bank_tranche: Pubkey,
    pub usdc_vault: Pubkey,
    pub usdc_vault_oracle: Pubkey,
    pub usdc_mint: Pubkey,
    pub usdc_vault_ata: Pubkey,
    pub destination_vault: Pubkey,
    pub destination_vault_oracle: Pubkey,
    pub destination_vault_asset_ata: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub yielding_token_program: Pubkey,
    pub idle_bank_mint_token_program: Pubkey,
    pub bank_mint_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<MigrateIdleBankMintVaultAccounts<'_, '_>> for MigrateIdleBankMintVaultKeys {
    fn from(accounts: MigrateIdleBankMintVaultAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            idle_bank_state: *accounts.idle_bank_state.key,
            idle_bank_mint: *accounts.idle_bank_mint.key,
            idle_bank_tranche: *accounts.idle_bank_tranche.key,
            idle_vault: *accounts.idle_vault.key,
            idle_vault_oracle: *accounts.idle_vault_oracle.key,
            idle_vault_team: *accounts.idle_vault_team.key,
            idle_vault_bank_mint_ata: *accounts.idle_vault_bank_mint_ata.key,
            yielding_bank_state: *accounts.yielding_bank_state.key,
            yielding_bank_mint: *accounts.yielding_bank_mint.key,
            yielding_bank_tranche: *accounts.yielding_bank_tranche.key,
            usdc_vault: *accounts.usdc_vault.key,
            usdc_vault_oracle: *accounts.usdc_vault_oracle.key,
            usdc_mint: *accounts.usdc_mint.key,
            usdc_vault_ata: *accounts.usdc_vault_ata.key,
            destination_vault: *accounts.destination_vault.key,
            destination_vault_oracle: *accounts.destination_vault_oracle.key,
            destination_vault_asset_ata: *accounts.destination_vault_asset_ata.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            yielding_token_program: *accounts.yielding_token_program.key,
            idle_bank_mint_token_program: *accounts.idle_bank_mint_token_program.key,
            bank_mint_token_program: *accounts.bank_mint_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MigrateIdleBankMintVaultKeys>
for [AccountMeta; MIGRATE_IDLE_BANK_MINT_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateIdleBankMintVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.idle_bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.idle_bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.idle_bank_tranche,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.idle_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.idle_vault_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.idle_vault_team,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.idle_vault_bank_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_bank_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_bank_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_bank_tranche,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.yielding_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.idle_bank_mint_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_mint_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; MIGRATE_IDLE_BANK_MINT_VAULT_IX_ACCOUNTS_LEN]>
for MigrateIdleBankMintVaultKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_IDLE_BANK_MINT_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            idle_bank_state: pubkeys[1],
            idle_bank_mint: pubkeys[2],
            idle_bank_tranche: pubkeys[3],
            idle_vault: pubkeys[4],
            idle_vault_oracle: pubkeys[5],
            idle_vault_team: pubkeys[6],
            idle_vault_bank_mint_ata: pubkeys[7],
            yielding_bank_state: pubkeys[8],
            yielding_bank_mint: pubkeys[9],
            yielding_bank_tranche: pubkeys[10],
            usdc_vault: pubkeys[11],
            usdc_vault_oracle: pubkeys[12],
            usdc_mint: pubkeys[13],
            usdc_vault_ata: pubkeys[14],
            destination_vault: pubkeys[15],
            destination_vault_oracle: pubkeys[16],
            destination_vault_asset_ata: pubkeys[17],
            vault_tranche_state: pubkeys[18],
            yielding_token_program: pubkeys[19],
            idle_bank_mint_token_program: pubkeys[20],
            bank_mint_token_program: pubkeys[21],
            associated_token_program: pubkeys[22],
            system_program: pubkeys[23],
        }
    }
}
impl<'info> From<MigrateIdleBankMintVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_IDLE_BANK_MINT_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateIdleBankMintVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.idle_bank_state.clone(),
            accounts.idle_bank_mint.clone(),
            accounts.idle_bank_tranche.clone(),
            accounts.idle_vault.clone(),
            accounts.idle_vault_oracle.clone(),
            accounts.idle_vault_team.clone(),
            accounts.idle_vault_bank_mint_ata.clone(),
            accounts.yielding_bank_state.clone(),
            accounts.yielding_bank_mint.clone(),
            accounts.yielding_bank_tranche.clone(),
            accounts.usdc_vault.clone(),
            accounts.usdc_vault_oracle.clone(),
            accounts.usdc_mint.clone(),
            accounts.usdc_vault_ata.clone(),
            accounts.destination_vault.clone(),
            accounts.destination_vault_oracle.clone(),
            accounts.destination_vault_asset_ata.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.yielding_token_program.clone(),
            accounts.idle_bank_mint_token_program.clone(),
            accounts.bank_mint_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MIGRATE_IDLE_BANK_MINT_VAULT_IX_ACCOUNTS_LEN]>
for MigrateIdleBankMintVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MIGRATE_IDLE_BANK_MINT_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: &arr[0],
            idle_bank_state: &arr[1],
            idle_bank_mint: &arr[2],
            idle_bank_tranche: &arr[3],
            idle_vault: &arr[4],
            idle_vault_oracle: &arr[5],
            idle_vault_team: &arr[6],
            idle_vault_bank_mint_ata: &arr[7],
            yielding_bank_state: &arr[8],
            yielding_bank_mint: &arr[9],
            yielding_bank_tranche: &arr[10],
            usdc_vault: &arr[11],
            usdc_vault_oracle: &arr[12],
            usdc_mint: &arr[13],
            usdc_vault_ata: &arr[14],
            destination_vault: &arr[15],
            destination_vault_oracle: &arr[16],
            destination_vault_asset_ata: &arr[17],
            vault_tranche_state: &arr[18],
            yielding_token_program: &arr[19],
            idle_bank_mint_token_program: &arr[20],
            bank_mint_token_program: &arr[21],
            associated_token_program: &arr[22],
            system_program: &arr[23],
        }
    }
}
pub const MIGRATE_IDLE_BANK_MINT_VAULT_IX_DISCM: [u8; 8usize] = [
    163, 248, 178, 119, 233, 196, 220, 212,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateIdleBankMintVaultIxData;
impl MigrateIdleBankMintVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_IDLE_BANK_MINT_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_IDLE_BANK_MINT_VAULT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_idle_bank_mint_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateIdleBankMintVaultKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_IDLE_BANK_MINT_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateIdleBankMintVaultIxData.try_to_vec()?,
    })
}
pub fn migrate_idle_bank_mint_vault_ix(
    keys: MigrateIdleBankMintVaultKeys,
) -> std::io::Result<Instruction> {
    migrate_idle_bank_mint_vault_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn migrate_idle_bank_mint_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateIdleBankMintVaultAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateIdleBankMintVaultKeys = accounts.into();
    let ix = migrate_idle_bank_mint_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_idle_bank_mint_vault_invoke(
    accounts: MigrateIdleBankMintVaultAccounts<'_, '_>,
) -> ProgramResult {
    migrate_idle_bank_mint_vault_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts)
}
pub fn migrate_idle_bank_mint_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateIdleBankMintVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateIdleBankMintVaultKeys = accounts.into();
    let ix = migrate_idle_bank_mint_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_idle_bank_mint_vault_invoke_signed(
    accounts: MigrateIdleBankMintVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_idle_bank_mint_vault_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_idle_bank_mint_vault_verify_account_keys(
    accounts: MigrateIdleBankMintVaultAccounts<'_, '_>,
    keys: MigrateIdleBankMintVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.idle_bank_state.key, keys.idle_bank_state),
        (*accounts.idle_bank_mint.key, keys.idle_bank_mint),
        (*accounts.idle_bank_tranche.key, keys.idle_bank_tranche),
        (*accounts.idle_vault.key, keys.idle_vault),
        (*accounts.idle_vault_oracle.key, keys.idle_vault_oracle),
        (*accounts.idle_vault_team.key, keys.idle_vault_team),
        (*accounts.idle_vault_bank_mint_ata.key, keys.idle_vault_bank_mint_ata),
        (*accounts.yielding_bank_state.key, keys.yielding_bank_state),
        (*accounts.yielding_bank_mint.key, keys.yielding_bank_mint),
        (*accounts.yielding_bank_tranche.key, keys.yielding_bank_tranche),
        (*accounts.usdc_vault.key, keys.usdc_vault),
        (*accounts.usdc_vault_oracle.key, keys.usdc_vault_oracle),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.usdc_vault_ata.key, keys.usdc_vault_ata),
        (*accounts.destination_vault.key, keys.destination_vault),
        (*accounts.destination_vault_oracle.key, keys.destination_vault_oracle),
        (*accounts.destination_vault_asset_ata.key, keys.destination_vault_asset_ata),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.yielding_token_program.key, keys.yielding_token_program),
        (*accounts.idle_bank_mint_token_program.key, keys.idle_bank_mint_token_program),
        (*accounts.bank_mint_token_program.key, keys.bank_mint_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_idle_bank_mint_vault_verify_writable_privileges<'me, 'info>(
    accounts: MigrateIdleBankMintVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.idle_bank_state,
        accounts.idle_bank_mint,
        accounts.idle_vault,
        accounts.idle_vault_oracle,
        accounts.idle_vault_team,
        accounts.idle_vault_bank_mint_ata,
        accounts.yielding_bank_state,
        accounts.yielding_bank_mint,
        accounts.usdc_vault,
        accounts.usdc_vault_ata,
        accounts.destination_vault,
        accounts.destination_vault_oracle,
        accounts.destination_vault_asset_ata,
        accounts.vault_tranche_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_idle_bank_mint_vault_verify_signer_privileges<'me, 'info>(
    accounts: MigrateIdleBankMintVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_idle_bank_mint_vault_verify_account_privileges<'me, 'info>(
    accounts: MigrateIdleBankMintVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_idle_bank_mint_vault_verify_writable_privileges(accounts)?;
    migrate_idle_bank_mint_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeAccounts<'me, 'info> {
    pub protocol_authority: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetProtocolFeeKeys {
    pub protocol_authority: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
}
impl From<SetProtocolFeeAccounts<'_, '_>> for SetProtocolFeeKeys {
    fn from(accounts: SetProtocolFeeAccounts) -> Self {
        Self {
            protocol_authority: *accounts.protocol_authority.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
        }
    }
}
impl From<SetProtocolFeeKeys> for [AccountMeta; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.protocol_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for SetProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            protocol_authority: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
        }
    }
}
impl<'info> From<SetProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.protocol_authority.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for SetProtocolFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            protocol_authority: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
        }
    }
}
pub const SET_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    173, 239, 83, 242, 136, 43, 144, 217,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetProtocolFeeIxArgs {
    pub args: SetProtocolFeeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetProtocolFeeIxData(pub SetProtocolFeeIxArgs);
impl From<SetProtocolFeeIxArgs> for SetProtocolFeeIxData {
    fn from(args: SetProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl SetProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SetProtocolFeeArgs>::deserialize(&mut reader)?
        };
        Ok(Self(SetProtocolFeeIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PROTOCOL_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: SetProtocolFeeKeys,
    args: SetProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_protocol_fee_ix(
    keys: SetProtocolFeeKeys,
    args: SetProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    set_protocol_fee_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn set_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetProtocolFeeAccounts<'_, '_>,
    args: SetProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: SetProtocolFeeKeys = accounts.into();
    let ix = set_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_protocol_fee_invoke(
    accounts: SetProtocolFeeAccounts<'_, '_>,
    args: SetProtocolFeeIxArgs,
) -> ProgramResult {
    set_protocol_fee_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn set_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetProtocolFeeAccounts<'_, '_>,
    args: SetProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetProtocolFeeKeys = accounts.into();
    let ix = set_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_protocol_fee_invoke_signed(
    accounts: SetProtocolFeeAccounts<'_, '_>,
    args: SetProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_protocol_fee_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_protocol_fee_verify_account_keys(
    accounts: SetProtocolFeeAccounts<'_, '_>,
    keys: SetProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.protocol_authority.key, keys.protocol_authority),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: SetProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: SetProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.protocol_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: SetProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_protocol_fee_verify_writable_privileges(accounts)?;
    set_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteDepositFeeExemptAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub user_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteDepositFeeExemptKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub user_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
}
impl From<ExecuteDepositFeeExemptAccounts<'_, '_>> for ExecuteDepositFeeExemptKeys {
    fn from(accounts: ExecuteDepositFeeExemptAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            share_mint: *accounts.share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            user_share_ata: *accounts.user_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
        }
    }
}
impl From<ExecuteDepositFeeExemptKeys>
for [AccountMeta; EXECUTE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteDepositFeeExemptKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteDepositFeeExemptKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            fee_vault: pubkeys[8],
            fee_vault_ata: pubkeys[9],
            user_share_ata: pubkeys[10],
            asset_token_program: pubkeys[11],
            share_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<ExecuteDepositFeeExemptAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteDepositFeeExemptAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.user_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteDepositFeeExemptAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXECUTE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            fee_vault: &arr[8],
            fee_vault_ata: &arr[9],
            user_share_ata: &arr[10],
            asset_token_program: &arr[11],
            share_token_program: &arr[12],
        }
    }
}
pub const EXECUTE_DEPOSIT_FEE_EXEMPT_IX_DISCM: [u8; 8usize] = [
    99, 227, 32, 27, 95, 184, 49, 2,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteDepositFeeExemptIxArgs {
    pub amount: u64,
    pub signer_seeds: Vec<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteDepositFeeExemptIxData(pub ExecuteDepositFeeExemptIxArgs);
impl From<ExecuteDepositFeeExemptIxArgs> for ExecuteDepositFeeExemptIxData {
    fn from(args: ExecuteDepositFeeExemptIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteDepositFeeExemptIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_DEPOSIT_FEE_EXEMPT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteDepositFeeExemptIxArgs {
                amount,
                signer_seeds,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_DEPOSIT_FEE_EXEMPT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.signer_seeds, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_deposit_fee_exempt_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteDepositFeeExemptKeys,
    args: ExecuteDepositFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteDepositFeeExemptIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_deposit_fee_exempt_ix(
    keys: ExecuteDepositFeeExemptKeys,
    args: ExecuteDepositFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    execute_deposit_fee_exempt_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn execute_deposit_fee_exempt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteDepositFeeExemptAccounts<'_, '_>,
    args: ExecuteDepositFeeExemptIxArgs,
) -> ProgramResult {
    let keys: ExecuteDepositFeeExemptKeys = accounts.into();
    let ix = execute_deposit_fee_exempt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_deposit_fee_exempt_invoke(
    accounts: ExecuteDepositFeeExemptAccounts<'_, '_>,
    args: ExecuteDepositFeeExemptIxArgs,
) -> ProgramResult {
    execute_deposit_fee_exempt_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn execute_deposit_fee_exempt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteDepositFeeExemptAccounts<'_, '_>,
    args: ExecuteDepositFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteDepositFeeExemptKeys = accounts.into();
    let ix = execute_deposit_fee_exempt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_deposit_fee_exempt_invoke_signed(
    accounts: ExecuteDepositFeeExemptAccounts<'_, '_>,
    args: ExecuteDepositFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_deposit_fee_exempt_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_deposit_fee_exempt_verify_account_keys(
    accounts: ExecuteDepositFeeExemptAccounts<'_, '_>,
    keys: ExecuteDepositFeeExemptKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.user_share_ata.key, keys.user_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_deposit_fee_exempt_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteDepositFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.user_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_deposit_fee_exempt_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteDepositFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_deposit_fee_exempt_verify_account_privileges<'me, 'info>(
    accounts: ExecuteDepositFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_deposit_fee_exempt_verify_writable_privileges(accounts)?;
    execute_deposit_fee_exempt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_SHARE_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteShareSwapAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub tranche_share_mint: &'me AccountInfo<'info>,
    pub user_share_ata: &'me AccountInfo<'info>,
    pub user_tranche_share_ata: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub tranche_share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteShareSwapKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub share_mint: Pubkey,
    pub tranche_share_mint: Pubkey,
    pub user_share_ata: Pubkey,
    pub user_tranche_share_ata: Pubkey,
    pub share_token_program: Pubkey,
    pub tranche_share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ExecuteShareSwapAccounts<'_, '_>> for ExecuteShareSwapKeys {
    fn from(accounts: ExecuteShareSwapAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            share_mint: *accounts.share_mint.key,
            tranche_share_mint: *accounts.tranche_share_mint.key,
            user_share_ata: *accounts.user_share_ata.key,
            user_tranche_share_ata: *accounts.user_tranche_share_ata.key,
            share_token_program: *accounts.share_token_program.key,
            tranche_share_token_program: *accounts.tranche_share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ExecuteShareSwapKeys> for [AccountMeta; EXECUTE_SHARE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteShareSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tranche_share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_tranche_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tranche_share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; EXECUTE_SHARE_SWAP_IX_ACCOUNTS_LEN]> for ExecuteShareSwapKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_SHARE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            share_mint: pubkeys[4],
            tranche_share_mint: pubkeys[5],
            user_share_ata: pubkeys[6],
            user_tranche_share_ata: pubkeys[7],
            share_token_program: pubkeys[8],
            tranche_share_token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<ExecuteShareSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_SHARE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteShareSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.share_mint.clone(),
            accounts.tranche_share_mint.clone(),
            accounts.user_share_ata.clone(),
            accounts.user_tranche_share_ata.clone(),
            accounts.share_token_program.clone(),
            accounts.tranche_share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXECUTE_SHARE_SWAP_IX_ACCOUNTS_LEN]>
for ExecuteShareSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXECUTE_SHARE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            share_mint: &arr[4],
            tranche_share_mint: &arr[5],
            user_share_ata: &arr[6],
            user_tranche_share_ata: &arr[7],
            share_token_program: &arr[8],
            tranche_share_token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const EXECUTE_SHARE_SWAP_IX_DISCM: [u8; 8usize] = [
    127, 53, 127, 156, 151, 80, 135, 67,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteShareSwapIxArgs {
    pub kind: TrancheKind,
    pub share_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteShareSwapIxData(pub ExecuteShareSwapIxArgs);
impl From<ExecuteShareSwapIxArgs> for ExecuteShareSwapIxData {
    fn from(args: ExecuteShareSwapIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteShareSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_SHARE_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteShareSwapIxArgs {
                kind,
                share_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_SHARE_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_share_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteShareSwapKeys,
    args: ExecuteShareSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_SHARE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteShareSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_share_swap_ix(
    keys: ExecuteShareSwapKeys,
    args: ExecuteShareSwapIxArgs,
) -> std::io::Result<Instruction> {
    execute_share_swap_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn execute_share_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteShareSwapAccounts<'_, '_>,
    args: ExecuteShareSwapIxArgs,
) -> ProgramResult {
    let keys: ExecuteShareSwapKeys = accounts.into();
    let ix = execute_share_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_share_swap_invoke(
    accounts: ExecuteShareSwapAccounts<'_, '_>,
    args: ExecuteShareSwapIxArgs,
) -> ProgramResult {
    execute_share_swap_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn execute_share_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteShareSwapAccounts<'_, '_>,
    args: ExecuteShareSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteShareSwapKeys = accounts.into();
    let ix = execute_share_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_share_swap_invoke_signed(
    accounts: ExecuteShareSwapAccounts<'_, '_>,
    args: ExecuteShareSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_share_swap_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_share_swap_verify_account_keys(
    accounts: ExecuteShareSwapAccounts<'_, '_>,
    keys: ExecuteShareSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.tranche_share_mint.key, keys.tranche_share_mint),
        (*accounts.user_share_ata.key, keys.user_share_ata),
        (*accounts.user_tranche_share_ata.key, keys.user_tranche_share_ata),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.tranche_share_token_program.key, keys.tranche_share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_share_swap_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteShareSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.share_mint,
        accounts.tranche_share_mint,
        accounts.user_share_ata,
        accounts.user_tranche_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_share_swap_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteShareSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_share_swap_verify_account_privileges<'me, 'info>(
    accounts: ExecuteShareSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_share_swap_verify_writable_privileges(accounts)?;
    execute_share_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteTrancheDepositFeeExemptAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub tranche_share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub user_tranche_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteTrancheDepositFeeExemptKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub tranche_share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub user_tranche_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ExecuteTrancheDepositFeeExemptAccounts<'_, '_>>
for ExecuteTrancheDepositFeeExemptKeys {
    fn from(accounts: ExecuteTrancheDepositFeeExemptAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            tranche_share_mint: *accounts.tranche_share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            user_tranche_share_ata: *accounts.user_tranche_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ExecuteTrancheDepositFeeExemptKeys>
for [AccountMeta; EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteTrancheDepositFeeExemptKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tranche_share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_tranche_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteTrancheDepositFeeExemptKeys {
    fn from(
        pubkeys: [Pubkey; EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            tranche_share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            fee_vault: pubkeys[8],
            fee_vault_ata: pubkeys[9],
            user_tranche_share_ata: pubkeys[10],
            asset_token_program: pubkeys[11],
            share_token_program: pubkeys[12],
            associated_token_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<ExecuteTrancheDepositFeeExemptAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteTrancheDepositFeeExemptAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.tranche_share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.user_tranche_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteTrancheDepositFeeExemptAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            tranche_share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            fee_vault: &arr[8],
            fee_vault_ata: &arr[9],
            user_tranche_share_ata: &arr[10],
            asset_token_program: &arr[11],
            share_token_program: &arr[12],
            associated_token_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_DISCM: [u8; 8usize] = [
    202, 226, 54, 182, 141, 178, 91, 231,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteTrancheDepositFeeExemptIxArgs {
    pub kind: TrancheKind,
    pub amount: u64,
    pub signer_seeds: Vec<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteTrancheDepositFeeExemptIxData(
    pub ExecuteTrancheDepositFeeExemptIxArgs,
);
impl From<ExecuteTrancheDepositFeeExemptIxArgs>
for ExecuteTrancheDepositFeeExemptIxData {
    fn from(args: ExecuteTrancheDepositFeeExemptIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteTrancheDepositFeeExemptIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteTrancheDepositFeeExemptIxArgs {
                kind,
                amount,
                signer_seeds,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.signer_seeds, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_tranche_deposit_fee_exempt_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteTrancheDepositFeeExemptKeys,
    args: ExecuteTrancheDepositFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_TRANCHE_DEPOSIT_FEE_EXEMPT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ExecuteTrancheDepositFeeExemptIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_tranche_deposit_fee_exempt_ix(
    keys: ExecuteTrancheDepositFeeExemptKeys,
    args: ExecuteTrancheDepositFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    execute_tranche_deposit_fee_exempt_ix_with_program_id(
        BANKINECO_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn execute_tranche_deposit_fee_exempt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTrancheDepositFeeExemptAccounts<'_, '_>,
    args: ExecuteTrancheDepositFeeExemptIxArgs,
) -> ProgramResult {
    let keys: ExecuteTrancheDepositFeeExemptKeys = accounts.into();
    let ix = execute_tranche_deposit_fee_exempt_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_tranche_deposit_fee_exempt_invoke(
    accounts: ExecuteTrancheDepositFeeExemptAccounts<'_, '_>,
    args: ExecuteTrancheDepositFeeExemptIxArgs,
) -> ProgramResult {
    execute_tranche_deposit_fee_exempt_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn execute_tranche_deposit_fee_exempt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTrancheDepositFeeExemptAccounts<'_, '_>,
    args: ExecuteTrancheDepositFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteTrancheDepositFeeExemptKeys = accounts.into();
    let ix = execute_tranche_deposit_fee_exempt_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_tranche_deposit_fee_exempt_invoke_signed(
    accounts: ExecuteTrancheDepositFeeExemptAccounts<'_, '_>,
    args: ExecuteTrancheDepositFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_tranche_deposit_fee_exempt_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_tranche_deposit_fee_exempt_verify_account_keys(
    accounts: ExecuteTrancheDepositFeeExemptAccounts<'_, '_>,
    keys: ExecuteTrancheDepositFeeExemptKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.tranche_share_mint.key, keys.tranche_share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.user_tranche_share_ata.key, keys.user_tranche_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_tranche_deposit_fee_exempt_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteTrancheDepositFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.tranche_share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.user_tranche_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_tranche_deposit_fee_exempt_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteTrancheDepositFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_tranche_deposit_fee_exempt_verify_account_privileges<'me, 'info>(
    accounts: ExecuteTrancheDepositFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_tranche_deposit_fee_exempt_verify_writable_privileges(accounts)?;
    execute_tranche_deposit_fee_exempt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteTrancheWithdrawFeeExemptAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub tranche_share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub user_tranche_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteTrancheWithdrawFeeExemptKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub tranche_share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub user_tranche_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ExecuteTrancheWithdrawFeeExemptAccounts<'_, '_>>
for ExecuteTrancheWithdrawFeeExemptKeys {
    fn from(accounts: ExecuteTrancheWithdrawFeeExemptAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            tranche_share_mint: *accounts.tranche_share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            user_tranche_share_ata: *accounts.user_tranche_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ExecuteTrancheWithdrawFeeExemptKeys>
for [AccountMeta; EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteTrancheWithdrawFeeExemptKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tranche_share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_tranche_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteTrancheWithdrawFeeExemptKeys {
    fn from(
        pubkeys: [Pubkey; EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            tranche_share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            user_tranche_share_ata: pubkeys[8],
            asset_token_program: pubkeys[9],
            share_token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<ExecuteTrancheWithdrawFeeExemptAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.tranche_share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.user_tranche_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteTrancheWithdrawFeeExemptAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            tranche_share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            user_tranche_share_ata: &arr[8],
            asset_token_program: &arr[9],
            share_token_program: &arr[10],
            associated_token_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM: [u8; 8usize] = [
    130, 36, 243, 242, 125, 2, 207, 206,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteTrancheWithdrawFeeExemptIxArgs {
    pub kind: TrancheKind,
    pub share_amount: u64,
    pub signer_seeds: Vec<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteTrancheWithdrawFeeExemptIxData(
    pub ExecuteTrancheWithdrawFeeExemptIxArgs,
);
impl From<ExecuteTrancheWithdrawFeeExemptIxArgs>
for ExecuteTrancheWithdrawFeeExemptIxData {
    fn from(args: ExecuteTrancheWithdrawFeeExemptIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteTrancheWithdrawFeeExemptIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kind: TrancheKind = crate::borsh_de_or_default(&mut reader)?;
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteTrancheWithdrawFeeExemptIxArgs {
                kind,
                share_amount,
                signer_seeds,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.signer_seeds, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_tranche_withdraw_fee_exempt_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteTrancheWithdrawFeeExemptKeys,
    args: ExecuteTrancheWithdrawFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ExecuteTrancheWithdrawFeeExemptIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_tranche_withdraw_fee_exempt_ix(
    keys: ExecuteTrancheWithdrawFeeExemptKeys,
    args: ExecuteTrancheWithdrawFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    execute_tranche_withdraw_fee_exempt_ix_with_program_id(
        BANKINECO_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn execute_tranche_withdraw_fee_exempt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'_, '_>,
    args: ExecuteTrancheWithdrawFeeExemptIxArgs,
) -> ProgramResult {
    let keys: ExecuteTrancheWithdrawFeeExemptKeys = accounts.into();
    let ix = execute_tranche_withdraw_fee_exempt_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_tranche_withdraw_fee_exempt_invoke(
    accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'_, '_>,
    args: ExecuteTrancheWithdrawFeeExemptIxArgs,
) -> ProgramResult {
    execute_tranche_withdraw_fee_exempt_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn execute_tranche_withdraw_fee_exempt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'_, '_>,
    args: ExecuteTrancheWithdrawFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteTrancheWithdrawFeeExemptKeys = accounts.into();
    let ix = execute_tranche_withdraw_fee_exempt_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_tranche_withdraw_fee_exempt_invoke_signed(
    accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'_, '_>,
    args: ExecuteTrancheWithdrawFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_tranche_withdraw_fee_exempt_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_tranche_withdraw_fee_exempt_verify_account_keys(
    accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'_, '_>,
    keys: ExecuteTrancheWithdrawFeeExemptKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.tranche_share_mint.key, keys.tranche_share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.user_tranche_share_ata.key, keys.user_tranche_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_tranche_withdraw_fee_exempt_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.tranche_share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.user_tranche_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_tranche_withdraw_fee_exempt_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_tranche_withdraw_fee_exempt_verify_account_privileges<'me, 'info>(
    accounts: ExecuteTrancheWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_tranche_withdraw_fee_exempt_verify_writable_privileges(accounts)?;
    execute_tranche_withdraw_fee_exempt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteWithdrawFeeExemptAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub user_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteWithdrawFeeExemptKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub user_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
}
impl From<ExecuteWithdrawFeeExemptAccounts<'_, '_>> for ExecuteWithdrawFeeExemptKeys {
    fn from(accounts: ExecuteWithdrawFeeExemptAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            share_mint: *accounts.share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            user_share_ata: *accounts.user_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
        }
    }
}
impl From<ExecuteWithdrawFeeExemptKeys>
for [AccountMeta; EXECUTE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteWithdrawFeeExemptKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteWithdrawFeeExemptKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            fee_vault: pubkeys[8],
            fee_vault_ata: pubkeys[9],
            user_share_ata: pubkeys[10],
            asset_token_program: pubkeys[11],
            share_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<ExecuteWithdrawFeeExemptAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteWithdrawFeeExemptAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.user_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteWithdrawFeeExemptAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXECUTE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            fee_vault: &arr[8],
            fee_vault_ata: &arr[9],
            user_share_ata: &arr[10],
            asset_token_program: &arr[11],
            share_token_program: &arr[12],
        }
    }
}
pub const EXECUTE_WITHDRAW_FEE_EXEMPT_IX_DISCM: [u8; 8usize] = [
    47, 77, 139, 253, 122, 181, 108, 216,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteWithdrawFeeExemptIxArgs {
    pub share_amount: u64,
    pub signer_seeds: Vec<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteWithdrawFeeExemptIxData(pub ExecuteWithdrawFeeExemptIxArgs);
impl From<ExecuteWithdrawFeeExemptIxArgs> for ExecuteWithdrawFeeExemptIxData {
    fn from(args: ExecuteWithdrawFeeExemptIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteWithdrawFeeExemptIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_WITHDRAW_FEE_EXEMPT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteWithdrawFeeExemptIxArgs {
                share_amount,
                signer_seeds,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_WITHDRAW_FEE_EXEMPT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.signer_seeds, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_withdraw_fee_exempt_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteWithdrawFeeExemptKeys,
    args: ExecuteWithdrawFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteWithdrawFeeExemptIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_withdraw_fee_exempt_ix(
    keys: ExecuteWithdrawFeeExemptKeys,
    args: ExecuteWithdrawFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    execute_withdraw_fee_exempt_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn execute_withdraw_fee_exempt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteWithdrawFeeExemptAccounts<'_, '_>,
    args: ExecuteWithdrawFeeExemptIxArgs,
) -> ProgramResult {
    let keys: ExecuteWithdrawFeeExemptKeys = accounts.into();
    let ix = execute_withdraw_fee_exempt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_withdraw_fee_exempt_invoke(
    accounts: ExecuteWithdrawFeeExemptAccounts<'_, '_>,
    args: ExecuteWithdrawFeeExemptIxArgs,
) -> ProgramResult {
    execute_withdraw_fee_exempt_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn execute_withdraw_fee_exempt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteWithdrawFeeExemptAccounts<'_, '_>,
    args: ExecuteWithdrawFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteWithdrawFeeExemptKeys = accounts.into();
    let ix = execute_withdraw_fee_exempt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_withdraw_fee_exempt_invoke_signed(
    accounts: ExecuteWithdrawFeeExemptAccounts<'_, '_>,
    args: ExecuteWithdrawFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_withdraw_fee_exempt_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_withdraw_fee_exempt_verify_account_keys(
    accounts: ExecuteWithdrawFeeExemptAccounts<'_, '_>,
    keys: ExecuteWithdrawFeeExemptKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.user_share_ata.key, keys.user_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_withdraw_fee_exempt_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.user_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_withdraw_fee_exempt_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_withdraw_fee_exempt_verify_account_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_withdraw_fee_exempt_verify_writable_privileges(accounts)?;
    execute_withdraw_fee_exempt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteWithdrawFromExternalAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub user_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteWithdrawFromExternalKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub user_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
}
impl From<ExecuteWithdrawFromExternalAccounts<'_, '_>>
for ExecuteWithdrawFromExternalKeys {
    fn from(accounts: ExecuteWithdrawFromExternalAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            share_mint: *accounts.share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            user_share_ata: *accounts.user_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
        }
    }
}
impl From<ExecuteWithdrawFromExternalKeys>
for [AccountMeta; EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteWithdrawFromExternalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_ACCOUNTS_LEN]>
for ExecuteWithdrawFromExternalKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            fee_vault: pubkeys[8],
            fee_vault_ata: pubkeys[9],
            user_share_ata: pubkeys[10],
            asset_token_program: pubkeys[11],
            share_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<ExecuteWithdrawFromExternalAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteWithdrawFromExternalAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.user_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_ACCOUNTS_LEN]>
for ExecuteWithdrawFromExternalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            fee_vault: &arr[8],
            fee_vault_ata: &arr[9],
            user_share_ata: &arr[10],
            asset_token_program: &arr[11],
            share_token_program: &arr[12],
        }
    }
}
pub const EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_DISCM: [u8; 8usize] = [
    91, 38, 26, 250, 138, 227, 18, 88,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteWithdrawFromExternalIxArgs {
    pub share_amount: u64,
    pub external_withdraw_ix_refs: Option<InstructionRefs>,
    pub external_liquidity_source: Option<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteWithdrawFromExternalIxData(pub ExecuteWithdrawFromExternalIxArgs);
impl From<ExecuteWithdrawFromExternalIxArgs> for ExecuteWithdrawFromExternalIxData {
    fn from(args: ExecuteWithdrawFromExternalIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteWithdrawFromExternalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let external_withdraw_ix_refs: Option<InstructionRefs> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let external_liquidity_source: Option<u8> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(ExecuteWithdrawFromExternalIxArgs {
                share_amount,
                external_withdraw_ix_refs,
                external_liquidity_source,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.external_withdraw_ix_refs,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.external_liquidity_source,
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
pub fn execute_withdraw_from_external_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteWithdrawFromExternalKeys,
    args: ExecuteWithdrawFromExternalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_WITHDRAW_FROM_EXTERNAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ExecuteWithdrawFromExternalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_withdraw_from_external_ix(
    keys: ExecuteWithdrawFromExternalKeys,
    args: ExecuteWithdrawFromExternalIxArgs,
) -> std::io::Result<Instruction> {
    execute_withdraw_from_external_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn execute_withdraw_from_external_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteWithdrawFromExternalAccounts<'_, '_>,
    args: ExecuteWithdrawFromExternalIxArgs,
) -> ProgramResult {
    let keys: ExecuteWithdrawFromExternalKeys = accounts.into();
    let ix = execute_withdraw_from_external_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_withdraw_from_external_invoke(
    accounts: ExecuteWithdrawFromExternalAccounts<'_, '_>,
    args: ExecuteWithdrawFromExternalIxArgs,
) -> ProgramResult {
    execute_withdraw_from_external_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn execute_withdraw_from_external_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteWithdrawFromExternalAccounts<'_, '_>,
    args: ExecuteWithdrawFromExternalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteWithdrawFromExternalKeys = accounts.into();
    let ix = execute_withdraw_from_external_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_withdraw_from_external_invoke_signed(
    accounts: ExecuteWithdrawFromExternalAccounts<'_, '_>,
    args: ExecuteWithdrawFromExternalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_withdraw_from_external_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_withdraw_from_external_verify_account_keys(
    accounts: ExecuteWithdrawFromExternalAccounts<'_, '_>,
    keys: ExecuteWithdrawFromExternalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.user_share_ata.key, keys.user_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_withdraw_from_external_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFromExternalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.user_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_withdraw_from_external_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFromExternalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_withdraw_from_external_verify_account_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFromExternalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_withdraw_from_external_verify_writable_privileges(accounts)?;
    execute_withdraw_from_external_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteWithdrawFromExternalFeeExemptAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub share_mint: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_ata: &'me AccountInfo<'info>,
    pub user_share_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteWithdrawFromExternalFeeExemptKeys {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub asset_mint: Pubkey,
    pub share_mint: Pubkey,
    pub user_asset_ata: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_ata: Pubkey,
    pub user_share_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
}
impl From<ExecuteWithdrawFromExternalFeeExemptAccounts<'_, '_>>
for ExecuteWithdrawFromExternalFeeExemptKeys {
    fn from(accounts: ExecuteWithdrawFromExternalFeeExemptAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            asset_mint: *accounts.asset_mint.key,
            share_mint: *accounts.share_mint.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_ata: *accounts.fee_vault_ata.key,
            user_share_ata: *accounts.user_share_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
        }
    }
}
impl From<ExecuteWithdrawFromExternalFeeExemptKeys>
for [AccountMeta; EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteWithdrawFromExternalFeeExemptKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_share_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for ExecuteWithdrawFromExternalFeeExemptKeys {
    fn from(
        pubkeys: [Pubkey; EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: pubkeys[0],
            vault: pubkeys[1],
            vault_oracle: pubkeys[2],
            vault_tranche_state: pubkeys[3],
            asset_mint: pubkeys[4],
            share_mint: pubkeys[5],
            user_asset_ata: pubkeys[6],
            vault_asset_ata: pubkeys[7],
            fee_vault: pubkeys[8],
            fee_vault_ata: pubkeys[9],
            user_share_ata: pubkeys[10],
            asset_token_program: pubkeys[11],
            share_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<ExecuteWithdrawFromExternalFeeExemptAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.asset_mint.clone(),
            accounts.share_mint.clone(),
            accounts.user_asset_ata.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_ata.clone(),
            accounts.user_share_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_ACCOUNTS_LEN],
> for ExecuteWithdrawFromExternalFeeExemptAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            vault: &arr[1],
            vault_oracle: &arr[2],
            vault_tranche_state: &arr[3],
            asset_mint: &arr[4],
            share_mint: &arr[5],
            user_asset_ata: &arr[6],
            vault_asset_ata: &arr[7],
            fee_vault: &arr[8],
            fee_vault_ata: &arr[9],
            user_share_ata: &arr[10],
            asset_token_program: &arr[11],
            share_token_program: &arr[12],
        }
    }
}
pub const EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_DISCM: [u8; 8usize] = [
    8, 18, 138, 251, 162, 172, 171, 185,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteWithdrawFromExternalFeeExemptIxArgs {
    pub share_amount: u64,
    pub external_withdraw_ix_refs: Option<InstructionRefs>,
    pub external_liquidity_source: Option<u8>,
    pub signer_seeds: Vec<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteWithdrawFromExternalFeeExemptIxData(
    pub ExecuteWithdrawFromExternalFeeExemptIxArgs,
);
impl From<ExecuteWithdrawFromExternalFeeExemptIxArgs>
for ExecuteWithdrawFromExternalFeeExemptIxData {
    fn from(args: ExecuteWithdrawFromExternalFeeExemptIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteWithdrawFromExternalFeeExemptIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let external_withdraw_ix_refs: Option<InstructionRefs> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let external_liquidity_source: Option<u8> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ExecuteWithdrawFromExternalFeeExemptIxArgs {
                share_amount,
                external_withdraw_ix_refs,
                external_liquidity_source,
                signer_seeds,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.external_withdraw_ix_refs,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.external_liquidity_source,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.signer_seeds, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_withdraw_from_external_fee_exempt_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteWithdrawFromExternalFeeExemptKeys,
    args: ExecuteWithdrawFromExternalFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_WITHDRAW_FROM_EXTERNAL_FEE_EXEMPT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ExecuteWithdrawFromExternalFeeExemptIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_withdraw_from_external_fee_exempt_ix(
    keys: ExecuteWithdrawFromExternalFeeExemptKeys,
    args: ExecuteWithdrawFromExternalFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    execute_withdraw_from_external_fee_exempt_ix_with_program_id(
        BANKINECO_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn execute_withdraw_from_external_fee_exempt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'_, '_>,
    args: ExecuteWithdrawFromExternalFeeExemptIxArgs,
) -> ProgramResult {
    let keys: ExecuteWithdrawFromExternalFeeExemptKeys = accounts.into();
    let ix = execute_withdraw_from_external_fee_exempt_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_withdraw_from_external_fee_exempt_invoke(
    accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'_, '_>,
    args: ExecuteWithdrawFromExternalFeeExemptIxArgs,
) -> ProgramResult {
    execute_withdraw_from_external_fee_exempt_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn execute_withdraw_from_external_fee_exempt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'_, '_>,
    args: ExecuteWithdrawFromExternalFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteWithdrawFromExternalFeeExemptKeys = accounts.into();
    let ix = execute_withdraw_from_external_fee_exempt_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_withdraw_from_external_fee_exempt_invoke_signed(
    accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'_, '_>,
    args: ExecuteWithdrawFromExternalFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_withdraw_from_external_fee_exempt_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_withdraw_from_external_fee_exempt_verify_account_keys(
    accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'_, '_>,
    keys: ExecuteWithdrawFromExternalFeeExemptKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.share_mint.key, keys.share_mint),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_ata.key, keys.fee_vault_ata),
        (*accounts.user_share_ata.key, keys.user_share_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_withdraw_from_external_fee_exempt_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.share_mint,
        accounts.user_asset_ata,
        accounts.vault_asset_ata,
        accounts.fee_vault,
        accounts.fee_vault_ata,
        accounts.user_share_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_withdraw_from_external_fee_exempt_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_withdraw_from_external_fee_exempt_verify_account_privileges<'me, 'info>(
    accounts: ExecuteWithdrawFromExternalFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_withdraw_from_external_fee_exempt_verify_writable_privileges(accounts)?;
    execute_withdraw_from_external_fee_exempt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct FulfillJuniorTrancheWithdrawFeeExemptAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub queue_payer: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_oracle: &'me AccountInfo<'info>,
    pub vault_tranche_state: &'me AccountInfo<'info>,
    pub withdrawal_queue: &'me AccountInfo<'info>,
    pub output_mint: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub owner_input_ata: &'me AccountInfo<'info>,
    pub owner_output_ata: &'me AccountInfo<'info>,
    pub vault_output_ata: &'me AccountInfo<'info>,
    pub fee_collector: &'me AccountInfo<'info>,
    pub fee_collector_output_ata: &'me AccountInfo<'info>,
    pub queue_input_ata: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub share_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FulfillJuniorTrancheWithdrawFeeExemptKeys {
    pub signer: Pubkey,
    pub owner: Pubkey,
    pub queue_payer: Pubkey,
    pub vault: Pubkey,
    pub vault_oracle: Pubkey,
    pub vault_tranche_state: Pubkey,
    pub withdrawal_queue: Pubkey,
    pub output_mint: Pubkey,
    pub input_mint: Pubkey,
    pub owner_input_ata: Pubkey,
    pub owner_output_ata: Pubkey,
    pub vault_output_ata: Pubkey,
    pub fee_collector: Pubkey,
    pub fee_collector_output_ata: Pubkey,
    pub queue_input_ata: Pubkey,
    pub asset_token_program: Pubkey,
    pub share_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<FulfillJuniorTrancheWithdrawFeeExemptAccounts<'_, '_>>
for FulfillJuniorTrancheWithdrawFeeExemptKeys {
    fn from(accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            owner: *accounts.owner.key,
            queue_payer: *accounts.queue_payer.key,
            vault: *accounts.vault.key,
            vault_oracle: *accounts.vault_oracle.key,
            vault_tranche_state: *accounts.vault_tranche_state.key,
            withdrawal_queue: *accounts.withdrawal_queue.key,
            output_mint: *accounts.output_mint.key,
            input_mint: *accounts.input_mint.key,
            owner_input_ata: *accounts.owner_input_ata.key,
            owner_output_ata: *accounts.owner_output_ata.key,
            vault_output_ata: *accounts.vault_output_ata.key,
            fee_collector: *accounts.fee_collector.key,
            fee_collector_output_ata: *accounts.fee_collector_output_ata.key,
            queue_input_ata: *accounts.queue_input_ata.key,
            asset_token_program: *accounts.asset_token_program.key,
            share_token_program: *accounts.share_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<FulfillJuniorTrancheWithdrawFeeExemptKeys>
for [AccountMeta; FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(keys: FulfillJuniorTrancheWithdrawFeeExemptKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.queue_payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_tranche_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdrawal_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_collector,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_collector_output_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.queue_input_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.share_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
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
impl From<[Pubkey; FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN]>
for FulfillJuniorTrancheWithdrawFeeExemptKeys {
    fn from(
        pubkeys: [Pubkey; FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: pubkeys[0],
            owner: pubkeys[1],
            queue_payer: pubkeys[2],
            vault: pubkeys[3],
            vault_oracle: pubkeys[4],
            vault_tranche_state: pubkeys[5],
            withdrawal_queue: pubkeys[6],
            output_mint: pubkeys[7],
            input_mint: pubkeys[8],
            owner_input_ata: pubkeys[9],
            owner_output_ata: pubkeys[10],
            vault_output_ata: pubkeys[11],
            fee_collector: pubkeys[12],
            fee_collector_output_ata: pubkeys[13],
            queue_input_ata: pubkeys[14],
            asset_token_program: pubkeys[15],
            share_token_program: pubkeys[16],
            associated_token_program: pubkeys[17],
            system_program: pubkeys[18],
        }
    }
}
impl<'info> From<FulfillJuniorTrancheWithdrawFeeExemptAccounts<'_, 'info>>
for [AccountInfo<'info>; FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] {
    fn from(accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.owner.clone(),
            accounts.queue_payer.clone(),
            accounts.vault.clone(),
            accounts.vault_oracle.clone(),
            accounts.vault_tranche_state.clone(),
            accounts.withdrawal_queue.clone(),
            accounts.output_mint.clone(),
            accounts.input_mint.clone(),
            accounts.owner_input_ata.clone(),
            accounts.owner_output_ata.clone(),
            accounts.vault_output_ata.clone(),
            accounts.fee_collector.clone(),
            accounts.fee_collector_output_ata.clone(),
            accounts.queue_input_ata.clone(),
            accounts.asset_token_program.clone(),
            accounts.share_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN],
> for FulfillJuniorTrancheWithdrawFeeExemptAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            owner: &arr[1],
            queue_payer: &arr[2],
            vault: &arr[3],
            vault_oracle: &arr[4],
            vault_tranche_state: &arr[5],
            withdrawal_queue: &arr[6],
            output_mint: &arr[7],
            input_mint: &arr[8],
            owner_input_ata: &arr[9],
            owner_output_ata: &arr[10],
            vault_output_ata: &arr[11],
            fee_collector: &arr[12],
            fee_collector_output_ata: &arr[13],
            queue_input_ata: &arr[14],
            asset_token_program: &arr[15],
            share_token_program: &arr[16],
            associated_token_program: &arr[17],
            system_program: &arr[18],
        }
    }
}
pub const FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM: [u8; 8usize] = [
    148, 163, 159, 43, 165, 131, 51, 85,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FulfillJuniorTrancheWithdrawFeeExemptIxArgs {
    pub signer_seeds: Vec<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FulfillJuniorTrancheWithdrawFeeExemptIxData(
    pub FulfillJuniorTrancheWithdrawFeeExemptIxArgs,
);
impl From<FulfillJuniorTrancheWithdrawFeeExemptIxArgs>
for FulfillJuniorTrancheWithdrawFeeExemptIxData {
    fn from(args: FulfillJuniorTrancheWithdrawFeeExemptIxArgs) -> Self {
        Self(args)
    }
}
impl FulfillJuniorTrancheWithdrawFeeExemptIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let signer_seeds: Vec<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FulfillJuniorTrancheWithdrawFeeExemptIxArgs {
                signer_seeds,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.signer_seeds, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_ix_with_program_id(
    program_id: Pubkey,
    keys: FulfillJuniorTrancheWithdrawFeeExemptKeys,
    args: FulfillJuniorTrancheWithdrawFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FULFILL_JUNIOR_TRANCHE_WITHDRAW_FEE_EXEMPT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: FulfillJuniorTrancheWithdrawFeeExemptIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_ix(
    keys: FulfillJuniorTrancheWithdrawFeeExemptKeys,
    args: FulfillJuniorTrancheWithdrawFeeExemptIxArgs,
) -> std::io::Result<Instruction> {
    fulfill_junior_tranche_withdraw_fee_exempt_ix_with_program_id(
        BANKINECO_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'_, '_>,
    args: FulfillJuniorTrancheWithdrawFeeExemptIxArgs,
) -> ProgramResult {
    let keys: FulfillJuniorTrancheWithdrawFeeExemptKeys = accounts.into();
    let ix = fulfill_junior_tranche_withdraw_fee_exempt_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_invoke(
    accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'_, '_>,
    args: FulfillJuniorTrancheWithdrawFeeExemptIxArgs,
) -> ProgramResult {
    fulfill_junior_tranche_withdraw_fee_exempt_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'_, '_>,
    args: FulfillJuniorTrancheWithdrawFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FulfillJuniorTrancheWithdrawFeeExemptKeys = accounts.into();
    let ix = fulfill_junior_tranche_withdraw_fee_exempt_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_invoke_signed(
    accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'_, '_>,
    args: FulfillJuniorTrancheWithdrawFeeExemptIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fulfill_junior_tranche_withdraw_fee_exempt_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_verify_account_keys(
    accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'_, '_>,
    keys: FulfillJuniorTrancheWithdrawFeeExemptKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.owner.key, keys.owner),
        (*accounts.queue_payer.key, keys.queue_payer),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_oracle.key, keys.vault_oracle),
        (*accounts.vault_tranche_state.key, keys.vault_tranche_state),
        (*accounts.withdrawal_queue.key, keys.withdrawal_queue),
        (*accounts.output_mint.key, keys.output_mint),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.owner_input_ata.key, keys.owner_input_ata),
        (*accounts.owner_output_ata.key, keys.owner_output_ata),
        (*accounts.vault_output_ata.key, keys.vault_output_ata),
        (*accounts.fee_collector.key, keys.fee_collector),
        (*accounts.fee_collector_output_ata.key, keys.fee_collector_output_ata),
        (*accounts.queue_input_ata.key, keys.queue_input_ata),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.share_token_program.key, keys.share_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_verify_writable_privileges<'me, 'info>(
    accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.queue_payer,
        accounts.vault,
        accounts.vault_tranche_state,
        accounts.withdrawal_queue,
        accounts.input_mint,
        accounts.owner_input_ata,
        accounts.owner_output_ata,
        accounts.vault_output_ata,
        accounts.fee_collector_output_ata,
        accounts.queue_input_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_verify_signer_privileges<'me, 'info>(
    accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fulfill_junior_tranche_withdraw_fee_exempt_verify_account_privileges<'me, 'info>(
    accounts: FulfillJuniorTrancheWithdrawFeeExemptAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fulfill_junior_tranche_withdraw_fee_exempt_verify_writable_privileges(accounts)?;
    fulfill_junior_tranche_withdraw_fee_exempt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VAULT_ROLES_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultRolesAccounts<'me, 'info> {
    pub curator: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeVaultRolesKeys {
    pub curator: Pubkey,
    pub vault: Pubkey,
}
impl From<InitializeVaultRolesAccounts<'_, '_>> for InitializeVaultRolesKeys {
    fn from(accounts: InitializeVaultRolesAccounts) -> Self {
        Self {
            curator: *accounts.curator.key,
            vault: *accounts.vault.key,
        }
    }
}
impl From<InitializeVaultRolesKeys>
for [AccountMeta; INITIALIZE_VAULT_ROLES_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultRolesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curator,
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
impl From<[Pubkey; INITIALIZE_VAULT_ROLES_IX_ACCOUNTS_LEN]>
for InitializeVaultRolesKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_VAULT_ROLES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curator: pubkeys[0],
            vault: pubkeys[1],
        }
    }
}
impl<'info> From<InitializeVaultRolesAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_ROLES_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultRolesAccounts<'_, 'info>) -> Self {
        [accounts.curator.clone(), accounts.vault.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_ROLES_IX_ACCOUNTS_LEN]>
for InitializeVaultRolesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_VAULT_ROLES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curator: &arr[0],
            vault: &arr[1],
        }
    }
}
pub const INITIALIZE_VAULT_ROLES_IX_DISCM: [u8; 8usize] = [
    227, 251, 100, 86, 154, 59, 199, 29,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeVaultRolesIxArgs {
    pub args: InitializeVaultRolesArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultRolesIxData(pub InitializeVaultRolesIxArgs);
impl From<InitializeVaultRolesIxArgs> for InitializeVaultRolesIxData {
    fn from(args: InitializeVaultRolesIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeVaultRolesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VAULT_ROLES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeVaultRolesArgs>::deserialize(&mut reader)?
        };
        Ok(Self(InitializeVaultRolesIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VAULT_ROLES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_roles_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultRolesKeys,
    args: InitializeVaultRolesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_ROLES_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeVaultRolesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_vault_roles_ix(
    keys: InitializeVaultRolesKeys,
    args: InitializeVaultRolesIxArgs,
) -> std::io::Result<Instruction> {
    initialize_vault_roles_ix_with_program_id(BANKINECO_PROGRAM_ID, keys, args)
}
pub fn initialize_vault_roles_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultRolesAccounts<'_, '_>,
    args: InitializeVaultRolesIxArgs,
) -> ProgramResult {
    let keys: InitializeVaultRolesKeys = accounts.into();
    let ix = initialize_vault_roles_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_roles_invoke(
    accounts: InitializeVaultRolesAccounts<'_, '_>,
    args: InitializeVaultRolesIxArgs,
) -> ProgramResult {
    initialize_vault_roles_invoke_with_program_id(BANKINECO_PROGRAM_ID, accounts, args)
}
pub fn initialize_vault_roles_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultRolesAccounts<'_, '_>,
    args: InitializeVaultRolesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultRolesKeys = accounts.into();
    let ix = initialize_vault_roles_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_roles_invoke_signed(
    accounts: InitializeVaultRolesAccounts<'_, '_>,
    args: InitializeVaultRolesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_roles_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_vault_roles_verify_account_keys(
    accounts: InitializeVaultRolesAccounts<'_, '_>,
    keys: InitializeVaultRolesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curator.key, keys.curator),
        (*accounts.vault.key, keys.vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_roles_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultRolesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_roles_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultRolesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_roles_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultRolesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_roles_verify_writable_privileges(accounts)?;
    initialize_vault_roles_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct SweepLegacyJuniorEscrowExcessAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub bank_state: &'me AccountInfo<'info>,
    pub tranche_state: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub junior_escrow_ata: &'me AccountInfo<'info>,
    pub destination_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SweepLegacyJuniorEscrowExcessKeys {
    pub manager: Pubkey,
    pub bank_state: Pubkey,
    pub tranche_state: Pubkey,
    pub bank_mint: Pubkey,
    pub junior_escrow_ata: Pubkey,
    pub destination_ata: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<SweepLegacyJuniorEscrowExcessAccounts<'_, '_>>
for SweepLegacyJuniorEscrowExcessKeys {
    fn from(accounts: SweepLegacyJuniorEscrowExcessAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            bank_state: *accounts.bank_state.key,
            tranche_state: *accounts.tranche_state.key,
            bank_mint: *accounts.bank_mint.key,
            junior_escrow_ata: *accounts.junior_escrow_ata.key,
            destination_ata: *accounts.destination_ata.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SweepLegacyJuniorEscrowExcessKeys>
for [AccountMeta; SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_ACCOUNTS_LEN] {
    fn from(keys: SweepLegacyJuniorEscrowExcessKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
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
                pubkey: keys.destination_ata,
                is_signer: false,
                is_writable: true,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_ACCOUNTS_LEN]>
for SweepLegacyJuniorEscrowExcessKeys {
    fn from(
        pubkeys: [Pubkey; SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: pubkeys[0],
            bank_state: pubkeys[1],
            tranche_state: pubkeys[2],
            bank_mint: pubkeys[3],
            junior_escrow_ata: pubkeys[4],
            destination_ata: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<SweepLegacyJuniorEscrowExcessAccounts<'_, 'info>>
for [AccountInfo<'info>; SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SweepLegacyJuniorEscrowExcessAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.bank_state.clone(),
            accounts.tranche_state.clone(),
            accounts.bank_mint.clone(),
            accounts.junior_escrow_ata.clone(),
            accounts.destination_ata.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_ACCOUNTS_LEN]>
for SweepLegacyJuniorEscrowExcessAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: &arr[0],
            bank_state: &arr[1],
            tranche_state: &arr[2],
            bank_mint: &arr[3],
            junior_escrow_ata: &arr[4],
            destination_ata: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_DISCM: [u8; 8usize] = [
    66, 219, 143, 146, 126, 14, 138, 156,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SweepLegacyJuniorEscrowExcessIxData;
impl SweepLegacyJuniorEscrowExcessIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sweep_legacy_junior_escrow_excess_ix_with_program_id(
    program_id: Pubkey,
    keys: SweepLegacyJuniorEscrowExcessKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWEEP_LEGACY_JUNIOR_ESCROW_EXCESS_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SweepLegacyJuniorEscrowExcessIxData.try_to_vec()?,
    })
}
pub fn sweep_legacy_junior_escrow_excess_ix(
    keys: SweepLegacyJuniorEscrowExcessKeys,
) -> std::io::Result<Instruction> {
    sweep_legacy_junior_escrow_excess_ix_with_program_id(BANKINECO_PROGRAM_ID, keys)
}
pub fn sweep_legacy_junior_escrow_excess_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SweepLegacyJuniorEscrowExcessAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SweepLegacyJuniorEscrowExcessKeys = accounts.into();
    let ix = sweep_legacy_junior_escrow_excess_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sweep_legacy_junior_escrow_excess_invoke(
    accounts: SweepLegacyJuniorEscrowExcessAccounts<'_, '_>,
) -> ProgramResult {
    sweep_legacy_junior_escrow_excess_invoke_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
    )
}
pub fn sweep_legacy_junior_escrow_excess_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SweepLegacyJuniorEscrowExcessAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SweepLegacyJuniorEscrowExcessKeys = accounts.into();
    let ix = sweep_legacy_junior_escrow_excess_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sweep_legacy_junior_escrow_excess_invoke_signed(
    accounts: SweepLegacyJuniorEscrowExcessAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sweep_legacy_junior_escrow_excess_invoke_signed_with_program_id(
        BANKINECO_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn sweep_legacy_junior_escrow_excess_verify_account_keys(
    accounts: SweepLegacyJuniorEscrowExcessAccounts<'_, '_>,
    keys: SweepLegacyJuniorEscrowExcessKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.bank_state.key, keys.bank_state),
        (*accounts.tranche_state.key, keys.tranche_state),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.junior_escrow_ata.key, keys.junior_escrow_ata),
        (*accounts.destination_ata.key, keys.destination_ata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sweep_legacy_junior_escrow_excess_verify_writable_privileges<'me, 'info>(
    accounts: SweepLegacyJuniorEscrowExcessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.junior_escrow_ata,
        accounts.destination_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sweep_legacy_junior_escrow_excess_verify_signer_privileges<'me, 'info>(
    accounts: SweepLegacyJuniorEscrowExcessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sweep_legacy_junior_escrow_excess_verify_account_privileges<'me, 'info>(
    accounts: SweepLegacyJuniorEscrowExcessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sweep_legacy_junior_escrow_excess_verify_writable_privileges(accounts)?;
    sweep_legacy_junior_escrow_excess_verify_signer_privileges(accounts)?;
    Ok(())
}
