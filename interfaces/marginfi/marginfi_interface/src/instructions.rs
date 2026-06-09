use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum MarginfiProgramIx {
    ConfigGroupFee(ConfigGroupFeeIxArgs),
    ConfigureBankRateLimits(ConfigureBankRateLimitsIxArgs),
    ConfigureDeleverageWithdrawalLimit(ConfigureDeleverageWithdrawalLimitIxArgs),
    ConfigureGroupRateLimits(ConfigureGroupRateLimitsIxArgs),
    DriftDeposit(DriftDepositIxArgs),
    DriftHarvestReward,
    DriftInitUser(DriftInitUserIxArgs),
    DriftWithdraw(DriftWithdrawIxArgs),
    EditGlobalFeeState(EditGlobalFeeStateIxArgs),
    EditStakedSettings(EditStakedSettingsIxArgs),
    EndDeleverage,
    EndLiquidation,
    InitBankMetadata,
    InitGlobalFeeState(InitGlobalFeeStateIxArgs),
    InitStakedSettings(InitStakedSettingsIxArgs),
    JuplendDeposit(JuplendDepositIxArgs),
    JuplendInitPosition(JuplendInitPositionIxArgs),
    JuplendWithdraw(JuplendWithdrawIxArgs),
    KaminoDeposit(KaminoDepositIxArgs),
    KaminoHarvestReward(KaminoHarvestRewardIxArgs),
    KaminoInitObligation(KaminoInitObligationIxArgs),
    KaminoWithdraw(KaminoWithdrawIxArgs),
    LendingAccountBorrow(LendingAccountBorrowIxArgs),
    LendingAccountClearEmissions,
    LendingAccountCloseBalance,
    LendingAccountDeposit(LendingAccountDepositIxArgs),
    LendingAccountEndFlashloan,
    LendingAccountLiquidate(LendingAccountLiquidateIxArgs),
    LendingAccountPulseHealth,
    LendingAccountRepay(LendingAccountRepayIxArgs),
    LendingAccountStartFlashloan(LendingAccountStartFlashloanIxArgs),
    LendingAccountWithdraw(LendingAccountWithdrawIxArgs),
    LendingPoolAccrueBankInterest,
    LendingPoolAddBank(LendingPoolAddBankIxArgs),
    LendingPoolAddBankDrift(LendingPoolAddBankDriftIxArgs),
    LendingPoolAddBankJuplend(LendingPoolAddBankJuplendIxArgs),
    LendingPoolAddBankKamino(LendingPoolAddBankKaminoIxArgs),
    LendingPoolAddBankPermissionless(LendingPoolAddBankPermissionlessIxArgs),
    LendingPoolAddBankSolend(LendingPoolAddBankSolendIxArgs),
    LendingPoolAddBankWithSeed(LendingPoolAddBankWithSeedIxArgs),
    LendingPoolCloneBank(LendingPoolCloneBankIxArgs),
    LendingPoolCloneEmode,
    LendingPoolCloseBank,
    LendingPoolCollectBankFees,
    LendingPoolConfigureBank(LendingPoolConfigureBankIxArgs),
    LendingPoolConfigureBankEmode(LendingPoolConfigureBankEmodeIxArgs),
    LendingPoolConfigureBankInterestOnly(LendingPoolConfigureBankInterestOnlyIxArgs),
    LendingPoolConfigureBankLimitsOnly(LendingPoolConfigureBankLimitsOnlyIxArgs),
    LendingPoolConfigureBankOracle(LendingPoolConfigureBankOracleIxArgs),
    LendingPoolForceTokenlessRepayComplete,
    LendingPoolHandleBankruptcy,
    LendingPoolPulseBankPriceCache,
    LendingPoolReclaimEmissionsVault,
    LendingPoolSetFixedOraclePrice(LendingPoolSetFixedOraclePriceIxArgs),
    LendingPoolUpdateFeesDestinationAccount,
    LendingPoolWithdrawFees(LendingPoolWithdrawFeesIxArgs),
    LendingPoolWithdrawFeesPermissionless(LendingPoolWithdrawFeesPermissionlessIxArgs),
    LendingPoolWithdrawInsurance(LendingPoolWithdrawInsuranceIxArgs),
    MarginfiAccountClose,
    MarginfiAccountCloseOrder,
    MarginfiAccountEndExecuteOrder,
    MarginfiAccountInitLiqRecord,
    MarginfiAccountInitialize,
    MarginfiAccountInitializePda(MarginfiAccountInitializePdaIxArgs),
    MarginfiAccountKeeperCloseOrder,
    MarginfiAccountPlaceOrder(MarginfiAccountPlaceOrderIxArgs),
    MarginfiAccountSetFreeze(MarginfiAccountSetFreezeIxArgs),
    MarginfiAccountSetKeeperCloseFlags(MarginfiAccountSetKeeperCloseFlagsIxArgs),
    MarginfiAccountStartExecuteOrder,
    MarginfiAccountUpdateEmissionsDestinationAccount,
    MarginfiGroupConfigure(MarginfiGroupConfigureIxArgs),
    MarginfiGroupInitialize,
    MigrateCurve,
    PanicPause,
    PanicUnpause,
    PanicUnpausePermissionless,
    PropagateFeeState,
    PropagateStakedSettings,
    PurgeDeleverageBalance,
    SolendDeposit(SolendDepositIxArgs),
    SolendInitObligation(SolendInitObligationIxArgs),
    SolendWithdraw(SolendWithdrawIxArgs),
    StartDeleverage,
    StartLiquidation,
    SuperAdminDeposit(SuperAdminDepositIxArgs),
    SuperAdminWithdraw(SuperAdminWithdrawIxArgs),
    TransferToNewAccount,
    TransferToNewAccountPda(TransferToNewAccountPdaIxArgs),
    UpdateDeleverageWithdrawals(UpdateDeleverageWithdrawalsIxArgs),
    UpdateGroupRateLimiter(UpdateGroupRateLimiterIxArgs),
    WriteBankMetadata(WriteBankMetadataIxArgs),
}
impl MarginfiProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CONFIG_GROUP_FEE_IX_DISCM) {
            let mut reader = &buf[CONFIG_GROUP_FEE_IX_DISCM.len()..];
            let enable_program_fee: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ConfigGroupFee(ConfigGroupFeeIxArgs {
                    enable_program_fee,
                }),
            );
        }
        if buf.starts_with(&CONFIGURE_BANK_RATE_LIMITS_IX_DISCM) {
            let mut reader = &buf[CONFIGURE_BANK_RATE_LIMITS_IX_DISCM.len()..];
            let hourly_max_outflow: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let daily_max_outflow: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ConfigureBankRateLimits(ConfigureBankRateLimitsIxArgs {
                    hourly_max_outflow,
                    daily_max_outflow,
                }),
            );
        }
        if buf.starts_with(&CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_DISCM) {
            let mut reader = &buf[CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_DISCM
                .len()..];
            let limit: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ConfigureDeleverageWithdrawalLimit(ConfigureDeleverageWithdrawalLimitIxArgs {
                    limit,
                }),
            );
        }
        if buf.starts_with(&CONFIGURE_GROUP_RATE_LIMITS_IX_DISCM) {
            let mut reader = &buf[CONFIGURE_GROUP_RATE_LIMITS_IX_DISCM.len()..];
            let hourly_max_outflow_usd: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let daily_max_outflow_usd: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ConfigureGroupRateLimits(ConfigureGroupRateLimitsIxArgs {
                    hourly_max_outflow_usd,
                    daily_max_outflow_usd,
                }),
            );
        }
        if buf.starts_with(&DRIFT_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DRIFT_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DriftDeposit(DriftDepositIxArgs { amount }));
        }
        if buf.starts_with(&DRIFT_HARVEST_REWARD_IX_DISCM) {
            return Ok(Self::DriftHarvestReward);
        }
        if buf.starts_with(&DRIFT_INIT_USER_IX_DISCM) {
            let mut reader = &buf[DRIFT_INIT_USER_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DriftInitUser(DriftInitUserIxArgs { amount }));
        }
        if buf.starts_with(&DRIFT_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[DRIFT_WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DriftWithdraw(DriftWithdrawIxArgs {
                    amount,
                    withdraw_all,
                }),
            );
        }
        if buf.starts_with(&EDIT_GLOBAL_FEE_STATE_IX_DISCM) {
            let mut reader = &buf[EDIT_GLOBAL_FEE_STATE_IX_DISCM.len()..];
            let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let bank_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
            let liquidation_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
            let order_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
            let program_fee_fixed = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            let program_fee_rate = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            let liquidation_max_fee = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            let order_execution_max_fee = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            return Ok(
                Self::EditGlobalFeeState(EditGlobalFeeStateIxArgs {
                    admin,
                    fee_wallet,
                    bank_init_flat_sol_fee,
                    liquidation_flat_sol_fee,
                    order_init_flat_sol_fee,
                    program_fee_fixed,
                    program_fee_rate,
                    liquidation_max_fee,
                    order_execution_max_fee,
                }),
            );
        }
        if buf.starts_with(&EDIT_STAKED_SETTINGS_IX_DISCM) {
            let mut reader = &buf[EDIT_STAKED_SETTINGS_IX_DISCM.len()..];
            let settings = if reader.is_empty() {
                Default::default()
            } else {
                <StakedSettingsEditConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::EditStakedSettings(EditStakedSettingsIxArgs {
                    settings,
                }),
            );
        }
        if buf.starts_with(&END_DELEVERAGE_IX_DISCM) {
            return Ok(Self::EndDeleverage);
        }
        if buf.starts_with(&END_LIQUIDATION_IX_DISCM) {
            return Ok(Self::EndLiquidation);
        }
        if buf.starts_with(&INIT_BANK_METADATA_IX_DISCM) {
            return Ok(Self::InitBankMetadata);
        }
        if buf.starts_with(&INIT_GLOBAL_FEE_STATE_IX_DISCM) {
            let mut reader = &buf[INIT_GLOBAL_FEE_STATE_IX_DISCM.len()..];
            let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let bank_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
            let liquidation_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
            let order_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
            let program_fee_fixed = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            let program_fee_rate = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            let liquidation_max_fee = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            let order_execution_max_fee = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitGlobalFeeState(InitGlobalFeeStateIxArgs {
                    admin,
                    fee_wallet,
                    bank_init_flat_sol_fee,
                    liquidation_flat_sol_fee,
                    order_init_flat_sol_fee,
                    program_fee_fixed,
                    program_fee_rate,
                    liquidation_max_fee,
                    order_execution_max_fee,
                }),
            );
        }
        if buf.starts_with(&INIT_STAKED_SETTINGS_IX_DISCM) {
            let mut reader = &buf[INIT_STAKED_SETTINGS_IX_DISCM.len()..];
            let settings = if reader.is_empty() {
                Default::default()
            } else {
                <StakedSettingsConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitStakedSettings(InitStakedSettingsIxArgs {
                    settings,
                }),
            );
        }
        if buf.starts_with(&JUPLEND_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[JUPLEND_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::JuplendDeposit(JuplendDepositIxArgs { amount }));
        }
        if buf.starts_with(&JUPLEND_INIT_POSITION_IX_DISCM) {
            let mut reader = &buf[JUPLEND_INIT_POSITION_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::JuplendInitPosition(JuplendInitPositionIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&JUPLEND_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[JUPLEND_WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::JuplendWithdraw(JuplendWithdrawIxArgs {
                    amount,
                    withdraw_all,
                }),
            );
        }
        if buf.starts_with(&KAMINO_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[KAMINO_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::KaminoDeposit(KaminoDepositIxArgs { amount }));
        }
        if buf.starts_with(&KAMINO_HARVEST_REWARD_IX_DISCM) {
            let mut reader = &buf[KAMINO_HARVEST_REWARD_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::KaminoHarvestReward(KaminoHarvestRewardIxArgs {
                    reward_index,
                }),
            );
        }
        if buf.starts_with(&KAMINO_INIT_OBLIGATION_IX_DISCM) {
            let mut reader = &buf[KAMINO_INIT_OBLIGATION_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::KaminoInitObligation(KaminoInitObligationIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&KAMINO_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[KAMINO_WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::KaminoWithdraw(KaminoWithdrawIxArgs {
                    amount,
                    withdraw_all,
                }),
            );
        }
        if buf.starts_with(&LENDING_ACCOUNT_BORROW_IX_DISCM) {
            let mut reader = &buf[LENDING_ACCOUNT_BORROW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingAccountBorrow(LendingAccountBorrowIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_DISCM) {
            return Ok(Self::LendingAccountClearEmissions);
        }
        if buf.starts_with(&LENDING_ACCOUNT_CLOSE_BALANCE_IX_DISCM) {
            return Ok(Self::LendingAccountCloseBalance);
        }
        if buf.starts_with(&LENDING_ACCOUNT_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[LENDING_ACCOUNT_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let deposit_up_to_limit: Option<bool> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::LendingAccountDeposit(LendingAccountDepositIxArgs {
                    amount,
                    deposit_up_to_limit,
                }),
            );
        }
        if buf.starts_with(&LENDING_ACCOUNT_END_FLASHLOAN_IX_DISCM) {
            return Ok(Self::LendingAccountEndFlashloan);
        }
        if buf.starts_with(&LENDING_ACCOUNT_LIQUIDATE_IX_DISCM) {
            let mut reader = &buf[LENDING_ACCOUNT_LIQUIDATE_IX_DISCM.len()..];
            let asset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let liquidatee_accounts: u8 = crate::borsh_de_or_default(&mut reader)?;
            let liquidator_accounts: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingAccountLiquidate(LendingAccountLiquidateIxArgs {
                    asset_amount,
                    liquidatee_accounts,
                    liquidator_accounts,
                }),
            );
        }
        if buf.starts_with(&LENDING_ACCOUNT_PULSE_HEALTH_IX_DISCM) {
            return Ok(Self::LendingAccountPulseHealth);
        }
        if buf.starts_with(&LENDING_ACCOUNT_REPAY_IX_DISCM) {
            let mut reader = &buf[LENDING_ACCOUNT_REPAY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let repay_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingAccountRepay(LendingAccountRepayIxArgs {
                    amount,
                    repay_all,
                }),
            );
        }
        if buf.starts_with(&LENDING_ACCOUNT_START_FLASHLOAN_IX_DISCM) {
            let mut reader = &buf[LENDING_ACCOUNT_START_FLASHLOAN_IX_DISCM.len()..];
            let end_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingAccountStartFlashloan(LendingAccountStartFlashloanIxArgs {
                    end_index,
                }),
            );
        }
        if buf.starts_with(&LENDING_ACCOUNT_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[LENDING_ACCOUNT_WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingAccountWithdraw(LendingAccountWithdrawIxArgs {
                    amount,
                    withdraw_all,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_ACCRUE_BANK_INTEREST_IX_DISCM) {
            return Ok(Self::LendingPoolAccrueBankInterest);
        }
        if buf.starts_with(&LENDING_POOL_ADD_BANK_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_ADD_BANK_IX_DISCM.len()..];
            let bank_config = if reader.is_empty() {
                Default::default()
            } else {
                <BankConfigCompact>::deserialize(&mut reader)?
            };
            return Ok(
                Self::LendingPoolAddBank(LendingPoolAddBankIxArgs {
                    bank_config,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_ADD_BANK_DRIFT_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_ADD_BANK_DRIFT_IX_DISCM.len()..];
            let bank_config = if reader.is_empty() {
                Default::default()
            } else {
                <DriftConfigCompact>::deserialize(&mut reader)?
            };
            let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolAddBankDrift(LendingPoolAddBankDriftIxArgs {
                    bank_config,
                    bank_seed,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_ADD_BANK_JUPLEND_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_ADD_BANK_JUPLEND_IX_DISCM.len()..];
            let bank_config = if reader.is_empty() {
                Default::default()
            } else {
                <JuplendConfigCompact>::deserialize(&mut reader)?
            };
            let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolAddBankJuplend(LendingPoolAddBankJuplendIxArgs {
                    bank_config,
                    bank_seed,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_ADD_BANK_KAMINO_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_ADD_BANK_KAMINO_IX_DISCM.len()..];
            let bank_config = if reader.is_empty() {
                Default::default()
            } else {
                <KaminoConfigCompact>::deserialize(&mut reader)?
            };
            let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolAddBankKamino(LendingPoolAddBankKaminoIxArgs {
                    bank_config,
                    bank_seed,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_DISCM.len()..];
            let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolAddBankPermissionless(LendingPoolAddBankPermissionlessIxArgs {
                    bank_seed,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_ADD_BANK_SOLEND_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_ADD_BANK_SOLEND_IX_DISCM.len()..];
            let bank_config = if reader.is_empty() {
                Default::default()
            } else {
                <SolendConfigCompact>::deserialize(&mut reader)?
            };
            let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolAddBankSolend(LendingPoolAddBankSolendIxArgs {
                    bank_config,
                    bank_seed,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_ADD_BANK_WITH_SEED_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_ADD_BANK_WITH_SEED_IX_DISCM.len()..];
            let bank_config = if reader.is_empty() {
                Default::default()
            } else {
                <BankConfigCompact>::deserialize(&mut reader)?
            };
            let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolAddBankWithSeed(LendingPoolAddBankWithSeedIxArgs {
                    bank_config,
                    bank_seed,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_CLONE_BANK_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_CLONE_BANK_IX_DISCM.len()..];
            let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolCloneBank(LendingPoolCloneBankIxArgs {
                    bank_seed,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_CLONE_EMODE_IX_DISCM) {
            return Ok(Self::LendingPoolCloneEmode);
        }
        if buf.starts_with(&LENDING_POOL_CLOSE_BANK_IX_DISCM) {
            return Ok(Self::LendingPoolCloseBank);
        }
        if buf.starts_with(&LENDING_POOL_COLLECT_BANK_FEES_IX_DISCM) {
            return Ok(Self::LendingPoolCollectBankFees);
        }
        if buf.starts_with(&LENDING_POOL_CONFIGURE_BANK_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_CONFIGURE_BANK_IX_DISCM.len()..];
            let bank_config_opt = if reader.is_empty() {
                Default::default()
            } else {
                <BankConfigOpt>::deserialize(&mut reader)?
            };
            return Ok(
                Self::LendingPoolConfigureBank(LendingPoolConfigureBankIxArgs {
                    bank_config_opt,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_CONFIGURE_BANK_EMODE_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_CONFIGURE_BANK_EMODE_IX_DISCM.len()..];
            let emode_tag: u16 = crate::borsh_de_or_default(&mut reader)?;
            let entries: [EmodeEntry; 10] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolConfigureBankEmode(LendingPoolConfigureBankEmodeIxArgs {
                    emode_tag,
                    entries,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_DISCM
                .len()..];
            let interest_rate_config = if reader.is_empty() {
                Default::default()
            } else {
                <InterestRateConfigOpt>::deserialize(&mut reader)?
            };
            return Ok(
                Self::LendingPoolConfigureBankInterestOnly(LendingPoolConfigureBankInterestOnlyIxArgs {
                    interest_rate_config,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_DISCM
                .len()..];
            let deposit_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let borrow_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let total_asset_value_init_limit: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::LendingPoolConfigureBankLimitsOnly(LendingPoolConfigureBankLimitsOnlyIxArgs {
                    deposit_limit,
                    borrow_limit,
                    total_asset_value_init_limit,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_DISCM.len()..];
            let setup: u8 = crate::borsh_de_or_default(&mut reader)?;
            let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolConfigureBankOracle(LendingPoolConfigureBankOracleIxArgs {
                    setup,
                    oracle,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_DISCM) {
            return Ok(Self::LendingPoolForceTokenlessRepayComplete);
        }
        if buf.starts_with(&LENDING_POOL_HANDLE_BANKRUPTCY_IX_DISCM) {
            return Ok(Self::LendingPoolHandleBankruptcy);
        }
        if buf.starts_with(&LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_DISCM) {
            return Ok(Self::LendingPoolPulseBankPriceCache);
        }
        if buf.starts_with(&LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_DISCM) {
            return Ok(Self::LendingPoolReclaimEmissionsVault);
        }
        if buf.starts_with(&LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_DISCM.len()..];
            let price = if reader.is_empty() {
                Default::default()
            } else {
                <WrappedI80F48>::deserialize(&mut reader)?
            };
            return Ok(
                Self::LendingPoolSetFixedOraclePrice(LendingPoolSetFixedOraclePriceIxArgs {
                    price,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_DISCM) {
            return Ok(Self::LendingPoolUpdateFeesDestinationAccount);
        }
        if buf.starts_with(&LENDING_POOL_WITHDRAW_FEES_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_WITHDRAW_FEES_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolWithdrawFees(LendingPoolWithdrawFeesIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_DISCM
                .len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolWithdrawFeesPermissionless(LendingPoolWithdrawFeesPermissionlessIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&LENDING_POOL_WITHDRAW_INSURANCE_IX_DISCM) {
            let mut reader = &buf[LENDING_POOL_WITHDRAW_INSURANCE_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LendingPoolWithdrawInsurance(LendingPoolWithdrawInsuranceIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_CLOSE_IX_DISCM) {
            return Ok(Self::MarginfiAccountClose);
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_CLOSE_ORDER_IX_DISCM) {
            return Ok(Self::MarginfiAccountCloseOrder);
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_DISCM) {
            return Ok(Self::MarginfiAccountEndExecuteOrder);
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_DISCM) {
            return Ok(Self::MarginfiAccountInitLiqRecord);
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_INITIALIZE_IX_DISCM) {
            return Ok(Self::MarginfiAccountInitialize);
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_DISCM) {
            let mut reader = &buf[MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_DISCM.len()..];
            let account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
            let third_party_id: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MarginfiAccountInitializePda(MarginfiAccountInitializePdaIxArgs {
                    account_index,
                    third_party_id,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_DISCM) {
            return Ok(Self::MarginfiAccountKeeperCloseOrder);
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_PLACE_ORDER_IX_DISCM) {
            let mut reader = &buf[MARGINFI_ACCOUNT_PLACE_ORDER_IX_DISCM.len()..];
            let bank_keys: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            let trigger = <OrderTrigger as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::MarginfiAccountPlaceOrder(MarginfiAccountPlaceOrderIxArgs {
                    bank_keys,
                    trigger,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_SET_FREEZE_IX_DISCM) {
            let mut reader = &buf[MARGINFI_ACCOUNT_SET_FREEZE_IX_DISCM.len()..];
            let frozen: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MarginfiAccountSetFreeze(MarginfiAccountSetFreezeIxArgs {
                    frozen,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_DISCM) {
            let mut reader = &buf[MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_DISCM
                .len()..];
            let bank_keys_opt: Option<Vec<Pubkey>> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::MarginfiAccountSetKeeperCloseFlags(MarginfiAccountSetKeeperCloseFlagsIxArgs {
                    bank_keys_opt,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_DISCM) {
            return Ok(Self::MarginfiAccountStartExecuteOrder);
        }
        if buf
            .starts_with(&MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_DISCM)
        {
            return Ok(Self::MarginfiAccountUpdateEmissionsDestinationAccount);
        }
        if buf.starts_with(&MARGINFI_GROUP_CONFIGURE_IX_DISCM) {
            let mut reader = &buf[MARGINFI_GROUP_CONFIGURE_IX_DISCM.len()..];
            let new_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            let new_emode_admin: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_curve_admin: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_limit_admin: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_flow_admin: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_emissions_admin: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_metadata_admin: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let new_risk_admin: Option<Pubkey> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let emode_max_init_leverage: Option<WrappedI80F48> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let emode_max_maint_leverage: Option<WrappedI80F48> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::MarginfiGroupConfigure(MarginfiGroupConfigureIxArgs {
                    new_admin,
                    new_emode_admin,
                    new_curve_admin,
                    new_limit_admin,
                    new_flow_admin,
                    new_emissions_admin,
                    new_metadata_admin,
                    new_risk_admin,
                    emode_max_init_leverage,
                    emode_max_maint_leverage,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_GROUP_INITIALIZE_IX_DISCM) {
            return Ok(Self::MarginfiGroupInitialize);
        }
        if buf.starts_with(&MIGRATE_CURVE_IX_DISCM) {
            return Ok(Self::MigrateCurve);
        }
        if buf.starts_with(&PANIC_PAUSE_IX_DISCM) {
            return Ok(Self::PanicPause);
        }
        if buf.starts_with(&PANIC_UNPAUSE_IX_DISCM) {
            return Ok(Self::PanicUnpause);
        }
        if buf.starts_with(&PANIC_UNPAUSE_PERMISSIONLESS_IX_DISCM) {
            return Ok(Self::PanicUnpausePermissionless);
        }
        if buf.starts_with(&PROPAGATE_FEE_STATE_IX_DISCM) {
            return Ok(Self::PropagateFeeState);
        }
        if buf.starts_with(&PROPAGATE_STAKED_SETTINGS_IX_DISCM) {
            return Ok(Self::PropagateStakedSettings);
        }
        if buf.starts_with(&PURGE_DELEVERAGE_BALANCE_IX_DISCM) {
            return Ok(Self::PurgeDeleverageBalance);
        }
        if buf.starts_with(&SOLEND_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[SOLEND_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SolendDeposit(SolendDepositIxArgs { amount }));
        }
        if buf.starts_with(&SOLEND_INIT_OBLIGATION_IX_DISCM) {
            let mut reader = &buf[SOLEND_INIT_OBLIGATION_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SolendInitObligation(SolendInitObligationIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&SOLEND_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[SOLEND_WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SolendWithdraw(SolendWithdrawIxArgs {
                    amount,
                    withdraw_all,
                }),
            );
        }
        if buf.starts_with(&START_DELEVERAGE_IX_DISCM) {
            return Ok(Self::StartDeleverage);
        }
        if buf.starts_with(&START_LIQUIDATION_IX_DISCM) {
            return Ok(Self::StartLiquidation);
        }
        if buf.starts_with(&SUPER_ADMIN_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[SUPER_ADMIN_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SuperAdminDeposit(SuperAdminDepositIxArgs { amount }));
        }
        if buf.starts_with(&SUPER_ADMIN_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[SUPER_ADMIN_WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SuperAdminWithdraw(SuperAdminWithdrawIxArgs { amount }));
        }
        if buf.starts_with(&TRANSFER_TO_NEW_ACCOUNT_IX_DISCM) {
            return Ok(Self::TransferToNewAccount);
        }
        if buf.starts_with(&TRANSFER_TO_NEW_ACCOUNT_PDA_IX_DISCM) {
            let mut reader = &buf[TRANSFER_TO_NEW_ACCOUNT_PDA_IX_DISCM.len()..];
            let account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
            let third_party_id: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TransferToNewAccountPda(TransferToNewAccountPdaIxArgs {
                    account_index,
                    third_party_id,
                }),
            );
        }
        if buf.starts_with(&UPDATE_DELEVERAGE_WITHDRAWALS_IX_DISCM) {
            let mut reader = &buf[UPDATE_DELEVERAGE_WITHDRAWALS_IX_DISCM.len()..];
            let outflow_usd: u32 = crate::borsh_de_or_default(&mut reader)?;
            let update_seq: u64 = crate::borsh_de_or_default(&mut reader)?;
            let event_start_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
            let event_end_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateDeleverageWithdrawals(UpdateDeleverageWithdrawalsIxArgs {
                    outflow_usd,
                    update_seq,
                    event_start_slot,
                    event_end_slot,
                }),
            );
        }
        if buf.starts_with(&UPDATE_GROUP_RATE_LIMITER_IX_DISCM) {
            let mut reader = &buf[UPDATE_GROUP_RATE_LIMITER_IX_DISCM.len()..];
            let outflow_usd: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let inflow_usd: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let update_seq: u64 = crate::borsh_de_or_default(&mut reader)?;
            let event_start_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
            let event_end_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateGroupRateLimiter(UpdateGroupRateLimiterIxArgs {
                    outflow_usd,
                    inflow_usd,
                    update_seq,
                    event_start_slot,
                    event_end_slot,
                }),
            );
        }
        if buf.starts_with(&WRITE_BANK_METADATA_IX_DISCM) {
            let mut reader = &buf[WRITE_BANK_METADATA_IX_DISCM.len()..];
            let ticker: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            let description: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WriteBankMetadata(WriteBankMetadataIxArgs {
                    ticker,
                    description,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::ConfigGroupFee(args) => {
                writer.write_all(&CONFIG_GROUP_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.enable_program_fee, &mut writer)?;
                Ok(())
            }
            Self::ConfigureBankRateLimits(args) => {
                writer.write_all(&CONFIGURE_BANK_RATE_LIMITS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.hourly_max_outflow, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.daily_max_outflow, &mut writer)?;
                Ok(())
            }
            Self::ConfigureDeleverageWithdrawalLimit(args) => {
                writer.write_all(&CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.limit, &mut writer)?;
                Ok(())
            }
            Self::ConfigureGroupRateLimits(args) => {
                writer.write_all(&CONFIGURE_GROUP_RATE_LIMITS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.hourly_max_outflow_usd,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.daily_max_outflow_usd,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::DriftDeposit(args) => {
                writer.write_all(&DRIFT_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::DriftHarvestReward => writer.write_all(&DRIFT_HARVEST_REWARD_IX_DISCM),
            Self::DriftInitUser(args) => {
                writer.write_all(&DRIFT_INIT_USER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::DriftWithdraw(args) => {
                writer.write_all(&DRIFT_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.withdraw_all, &mut writer)?;
                Ok(())
            }
            Self::EditGlobalFeeState(args) => {
                writer.write_all(&EDIT_GLOBAL_FEE_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.admin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_wallet, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.bank_init_flat_sol_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.liquidation_flat_sol_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.order_init_flat_sol_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.program_fee_fixed, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.program_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidation_max_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.order_execution_max_fee,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::EditStakedSettings(args) => {
                writer.write_all(&EDIT_STAKED_SETTINGS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.settings, &mut writer)?;
                Ok(())
            }
            Self::EndDeleverage => writer.write_all(&END_DELEVERAGE_IX_DISCM),
            Self::EndLiquidation => writer.write_all(&END_LIQUIDATION_IX_DISCM),
            Self::InitBankMetadata => writer.write_all(&INIT_BANK_METADATA_IX_DISCM),
            Self::InitGlobalFeeState(args) => {
                writer.write_all(&INIT_GLOBAL_FEE_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.admin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_wallet, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.bank_init_flat_sol_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.liquidation_flat_sol_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.order_init_flat_sol_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.program_fee_fixed, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.program_fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidation_max_fee,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.order_execution_max_fee,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitStakedSettings(args) => {
                writer.write_all(&INIT_STAKED_SETTINGS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.settings, &mut writer)?;
                Ok(())
            }
            Self::JuplendDeposit(args) => {
                writer.write_all(&JUPLEND_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::JuplendInitPosition(args) => {
                writer.write_all(&JUPLEND_INIT_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::JuplendWithdraw(args) => {
                writer.write_all(&JUPLEND_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.withdraw_all, &mut writer)?;
                Ok(())
            }
            Self::KaminoDeposit(args) => {
                writer.write_all(&KAMINO_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::KaminoHarvestReward(args) => {
                writer.write_all(&KAMINO_HARVEST_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                Ok(())
            }
            Self::KaminoInitObligation(args) => {
                writer.write_all(&KAMINO_INIT_OBLIGATION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::KaminoWithdraw(args) => {
                writer.write_all(&KAMINO_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.withdraw_all, &mut writer)?;
                Ok(())
            }
            Self::LendingAccountBorrow(args) => {
                writer.write_all(&LENDING_ACCOUNT_BORROW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::LendingAccountClearEmissions => {
                writer.write_all(&LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_DISCM)
            }
            Self::LendingAccountCloseBalance => {
                writer.write_all(&LENDING_ACCOUNT_CLOSE_BALANCE_IX_DISCM)
            }
            Self::LendingAccountDeposit(args) => {
                writer.write_all(&LENDING_ACCOUNT_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.deposit_up_to_limit,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::LendingAccountEndFlashloan => {
                writer.write_all(&LENDING_ACCOUNT_END_FLASHLOAN_IX_DISCM)
            }
            Self::LendingAccountLiquidate(args) => {
                writer.write_all(&LENDING_ACCOUNT_LIQUIDATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.asset_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidatee_accounts,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.liquidator_accounts,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::LendingAccountPulseHealth => {
                writer.write_all(&LENDING_ACCOUNT_PULSE_HEALTH_IX_DISCM)
            }
            Self::LendingAccountRepay(args) => {
                writer.write_all(&LENDING_ACCOUNT_REPAY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.repay_all, &mut writer)?;
                Ok(())
            }
            Self::LendingAccountStartFlashloan(args) => {
                writer.write_all(&LENDING_ACCOUNT_START_FLASHLOAN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.end_index, &mut writer)?;
                Ok(())
            }
            Self::LendingAccountWithdraw(args) => {
                writer.write_all(&LENDING_ACCOUNT_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.withdraw_all, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolAccrueBankInterest => {
                writer.write_all(&LENDING_POOL_ACCRUE_BANK_INTEREST_IX_DISCM)
            }
            Self::LendingPoolAddBank(args) => {
                writer.write_all(&LENDING_POOL_ADD_BANK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_config, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolAddBankDrift(args) => {
                writer.write_all(&LENDING_POOL_ADD_BANK_DRIFT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bank_seed, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolAddBankJuplend(args) => {
                writer.write_all(&LENDING_POOL_ADD_BANK_JUPLEND_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bank_seed, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolAddBankKamino(args) => {
                writer.write_all(&LENDING_POOL_ADD_BANK_KAMINO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bank_seed, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolAddBankPermissionless(args) => {
                writer.write_all(&LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_seed, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolAddBankSolend(args) => {
                writer.write_all(&LENDING_POOL_ADD_BANK_SOLEND_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bank_seed, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolAddBankWithSeed(args) => {
                writer.write_all(&LENDING_POOL_ADD_BANK_WITH_SEED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bank_seed, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolCloneBank(args) => {
                writer.write_all(&LENDING_POOL_CLONE_BANK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_seed, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolCloneEmode => {
                writer.write_all(&LENDING_POOL_CLONE_EMODE_IX_DISCM)
            }
            Self::LendingPoolCloseBank => {
                writer.write_all(&LENDING_POOL_CLOSE_BANK_IX_DISCM)
            }
            Self::LendingPoolCollectBankFees => {
                writer.write_all(&LENDING_POOL_COLLECT_BANK_FEES_IX_DISCM)
            }
            Self::LendingPoolConfigureBank(args) => {
                writer.write_all(&LENDING_POOL_CONFIGURE_BANK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_config_opt, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolConfigureBankEmode(args) => {
                writer.write_all(&LENDING_POOL_CONFIGURE_BANK_EMODE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.emode_tag, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.entries, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolConfigureBankInterestOnly(args) => {
                writer.write_all(&LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.interest_rate_config,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::LendingPoolConfigureBankLimitsOnly(args) => {
                writer.write_all(&LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.deposit_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.borrow_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.total_asset_value_init_limit,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::LendingPoolConfigureBankOracle(args) => {
                writer.write_all(&LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.setup, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.oracle, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolForceTokenlessRepayComplete => {
                writer.write_all(&LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_DISCM)
            }
            Self::LendingPoolHandleBankruptcy => {
                writer.write_all(&LENDING_POOL_HANDLE_BANKRUPTCY_IX_DISCM)
            }
            Self::LendingPoolPulseBankPriceCache => {
                writer.write_all(&LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_DISCM)
            }
            Self::LendingPoolReclaimEmissionsVault => {
                writer.write_all(&LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_DISCM)
            }
            Self::LendingPoolSetFixedOraclePrice(args) => {
                writer.write_all(&LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolUpdateFeesDestinationAccount => {
                writer.write_all(&LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_DISCM)
            }
            Self::LendingPoolWithdrawFees(args) => {
                writer.write_all(&LENDING_POOL_WITHDRAW_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolWithdrawFeesPermissionless(args) => {
                writer.write_all(&LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::LendingPoolWithdrawInsurance(args) => {
                writer.write_all(&LENDING_POOL_WITHDRAW_INSURANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::MarginfiAccountClose => {
                writer.write_all(&MARGINFI_ACCOUNT_CLOSE_IX_DISCM)
            }
            Self::MarginfiAccountCloseOrder => {
                writer.write_all(&MARGINFI_ACCOUNT_CLOSE_ORDER_IX_DISCM)
            }
            Self::MarginfiAccountEndExecuteOrder => {
                writer.write_all(&MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_DISCM)
            }
            Self::MarginfiAccountInitLiqRecord => {
                writer.write_all(&MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_DISCM)
            }
            Self::MarginfiAccountInitialize => {
                writer.write_all(&MARGINFI_ACCOUNT_INITIALIZE_IX_DISCM)
            }
            Self::MarginfiAccountInitializePda(args) => {
                writer.write_all(&MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.account_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.third_party_id, &mut writer)?;
                Ok(())
            }
            Self::MarginfiAccountKeeperCloseOrder => {
                writer.write_all(&MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_DISCM)
            }
            Self::MarginfiAccountPlaceOrder(args) => {
                writer.write_all(&MARGINFI_ACCOUNT_PLACE_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_keys, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.trigger, &mut writer)?;
                Ok(())
            }
            Self::MarginfiAccountSetFreeze(args) => {
                writer.write_all(&MARGINFI_ACCOUNT_SET_FREEZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.frozen, &mut writer)?;
                Ok(())
            }
            Self::MarginfiAccountSetKeeperCloseFlags(args) => {
                writer.write_all(&MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bank_keys_opt, &mut writer)?;
                Ok(())
            }
            Self::MarginfiAccountStartExecuteOrder => {
                writer.write_all(&MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_DISCM)
            }
            Self::MarginfiAccountUpdateEmissionsDestinationAccount => {
                writer
                    .write_all(
                        &MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_DISCM,
                    )
            }
            Self::MarginfiGroupConfigure(args) => {
                writer.write_all(&MARGINFI_GROUP_CONFIGURE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_admin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_emode_admin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_curve_admin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_limit_admin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_flow_admin, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.new_emissions_admin,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.new_metadata_admin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_risk_admin, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.emode_max_init_leverage,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.emode_max_maint_leverage,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::MarginfiGroupInitialize => {
                writer.write_all(&MARGINFI_GROUP_INITIALIZE_IX_DISCM)
            }
            Self::MigrateCurve => writer.write_all(&MIGRATE_CURVE_IX_DISCM),
            Self::PanicPause => writer.write_all(&PANIC_PAUSE_IX_DISCM),
            Self::PanicUnpause => writer.write_all(&PANIC_UNPAUSE_IX_DISCM),
            Self::PanicUnpausePermissionless => {
                writer.write_all(&PANIC_UNPAUSE_PERMISSIONLESS_IX_DISCM)
            }
            Self::PropagateFeeState => writer.write_all(&PROPAGATE_FEE_STATE_IX_DISCM),
            Self::PropagateStakedSettings => {
                writer.write_all(&PROPAGATE_STAKED_SETTINGS_IX_DISCM)
            }
            Self::PurgeDeleverageBalance => {
                writer.write_all(&PURGE_DELEVERAGE_BALANCE_IX_DISCM)
            }
            Self::SolendDeposit(args) => {
                writer.write_all(&SOLEND_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::SolendInitObligation(args) => {
                writer.write_all(&SOLEND_INIT_OBLIGATION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::SolendWithdraw(args) => {
                writer.write_all(&SOLEND_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.withdraw_all, &mut writer)?;
                Ok(())
            }
            Self::StartDeleverage => writer.write_all(&START_DELEVERAGE_IX_DISCM),
            Self::StartLiquidation => writer.write_all(&START_LIQUIDATION_IX_DISCM),
            Self::SuperAdminDeposit(args) => {
                writer.write_all(&SUPER_ADMIN_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::SuperAdminWithdraw(args) => {
                writer.write_all(&SUPER_ADMIN_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::TransferToNewAccount => {
                writer.write_all(&TRANSFER_TO_NEW_ACCOUNT_IX_DISCM)
            }
            Self::TransferToNewAccountPda(args) => {
                writer.write_all(&TRANSFER_TO_NEW_ACCOUNT_PDA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.account_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.third_party_id, &mut writer)?;
                Ok(())
            }
            Self::UpdateDeleverageWithdrawals(args) => {
                writer.write_all(&UPDATE_DELEVERAGE_WITHDRAWALS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.outflow_usd, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.update_seq, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.event_start_slot, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.event_end_slot, &mut writer)?;
                Ok(())
            }
            Self::UpdateGroupRateLimiter(args) => {
                writer.write_all(&UPDATE_GROUP_RATE_LIMITER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.outflow_usd, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.inflow_usd, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.update_seq, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.event_start_slot, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.event_end_slot, &mut writer)?;
                Ok(())
            }
            Self::WriteBankMetadata(args) => {
                writer.write_all(&WRITE_BANK_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.ticker, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.description, &mut writer)?;
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
pub const CONFIG_GROUP_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ConfigGroupFeeAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub global_fee_admin: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigGroupFeeKeys {
    pub marginfi_group: Pubkey,
    pub global_fee_admin: Pubkey,
    pub fee_state: Pubkey,
}
impl From<ConfigGroupFeeAccounts<'_, '_>> for ConfigGroupFeeKeys {
    fn from(accounts: ConfigGroupFeeAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            global_fee_admin: *accounts.global_fee_admin.key,
            fee_state: *accounts.fee_state.key,
        }
    }
}
impl From<ConfigGroupFeeKeys> for [AccountMeta; CONFIG_GROUP_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigGroupFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_fee_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONFIG_GROUP_FEE_IX_ACCOUNTS_LEN]> for ConfigGroupFeeKeys {
    fn from(pubkeys: [Pubkey; CONFIG_GROUP_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            global_fee_admin: pubkeys[1],
            fee_state: pubkeys[2],
        }
    }
}
impl<'info> From<ConfigGroupFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CONFIG_GROUP_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConfigGroupFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.global_fee_admin.clone(),
            accounts.fee_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONFIG_GROUP_FEE_IX_ACCOUNTS_LEN]>
for ConfigGroupFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CONFIG_GROUP_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: &arr[0],
            global_fee_admin: &arr[1],
            fee_state: &arr[2],
        }
    }
}
pub const CONFIG_GROUP_FEE_IX_DISCM: [u8; 8usize] = [
    231, 205, 66, 242, 220, 87, 145, 38,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigGroupFeeIxArgs {
    pub enable_program_fee: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigGroupFeeIxData(pub ConfigGroupFeeIxArgs);
impl From<ConfigGroupFeeIxArgs> for ConfigGroupFeeIxData {
    fn from(args: ConfigGroupFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigGroupFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_GROUP_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let enable_program_fee: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ConfigGroupFeeIxArgs {
                enable_program_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_GROUP_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.enable_program_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn config_group_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigGroupFeeKeys,
    args: ConfigGroupFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIG_GROUP_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConfigGroupFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn config_group_fee_ix(
    keys: ConfigGroupFeeKeys,
    args: ConfigGroupFeeIxArgs,
) -> std::io::Result<Instruction> {
    config_group_fee_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn config_group_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigGroupFeeAccounts<'_, '_>,
    args: ConfigGroupFeeIxArgs,
) -> ProgramResult {
    let keys: ConfigGroupFeeKeys = accounts.into();
    let ix = config_group_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn config_group_fee_invoke(
    accounts: ConfigGroupFeeAccounts<'_, '_>,
    args: ConfigGroupFeeIxArgs,
) -> ProgramResult {
    config_group_fee_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn config_group_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigGroupFeeAccounts<'_, '_>,
    args: ConfigGroupFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigGroupFeeKeys = accounts.into();
    let ix = config_group_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn config_group_fee_invoke_signed(
    accounts: ConfigGroupFeeAccounts<'_, '_>,
    args: ConfigGroupFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    config_group_fee_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn config_group_fee_verify_account_keys(
    accounts: ConfigGroupFeeAccounts<'_, '_>,
    keys: ConfigGroupFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.global_fee_admin.key, keys.global_fee_admin),
        (*accounts.fee_state.key, keys.fee_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn config_group_fee_verify_writable_privileges<'me, 'info>(
    accounts: ConfigGroupFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_group] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn config_group_fee_verify_signer_privileges<'me, 'info>(
    accounts: ConfigGroupFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_fee_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn config_group_fee_verify_account_privileges<'me, 'info>(
    accounts: ConfigGroupFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    config_group_fee_verify_writable_privileges(accounts)?;
    config_group_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONFIGURE_BANK_RATE_LIMITS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ConfigureBankRateLimitsAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigureBankRateLimitsKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub bank: Pubkey,
}
impl From<ConfigureBankRateLimitsAccounts<'_, '_>> for ConfigureBankRateLimitsKeys {
    fn from(accounts: ConfigureBankRateLimitsAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<ConfigureBankRateLimitsKeys>
for [AccountMeta; CONFIGURE_BANK_RATE_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigureBankRateLimitsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CONFIGURE_BANK_RATE_LIMITS_IX_ACCOUNTS_LEN]>
for ConfigureBankRateLimitsKeys {
    fn from(pubkeys: [Pubkey; CONFIGURE_BANK_RATE_LIMITS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<ConfigureBankRateLimitsAccounts<'_, 'info>>
for [AccountInfo<'info>; CONFIGURE_BANK_RATE_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConfigureBankRateLimitsAccounts<'_, 'info>) -> Self {
        [accounts.group.clone(), accounts.admin.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONFIGURE_BANK_RATE_LIMITS_IX_ACCOUNTS_LEN]>
for ConfigureBankRateLimitsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONFIGURE_BANK_RATE_LIMITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const CONFIGURE_BANK_RATE_LIMITS_IX_DISCM: [u8; 8usize] = [
    175, 84, 85, 221, 206, 220, 110, 174,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigureBankRateLimitsIxArgs {
    pub hourly_max_outflow: Option<u64>,
    pub daily_max_outflow: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigureBankRateLimitsIxData(pub ConfigureBankRateLimitsIxArgs);
impl From<ConfigureBankRateLimitsIxArgs> for ConfigureBankRateLimitsIxData {
    fn from(args: ConfigureBankRateLimitsIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigureBankRateLimitsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURE_BANK_RATE_LIMITS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let hourly_max_outflow: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let daily_max_outflow: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ConfigureBankRateLimitsIxArgs {
                hourly_max_outflow,
                daily_max_outflow,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURE_BANK_RATE_LIMITS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.hourly_max_outflow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.daily_max_outflow, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn configure_bank_rate_limits_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigureBankRateLimitsKeys,
    args: ConfigureBankRateLimitsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIGURE_BANK_RATE_LIMITS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConfigureBankRateLimitsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn configure_bank_rate_limits_ix(
    keys: ConfigureBankRateLimitsKeys,
    args: ConfigureBankRateLimitsIxArgs,
) -> std::io::Result<Instruction> {
    configure_bank_rate_limits_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn configure_bank_rate_limits_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureBankRateLimitsAccounts<'_, '_>,
    args: ConfigureBankRateLimitsIxArgs,
) -> ProgramResult {
    let keys: ConfigureBankRateLimitsKeys = accounts.into();
    let ix = configure_bank_rate_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn configure_bank_rate_limits_invoke(
    accounts: ConfigureBankRateLimitsAccounts<'_, '_>,
    args: ConfigureBankRateLimitsIxArgs,
) -> ProgramResult {
    configure_bank_rate_limits_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn configure_bank_rate_limits_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureBankRateLimitsAccounts<'_, '_>,
    args: ConfigureBankRateLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigureBankRateLimitsKeys = accounts.into();
    let ix = configure_bank_rate_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn configure_bank_rate_limits_invoke_signed(
    accounts: ConfigureBankRateLimitsAccounts<'_, '_>,
    args: ConfigureBankRateLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    configure_bank_rate_limits_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn configure_bank_rate_limits_verify_account_keys(
    accounts: ConfigureBankRateLimitsAccounts<'_, '_>,
    keys: ConfigureBankRateLimitsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn configure_bank_rate_limits_verify_writable_privileges<'me, 'info>(
    accounts: ConfigureBankRateLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn configure_bank_rate_limits_verify_signer_privileges<'me, 'info>(
    accounts: ConfigureBankRateLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn configure_bank_rate_limits_verify_account_privileges<'me, 'info>(
    accounts: ConfigureBankRateLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    configure_bank_rate_limits_verify_writable_privileges(accounts)?;
    configure_bank_rate_limits_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ConfigureDeleverageWithdrawalLimitAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigureDeleverageWithdrawalLimitKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
}
impl From<ConfigureDeleverageWithdrawalLimitAccounts<'_, '_>>
for ConfigureDeleverageWithdrawalLimitKeys {
    fn from(accounts: ConfigureDeleverageWithdrawalLimitAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<ConfigureDeleverageWithdrawalLimitKeys>
for [AccountMeta; CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigureDeleverageWithdrawalLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
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
impl From<[Pubkey; CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN]>
for ConfigureDeleverageWithdrawalLimitKeys {
    fn from(
        pubkeys: [Pubkey; CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<ConfigureDeleverageWithdrawalLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConfigureDeleverageWithdrawalLimitAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_group.clone(), accounts.admin.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN]>
for ConfigureDeleverageWithdrawalLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_DISCM: [u8; 8usize] = [
    28, 132, 205, 158, 67, 77, 177, 63,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigureDeleverageWithdrawalLimitIxArgs {
    pub limit: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigureDeleverageWithdrawalLimitIxData(
    pub ConfigureDeleverageWithdrawalLimitIxArgs,
);
impl From<ConfigureDeleverageWithdrawalLimitIxArgs>
for ConfigureDeleverageWithdrawalLimitIxData {
    fn from(args: ConfigureDeleverageWithdrawalLimitIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigureDeleverageWithdrawalLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let limit: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ConfigureDeleverageWithdrawalLimitIxArgs {
                limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn configure_deleverage_withdrawal_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigureDeleverageWithdrawalLimitKeys,
    args: ConfigureDeleverageWithdrawalLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIGURE_DELEVERAGE_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ConfigureDeleverageWithdrawalLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn configure_deleverage_withdrawal_limit_ix(
    keys: ConfigureDeleverageWithdrawalLimitKeys,
    args: ConfigureDeleverageWithdrawalLimitIxArgs,
) -> std::io::Result<Instruction> {
    configure_deleverage_withdrawal_limit_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn configure_deleverage_withdrawal_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureDeleverageWithdrawalLimitAccounts<'_, '_>,
    args: ConfigureDeleverageWithdrawalLimitIxArgs,
) -> ProgramResult {
    let keys: ConfigureDeleverageWithdrawalLimitKeys = accounts.into();
    let ix = configure_deleverage_withdrawal_limit_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn configure_deleverage_withdrawal_limit_invoke(
    accounts: ConfigureDeleverageWithdrawalLimitAccounts<'_, '_>,
    args: ConfigureDeleverageWithdrawalLimitIxArgs,
) -> ProgramResult {
    configure_deleverage_withdrawal_limit_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn configure_deleverage_withdrawal_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureDeleverageWithdrawalLimitAccounts<'_, '_>,
    args: ConfigureDeleverageWithdrawalLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigureDeleverageWithdrawalLimitKeys = accounts.into();
    let ix = configure_deleverage_withdrawal_limit_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn configure_deleverage_withdrawal_limit_invoke_signed(
    accounts: ConfigureDeleverageWithdrawalLimitAccounts<'_, '_>,
    args: ConfigureDeleverageWithdrawalLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    configure_deleverage_withdrawal_limit_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn configure_deleverage_withdrawal_limit_verify_account_keys(
    accounts: ConfigureDeleverageWithdrawalLimitAccounts<'_, '_>,
    keys: ConfigureDeleverageWithdrawalLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn configure_deleverage_withdrawal_limit_verify_writable_privileges<'me, 'info>(
    accounts: ConfigureDeleverageWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_group] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn configure_deleverage_withdrawal_limit_verify_signer_privileges<'me, 'info>(
    accounts: ConfigureDeleverageWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn configure_deleverage_withdrawal_limit_verify_account_privileges<'me, 'info>(
    accounts: ConfigureDeleverageWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    configure_deleverage_withdrawal_limit_verify_writable_privileges(accounts)?;
    configure_deleverage_withdrawal_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONFIGURE_GROUP_RATE_LIMITS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ConfigureGroupRateLimitsAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigureGroupRateLimitsKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
}
impl From<ConfigureGroupRateLimitsAccounts<'_, '_>> for ConfigureGroupRateLimitsKeys {
    fn from(accounts: ConfigureGroupRateLimitsAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<ConfigureGroupRateLimitsKeys>
for [AccountMeta; CONFIGURE_GROUP_RATE_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigureGroupRateLimitsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
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
impl From<[Pubkey; CONFIGURE_GROUP_RATE_LIMITS_IX_ACCOUNTS_LEN]>
for ConfigureGroupRateLimitsKeys {
    fn from(pubkeys: [Pubkey; CONFIGURE_GROUP_RATE_LIMITS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<ConfigureGroupRateLimitsAccounts<'_, 'info>>
for [AccountInfo<'info>; CONFIGURE_GROUP_RATE_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConfigureGroupRateLimitsAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_group.clone(), accounts.admin.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONFIGURE_GROUP_RATE_LIMITS_IX_ACCOUNTS_LEN]>
for ConfigureGroupRateLimitsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONFIGURE_GROUP_RATE_LIMITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const CONFIGURE_GROUP_RATE_LIMITS_IX_DISCM: [u8; 8usize] = [
    111, 47, 213, 142, 158, 51, 226, 102,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigureGroupRateLimitsIxArgs {
    pub hourly_max_outflow_usd: Option<u64>,
    pub daily_max_outflow_usd: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigureGroupRateLimitsIxData(pub ConfigureGroupRateLimitsIxArgs);
impl From<ConfigureGroupRateLimitsIxArgs> for ConfigureGroupRateLimitsIxData {
    fn from(args: ConfigureGroupRateLimitsIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigureGroupRateLimitsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURE_GROUP_RATE_LIMITS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let hourly_max_outflow_usd: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let daily_max_outflow_usd: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(ConfigureGroupRateLimitsIxArgs {
                hourly_max_outflow_usd,
                daily_max_outflow_usd,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURE_GROUP_RATE_LIMITS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.hourly_max_outflow_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.daily_max_outflow_usd, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn configure_group_rate_limits_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigureGroupRateLimitsKeys,
    args: ConfigureGroupRateLimitsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIGURE_GROUP_RATE_LIMITS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConfigureGroupRateLimitsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn configure_group_rate_limits_ix(
    keys: ConfigureGroupRateLimitsKeys,
    args: ConfigureGroupRateLimitsIxArgs,
) -> std::io::Result<Instruction> {
    configure_group_rate_limits_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn configure_group_rate_limits_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureGroupRateLimitsAccounts<'_, '_>,
    args: ConfigureGroupRateLimitsIxArgs,
) -> ProgramResult {
    let keys: ConfigureGroupRateLimitsKeys = accounts.into();
    let ix = configure_group_rate_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn configure_group_rate_limits_invoke(
    accounts: ConfigureGroupRateLimitsAccounts<'_, '_>,
    args: ConfigureGroupRateLimitsIxArgs,
) -> ProgramResult {
    configure_group_rate_limits_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn configure_group_rate_limits_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureGroupRateLimitsAccounts<'_, '_>,
    args: ConfigureGroupRateLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigureGroupRateLimitsKeys = accounts.into();
    let ix = configure_group_rate_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn configure_group_rate_limits_invoke_signed(
    accounts: ConfigureGroupRateLimitsAccounts<'_, '_>,
    args: ConfigureGroupRateLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    configure_group_rate_limits_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn configure_group_rate_limits_verify_account_keys(
    accounts: ConfigureGroupRateLimitsAccounts<'_, '_>,
    keys: ConfigureGroupRateLimitsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn configure_group_rate_limits_verify_writable_privileges<'me, 'info>(
    accounts: ConfigureGroupRateLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_group] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn configure_group_rate_limits_verify_signer_privileges<'me, 'info>(
    accounts: ConfigureGroupRateLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn configure_group_rate_limits_verify_account_privileges<'me, 'info>(
    accounts: ConfigureGroupRateLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    configure_group_rate_limits_verify_writable_privileges(accounts)?;
    configure_group_rate_limits_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_DEPOSIT_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct DriftDepositAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub drift_oracle: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub integration_acc_3: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftDepositKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub drift_oracle: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub signer_token_account: Pubkey,
    pub drift_state: Pubkey,
    pub integration_acc_2: Pubkey,
    pub integration_acc_3: Pubkey,
    pub integration_acc_1: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub mint: Pubkey,
    pub drift_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DriftDepositAccounts<'_, '_>> for DriftDepositKeys {
    fn from(accounts: DriftDepositAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            drift_oracle: *accounts.drift_oracle.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            signer_token_account: *accounts.signer_token_account.key,
            drift_state: *accounts.drift_state.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            integration_acc_3: *accounts.integration_acc_3.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            mint: *accounts.mint.key,
            drift_program: *accounts.drift_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DriftDepositKeys> for [AccountMeta; DRIFT_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_3,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
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
impl From<[Pubkey; DRIFT_DEPOSIT_IX_ACCOUNTS_LEN]> for DriftDepositKeys {
    fn from(pubkeys: [Pubkey; DRIFT_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            drift_oracle: pubkeys[4],
            liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            signer_token_account: pubkeys[7],
            drift_state: pubkeys[8],
            integration_acc_2: pubkeys[9],
            integration_acc_3: pubkeys[10],
            integration_acc_1: pubkeys[11],
            drift_spot_market_vault: pubkeys[12],
            mint: pubkeys[13],
            drift_program: pubkeys[14],
            token_program: pubkeys[15],
            system_program: pubkeys[16],
        }
    }
}
impl<'info> From<DriftDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.drift_oracle.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.signer_token_account.clone(),
            accounts.drift_state.clone(),
            accounts.integration_acc_2.clone(),
            accounts.integration_acc_3.clone(),
            accounts.integration_acc_1.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.mint.clone(),
            accounts.drift_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DRIFT_DEPOSIT_IX_ACCOUNTS_LEN]>
for DriftDepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DRIFT_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            drift_oracle: &arr[4],
            liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            signer_token_account: &arr[7],
            drift_state: &arr[8],
            integration_acc_2: &arr[9],
            integration_acc_3: &arr[10],
            integration_acc_1: &arr[11],
            drift_spot_market_vault: &arr[12],
            mint: &arr[13],
            drift_program: &arr[14],
            token_program: &arr[15],
            system_program: &arr[16],
        }
    }
}
pub const DRIFT_DEPOSIT_IX_DISCM: [u8; 8usize] = [252, 63, 250, 201, 98, 55, 130, 12];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftDepositIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftDepositIxData(pub DriftDepositIxArgs);
impl From<DriftDepositIxArgs> for DriftDepositIxData {
    fn from(args: DriftDepositIxArgs) -> Self {
        Self(args)
    }
}
impl DriftDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DriftDepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftDepositKeys,
    args: DriftDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DriftDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_deposit_ix(
    keys: DriftDepositKeys,
    args: DriftDepositIxArgs,
) -> std::io::Result<Instruction> {
    drift_deposit_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn drift_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftDepositAccounts<'_, '_>,
    args: DriftDepositIxArgs,
) -> ProgramResult {
    let keys: DriftDepositKeys = accounts.into();
    let ix = drift_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_deposit_invoke(
    accounts: DriftDepositAccounts<'_, '_>,
    args: DriftDepositIxArgs,
) -> ProgramResult {
    drift_deposit_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn drift_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftDepositAccounts<'_, '_>,
    args: DriftDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftDepositKeys = accounts.into();
    let ix = drift_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_deposit_invoke_signed(
    accounts: DriftDepositAccounts<'_, '_>,
    args: DriftDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_deposit_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_deposit_verify_account_keys(
    accounts: DriftDepositAccounts<'_, '_>,
    keys: DriftDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.drift_oracle.key, keys.drift_oracle),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.integration_acc_3.key, keys.integration_acc_3),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.mint.key, keys.mint),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_deposit_verify_writable_privileges<'me, 'info>(
    accounts: DriftDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.signer_token_account,
        accounts.integration_acc_2,
        accounts.integration_acc_3,
        accounts.integration_acc_1,
        accounts.drift_spot_market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_deposit_verify_signer_privileges<'me, 'info>(
    accounts: DriftDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_deposit_verify_account_privileges<'me, 'info>(
    accounts: DriftDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_deposit_verify_writable_privileges(accounts)?;
    drift_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_HARVEST_REWARD_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DriftHarvestRewardAccounts<'me, 'info> {
    pub bank: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub intermediary_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub integration_acc_3: &'me AccountInfo<'info>,
    pub harvest_drift_spot_market: &'me AccountInfo<'info>,
    pub harvest_drift_spot_market_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftHarvestRewardKeys {
    pub bank: Pubkey,
    pub fee_state: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub intermediary_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub drift_state: Pubkey,
    pub integration_acc_2: Pubkey,
    pub integration_acc_3: Pubkey,
    pub harvest_drift_spot_market: Pubkey,
    pub harvest_drift_spot_market_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub reward_mint: Pubkey,
    pub drift_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<DriftHarvestRewardAccounts<'_, '_>> for DriftHarvestRewardKeys {
    fn from(accounts: DriftHarvestRewardAccounts) -> Self {
        Self {
            bank: *accounts.bank.key,
            fee_state: *accounts.fee_state.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            intermediary_token_account: *accounts.intermediary_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            drift_state: *accounts.drift_state.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            integration_acc_3: *accounts.integration_acc_3.key,
            harvest_drift_spot_market: *accounts.harvest_drift_spot_market.key,
            harvest_drift_spot_market_vault: *accounts
                .harvest_drift_spot_market_vault
                .key,
            drift_signer: *accounts.drift_signer.key,
            reward_mint: *accounts.reward_mint.key,
            drift_program: *accounts.drift_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DriftHarvestRewardKeys>
for [AccountMeta; DRIFT_HARVEST_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftHarvestRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.intermediary_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_3,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.harvest_drift_spot_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.harvest_drift_spot_market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
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
impl From<[Pubkey; DRIFT_HARVEST_REWARD_IX_ACCOUNTS_LEN]> for DriftHarvestRewardKeys {
    fn from(pubkeys: [Pubkey; DRIFT_HARVEST_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank: pubkeys[0],
            fee_state: pubkeys[1],
            liquidity_vault_authority: pubkeys[2],
            intermediary_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            drift_state: pubkeys[5],
            integration_acc_2: pubkeys[6],
            integration_acc_3: pubkeys[7],
            harvest_drift_spot_market: pubkeys[8],
            harvest_drift_spot_market_vault: pubkeys[9],
            drift_signer: pubkeys[10],
            reward_mint: pubkeys[11],
            drift_program: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<DriftHarvestRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_HARVEST_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftHarvestRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.bank.clone(),
            accounts.fee_state.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.intermediary_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.drift_state.clone(),
            accounts.integration_acc_2.clone(),
            accounts.integration_acc_3.clone(),
            accounts.harvest_drift_spot_market.clone(),
            accounts.harvest_drift_spot_market_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.reward_mint.clone(),
            accounts.drift_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DRIFT_HARVEST_REWARD_IX_ACCOUNTS_LEN]>
for DriftHarvestRewardAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DRIFT_HARVEST_REWARD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bank: &arr[0],
            fee_state: &arr[1],
            liquidity_vault_authority: &arr[2],
            intermediary_token_account: &arr[3],
            destination_token_account: &arr[4],
            drift_state: &arr[5],
            integration_acc_2: &arr[6],
            integration_acc_3: &arr[7],
            harvest_drift_spot_market: &arr[8],
            harvest_drift_spot_market_vault: &arr[9],
            drift_signer: &arr[10],
            reward_mint: &arr[11],
            drift_program: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const DRIFT_HARVEST_REWARD_IX_DISCM: [u8; 8usize] = [
    167, 161, 240, 194, 138, 54, 87, 189,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DriftHarvestRewardIxData;
impl DriftHarvestRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_HARVEST_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_HARVEST_REWARD_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_harvest_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftHarvestRewardKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_HARVEST_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DriftHarvestRewardIxData.try_to_vec()?,
    })
}
pub fn drift_harvest_reward_ix(
    keys: DriftHarvestRewardKeys,
) -> std::io::Result<Instruction> {
    drift_harvest_reward_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn drift_harvest_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftHarvestRewardAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DriftHarvestRewardKeys = accounts.into();
    let ix = drift_harvest_reward_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_harvest_reward_invoke(
    accounts: DriftHarvestRewardAccounts<'_, '_>,
) -> ProgramResult {
    drift_harvest_reward_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn drift_harvest_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftHarvestRewardAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftHarvestRewardKeys = accounts.into();
    let ix = drift_harvest_reward_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_harvest_reward_invoke_signed(
    accounts: DriftHarvestRewardAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_harvest_reward_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn drift_harvest_reward_verify_account_keys(
    accounts: DriftHarvestRewardAccounts<'_, '_>,
    keys: DriftHarvestRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bank.key, keys.bank),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.intermediary_token_account.key, keys.intermediary_token_account),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.integration_acc_3.key, keys.integration_acc_3),
        (*accounts.harvest_drift_spot_market.key, keys.harvest_drift_spot_market),
        (
            *accounts.harvest_drift_spot_market_vault.key,
            keys.harvest_drift_spot_market_vault,
        ),
        (*accounts.drift_signer.key, keys.drift_signer),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_harvest_reward_verify_writable_privileges<'me, 'info>(
    accounts: DriftHarvestRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.intermediary_token_account,
        accounts.destination_token_account,
        accounts.integration_acc_2,
        accounts.integration_acc_3,
        accounts.harvest_drift_spot_market,
        accounts.harvest_drift_spot_market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_harvest_reward_verify_account_privileges<'me, 'info>(
    accounts: DriftHarvestRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_harvest_reward_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_INIT_USER_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct DriftInitUserAccounts<'me, 'info> {
    pub fee_payer: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub integration_acc_3: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub drift_oracle: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftInitUserKeys {
    pub fee_payer: Pubkey,
    pub signer_token_account: Pubkey,
    pub bank: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub mint: Pubkey,
    pub integration_acc_3: Pubkey,
    pub integration_acc_2: Pubkey,
    pub drift_state: Pubkey,
    pub integration_acc_1: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub drift_oracle: Pubkey,
    pub drift_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<DriftInitUserAccounts<'_, '_>> for DriftInitUserKeys {
    fn from(accounts: DriftInitUserAccounts) -> Self {
        Self {
            fee_payer: *accounts.fee_payer.key,
            signer_token_account: *accounts.signer_token_account.key,
            bank: *accounts.bank.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            mint: *accounts.mint.key,
            integration_acc_3: *accounts.integration_acc_3.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            drift_state: *accounts.drift_state.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            drift_oracle: *accounts.drift_oracle.key,
            drift_program: *accounts.drift_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DriftInitUserKeys> for [AccountMeta; DRIFT_INIT_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftInitUserKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_3,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRIFT_INIT_USER_IX_ACCOUNTS_LEN]> for DriftInitUserKeys {
    fn from(pubkeys: [Pubkey; DRIFT_INIT_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_payer: pubkeys[0],
            signer_token_account: pubkeys[1],
            bank: pubkeys[2],
            liquidity_vault_authority: pubkeys[3],
            liquidity_vault: pubkeys[4],
            mint: pubkeys[5],
            integration_acc_3: pubkeys[6],
            integration_acc_2: pubkeys[7],
            drift_state: pubkeys[8],
            integration_acc_1: pubkeys[9],
            drift_spot_market_vault: pubkeys[10],
            drift_oracle: pubkeys[11],
            drift_program: pubkeys[12],
            token_program: pubkeys[13],
            rent: pubkeys[14],
            system_program: pubkeys[15],
        }
    }
}
impl<'info> From<DriftInitUserAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_INIT_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftInitUserAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_payer.clone(),
            accounts.signer_token_account.clone(),
            accounts.bank.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.mint.clone(),
            accounts.integration_acc_3.clone(),
            accounts.integration_acc_2.clone(),
            accounts.drift_state.clone(),
            accounts.integration_acc_1.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.drift_oracle.clone(),
            accounts.drift_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DRIFT_INIT_USER_IX_ACCOUNTS_LEN]>
for DriftInitUserAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DRIFT_INIT_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_payer: &arr[0],
            signer_token_account: &arr[1],
            bank: &arr[2],
            liquidity_vault_authority: &arr[3],
            liquidity_vault: &arr[4],
            mint: &arr[5],
            integration_acc_3: &arr[6],
            integration_acc_2: &arr[7],
            drift_state: &arr[8],
            integration_acc_1: &arr[9],
            drift_spot_market_vault: &arr[10],
            drift_oracle: &arr[11],
            drift_program: &arr[12],
            token_program: &arr[13],
            rent: &arr[14],
            system_program: &arr[15],
        }
    }
}
pub const DRIFT_INIT_USER_IX_DISCM: [u8; 8usize] = [29, 18, 236, 190, 29, 254, 114, 169];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftInitUserIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftInitUserIxData(pub DriftInitUserIxArgs);
impl From<DriftInitUserIxArgs> for DriftInitUserIxData {
    fn from(args: DriftInitUserIxArgs) -> Self {
        Self(args)
    }
}
impl DriftInitUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_INIT_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DriftInitUserIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_INIT_USER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_init_user_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftInitUserKeys,
    args: DriftInitUserIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_INIT_USER_IX_ACCOUNTS_LEN] = keys.into();
    let data: DriftInitUserIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_init_user_ix(
    keys: DriftInitUserKeys,
    args: DriftInitUserIxArgs,
) -> std::io::Result<Instruction> {
    drift_init_user_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn drift_init_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftInitUserAccounts<'_, '_>,
    args: DriftInitUserIxArgs,
) -> ProgramResult {
    let keys: DriftInitUserKeys = accounts.into();
    let ix = drift_init_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_init_user_invoke(
    accounts: DriftInitUserAccounts<'_, '_>,
    args: DriftInitUserIxArgs,
) -> ProgramResult {
    drift_init_user_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn drift_init_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftInitUserAccounts<'_, '_>,
    args: DriftInitUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftInitUserKeys = accounts.into();
    let ix = drift_init_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_init_user_invoke_signed(
    accounts: DriftInitUserAccounts<'_, '_>,
    args: DriftInitUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_init_user_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_init_user_verify_account_keys(
    accounts: DriftInitUserAccounts<'_, '_>,
    keys: DriftInitUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.bank.key, keys.bank),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.mint.key, keys.mint),
        (*accounts.integration_acc_3.key, keys.integration_acc_3),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.drift_oracle.key, keys.drift_oracle),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_init_user_verify_writable_privileges<'me, 'info>(
    accounts: DriftInitUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_payer,
        accounts.signer_token_account,
        accounts.liquidity_vault,
        accounts.mint,
        accounts.integration_acc_3,
        accounts.integration_acc_2,
        accounts.drift_state,
        accounts.integration_acc_1,
        accounts.drift_spot_market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_init_user_verify_signer_privileges<'me, 'info>(
    accounts: DriftInitUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_init_user_verify_account_privileges<'me, 'info>(
    accounts: DriftInitUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_init_user_verify_writable_privileges(accounts)?;
    drift_init_user_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_WITHDRAW_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct DriftWithdrawAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub drift_oracle: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub integration_acc_3: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub drift_reward_oracle: &'me AccountInfo<'info>,
    pub drift_reward_spot_market: &'me AccountInfo<'info>,
    pub drift_reward_mint: &'me AccountInfo<'info>,
    pub drift_reward_oracle_2: &'me AccountInfo<'info>,
    pub drift_reward_spot_market_2: &'me AccountInfo<'info>,
    pub drift_reward_mint_2: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftWithdrawKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub drift_oracle: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub destination_token_account: Pubkey,
    pub drift_state: Pubkey,
    pub integration_acc_2: Pubkey,
    pub integration_acc_3: Pubkey,
    pub integration_acc_1: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub drift_reward_oracle: Pubkey,
    pub drift_reward_spot_market: Pubkey,
    pub drift_reward_mint: Pubkey,
    pub drift_reward_oracle_2: Pubkey,
    pub drift_reward_spot_market_2: Pubkey,
    pub drift_reward_mint_2: Pubkey,
    pub drift_signer: Pubkey,
    pub mint: Pubkey,
    pub drift_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DriftWithdrawAccounts<'_, '_>> for DriftWithdrawKeys {
    fn from(accounts: DriftWithdrawAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            drift_oracle: *accounts.drift_oracle.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            destination_token_account: *accounts.destination_token_account.key,
            drift_state: *accounts.drift_state.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            integration_acc_3: *accounts.integration_acc_3.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            drift_reward_oracle: *accounts.drift_reward_oracle.key,
            drift_reward_spot_market: *accounts.drift_reward_spot_market.key,
            drift_reward_mint: *accounts.drift_reward_mint.key,
            drift_reward_oracle_2: *accounts.drift_reward_oracle_2.key,
            drift_reward_spot_market_2: *accounts.drift_reward_spot_market_2.key,
            drift_reward_mint_2: *accounts.drift_reward_mint_2.key,
            drift_signer: *accounts.drift_signer.key,
            mint: *accounts.mint.key,
            drift_program: *accounts.drift_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DriftWithdrawKeys> for [AccountMeta; DRIFT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_3,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_reward_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_reward_spot_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_reward_oracle_2,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_reward_spot_market_2,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_reward_mint_2,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
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
impl From<[Pubkey; DRIFT_WITHDRAW_IX_ACCOUNTS_LEN]> for DriftWithdrawKeys {
    fn from(pubkeys: [Pubkey; DRIFT_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            drift_oracle: pubkeys[4],
            liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            destination_token_account: pubkeys[7],
            drift_state: pubkeys[8],
            integration_acc_2: pubkeys[9],
            integration_acc_3: pubkeys[10],
            integration_acc_1: pubkeys[11],
            drift_spot_market_vault: pubkeys[12],
            drift_reward_oracle: pubkeys[13],
            drift_reward_spot_market: pubkeys[14],
            drift_reward_mint: pubkeys[15],
            drift_reward_oracle_2: pubkeys[16],
            drift_reward_spot_market_2: pubkeys[17],
            drift_reward_mint_2: pubkeys[18],
            drift_signer: pubkeys[19],
            mint: pubkeys[20],
            drift_program: pubkeys[21],
            token_program: pubkeys[22],
            system_program: pubkeys[23],
        }
    }
}
impl<'info> From<DriftWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.drift_oracle.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.destination_token_account.clone(),
            accounts.drift_state.clone(),
            accounts.integration_acc_2.clone(),
            accounts.integration_acc_3.clone(),
            accounts.integration_acc_1.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.drift_reward_oracle.clone(),
            accounts.drift_reward_spot_market.clone(),
            accounts.drift_reward_mint.clone(),
            accounts.drift_reward_oracle_2.clone(),
            accounts.drift_reward_spot_market_2.clone(),
            accounts.drift_reward_mint_2.clone(),
            accounts.drift_signer.clone(),
            accounts.mint.clone(),
            accounts.drift_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DRIFT_WITHDRAW_IX_ACCOUNTS_LEN]>
for DriftWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DRIFT_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            drift_oracle: &arr[4],
            liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            destination_token_account: &arr[7],
            drift_state: &arr[8],
            integration_acc_2: &arr[9],
            integration_acc_3: &arr[10],
            integration_acc_1: &arr[11],
            drift_spot_market_vault: &arr[12],
            drift_reward_oracle: &arr[13],
            drift_reward_spot_market: &arr[14],
            drift_reward_mint: &arr[15],
            drift_reward_oracle_2: &arr[16],
            drift_reward_spot_market_2: &arr[17],
            drift_reward_mint_2: &arr[18],
            drift_signer: &arr[19],
            mint: &arr[20],
            drift_program: &arr[21],
            token_program: &arr[22],
            system_program: &arr[23],
        }
    }
}
pub const DRIFT_WITHDRAW_IX_DISCM: [u8; 8usize] = [86, 59, 186, 123, 183, 181, 234, 137];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftWithdrawIxArgs {
    pub amount: u64,
    pub withdraw_all: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftWithdrawIxData(pub DriftWithdrawIxArgs);
impl From<DriftWithdrawIxArgs> for DriftWithdrawIxData {
    fn from(args: DriftWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl DriftWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DriftWithdrawIxArgs {
                amount,
                withdraw_all,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftWithdrawKeys,
    args: DriftWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: DriftWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_withdraw_ix(
    keys: DriftWithdrawKeys,
    args: DriftWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    drift_withdraw_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn drift_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftWithdrawAccounts<'_, '_>,
    args: DriftWithdrawIxArgs,
) -> ProgramResult {
    let keys: DriftWithdrawKeys = accounts.into();
    let ix = drift_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_withdraw_invoke(
    accounts: DriftWithdrawAccounts<'_, '_>,
    args: DriftWithdrawIxArgs,
) -> ProgramResult {
    drift_withdraw_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn drift_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftWithdrawAccounts<'_, '_>,
    args: DriftWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftWithdrawKeys = accounts.into();
    let ix = drift_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_withdraw_invoke_signed(
    accounts: DriftWithdrawAccounts<'_, '_>,
    args: DriftWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_withdraw_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_withdraw_verify_account_keys(
    accounts: DriftWithdrawAccounts<'_, '_>,
    keys: DriftWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.drift_oracle.key, keys.drift_oracle),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.integration_acc_3.key, keys.integration_acc_3),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.drift_reward_oracle.key, keys.drift_reward_oracle),
        (*accounts.drift_reward_spot_market.key, keys.drift_reward_spot_market),
        (*accounts.drift_reward_mint.key, keys.drift_reward_mint),
        (*accounts.drift_reward_oracle_2.key, keys.drift_reward_oracle_2),
        (*accounts.drift_reward_spot_market_2.key, keys.drift_reward_spot_market_2),
        (*accounts.drift_reward_mint_2.key, keys.drift_reward_mint_2),
        (*accounts.drift_signer.key, keys.drift_signer),
        (*accounts.mint.key, keys.mint),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: DriftWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.destination_token_account,
        accounts.integration_acc_2,
        accounts.integration_acc_3,
        accounts.integration_acc_1,
        accounts.drift_spot_market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: DriftWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_withdraw_verify_account_privileges<'me, 'info>(
    accounts: DriftWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_withdraw_verify_writable_privileges(accounts)?;
    drift_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EDIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct EditGlobalFeeStateAccounts<'me, 'info> {
    pub global_fee_admin: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EditGlobalFeeStateKeys {
    pub global_fee_admin: Pubkey,
    pub fee_state: Pubkey,
}
impl From<EditGlobalFeeStateAccounts<'_, '_>> for EditGlobalFeeStateKeys {
    fn from(accounts: EditGlobalFeeStateAccounts) -> Self {
        Self {
            global_fee_admin: *accounts.global_fee_admin.key,
            fee_state: *accounts.fee_state.key,
        }
    }
}
impl From<EditGlobalFeeStateKeys>
for [AccountMeta; EDIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: EditGlobalFeeStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_fee_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; EDIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN]> for EditGlobalFeeStateKeys {
    fn from(pubkeys: [Pubkey; EDIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_fee_admin: pubkeys[0],
            fee_state: pubkeys[1],
        }
    }
}
impl<'info> From<EditGlobalFeeStateAccounts<'_, 'info>>
for [AccountInfo<'info>; EDIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: EditGlobalFeeStateAccounts<'_, 'info>) -> Self {
        [accounts.global_fee_admin.clone(), accounts.fee_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EDIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN]>
for EditGlobalFeeStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EDIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global_fee_admin: &arr[0],
            fee_state: &arr[1],
        }
    }
}
pub const EDIT_GLOBAL_FEE_STATE_IX_DISCM: [u8; 8usize] = [
    52, 62, 35, 129, 93, 69, 165, 202,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EditGlobalFeeStateIxArgs {
    pub admin: Pubkey,
    pub fee_wallet: Pubkey,
    pub bank_init_flat_sol_fee: u32,
    pub liquidation_flat_sol_fee: u32,
    pub order_init_flat_sol_fee: u32,
    pub program_fee_fixed: WrappedI80F48,
    pub program_fee_rate: WrappedI80F48,
    pub liquidation_max_fee: WrappedI80F48,
    pub order_execution_max_fee: WrappedI80F48,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EditGlobalFeeStateIxData(pub EditGlobalFeeStateIxArgs);
impl From<EditGlobalFeeStateIxArgs> for EditGlobalFeeStateIxData {
    fn from(args: EditGlobalFeeStateIxArgs) -> Self {
        Self(args)
    }
}
impl EditGlobalFeeStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EDIT_GLOBAL_FEE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let order_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let program_fee_fixed = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let program_fee_rate = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liquidation_max_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let order_execution_max_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        Ok(
            Self(EditGlobalFeeStateIxArgs {
                admin,
                fee_wallet,
                bank_init_flat_sol_fee,
                liquidation_flat_sol_fee,
                order_init_flat_sol_fee,
                program_fee_fixed,
                program_fee_rate,
                liquidation_max_fee,
                order_execution_max_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EDIT_GLOBAL_FEE_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bank_init_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidation_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.order_init_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.program_fee_fixed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.program_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidation_max_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.order_execution_max_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn edit_global_fee_state_ix_with_program_id(
    program_id: Pubkey,
    keys: EditGlobalFeeStateKeys,
    args: EditGlobalFeeStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EDIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: EditGlobalFeeStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn edit_global_fee_state_ix(
    keys: EditGlobalFeeStateKeys,
    args: EditGlobalFeeStateIxArgs,
) -> std::io::Result<Instruction> {
    edit_global_fee_state_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn edit_global_fee_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EditGlobalFeeStateAccounts<'_, '_>,
    args: EditGlobalFeeStateIxArgs,
) -> ProgramResult {
    let keys: EditGlobalFeeStateKeys = accounts.into();
    let ix = edit_global_fee_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn edit_global_fee_state_invoke(
    accounts: EditGlobalFeeStateAccounts<'_, '_>,
    args: EditGlobalFeeStateIxArgs,
) -> ProgramResult {
    edit_global_fee_state_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn edit_global_fee_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EditGlobalFeeStateAccounts<'_, '_>,
    args: EditGlobalFeeStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EditGlobalFeeStateKeys = accounts.into();
    let ix = edit_global_fee_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn edit_global_fee_state_invoke_signed(
    accounts: EditGlobalFeeStateAccounts<'_, '_>,
    args: EditGlobalFeeStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    edit_global_fee_state_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn edit_global_fee_state_verify_account_keys(
    accounts: EditGlobalFeeStateAccounts<'_, '_>,
    keys: EditGlobalFeeStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global_fee_admin.key, keys.global_fee_admin),
        (*accounts.fee_state.key, keys.fee_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn edit_global_fee_state_verify_writable_privileges<'me, 'info>(
    accounts: EditGlobalFeeStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn edit_global_fee_state_verify_signer_privileges<'me, 'info>(
    accounts: EditGlobalFeeStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_fee_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn edit_global_fee_state_verify_account_privileges<'me, 'info>(
    accounts: EditGlobalFeeStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    edit_global_fee_state_verify_writable_privileges(accounts)?;
    edit_global_fee_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EDIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct EditStakedSettingsAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub staked_settings: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EditStakedSettingsKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
    pub staked_settings: Pubkey,
}
impl From<EditStakedSettingsAccounts<'_, '_>> for EditStakedSettingsKeys {
    fn from(accounts: EditStakedSettingsAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
            staked_settings: *accounts.staked_settings.key,
        }
    }
}
impl From<EditStakedSettingsKeys>
for [AccountMeta; EDIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN] {
    fn from(keys: EditStakedSettingsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staked_settings,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; EDIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN]> for EditStakedSettingsKeys {
    fn from(pubkeys: [Pubkey; EDIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
            staked_settings: pubkeys[2],
        }
    }
}
impl<'info> From<EditStakedSettingsAccounts<'_, 'info>>
for [AccountInfo<'info>; EDIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN] {
    fn from(accounts: EditStakedSettingsAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.admin.clone(),
            accounts.staked_settings.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EDIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN]>
for EditStakedSettingsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EDIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
            staked_settings: &arr[2],
        }
    }
}
pub const EDIT_STAKED_SETTINGS_IX_DISCM: [u8; 8usize] = [
    11, 108, 215, 87, 240, 9, 66, 241,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EditStakedSettingsIxArgs {
    pub settings: StakedSettingsEditConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EditStakedSettingsIxData(pub EditStakedSettingsIxArgs);
impl From<EditStakedSettingsIxArgs> for EditStakedSettingsIxData {
    fn from(args: EditStakedSettingsIxArgs) -> Self {
        Self(args)
    }
}
impl EditStakedSettingsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EDIT_STAKED_SETTINGS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let settings = if reader.is_empty() {
            Default::default()
        } else {
            <StakedSettingsEditConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(EditStakedSettingsIxArgs {
                settings,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EDIT_STAKED_SETTINGS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.settings, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn edit_staked_settings_ix_with_program_id(
    program_id: Pubkey,
    keys: EditStakedSettingsKeys,
    args: EditStakedSettingsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EDIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN] = keys.into();
    let data: EditStakedSettingsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn edit_staked_settings_ix(
    keys: EditStakedSettingsKeys,
    args: EditStakedSettingsIxArgs,
) -> std::io::Result<Instruction> {
    edit_staked_settings_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn edit_staked_settings_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EditStakedSettingsAccounts<'_, '_>,
    args: EditStakedSettingsIxArgs,
) -> ProgramResult {
    let keys: EditStakedSettingsKeys = accounts.into();
    let ix = edit_staked_settings_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn edit_staked_settings_invoke(
    accounts: EditStakedSettingsAccounts<'_, '_>,
    args: EditStakedSettingsIxArgs,
) -> ProgramResult {
    edit_staked_settings_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn edit_staked_settings_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EditStakedSettingsAccounts<'_, '_>,
    args: EditStakedSettingsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EditStakedSettingsKeys = accounts.into();
    let ix = edit_staked_settings_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn edit_staked_settings_invoke_signed(
    accounts: EditStakedSettingsAccounts<'_, '_>,
    args: EditStakedSettingsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    edit_staked_settings_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn edit_staked_settings_verify_account_keys(
    accounts: EditStakedSettingsAccounts<'_, '_>,
    keys: EditStakedSettingsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
        (*accounts.staked_settings.key, keys.staked_settings),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn edit_staked_settings_verify_writable_privileges<'me, 'info>(
    accounts: EditStakedSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.staked_settings] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn edit_staked_settings_verify_signer_privileges<'me, 'info>(
    accounts: EditStakedSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn edit_staked_settings_verify_account_privileges<'me, 'info>(
    accounts: EditStakedSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    edit_staked_settings_verify_writable_privileges(accounts)?;
    edit_staked_settings_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const END_DELEVERAGE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct EndDeleverageAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub liquidation_record: &'me AccountInfo<'info>,
    pub group: &'me AccountInfo<'info>,
    pub risk_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EndDeleverageKeys {
    pub marginfi_account: Pubkey,
    pub liquidation_record: Pubkey,
    pub group: Pubkey,
    pub risk_admin: Pubkey,
}
impl From<EndDeleverageAccounts<'_, '_>> for EndDeleverageKeys {
    fn from(accounts: EndDeleverageAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            liquidation_record: *accounts.liquidation_record.key,
            group: *accounts.group.key,
            risk_admin: *accounts.risk_admin.key,
        }
    }
}
impl From<EndDeleverageKeys> for [AccountMeta; END_DELEVERAGE_IX_ACCOUNTS_LEN] {
    fn from(keys: EndDeleverageKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.risk_admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; END_DELEVERAGE_IX_ACCOUNTS_LEN]> for EndDeleverageKeys {
    fn from(pubkeys: [Pubkey; END_DELEVERAGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            liquidation_record: pubkeys[1],
            group: pubkeys[2],
            risk_admin: pubkeys[3],
        }
    }
}
impl<'info> From<EndDeleverageAccounts<'_, 'info>>
for [AccountInfo<'info>; END_DELEVERAGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: EndDeleverageAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.liquidation_record.clone(),
            accounts.group.clone(),
            accounts.risk_admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; END_DELEVERAGE_IX_ACCOUNTS_LEN]>
for EndDeleverageAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; END_DELEVERAGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: &arr[0],
            liquidation_record: &arr[1],
            group: &arr[2],
            risk_admin: &arr[3],
        }
    }
}
pub const END_DELEVERAGE_IX_DISCM: [u8; 8usize] = [
    114, 14, 250, 143, 252, 104, 214, 209,
];
#[derive(Clone, Debug, PartialEq)]
pub struct EndDeleverageIxData;
impl EndDeleverageIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != END_DELEVERAGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&END_DELEVERAGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn end_deleverage_ix_with_program_id(
    program_id: Pubkey,
    keys: EndDeleverageKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; END_DELEVERAGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: EndDeleverageIxData.try_to_vec()?,
    })
}
pub fn end_deleverage_ix(keys: EndDeleverageKeys) -> std::io::Result<Instruction> {
    end_deleverage_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn end_deleverage_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EndDeleverageAccounts<'_, '_>,
) -> ProgramResult {
    let keys: EndDeleverageKeys = accounts.into();
    let ix = end_deleverage_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn end_deleverage_invoke(accounts: EndDeleverageAccounts<'_, '_>) -> ProgramResult {
    end_deleverage_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn end_deleverage_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EndDeleverageAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EndDeleverageKeys = accounts.into();
    let ix = end_deleverage_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn end_deleverage_invoke_signed(
    accounts: EndDeleverageAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    end_deleverage_invoke_signed_with_program_id(MARGINFI_PROGRAM_ID, accounts, seeds)
}
pub fn end_deleverage_verify_account_keys(
    accounts: EndDeleverageAccounts<'_, '_>,
    keys: EndDeleverageKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.liquidation_record.key, keys.liquidation_record),
        (*accounts.group.key, keys.group),
        (*accounts.risk_admin.key, keys.risk_admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn end_deleverage_verify_writable_privileges<'me, 'info>(
    accounts: EndDeleverageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account, accounts.liquidation_record] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn end_deleverage_verify_signer_privileges<'me, 'info>(
    accounts: EndDeleverageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.risk_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn end_deleverage_verify_account_privileges<'me, 'info>(
    accounts: EndDeleverageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    end_deleverage_verify_writable_privileges(accounts)?;
    end_deleverage_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const END_LIQUIDATION_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct EndLiquidationAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub liquidation_record: &'me AccountInfo<'info>,
    pub liquidation_receiver: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub global_fee_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EndLiquidationKeys {
    pub marginfi_account: Pubkey,
    pub liquidation_record: Pubkey,
    pub liquidation_receiver: Pubkey,
    pub fee_state: Pubkey,
    pub global_fee_wallet: Pubkey,
    pub system_program: Pubkey,
}
impl From<EndLiquidationAccounts<'_, '_>> for EndLiquidationKeys {
    fn from(accounts: EndLiquidationAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            liquidation_record: *accounts.liquidation_record.key,
            liquidation_receiver: *accounts.liquidation_receiver.key,
            fee_state: *accounts.fee_state.key,
            global_fee_wallet: *accounts.global_fee_wallet.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<EndLiquidationKeys> for [AccountMeta; END_LIQUIDATION_IX_ACCOUNTS_LEN] {
    fn from(keys: EndLiquidationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_receiver,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_fee_wallet,
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
impl From<[Pubkey; END_LIQUIDATION_IX_ACCOUNTS_LEN]> for EndLiquidationKeys {
    fn from(pubkeys: [Pubkey; END_LIQUIDATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            liquidation_record: pubkeys[1],
            liquidation_receiver: pubkeys[2],
            fee_state: pubkeys[3],
            global_fee_wallet: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<EndLiquidationAccounts<'_, 'info>>
for [AccountInfo<'info>; END_LIQUIDATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: EndLiquidationAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.liquidation_record.clone(),
            accounts.liquidation_receiver.clone(),
            accounts.fee_state.clone(),
            accounts.global_fee_wallet.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; END_LIQUIDATION_IX_ACCOUNTS_LEN]>
for EndLiquidationAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; END_LIQUIDATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: &arr[0],
            liquidation_record: &arr[1],
            liquidation_receiver: &arr[2],
            fee_state: &arr[3],
            global_fee_wallet: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const END_LIQUIDATION_IX_DISCM: [u8; 8usize] = [110, 11, 244, 54, 229, 181, 22, 184];
#[derive(Clone, Debug, PartialEq)]
pub struct EndLiquidationIxData;
impl EndLiquidationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != END_LIQUIDATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&END_LIQUIDATION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn end_liquidation_ix_with_program_id(
    program_id: Pubkey,
    keys: EndLiquidationKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; END_LIQUIDATION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: EndLiquidationIxData.try_to_vec()?,
    })
}
pub fn end_liquidation_ix(keys: EndLiquidationKeys) -> std::io::Result<Instruction> {
    end_liquidation_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn end_liquidation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EndLiquidationAccounts<'_, '_>,
) -> ProgramResult {
    let keys: EndLiquidationKeys = accounts.into();
    let ix = end_liquidation_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn end_liquidation_invoke(
    accounts: EndLiquidationAccounts<'_, '_>,
) -> ProgramResult {
    end_liquidation_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn end_liquidation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EndLiquidationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EndLiquidationKeys = accounts.into();
    let ix = end_liquidation_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn end_liquidation_invoke_signed(
    accounts: EndLiquidationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    end_liquidation_invoke_signed_with_program_id(MARGINFI_PROGRAM_ID, accounts, seeds)
}
pub fn end_liquidation_verify_account_keys(
    accounts: EndLiquidationAccounts<'_, '_>,
    keys: EndLiquidationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.liquidation_record.key, keys.liquidation_record),
        (*accounts.liquidation_receiver.key, keys.liquidation_receiver),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.global_fee_wallet.key, keys.global_fee_wallet),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn end_liquidation_verify_writable_privileges<'me, 'info>(
    accounts: EndLiquidationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.liquidation_record,
        accounts.liquidation_receiver,
        accounts.global_fee_wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn end_liquidation_verify_signer_privileges<'me, 'info>(
    accounts: EndLiquidationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.liquidation_receiver] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn end_liquidation_verify_account_privileges<'me, 'info>(
    accounts: EndLiquidationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    end_liquidation_verify_writable_privileges(accounts)?;
    end_liquidation_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_BANK_METADATA_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitBankMetadataAccounts<'me, 'info> {
    pub bank: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitBankMetadataKeys {
    pub bank: Pubkey,
    pub fee_payer: Pubkey,
    pub metadata: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitBankMetadataAccounts<'_, '_>> for InitBankMetadataKeys {
    fn from(accounts: InitBankMetadataAccounts) -> Self {
        Self {
            bank: *accounts.bank.key,
            fee_payer: *accounts.fee_payer.key,
            metadata: *accounts.metadata.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitBankMetadataKeys> for [AccountMeta; INIT_BANK_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitBankMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata,
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
impl From<[Pubkey; INIT_BANK_METADATA_IX_ACCOUNTS_LEN]> for InitBankMetadataKeys {
    fn from(pubkeys: [Pubkey; INIT_BANK_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank: pubkeys[0],
            fee_payer: pubkeys[1],
            metadata: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitBankMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_BANK_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitBankMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.bank.clone(),
            accounts.fee_payer.clone(),
            accounts.metadata.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_BANK_METADATA_IX_ACCOUNTS_LEN]>
for InitBankMetadataAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_BANK_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank: &arr[0],
            fee_payer: &arr[1],
            metadata: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INIT_BANK_METADATA_IX_DISCM: [u8; 8usize] = [
    94, 239, 50, 136, 137, 204, 254, 213,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitBankMetadataIxData;
impl InitBankMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_BANK_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_BANK_METADATA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_bank_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: InitBankMetadataKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_BANK_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitBankMetadataIxData.try_to_vec()?,
    })
}
pub fn init_bank_metadata_ix(
    keys: InitBankMetadataKeys,
) -> std::io::Result<Instruction> {
    init_bank_metadata_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn init_bank_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitBankMetadataAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitBankMetadataKeys = accounts.into();
    let ix = init_bank_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_bank_metadata_invoke(
    accounts: InitBankMetadataAccounts<'_, '_>,
) -> ProgramResult {
    init_bank_metadata_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn init_bank_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitBankMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitBankMetadataKeys = accounts.into();
    let ix = init_bank_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_bank_metadata_invoke_signed(
    accounts: InitBankMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_bank_metadata_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn init_bank_metadata_verify_account_keys(
    accounts: InitBankMetadataAccounts<'_, '_>,
    keys: InitBankMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bank.key, keys.bank),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_bank_metadata_verify_writable_privileges<'me, 'info>(
    accounts: InitBankMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_payer, accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_bank_metadata_verify_signer_privileges<'me, 'info>(
    accounts: InitBankMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_bank_metadata_verify_account_privileges<'me, 'info>(
    accounts: InitBankMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_bank_metadata_verify_writable_privileges(accounts)?;
    init_bank_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitGlobalFeeStateAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitGlobalFeeStateKeys {
    pub payer: Pubkey,
    pub fee_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitGlobalFeeStateAccounts<'_, '_>> for InitGlobalFeeStateKeys {
    fn from(accounts: InitGlobalFeeStateAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            fee_state: *accounts.fee_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitGlobalFeeStateKeys>
for [AccountMeta; INIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitGlobalFeeStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
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
impl From<[Pubkey; INIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN]> for InitGlobalFeeStateKeys {
    fn from(pubkeys: [Pubkey; INIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            fee_state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitGlobalFeeStateAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitGlobalFeeStateAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.fee_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN]>
for InitGlobalFeeStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            fee_state: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INIT_GLOBAL_FEE_STATE_IX_DISCM: [u8; 8usize] = [
    82, 48, 247, 59, 220, 109, 231, 44,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitGlobalFeeStateIxArgs {
    pub admin: Pubkey,
    pub fee_wallet: Pubkey,
    pub bank_init_flat_sol_fee: u32,
    pub liquidation_flat_sol_fee: u32,
    pub order_init_flat_sol_fee: u32,
    pub program_fee_fixed: WrappedI80F48,
    pub program_fee_rate: WrappedI80F48,
    pub liquidation_max_fee: WrappedI80F48,
    pub order_execution_max_fee: WrappedI80F48,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitGlobalFeeStateIxData(pub InitGlobalFeeStateIxArgs);
impl From<InitGlobalFeeStateIxArgs> for InitGlobalFeeStateIxData {
    fn from(args: InitGlobalFeeStateIxArgs) -> Self {
        Self(args)
    }
}
impl InitGlobalFeeStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_GLOBAL_FEE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let order_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let program_fee_fixed = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let program_fee_rate = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liquidation_max_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let order_execution_max_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitGlobalFeeStateIxArgs {
                admin,
                fee_wallet,
                bank_init_flat_sol_fee,
                liquidation_flat_sol_fee,
                order_init_flat_sol_fee,
                program_fee_fixed,
                program_fee_rate,
                liquidation_max_fee,
                order_execution_max_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_GLOBAL_FEE_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bank_init_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidation_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.order_init_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.program_fee_fixed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.program_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidation_max_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.order_execution_max_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_global_fee_state_ix_with_program_id(
    program_id: Pubkey,
    keys: InitGlobalFeeStateKeys,
    args: InitGlobalFeeStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_GLOBAL_FEE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitGlobalFeeStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_global_fee_state_ix(
    keys: InitGlobalFeeStateKeys,
    args: InitGlobalFeeStateIxArgs,
) -> std::io::Result<Instruction> {
    init_global_fee_state_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn init_global_fee_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitGlobalFeeStateAccounts<'_, '_>,
    args: InitGlobalFeeStateIxArgs,
) -> ProgramResult {
    let keys: InitGlobalFeeStateKeys = accounts.into();
    let ix = init_global_fee_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_global_fee_state_invoke(
    accounts: InitGlobalFeeStateAccounts<'_, '_>,
    args: InitGlobalFeeStateIxArgs,
) -> ProgramResult {
    init_global_fee_state_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn init_global_fee_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitGlobalFeeStateAccounts<'_, '_>,
    args: InitGlobalFeeStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitGlobalFeeStateKeys = accounts.into();
    let ix = init_global_fee_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_global_fee_state_invoke_signed(
    accounts: InitGlobalFeeStateAccounts<'_, '_>,
    args: InitGlobalFeeStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_global_fee_state_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_global_fee_state_verify_account_keys(
    accounts: InitGlobalFeeStateAccounts<'_, '_>,
    keys: InitGlobalFeeStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_global_fee_state_verify_writable_privileges<'me, 'info>(
    accounts: InitGlobalFeeStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.fee_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_global_fee_state_verify_signer_privileges<'me, 'info>(
    accounts: InitGlobalFeeStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_global_fee_state_verify_account_privileges<'me, 'info>(
    accounts: InitGlobalFeeStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_global_fee_state_verify_writable_privileges(accounts)?;
    init_global_fee_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitStakedSettingsAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub staked_settings: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitStakedSettingsKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
    pub fee_payer: Pubkey,
    pub staked_settings: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitStakedSettingsAccounts<'_, '_>> for InitStakedSettingsKeys {
    fn from(accounts: InitStakedSettingsAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
            fee_payer: *accounts.fee_payer.key,
            staked_settings: *accounts.staked_settings.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitStakedSettingsKeys>
for [AccountMeta; INIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitStakedSettingsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staked_settings,
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
impl From<[Pubkey; INIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN]> for InitStakedSettingsKeys {
    fn from(pubkeys: [Pubkey; INIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
            fee_payer: pubkeys[2],
            staked_settings: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitStakedSettingsAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitStakedSettingsAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.admin.clone(),
            accounts.fee_payer.clone(),
            accounts.staked_settings.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN]>
for InitStakedSettingsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
            fee_payer: &arr[2],
            staked_settings: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INIT_STAKED_SETTINGS_IX_DISCM: [u8; 8usize] = [
    52, 35, 149, 44, 69, 86, 69, 80,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitStakedSettingsIxArgs {
    pub settings: StakedSettingsConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitStakedSettingsIxData(pub InitStakedSettingsIxArgs);
impl From<InitStakedSettingsIxArgs> for InitStakedSettingsIxData {
    fn from(args: InitStakedSettingsIxArgs) -> Self {
        Self(args)
    }
}
impl InitStakedSettingsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_STAKED_SETTINGS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let settings = if reader.is_empty() {
            Default::default()
        } else {
            <StakedSettingsConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitStakedSettingsIxArgs {
                settings,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_STAKED_SETTINGS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.settings, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_staked_settings_ix_with_program_id(
    program_id: Pubkey,
    keys: InitStakedSettingsKeys,
    args: InitStakedSettingsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_STAKED_SETTINGS_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitStakedSettingsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_staked_settings_ix(
    keys: InitStakedSettingsKeys,
    args: InitStakedSettingsIxArgs,
) -> std::io::Result<Instruction> {
    init_staked_settings_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn init_staked_settings_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitStakedSettingsAccounts<'_, '_>,
    args: InitStakedSettingsIxArgs,
) -> ProgramResult {
    let keys: InitStakedSettingsKeys = accounts.into();
    let ix = init_staked_settings_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_staked_settings_invoke(
    accounts: InitStakedSettingsAccounts<'_, '_>,
    args: InitStakedSettingsIxArgs,
) -> ProgramResult {
    init_staked_settings_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn init_staked_settings_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitStakedSettingsAccounts<'_, '_>,
    args: InitStakedSettingsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitStakedSettingsKeys = accounts.into();
    let ix = init_staked_settings_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_staked_settings_invoke_signed(
    accounts: InitStakedSettingsAccounts<'_, '_>,
    args: InitStakedSettingsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_staked_settings_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_staked_settings_verify_account_keys(
    accounts: InitStakedSettingsAccounts<'_, '_>,
    keys: InitStakedSettingsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.staked_settings.key, keys.staked_settings),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_staked_settings_verify_writable_privileges<'me, 'info>(
    accounts: InitStakedSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_payer, accounts.staked_settings] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_staked_settings_verify_signer_privileges<'me, 'info>(
    accounts: InitStakedSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_staked_settings_verify_account_privileges<'me, 'info>(
    accounts: InitStakedSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_staked_settings_verify_writable_privileges(accounts)?;
    init_staked_settings_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const JUPLEND_DEPOSIT_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct JuplendDepositAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub juplend_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JuplendDepositKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub signer_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub mint: Pubkey,
    pub integration_acc_1: Pubkey,
    pub f_token_mint: Pubkey,
    pub integration_acc_2: Pubkey,
    pub lending_admin: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub juplend_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<JuplendDepositAccounts<'_, '_>> for JuplendDepositKeys {
    fn from(accounts: JuplendDepositAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            signer_token_account: *accounts.signer_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            mint: *accounts.mint.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            f_token_mint: *accounts.f_token_mint.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            lending_admin: *accounts.lending_admin.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            juplend_program: *accounts.juplend_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<JuplendDepositKeys> for [AccountMeta; JUPLEND_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: JuplendDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.juplend_program,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; JUPLEND_DEPOSIT_IX_ACCOUNTS_LEN]> for JuplendDepositKeys {
    fn from(pubkeys: [Pubkey; JUPLEND_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            signer_token_account: pubkeys[4],
            liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            mint: pubkeys[7],
            integration_acc_1: pubkeys[8],
            f_token_mint: pubkeys[9],
            integration_acc_2: pubkeys[10],
            lending_admin: pubkeys[11],
            supply_token_reserves_liquidity: pubkeys[12],
            lending_supply_position_on_liquidity: pubkeys[13],
            rate_model: pubkeys[14],
            vault: pubkeys[15],
            liquidity: pubkeys[16],
            liquidity_program: pubkeys[17],
            rewards_rate_model: pubkeys[18],
            juplend_program: pubkeys[19],
            token_program: pubkeys[20],
            associated_token_program: pubkeys[21],
            system_program: pubkeys[22],
        }
    }
}
impl<'info> From<JuplendDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; JUPLEND_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: JuplendDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.signer_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.mint.clone(),
            accounts.integration_acc_1.clone(),
            accounts.f_token_mint.clone(),
            accounts.integration_acc_2.clone(),
            accounts.lending_admin.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.juplend_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; JUPLEND_DEPOSIT_IX_ACCOUNTS_LEN]>
for JuplendDepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; JUPLEND_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            signer_token_account: &arr[4],
            liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            mint: &arr[7],
            integration_acc_1: &arr[8],
            f_token_mint: &arr[9],
            integration_acc_2: &arr[10],
            lending_admin: &arr[11],
            supply_token_reserves_liquidity: &arr[12],
            lending_supply_position_on_liquidity: &arr[13],
            rate_model: &arr[14],
            vault: &arr[15],
            liquidity: &arr[16],
            liquidity_program: &arr[17],
            rewards_rate_model: &arr[18],
            juplend_program: &arr[19],
            token_program: &arr[20],
            associated_token_program: &arr[21],
            system_program: &arr[22],
        }
    }
}
pub const JUPLEND_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    114, 11, 218, 81, 183, 165, 143, 255,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JuplendDepositIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct JuplendDepositIxData(pub JuplendDepositIxArgs);
impl From<JuplendDepositIxArgs> for JuplendDepositIxData {
    fn from(args: JuplendDepositIxArgs) -> Self {
        Self(args)
    }
}
impl JuplendDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != JUPLEND_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(JuplendDepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&JUPLEND_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn juplend_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: JuplendDepositKeys,
    args: JuplendDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; JUPLEND_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: JuplendDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn juplend_deposit_ix(
    keys: JuplendDepositKeys,
    args: JuplendDepositIxArgs,
) -> std::io::Result<Instruction> {
    juplend_deposit_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn juplend_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: JuplendDepositAccounts<'_, '_>,
    args: JuplendDepositIxArgs,
) -> ProgramResult {
    let keys: JuplendDepositKeys = accounts.into();
    let ix = juplend_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn juplend_deposit_invoke(
    accounts: JuplendDepositAccounts<'_, '_>,
    args: JuplendDepositIxArgs,
) -> ProgramResult {
    juplend_deposit_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn juplend_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: JuplendDepositAccounts<'_, '_>,
    args: JuplendDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: JuplendDepositKeys = accounts.into();
    let ix = juplend_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn juplend_deposit_invoke_signed(
    accounts: JuplendDepositAccounts<'_, '_>,
    args: JuplendDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    juplend_deposit_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn juplend_deposit_verify_account_keys(
    accounts: JuplendDepositAccounts<'_, '_>,
    keys: JuplendDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.mint.key, keys.mint),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.lending_admin.key, keys.lending_admin),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.juplend_program.key, keys.juplend_program),
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
pub fn juplend_deposit_verify_writable_privileges<'me, 'info>(
    accounts: JuplendDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.group,
        accounts.marginfi_account,
        accounts.bank,
        accounts.signer_token_account,
        accounts.liquidity_vault_authority,
        accounts.liquidity_vault,
        accounts.integration_acc_1,
        accounts.f_token_mint,
        accounts.integration_acc_2,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn juplend_deposit_verify_signer_privileges<'me, 'info>(
    accounts: JuplendDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn juplend_deposit_verify_account_privileges<'me, 'info>(
    accounts: JuplendDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    juplend_deposit_verify_writable_privileges(accounts)?;
    juplend_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const JUPLEND_INIT_POSITION_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct JuplendInitPositionAccounts<'me, 'info> {
    pub fee_payer: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub juplend_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JuplendInitPositionKeys {
    pub fee_payer: Pubkey,
    pub signer_token_account: Pubkey,
    pub bank: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub mint: Pubkey,
    pub integration_acc_1: Pubkey,
    pub f_token_mint: Pubkey,
    pub integration_acc_2: Pubkey,
    pub lending_admin: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub juplend_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<JuplendInitPositionAccounts<'_, '_>> for JuplendInitPositionKeys {
    fn from(accounts: JuplendInitPositionAccounts) -> Self {
        Self {
            fee_payer: *accounts.fee_payer.key,
            signer_token_account: *accounts.signer_token_account.key,
            bank: *accounts.bank.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            mint: *accounts.mint.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            f_token_mint: *accounts.f_token_mint.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            lending_admin: *accounts.lending_admin.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            juplend_program: *accounts.juplend_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<JuplendInitPositionKeys>
for [AccountMeta; JUPLEND_INIT_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: JuplendInitPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.juplend_program,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; JUPLEND_INIT_POSITION_IX_ACCOUNTS_LEN]> for JuplendInitPositionKeys {
    fn from(pubkeys: [Pubkey; JUPLEND_INIT_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_payer: pubkeys[0],
            signer_token_account: pubkeys[1],
            bank: pubkeys[2],
            liquidity_vault_authority: pubkeys[3],
            liquidity_vault: pubkeys[4],
            mint: pubkeys[5],
            integration_acc_1: pubkeys[6],
            f_token_mint: pubkeys[7],
            integration_acc_2: pubkeys[8],
            lending_admin: pubkeys[9],
            supply_token_reserves_liquidity: pubkeys[10],
            lending_supply_position_on_liquidity: pubkeys[11],
            rate_model: pubkeys[12],
            vault: pubkeys[13],
            liquidity: pubkeys[14],
            liquidity_program: pubkeys[15],
            rewards_rate_model: pubkeys[16],
            juplend_program: pubkeys[17],
            token_program: pubkeys[18],
            associated_token_program: pubkeys[19],
            system_program: pubkeys[20],
        }
    }
}
impl<'info> From<JuplendInitPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; JUPLEND_INIT_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: JuplendInitPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_payer.clone(),
            accounts.signer_token_account.clone(),
            accounts.bank.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.mint.clone(),
            accounts.integration_acc_1.clone(),
            accounts.f_token_mint.clone(),
            accounts.integration_acc_2.clone(),
            accounts.lending_admin.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.juplend_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; JUPLEND_INIT_POSITION_IX_ACCOUNTS_LEN]>
for JuplendInitPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; JUPLEND_INIT_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fee_payer: &arr[0],
            signer_token_account: &arr[1],
            bank: &arr[2],
            liquidity_vault_authority: &arr[3],
            liquidity_vault: &arr[4],
            mint: &arr[5],
            integration_acc_1: &arr[6],
            f_token_mint: &arr[7],
            integration_acc_2: &arr[8],
            lending_admin: &arr[9],
            supply_token_reserves_liquidity: &arr[10],
            lending_supply_position_on_liquidity: &arr[11],
            rate_model: &arr[12],
            vault: &arr[13],
            liquidity: &arr[14],
            liquidity_program: &arr[15],
            rewards_rate_model: &arr[16],
            juplend_program: &arr[17],
            token_program: &arr[18],
            associated_token_program: &arr[19],
            system_program: &arr[20],
        }
    }
}
pub const JUPLEND_INIT_POSITION_IX_DISCM: [u8; 8usize] = [
    176, 255, 151, 106, 5, 207, 74, 215,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JuplendInitPositionIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct JuplendInitPositionIxData(pub JuplendInitPositionIxArgs);
impl From<JuplendInitPositionIxArgs> for JuplendInitPositionIxData {
    fn from(args: JuplendInitPositionIxArgs) -> Self {
        Self(args)
    }
}
impl JuplendInitPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != JUPLEND_INIT_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(JuplendInitPositionIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&JUPLEND_INIT_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn juplend_init_position_ix_with_program_id(
    program_id: Pubkey,
    keys: JuplendInitPositionKeys,
    args: JuplendInitPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; JUPLEND_INIT_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: JuplendInitPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn juplend_init_position_ix(
    keys: JuplendInitPositionKeys,
    args: JuplendInitPositionIxArgs,
) -> std::io::Result<Instruction> {
    juplend_init_position_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn juplend_init_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: JuplendInitPositionAccounts<'_, '_>,
    args: JuplendInitPositionIxArgs,
) -> ProgramResult {
    let keys: JuplendInitPositionKeys = accounts.into();
    let ix = juplend_init_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn juplend_init_position_invoke(
    accounts: JuplendInitPositionAccounts<'_, '_>,
    args: JuplendInitPositionIxArgs,
) -> ProgramResult {
    juplend_init_position_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn juplend_init_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: JuplendInitPositionAccounts<'_, '_>,
    args: JuplendInitPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: JuplendInitPositionKeys = accounts.into();
    let ix = juplend_init_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn juplend_init_position_invoke_signed(
    accounts: JuplendInitPositionAccounts<'_, '_>,
    args: JuplendInitPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    juplend_init_position_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn juplend_init_position_verify_account_keys(
    accounts: JuplendInitPositionAccounts<'_, '_>,
    keys: JuplendInitPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.bank.key, keys.bank),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.mint.key, keys.mint),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.lending_admin.key, keys.lending_admin),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.juplend_program.key, keys.juplend_program),
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
pub fn juplend_init_position_verify_writable_privileges<'me, 'info>(
    accounts: JuplendInitPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_payer,
        accounts.signer_token_account,
        accounts.bank,
        accounts.liquidity_vault_authority,
        accounts.liquidity_vault,
        accounts.integration_acc_1,
        accounts.f_token_mint,
        accounts.integration_acc_2,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn juplend_init_position_verify_signer_privileges<'me, 'info>(
    accounts: JuplendInitPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn juplend_init_position_verify_account_privileges<'me, 'info>(
    accounts: JuplendInitPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    juplend_init_position_verify_writable_privileges(accounts)?;
    juplend_init_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const JUPLEND_WITHDRAW_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct JuplendWithdrawAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub integration_acc_3: &'me AccountInfo<'info>,
    pub lending_admin: &'me AccountInfo<'info>,
    pub supply_token_reserves_liquidity: &'me AccountInfo<'info>,
    pub lending_supply_position_on_liquidity: &'me AccountInfo<'info>,
    pub rate_model: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub claim_account: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub rewards_rate_model: &'me AccountInfo<'info>,
    pub juplend_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JuplendWithdrawKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub destination_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub mint: Pubkey,
    pub integration_acc_1: Pubkey,
    pub f_token_mint: Pubkey,
    pub integration_acc_2: Pubkey,
    pub integration_acc_3: Pubkey,
    pub lending_admin: Pubkey,
    pub supply_token_reserves_liquidity: Pubkey,
    pub lending_supply_position_on_liquidity: Pubkey,
    pub rate_model: Pubkey,
    pub vault: Pubkey,
    pub claim_account: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub rewards_rate_model: Pubkey,
    pub juplend_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<JuplendWithdrawAccounts<'_, '_>> for JuplendWithdrawKeys {
    fn from(accounts: JuplendWithdrawAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            destination_token_account: *accounts.destination_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            mint: *accounts.mint.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            f_token_mint: *accounts.f_token_mint.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            integration_acc_3: *accounts.integration_acc_3.key,
            lending_admin: *accounts.lending_admin.key,
            supply_token_reserves_liquidity: *accounts
                .supply_token_reserves_liquidity
                .key,
            lending_supply_position_on_liquidity: *accounts
                .lending_supply_position_on_liquidity
                .key,
            rate_model: *accounts.rate_model.key,
            vault: *accounts.vault.key,
            claim_account: *accounts.claim_account.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            rewards_rate_model: *accounts.rewards_rate_model.key,
            juplend_program: *accounts.juplend_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<JuplendWithdrawKeys> for [AccountMeta; JUPLEND_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: JuplendWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_3,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.supply_token_reserves_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_supply_position_on_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.claim_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rewards_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.juplend_program,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; JUPLEND_WITHDRAW_IX_ACCOUNTS_LEN]> for JuplendWithdrawKeys {
    fn from(pubkeys: [Pubkey; JUPLEND_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            destination_token_account: pubkeys[4],
            liquidity_vault_authority: pubkeys[5],
            mint: pubkeys[6],
            integration_acc_1: pubkeys[7],
            f_token_mint: pubkeys[8],
            integration_acc_2: pubkeys[9],
            integration_acc_3: pubkeys[10],
            lending_admin: pubkeys[11],
            supply_token_reserves_liquidity: pubkeys[12],
            lending_supply_position_on_liquidity: pubkeys[13],
            rate_model: pubkeys[14],
            vault: pubkeys[15],
            claim_account: pubkeys[16],
            liquidity: pubkeys[17],
            liquidity_program: pubkeys[18],
            rewards_rate_model: pubkeys[19],
            juplend_program: pubkeys[20],
            token_program: pubkeys[21],
            associated_token_program: pubkeys[22],
            system_program: pubkeys[23],
        }
    }
}
impl<'info> From<JuplendWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; JUPLEND_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: JuplendWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.destination_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.mint.clone(),
            accounts.integration_acc_1.clone(),
            accounts.f_token_mint.clone(),
            accounts.integration_acc_2.clone(),
            accounts.integration_acc_3.clone(),
            accounts.lending_admin.clone(),
            accounts.supply_token_reserves_liquidity.clone(),
            accounts.lending_supply_position_on_liquidity.clone(),
            accounts.rate_model.clone(),
            accounts.vault.clone(),
            accounts.claim_account.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.rewards_rate_model.clone(),
            accounts.juplend_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; JUPLEND_WITHDRAW_IX_ACCOUNTS_LEN]>
for JuplendWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; JUPLEND_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            destination_token_account: &arr[4],
            liquidity_vault_authority: &arr[5],
            mint: &arr[6],
            integration_acc_1: &arr[7],
            f_token_mint: &arr[8],
            integration_acc_2: &arr[9],
            integration_acc_3: &arr[10],
            lending_admin: &arr[11],
            supply_token_reserves_liquidity: &arr[12],
            lending_supply_position_on_liquidity: &arr[13],
            rate_model: &arr[14],
            vault: &arr[15],
            claim_account: &arr[16],
            liquidity: &arr[17],
            liquidity_program: &arr[18],
            rewards_rate_model: &arr[19],
            juplend_program: &arr[20],
            token_program: &arr[21],
            associated_token_program: &arr[22],
            system_program: &arr[23],
        }
    }
}
pub const JUPLEND_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    245, 164, 253, 202, 53, 77, 251, 221,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JuplendWithdrawIxArgs {
    pub amount: u64,
    pub withdraw_all: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct JuplendWithdrawIxData(pub JuplendWithdrawIxArgs);
impl From<JuplendWithdrawIxArgs> for JuplendWithdrawIxData {
    fn from(args: JuplendWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl JuplendWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != JUPLEND_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(JuplendWithdrawIxArgs {
                amount,
                withdraw_all,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&JUPLEND_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn juplend_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: JuplendWithdrawKeys,
    args: JuplendWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; JUPLEND_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: JuplendWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn juplend_withdraw_ix(
    keys: JuplendWithdrawKeys,
    args: JuplendWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    juplend_withdraw_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn juplend_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: JuplendWithdrawAccounts<'_, '_>,
    args: JuplendWithdrawIxArgs,
) -> ProgramResult {
    let keys: JuplendWithdrawKeys = accounts.into();
    let ix = juplend_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn juplend_withdraw_invoke(
    accounts: JuplendWithdrawAccounts<'_, '_>,
    args: JuplendWithdrawIxArgs,
) -> ProgramResult {
    juplend_withdraw_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn juplend_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: JuplendWithdrawAccounts<'_, '_>,
    args: JuplendWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: JuplendWithdrawKeys = accounts.into();
    let ix = juplend_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn juplend_withdraw_invoke_signed(
    accounts: JuplendWithdrawAccounts<'_, '_>,
    args: JuplendWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    juplend_withdraw_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn juplend_withdraw_verify_account_keys(
    accounts: JuplendWithdrawAccounts<'_, '_>,
    keys: JuplendWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.integration_acc_3.key, keys.integration_acc_3),
        (*accounts.lending_admin.key, keys.lending_admin),
        (
            *accounts.supply_token_reserves_liquidity.key,
            keys.supply_token_reserves_liquidity,
        ),
        (
            *accounts.lending_supply_position_on_liquidity.key,
            keys.lending_supply_position_on_liquidity,
        ),
        (*accounts.rate_model.key, keys.rate_model),
        (*accounts.vault.key, keys.vault),
        (*accounts.claim_account.key, keys.claim_account),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.rewards_rate_model.key, keys.rewards_rate_model),
        (*accounts.juplend_program.key, keys.juplend_program),
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
pub fn juplend_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: JuplendWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.destination_token_account,
        accounts.liquidity_vault_authority,
        accounts.integration_acc_1,
        accounts.f_token_mint,
        accounts.integration_acc_2,
        accounts.integration_acc_3,
        accounts.supply_token_reserves_liquidity,
        accounts.lending_supply_position_on_liquidity,
        accounts.vault,
        accounts.claim_account,
        accounts.liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn juplend_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: JuplendWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn juplend_withdraw_verify_account_privileges<'me, 'info>(
    accounts: JuplendWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    juplend_withdraw_verify_writable_privileges(accounts)?;
    juplend_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const KAMINO_DEPOSIT_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct KaminoDepositAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_destination_deposit_collateral: &'me AccountInfo<'info>,
    pub obligation_farm_user_state: &'me AccountInfo<'info>,
    pub reserve_farm_state: &'me AccountInfo<'info>,
    pub kamino_program: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KaminoDepositKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub signer_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub integration_acc_2: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub integration_acc_1: Pubkey,
    pub mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_destination_deposit_collateral: Pubkey,
    pub obligation_farm_user_state: Pubkey,
    pub reserve_farm_state: Pubkey,
    pub kamino_program: Pubkey,
    pub farms_program: Pubkey,
    pub collateral_token_program: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<KaminoDepositAccounts<'_, '_>> for KaminoDepositKeys {
    fn from(accounts: KaminoDepositAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            signer_token_account: *accounts.signer_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            mint: *accounts.mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_destination_deposit_collateral: *accounts
                .reserve_destination_deposit_collateral
                .key,
            obligation_farm_user_state: *accounts.obligation_farm_user_state.key,
            reserve_farm_state: *accounts.reserve_farm_state.key,
            kamino_program: *accounts.kamino_program.key,
            farms_program: *accounts.farms_program.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<KaminoDepositKeys> for [AccountMeta; KAMINO_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: KaminoDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_deposit_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kamino_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; KAMINO_DEPOSIT_IX_ACCOUNTS_LEN]> for KaminoDepositKeys {
    fn from(pubkeys: [Pubkey; KAMINO_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            signer_token_account: pubkeys[4],
            liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            integration_acc_2: pubkeys[7],
            lending_market: pubkeys[8],
            lending_market_authority: pubkeys[9],
            integration_acc_1: pubkeys[10],
            mint: pubkeys[11],
            reserve_liquidity_supply: pubkeys[12],
            reserve_collateral_mint: pubkeys[13],
            reserve_destination_deposit_collateral: pubkeys[14],
            obligation_farm_user_state: pubkeys[15],
            reserve_farm_state: pubkeys[16],
            kamino_program: pubkeys[17],
            farms_program: pubkeys[18],
            collateral_token_program: pubkeys[19],
            liquidity_token_program: pubkeys[20],
            instruction_sysvar_account: pubkeys[21],
        }
    }
}
impl<'info> From<KaminoDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; KAMINO_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: KaminoDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.signer_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.integration_acc_2.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.integration_acc_1.clone(),
            accounts.mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_destination_deposit_collateral.clone(),
            accounts.obligation_farm_user_state.clone(),
            accounts.reserve_farm_state.clone(),
            accounts.kamino_program.clone(),
            accounts.farms_program.clone(),
            accounts.collateral_token_program.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; KAMINO_DEPOSIT_IX_ACCOUNTS_LEN]>
for KaminoDepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; KAMINO_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            signer_token_account: &arr[4],
            liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            integration_acc_2: &arr[7],
            lending_market: &arr[8],
            lending_market_authority: &arr[9],
            integration_acc_1: &arr[10],
            mint: &arr[11],
            reserve_liquidity_supply: &arr[12],
            reserve_collateral_mint: &arr[13],
            reserve_destination_deposit_collateral: &arr[14],
            obligation_farm_user_state: &arr[15],
            reserve_farm_state: &arr[16],
            kamino_program: &arr[17],
            farms_program: &arr[18],
            collateral_token_program: &arr[19],
            liquidity_token_program: &arr[20],
            instruction_sysvar_account: &arr[21],
        }
    }
}
pub const KAMINO_DEPOSIT_IX_DISCM: [u8; 8usize] = [237, 8, 188, 187, 115, 99, 49, 85];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KaminoDepositIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct KaminoDepositIxData(pub KaminoDepositIxArgs);
impl From<KaminoDepositIxArgs> for KaminoDepositIxData {
    fn from(args: KaminoDepositIxArgs) -> Self {
        Self(args)
    }
}
impl KaminoDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KAMINO_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(KaminoDepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KAMINO_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn kamino_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: KaminoDepositKeys,
    args: KaminoDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; KAMINO_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: KaminoDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn kamino_deposit_ix(
    keys: KaminoDepositKeys,
    args: KaminoDepositIxArgs,
) -> std::io::Result<Instruction> {
    kamino_deposit_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn kamino_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: KaminoDepositAccounts<'_, '_>,
    args: KaminoDepositIxArgs,
) -> ProgramResult {
    let keys: KaminoDepositKeys = accounts.into();
    let ix = kamino_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn kamino_deposit_invoke(
    accounts: KaminoDepositAccounts<'_, '_>,
    args: KaminoDepositIxArgs,
) -> ProgramResult {
    kamino_deposit_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn kamino_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: KaminoDepositAccounts<'_, '_>,
    args: KaminoDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: KaminoDepositKeys = accounts.into();
    let ix = kamino_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn kamino_deposit_invoke_signed(
    accounts: KaminoDepositAccounts<'_, '_>,
    args: KaminoDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    kamino_deposit_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn kamino_deposit_verify_account_keys(
    accounts: KaminoDepositAccounts<'_, '_>,
    keys: KaminoDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.mint.key, keys.mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (
            *accounts.reserve_destination_deposit_collateral.key,
            keys.reserve_destination_deposit_collateral,
        ),
        (*accounts.obligation_farm_user_state.key, keys.obligation_farm_user_state),
        (*accounts.reserve_farm_state.key, keys.reserve_farm_state),
        (*accounts.kamino_program.key, keys.kamino_program),
        (*accounts.farms_program.key, keys.farms_program),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn kamino_deposit_verify_writable_privileges<'me, 'info>(
    accounts: KaminoDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.signer_token_account,
        accounts.liquidity_vault_authority,
        accounts.liquidity_vault,
        accounts.integration_acc_2,
        accounts.integration_acc_1,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_destination_deposit_collateral,
        accounts.obligation_farm_user_state,
        accounts.reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn kamino_deposit_verify_signer_privileges<'me, 'info>(
    accounts: KaminoDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn kamino_deposit_verify_account_privileges<'me, 'info>(
    accounts: KaminoDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    kamino_deposit_verify_writable_privileges(accounts)?;
    kamino_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const KAMINO_HARVEST_REWARD_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct KaminoHarvestRewardAccounts<'me, 'info> {
    pub bank: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub user_reward_ata: &'me AccountInfo<'info>,
    pub rewards_vault: &'me AccountInfo<'info>,
    pub rewards_treasury_vault: &'me AccountInfo<'info>,
    pub farm_vaults_authority: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KaminoHarvestRewardKeys {
    pub bank: Pubkey,
    pub fee_state: Pubkey,
    pub destination_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub global_config: Pubkey,
    pub reward_mint: Pubkey,
    pub user_reward_ata: Pubkey,
    pub rewards_vault: Pubkey,
    pub rewards_treasury_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub scope_prices: Pubkey,
    pub farms_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<KaminoHarvestRewardAccounts<'_, '_>> for KaminoHarvestRewardKeys {
    fn from(accounts: KaminoHarvestRewardAccounts) -> Self {
        Self {
            bank: *accounts.bank.key,
            fee_state: *accounts.fee_state.key,
            destination_token_account: *accounts.destination_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            global_config: *accounts.global_config.key,
            reward_mint: *accounts.reward_mint.key,
            user_reward_ata: *accounts.user_reward_ata.key,
            rewards_vault: *accounts.rewards_vault.key,
            rewards_treasury_vault: *accounts.rewards_treasury_vault.key,
            farm_vaults_authority: *accounts.farm_vaults_authority.key,
            scope_prices: *accounts.scope_prices.key,
            farms_program: *accounts.farms_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<KaminoHarvestRewardKeys>
for [AccountMeta; KAMINO_HARVEST_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: KaminoHarvestRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
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
                pubkey: keys.user_reward_ata,
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
                pubkey: keys.farms_program,
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
impl From<[Pubkey; KAMINO_HARVEST_REWARD_IX_ACCOUNTS_LEN]> for KaminoHarvestRewardKeys {
    fn from(pubkeys: [Pubkey; KAMINO_HARVEST_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bank: pubkeys[0],
            fee_state: pubkeys[1],
            destination_token_account: pubkeys[2],
            liquidity_vault_authority: pubkeys[3],
            user_state: pubkeys[4],
            farm_state: pubkeys[5],
            global_config: pubkeys[6],
            reward_mint: pubkeys[7],
            user_reward_ata: pubkeys[8],
            rewards_vault: pubkeys[9],
            rewards_treasury_vault: pubkeys[10],
            farm_vaults_authority: pubkeys[11],
            scope_prices: pubkeys[12],
            farms_program: pubkeys[13],
            token_program: pubkeys[14],
        }
    }
}
impl<'info> From<KaminoHarvestRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; KAMINO_HARVEST_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: KaminoHarvestRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.bank.clone(),
            accounts.fee_state.clone(),
            accounts.destination_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.global_config.clone(),
            accounts.reward_mint.clone(),
            accounts.user_reward_ata.clone(),
            accounts.rewards_vault.clone(),
            accounts.rewards_treasury_vault.clone(),
            accounts.farm_vaults_authority.clone(),
            accounts.scope_prices.clone(),
            accounts.farms_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; KAMINO_HARVEST_REWARD_IX_ACCOUNTS_LEN]>
for KaminoHarvestRewardAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; KAMINO_HARVEST_REWARD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bank: &arr[0],
            fee_state: &arr[1],
            destination_token_account: &arr[2],
            liquidity_vault_authority: &arr[3],
            user_state: &arr[4],
            farm_state: &arr[5],
            global_config: &arr[6],
            reward_mint: &arr[7],
            user_reward_ata: &arr[8],
            rewards_vault: &arr[9],
            rewards_treasury_vault: &arr[10],
            farm_vaults_authority: &arr[11],
            scope_prices: &arr[12],
            farms_program: &arr[13],
            token_program: &arr[14],
        }
    }
}
pub const KAMINO_HARVEST_REWARD_IX_DISCM: [u8; 8usize] = [
    163, 202, 248, 141, 106, 20, 116, 5,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KaminoHarvestRewardIxArgs {
    pub reward_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct KaminoHarvestRewardIxData(pub KaminoHarvestRewardIxArgs);
impl From<KaminoHarvestRewardIxArgs> for KaminoHarvestRewardIxData {
    fn from(args: KaminoHarvestRewardIxArgs) -> Self {
        Self(args)
    }
}
impl KaminoHarvestRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KAMINO_HARVEST_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(KaminoHarvestRewardIxArgs {
                reward_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KAMINO_HARVEST_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn kamino_harvest_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: KaminoHarvestRewardKeys,
    args: KaminoHarvestRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; KAMINO_HARVEST_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: KaminoHarvestRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn kamino_harvest_reward_ix(
    keys: KaminoHarvestRewardKeys,
    args: KaminoHarvestRewardIxArgs,
) -> std::io::Result<Instruction> {
    kamino_harvest_reward_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn kamino_harvest_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: KaminoHarvestRewardAccounts<'_, '_>,
    args: KaminoHarvestRewardIxArgs,
) -> ProgramResult {
    let keys: KaminoHarvestRewardKeys = accounts.into();
    let ix = kamino_harvest_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn kamino_harvest_reward_invoke(
    accounts: KaminoHarvestRewardAccounts<'_, '_>,
    args: KaminoHarvestRewardIxArgs,
) -> ProgramResult {
    kamino_harvest_reward_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn kamino_harvest_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: KaminoHarvestRewardAccounts<'_, '_>,
    args: KaminoHarvestRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: KaminoHarvestRewardKeys = accounts.into();
    let ix = kamino_harvest_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn kamino_harvest_reward_invoke_signed(
    accounts: KaminoHarvestRewardAccounts<'_, '_>,
    args: KaminoHarvestRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    kamino_harvest_reward_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn kamino_harvest_reward_verify_account_keys(
    accounts: KaminoHarvestRewardAccounts<'_, '_>,
    keys: KaminoHarvestRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bank.key, keys.bank),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.user_reward_ata.key, keys.user_reward_ata),
        (*accounts.rewards_vault.key, keys.rewards_vault),
        (*accounts.rewards_treasury_vault.key, keys.rewards_treasury_vault),
        (*accounts.farm_vaults_authority.key, keys.farm_vaults_authority),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.farms_program.key, keys.farms_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn kamino_harvest_reward_verify_writable_privileges<'me, 'info>(
    accounts: KaminoHarvestRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.destination_token_account,
        accounts.liquidity_vault_authority,
        accounts.user_state,
        accounts.farm_state,
        accounts.user_reward_ata,
        accounts.rewards_vault,
        accounts.rewards_treasury_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn kamino_harvest_reward_verify_account_privileges<'me, 'info>(
    accounts: KaminoHarvestRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    kamino_harvest_reward_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const KAMINO_INIT_OBLIGATION_IX_ACCOUNTS_LEN: usize = 27;
#[derive(Copy, Clone, Debug)]
pub struct KaminoInitObligationAccounts<'me, 'info> {
    pub fee_payer: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub user_metadata: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_destination_deposit_collateral: &'me AccountInfo<'info>,
    pub pyth_oracle: &'me AccountInfo<'info>,
    pub switchboard_price_oracle: &'me AccountInfo<'info>,
    pub switchboard_twap_oracle: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub obligation_farm_user_state: &'me AccountInfo<'info>,
    pub reserve_farm_state: &'me AccountInfo<'info>,
    pub kamino_program: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KaminoInitObligationKeys {
    pub fee_payer: Pubkey,
    pub bank: Pubkey,
    pub signer_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub integration_acc_2: Pubkey,
    pub user_metadata: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub integration_acc_1: Pubkey,
    pub mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_destination_deposit_collateral: Pubkey,
    pub pyth_oracle: Pubkey,
    pub switchboard_price_oracle: Pubkey,
    pub switchboard_twap_oracle: Pubkey,
    pub scope_prices: Pubkey,
    pub obligation_farm_user_state: Pubkey,
    pub reserve_farm_state: Pubkey,
    pub kamino_program: Pubkey,
    pub farms_program: Pubkey,
    pub collateral_token_program: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<KaminoInitObligationAccounts<'_, '_>> for KaminoInitObligationKeys {
    fn from(accounts: KaminoInitObligationAccounts) -> Self {
        Self {
            fee_payer: *accounts.fee_payer.key,
            bank: *accounts.bank.key,
            signer_token_account: *accounts.signer_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            user_metadata: *accounts.user_metadata.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            mint: *accounts.mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_destination_deposit_collateral: *accounts
                .reserve_destination_deposit_collateral
                .key,
            pyth_oracle: *accounts.pyth_oracle.key,
            switchboard_price_oracle: *accounts.switchboard_price_oracle.key,
            switchboard_twap_oracle: *accounts.switchboard_twap_oracle.key,
            scope_prices: *accounts.scope_prices.key,
            obligation_farm_user_state: *accounts.obligation_farm_user_state.key,
            reserve_farm_state: *accounts.reserve_farm_state.key,
            kamino_program: *accounts.kamino_program.key,
            farms_program: *accounts.farms_program.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<KaminoInitObligationKeys>
for [AccountMeta; KAMINO_INIT_OBLIGATION_IX_ACCOUNTS_LEN] {
    fn from(keys: KaminoInitObligationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_deposit_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_price_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_twap_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kamino_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
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
impl From<[Pubkey; KAMINO_INIT_OBLIGATION_IX_ACCOUNTS_LEN]>
for KaminoInitObligationKeys {
    fn from(pubkeys: [Pubkey; KAMINO_INIT_OBLIGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_payer: pubkeys[0],
            bank: pubkeys[1],
            signer_token_account: pubkeys[2],
            liquidity_vault_authority: pubkeys[3],
            liquidity_vault: pubkeys[4],
            integration_acc_2: pubkeys[5],
            user_metadata: pubkeys[6],
            lending_market: pubkeys[7],
            lending_market_authority: pubkeys[8],
            integration_acc_1: pubkeys[9],
            mint: pubkeys[10],
            reserve_liquidity_supply: pubkeys[11],
            reserve_collateral_mint: pubkeys[12],
            reserve_destination_deposit_collateral: pubkeys[13],
            pyth_oracle: pubkeys[14],
            switchboard_price_oracle: pubkeys[15],
            switchboard_twap_oracle: pubkeys[16],
            scope_prices: pubkeys[17],
            obligation_farm_user_state: pubkeys[18],
            reserve_farm_state: pubkeys[19],
            kamino_program: pubkeys[20],
            farms_program: pubkeys[21],
            collateral_token_program: pubkeys[22],
            liquidity_token_program: pubkeys[23],
            instruction_sysvar_account: pubkeys[24],
            rent: pubkeys[25],
            system_program: pubkeys[26],
        }
    }
}
impl<'info> From<KaminoInitObligationAccounts<'_, 'info>>
for [AccountInfo<'info>; KAMINO_INIT_OBLIGATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: KaminoInitObligationAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_payer.clone(),
            accounts.bank.clone(),
            accounts.signer_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.integration_acc_2.clone(),
            accounts.user_metadata.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.integration_acc_1.clone(),
            accounts.mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_destination_deposit_collateral.clone(),
            accounts.pyth_oracle.clone(),
            accounts.switchboard_price_oracle.clone(),
            accounts.switchboard_twap_oracle.clone(),
            accounts.scope_prices.clone(),
            accounts.obligation_farm_user_state.clone(),
            accounts.reserve_farm_state.clone(),
            accounts.kamino_program.clone(),
            accounts.farms_program.clone(),
            accounts.collateral_token_program.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; KAMINO_INIT_OBLIGATION_IX_ACCOUNTS_LEN]>
for KaminoInitObligationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; KAMINO_INIT_OBLIGATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fee_payer: &arr[0],
            bank: &arr[1],
            signer_token_account: &arr[2],
            liquidity_vault_authority: &arr[3],
            liquidity_vault: &arr[4],
            integration_acc_2: &arr[5],
            user_metadata: &arr[6],
            lending_market: &arr[7],
            lending_market_authority: &arr[8],
            integration_acc_1: &arr[9],
            mint: &arr[10],
            reserve_liquidity_supply: &arr[11],
            reserve_collateral_mint: &arr[12],
            reserve_destination_deposit_collateral: &arr[13],
            pyth_oracle: &arr[14],
            switchboard_price_oracle: &arr[15],
            switchboard_twap_oracle: &arr[16],
            scope_prices: &arr[17],
            obligation_farm_user_state: &arr[18],
            reserve_farm_state: &arr[19],
            kamino_program: &arr[20],
            farms_program: &arr[21],
            collateral_token_program: &arr[22],
            liquidity_token_program: &arr[23],
            instruction_sysvar_account: &arr[24],
            rent: &arr[25],
            system_program: &arr[26],
        }
    }
}
pub const KAMINO_INIT_OBLIGATION_IX_DISCM: [u8; 8usize] = [
    253, 177, 160, 225, 70, 156, 217, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KaminoInitObligationIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct KaminoInitObligationIxData(pub KaminoInitObligationIxArgs);
impl From<KaminoInitObligationIxArgs> for KaminoInitObligationIxData {
    fn from(args: KaminoInitObligationIxArgs) -> Self {
        Self(args)
    }
}
impl KaminoInitObligationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KAMINO_INIT_OBLIGATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(KaminoInitObligationIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KAMINO_INIT_OBLIGATION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn kamino_init_obligation_ix_with_program_id(
    program_id: Pubkey,
    keys: KaminoInitObligationKeys,
    args: KaminoInitObligationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; KAMINO_INIT_OBLIGATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: KaminoInitObligationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn kamino_init_obligation_ix(
    keys: KaminoInitObligationKeys,
    args: KaminoInitObligationIxArgs,
) -> std::io::Result<Instruction> {
    kamino_init_obligation_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn kamino_init_obligation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: KaminoInitObligationAccounts<'_, '_>,
    args: KaminoInitObligationIxArgs,
) -> ProgramResult {
    let keys: KaminoInitObligationKeys = accounts.into();
    let ix = kamino_init_obligation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn kamino_init_obligation_invoke(
    accounts: KaminoInitObligationAccounts<'_, '_>,
    args: KaminoInitObligationIxArgs,
) -> ProgramResult {
    kamino_init_obligation_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn kamino_init_obligation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: KaminoInitObligationAccounts<'_, '_>,
    args: KaminoInitObligationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: KaminoInitObligationKeys = accounts.into();
    let ix = kamino_init_obligation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn kamino_init_obligation_invoke_signed(
    accounts: KaminoInitObligationAccounts<'_, '_>,
    args: KaminoInitObligationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    kamino_init_obligation_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn kamino_init_obligation_verify_account_keys(
    accounts: KaminoInitObligationAccounts<'_, '_>,
    keys: KaminoInitObligationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.bank.key, keys.bank),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.user_metadata.key, keys.user_metadata),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.mint.key, keys.mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (
            *accounts.reserve_destination_deposit_collateral.key,
            keys.reserve_destination_deposit_collateral,
        ),
        (*accounts.pyth_oracle.key, keys.pyth_oracle),
        (*accounts.switchboard_price_oracle.key, keys.switchboard_price_oracle),
        (*accounts.switchboard_twap_oracle.key, keys.switchboard_twap_oracle),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.obligation_farm_user_state.key, keys.obligation_farm_user_state),
        (*accounts.reserve_farm_state.key, keys.reserve_farm_state),
        (*accounts.kamino_program.key, keys.kamino_program),
        (*accounts.farms_program.key, keys.farms_program),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn kamino_init_obligation_verify_writable_privileges<'me, 'info>(
    accounts: KaminoInitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_payer,
        accounts.signer_token_account,
        accounts.liquidity_vault_authority,
        accounts.liquidity_vault,
        accounts.integration_acc_2,
        accounts.user_metadata,
        accounts.integration_acc_1,
        accounts.mint,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_destination_deposit_collateral,
        accounts.obligation_farm_user_state,
        accounts.reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn kamino_init_obligation_verify_signer_privileges<'me, 'info>(
    accounts: KaminoInitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn kamino_init_obligation_verify_account_privileges<'me, 'info>(
    accounts: KaminoInitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    kamino_init_obligation_verify_writable_privileges(accounts)?;
    kamino_init_obligation_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const KAMINO_WITHDRAW_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct KaminoWithdrawAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_source_collateral: &'me AccountInfo<'info>,
    pub obligation_farm_user_state: &'me AccountInfo<'info>,
    pub reserve_farm_state: &'me AccountInfo<'info>,
    pub kamino_program: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KaminoWithdrawKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub destination_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub integration_acc_2: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub integration_acc_1: Pubkey,
    pub mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_source_collateral: Pubkey,
    pub obligation_farm_user_state: Pubkey,
    pub reserve_farm_state: Pubkey,
    pub kamino_program: Pubkey,
    pub farms_program: Pubkey,
    pub collateral_token_program: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<KaminoWithdrawAccounts<'_, '_>> for KaminoWithdrawKeys {
    fn from(accounts: KaminoWithdrawAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            destination_token_account: *accounts.destination_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            mint: *accounts.mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_source_collateral: *accounts.reserve_source_collateral.key,
            obligation_farm_user_state: *accounts.obligation_farm_user_state.key,
            reserve_farm_state: *accounts.reserve_farm_state.key,
            kamino_program: *accounts.kamino_program.key,
            farms_program: *accounts.farms_program.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<KaminoWithdrawKeys> for [AccountMeta; KAMINO_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: KaminoWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kamino_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; KAMINO_WITHDRAW_IX_ACCOUNTS_LEN]> for KaminoWithdrawKeys {
    fn from(pubkeys: [Pubkey; KAMINO_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            destination_token_account: pubkeys[4],
            liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            integration_acc_2: pubkeys[7],
            lending_market: pubkeys[8],
            lending_market_authority: pubkeys[9],
            integration_acc_1: pubkeys[10],
            mint: pubkeys[11],
            reserve_liquidity_supply: pubkeys[12],
            reserve_collateral_mint: pubkeys[13],
            reserve_source_collateral: pubkeys[14],
            obligation_farm_user_state: pubkeys[15],
            reserve_farm_state: pubkeys[16],
            kamino_program: pubkeys[17],
            farms_program: pubkeys[18],
            collateral_token_program: pubkeys[19],
            liquidity_token_program: pubkeys[20],
            instruction_sysvar_account: pubkeys[21],
        }
    }
}
impl<'info> From<KaminoWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; KAMINO_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: KaminoWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.destination_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.integration_acc_2.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.integration_acc_1.clone(),
            accounts.mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_source_collateral.clone(),
            accounts.obligation_farm_user_state.clone(),
            accounts.reserve_farm_state.clone(),
            accounts.kamino_program.clone(),
            accounts.farms_program.clone(),
            accounts.collateral_token_program.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; KAMINO_WITHDRAW_IX_ACCOUNTS_LEN]>
for KaminoWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; KAMINO_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            destination_token_account: &arr[4],
            liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            integration_acc_2: &arr[7],
            lending_market: &arr[8],
            lending_market_authority: &arr[9],
            integration_acc_1: &arr[10],
            mint: &arr[11],
            reserve_liquidity_supply: &arr[12],
            reserve_collateral_mint: &arr[13],
            reserve_source_collateral: &arr[14],
            obligation_farm_user_state: &arr[15],
            reserve_farm_state: &arr[16],
            kamino_program: &arr[17],
            farms_program: &arr[18],
            collateral_token_program: &arr[19],
            liquidity_token_program: &arr[20],
            instruction_sysvar_account: &arr[21],
        }
    }
}
pub const KAMINO_WITHDRAW_IX_DISCM: [u8; 8usize] = [199, 101, 41, 45, 213, 98, 224, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KaminoWithdrawIxArgs {
    pub amount: u64,
    pub withdraw_all: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct KaminoWithdrawIxData(pub KaminoWithdrawIxArgs);
impl From<KaminoWithdrawIxArgs> for KaminoWithdrawIxData {
    fn from(args: KaminoWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl KaminoWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KAMINO_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(KaminoWithdrawIxArgs {
                amount,
                withdraw_all,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KAMINO_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn kamino_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: KaminoWithdrawKeys,
    args: KaminoWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; KAMINO_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: KaminoWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn kamino_withdraw_ix(
    keys: KaminoWithdrawKeys,
    args: KaminoWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    kamino_withdraw_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn kamino_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: KaminoWithdrawAccounts<'_, '_>,
    args: KaminoWithdrawIxArgs,
) -> ProgramResult {
    let keys: KaminoWithdrawKeys = accounts.into();
    let ix = kamino_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn kamino_withdraw_invoke(
    accounts: KaminoWithdrawAccounts<'_, '_>,
    args: KaminoWithdrawIxArgs,
) -> ProgramResult {
    kamino_withdraw_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn kamino_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: KaminoWithdrawAccounts<'_, '_>,
    args: KaminoWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: KaminoWithdrawKeys = accounts.into();
    let ix = kamino_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn kamino_withdraw_invoke_signed(
    accounts: KaminoWithdrawAccounts<'_, '_>,
    args: KaminoWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    kamino_withdraw_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn kamino_withdraw_verify_account_keys(
    accounts: KaminoWithdrawAccounts<'_, '_>,
    keys: KaminoWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.mint.key, keys.mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_source_collateral.key, keys.reserve_source_collateral),
        (*accounts.obligation_farm_user_state.key, keys.obligation_farm_user_state),
        (*accounts.reserve_farm_state.key, keys.reserve_farm_state),
        (*accounts.kamino_program.key, keys.kamino_program),
        (*accounts.farms_program.key, keys.farms_program),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn kamino_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: KaminoWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.destination_token_account,
        accounts.liquidity_vault_authority,
        accounts.liquidity_vault,
        accounts.integration_acc_2,
        accounts.integration_acc_1,
        accounts.mint,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_source_collateral,
        accounts.obligation_farm_user_state,
        accounts.reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn kamino_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: KaminoWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn kamino_withdraw_verify_account_privileges<'me, 'info>(
    accounts: KaminoWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    kamino_withdraw_verify_writable_privileges(accounts)?;
    kamino_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_BORROW_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountBorrowAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub bank_liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountBorrowKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub destination_token_account: Pubkey,
    pub bank_liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingAccountBorrowAccounts<'_, '_>> for LendingAccountBorrowKeys {
    fn from(accounts: LendingAccountBorrowAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            destination_token_account: *accounts.destination_token_account.key,
            bank_liquidity_vault_authority: *accounts.bank_liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingAccountBorrowKeys>
for [AccountMeta; LENDING_ACCOUNT_BORROW_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountBorrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
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
impl From<[Pubkey; LENDING_ACCOUNT_BORROW_IX_ACCOUNTS_LEN]>
for LendingAccountBorrowKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            destination_token_account: pubkeys[4],
            bank_liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<LendingAccountBorrowAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_BORROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountBorrowAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.destination_token_account.clone(),
            accounts.bank_liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_BORROW_IX_ACCOUNTS_LEN]>
for LendingAccountBorrowAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_BORROW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            destination_token_account: &arr[4],
            bank_liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const LENDING_ACCOUNT_BORROW_IX_DISCM: [u8; 8usize] = [
    4, 126, 116, 53, 48, 5, 212, 31,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountBorrowIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountBorrowIxData(pub LendingAccountBorrowIxArgs);
impl From<LendingAccountBorrowIxArgs> for LendingAccountBorrowIxData {
    fn from(args: LendingAccountBorrowIxArgs) -> Self {
        Self(args)
    }
}
impl LendingAccountBorrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_BORROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingAccountBorrowIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_BORROW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_borrow_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountBorrowKeys,
    args: LendingAccountBorrowIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_BORROW_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingAccountBorrowIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_account_borrow_ix(
    keys: LendingAccountBorrowKeys,
    args: LendingAccountBorrowIxArgs,
) -> std::io::Result<Instruction> {
    lending_account_borrow_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_account_borrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountBorrowAccounts<'_, '_>,
    args: LendingAccountBorrowIxArgs,
) -> ProgramResult {
    let keys: LendingAccountBorrowKeys = accounts.into();
    let ix = lending_account_borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_borrow_invoke(
    accounts: LendingAccountBorrowAccounts<'_, '_>,
    args: LendingAccountBorrowIxArgs,
) -> ProgramResult {
    lending_account_borrow_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn lending_account_borrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountBorrowAccounts<'_, '_>,
    args: LendingAccountBorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountBorrowKeys = accounts.into();
    let ix = lending_account_borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_borrow_invoke_signed(
    accounts: LendingAccountBorrowAccounts<'_, '_>,
    args: LendingAccountBorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_borrow_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_account_borrow_verify_account_keys(
    accounts: LendingAccountBorrowAccounts<'_, '_>,
    keys: LendingAccountBorrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (
            *accounts.bank_liquidity_vault_authority.key,
            keys.bank_liquidity_vault_authority,
        ),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_borrow_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.destination_token_account,
        accounts.liquidity_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_borrow_verify_signer_privileges<'me, 'info>(
    accounts: LendingAccountBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_account_borrow_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_borrow_verify_writable_privileges(accounts)?;
    lending_account_borrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountClearEmissionsAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountClearEmissionsKeys {
    pub marginfi_account: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingAccountClearEmissionsAccounts<'_, '_>>
for LendingAccountClearEmissionsKeys {
    fn from(accounts: LendingAccountClearEmissionsAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingAccountClearEmissionsKeys>
for [AccountMeta; LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountClearEmissionsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_ACCOUNTS_LEN]>
for LendingAccountClearEmissionsKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            bank: pubkeys[1],
        }
    }
}
impl<'info> From<LendingAccountClearEmissionsAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountClearEmissionsAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_account.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_ACCOUNTS_LEN]>
for LendingAccountClearEmissionsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            bank: &arr[1],
        }
    }
}
pub const LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_DISCM: [u8; 8usize] = [
    239, 4, 221, 98, 45, 167, 201, 244,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountClearEmissionsIxData;
impl LendingAccountClearEmissionsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_clear_emissions_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountClearEmissionsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_CLEAR_EMISSIONS_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingAccountClearEmissionsIxData.try_to_vec()?,
    })
}
pub fn lending_account_clear_emissions_ix(
    keys: LendingAccountClearEmissionsKeys,
) -> std::io::Result<Instruction> {
    lending_account_clear_emissions_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_account_clear_emissions_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountClearEmissionsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingAccountClearEmissionsKeys = accounts.into();
    let ix = lending_account_clear_emissions_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_clear_emissions_invoke(
    accounts: LendingAccountClearEmissionsAccounts<'_, '_>,
) -> ProgramResult {
    lending_account_clear_emissions_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn lending_account_clear_emissions_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountClearEmissionsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountClearEmissionsKeys = accounts.into();
    let ix = lending_account_clear_emissions_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_clear_emissions_invoke_signed(
    accounts: LendingAccountClearEmissionsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_clear_emissions_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_account_clear_emissions_verify_account_keys(
    accounts: LendingAccountClearEmissionsAccounts<'_, '_>,
    keys: LendingAccountClearEmissionsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_clear_emissions_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountClearEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_clear_emissions_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountClearEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_clear_emissions_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_CLOSE_BALANCE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountCloseBalanceAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountCloseBalanceKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingAccountCloseBalanceAccounts<'_, '_>>
for LendingAccountCloseBalanceKeys {
    fn from(accounts: LendingAccountCloseBalanceAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingAccountCloseBalanceKeys>
for [AccountMeta; LENDING_ACCOUNT_CLOSE_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountCloseBalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_ACCOUNT_CLOSE_BALANCE_IX_ACCOUNTS_LEN]>
for LendingAccountCloseBalanceKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_CLOSE_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
        }
    }
}
impl<'info> From<LendingAccountCloseBalanceAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_CLOSE_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountCloseBalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_CLOSE_BALANCE_IX_ACCOUNTS_LEN]>
for LendingAccountCloseBalanceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_CLOSE_BALANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
        }
    }
}
pub const LENDING_ACCOUNT_CLOSE_BALANCE_IX_DISCM: [u8; 8usize] = [
    245, 54, 41, 4, 243, 202, 31, 17,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountCloseBalanceIxData;
impl LendingAccountCloseBalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_CLOSE_BALANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_CLOSE_BALANCE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_close_balance_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountCloseBalanceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_CLOSE_BALANCE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingAccountCloseBalanceIxData.try_to_vec()?,
    })
}
pub fn lending_account_close_balance_ix(
    keys: LendingAccountCloseBalanceKeys,
) -> std::io::Result<Instruction> {
    lending_account_close_balance_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_account_close_balance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountCloseBalanceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingAccountCloseBalanceKeys = accounts.into();
    let ix = lending_account_close_balance_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_close_balance_invoke(
    accounts: LendingAccountCloseBalanceAccounts<'_, '_>,
) -> ProgramResult {
    lending_account_close_balance_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn lending_account_close_balance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountCloseBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountCloseBalanceKeys = accounts.into();
    let ix = lending_account_close_balance_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_close_balance_invoke_signed(
    accounts: LendingAccountCloseBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_close_balance_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_account_close_balance_verify_account_keys(
    accounts: LendingAccountCloseBalanceAccounts<'_, '_>,
    keys: LendingAccountCloseBalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_close_balance_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountCloseBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account, accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_close_balance_verify_signer_privileges<'me, 'info>(
    accounts: LendingAccountCloseBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_account_close_balance_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountCloseBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_close_balance_verify_writable_privileges(accounts)?;
    lending_account_close_balance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_DEPOSIT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountDepositAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountDepositKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub signer_token_account: Pubkey,
    pub liquidity_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingAccountDepositAccounts<'_, '_>> for LendingAccountDepositKeys {
    fn from(accounts: LendingAccountDepositAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            signer_token_account: *accounts.signer_token_account.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingAccountDepositKeys>
for [AccountMeta; LENDING_ACCOUNT_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
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
impl From<[Pubkey; LENDING_ACCOUNT_DEPOSIT_IX_ACCOUNTS_LEN]>
for LendingAccountDepositKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            signer_token_account: pubkeys[4],
            liquidity_vault: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<LendingAccountDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.signer_token_account.clone(),
            accounts.liquidity_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_DEPOSIT_IX_ACCOUNTS_LEN]>
for LendingAccountDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            signer_token_account: &arr[4],
            liquidity_vault: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const LENDING_ACCOUNT_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    171, 94, 235, 103, 82, 64, 212, 140,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountDepositIxArgs {
    pub amount: u64,
    pub deposit_up_to_limit: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountDepositIxData(pub LendingAccountDepositIxArgs);
impl From<LendingAccountDepositIxArgs> for LendingAccountDepositIxData {
    fn from(args: LendingAccountDepositIxArgs) -> Self {
        Self(args)
    }
}
impl LendingAccountDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_up_to_limit: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingAccountDepositIxArgs {
                amount,
                deposit_up_to_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.deposit_up_to_limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountDepositKeys,
    args: LendingAccountDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingAccountDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_account_deposit_ix(
    keys: LendingAccountDepositKeys,
    args: LendingAccountDepositIxArgs,
) -> std::io::Result<Instruction> {
    lending_account_deposit_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_account_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountDepositAccounts<'_, '_>,
    args: LendingAccountDepositIxArgs,
) -> ProgramResult {
    let keys: LendingAccountDepositKeys = accounts.into();
    let ix = lending_account_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_deposit_invoke(
    accounts: LendingAccountDepositAccounts<'_, '_>,
    args: LendingAccountDepositIxArgs,
) -> ProgramResult {
    lending_account_deposit_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn lending_account_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountDepositAccounts<'_, '_>,
    args: LendingAccountDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountDepositKeys = accounts.into();
    let ix = lending_account_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_deposit_invoke_signed(
    accounts: LendingAccountDepositAccounts<'_, '_>,
    args: LendingAccountDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_deposit_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_account_deposit_verify_account_keys(
    accounts: LendingAccountDepositAccounts<'_, '_>,
    keys: LendingAccountDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_deposit_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.signer_token_account,
        accounts.liquidity_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_deposit_verify_signer_privileges<'me, 'info>(
    accounts: LendingAccountDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_account_deposit_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_deposit_verify_writable_privileges(accounts)?;
    lending_account_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_END_FLASHLOAN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountEndFlashloanAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountEndFlashloanKeys {
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
}
impl From<LendingAccountEndFlashloanAccounts<'_, '_>>
for LendingAccountEndFlashloanKeys {
    fn from(accounts: LendingAccountEndFlashloanAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<LendingAccountEndFlashloanKeys>
for [AccountMeta; LENDING_ACCOUNT_END_FLASHLOAN_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountEndFlashloanKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_ACCOUNT_END_FLASHLOAN_IX_ACCOUNTS_LEN]>
for LendingAccountEndFlashloanKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_END_FLASHLOAN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<LendingAccountEndFlashloanAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_END_FLASHLOAN_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountEndFlashloanAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_account.clone(), accounts.authority.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_END_FLASHLOAN_IX_ACCOUNTS_LEN]>
for LendingAccountEndFlashloanAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_END_FLASHLOAN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const LENDING_ACCOUNT_END_FLASHLOAN_IX_DISCM: [u8; 8usize] = [
    105, 124, 201, 106, 153, 2, 8, 156,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountEndFlashloanIxData;
impl LendingAccountEndFlashloanIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_END_FLASHLOAN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_END_FLASHLOAN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_end_flashloan_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountEndFlashloanKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_END_FLASHLOAN_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingAccountEndFlashloanIxData.try_to_vec()?,
    })
}
pub fn lending_account_end_flashloan_ix(
    keys: LendingAccountEndFlashloanKeys,
) -> std::io::Result<Instruction> {
    lending_account_end_flashloan_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_account_end_flashloan_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountEndFlashloanAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingAccountEndFlashloanKeys = accounts.into();
    let ix = lending_account_end_flashloan_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_end_flashloan_invoke(
    accounts: LendingAccountEndFlashloanAccounts<'_, '_>,
) -> ProgramResult {
    lending_account_end_flashloan_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn lending_account_end_flashloan_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountEndFlashloanAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountEndFlashloanKeys = accounts.into();
    let ix = lending_account_end_flashloan_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_end_flashloan_invoke_signed(
    accounts: LendingAccountEndFlashloanAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_end_flashloan_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_account_end_flashloan_verify_account_keys(
    accounts: LendingAccountEndFlashloanAccounts<'_, '_>,
    keys: LendingAccountEndFlashloanKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_end_flashloan_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountEndFlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_end_flashloan_verify_signer_privileges<'me, 'info>(
    accounts: LendingAccountEndFlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_account_end_flashloan_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountEndFlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_end_flashloan_verify_writable_privileges(accounts)?;
    lending_account_end_flashloan_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_LIQUIDATE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountLiquidateAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub asset_bank: &'me AccountInfo<'info>,
    pub liab_bank: &'me AccountInfo<'info>,
    pub liquidator_marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub liquidatee_marginfi_account: &'me AccountInfo<'info>,
    pub bank_liquidity_vault_authority: &'me AccountInfo<'info>,
    pub bank_liquidity_vault: &'me AccountInfo<'info>,
    pub bank_insurance_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountLiquidateKeys {
    pub group: Pubkey,
    pub asset_bank: Pubkey,
    pub liab_bank: Pubkey,
    pub liquidator_marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub liquidatee_marginfi_account: Pubkey,
    pub bank_liquidity_vault_authority: Pubkey,
    pub bank_liquidity_vault: Pubkey,
    pub bank_insurance_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingAccountLiquidateAccounts<'_, '_>> for LendingAccountLiquidateKeys {
    fn from(accounts: LendingAccountLiquidateAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            asset_bank: *accounts.asset_bank.key,
            liab_bank: *accounts.liab_bank.key,
            liquidator_marginfi_account: *accounts.liquidator_marginfi_account.key,
            authority: *accounts.authority.key,
            liquidatee_marginfi_account: *accounts.liquidatee_marginfi_account.key,
            bank_liquidity_vault_authority: *accounts.bank_liquidity_vault_authority.key,
            bank_liquidity_vault: *accounts.bank_liquidity_vault.key,
            bank_insurance_vault: *accounts.bank_insurance_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingAccountLiquidateKeys>
for [AccountMeta; LENDING_ACCOUNT_LIQUIDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountLiquidateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liab_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidator_marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidatee_marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank_liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_insurance_vault,
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
impl From<[Pubkey; LENDING_ACCOUNT_LIQUIDATE_IX_ACCOUNTS_LEN]>
for LendingAccountLiquidateKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_LIQUIDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            asset_bank: pubkeys[1],
            liab_bank: pubkeys[2],
            liquidator_marginfi_account: pubkeys[3],
            authority: pubkeys[4],
            liquidatee_marginfi_account: pubkeys[5],
            bank_liquidity_vault_authority: pubkeys[6],
            bank_liquidity_vault: pubkeys[7],
            bank_insurance_vault: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<LendingAccountLiquidateAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_LIQUIDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountLiquidateAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.asset_bank.clone(),
            accounts.liab_bank.clone(),
            accounts.liquidator_marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.liquidatee_marginfi_account.clone(),
            accounts.bank_liquidity_vault_authority.clone(),
            accounts.bank_liquidity_vault.clone(),
            accounts.bank_insurance_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_LIQUIDATE_IX_ACCOUNTS_LEN]>
for LendingAccountLiquidateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_LIQUIDATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            asset_bank: &arr[1],
            liab_bank: &arr[2],
            liquidator_marginfi_account: &arr[3],
            authority: &arr[4],
            liquidatee_marginfi_account: &arr[5],
            bank_liquidity_vault_authority: &arr[6],
            bank_liquidity_vault: &arr[7],
            bank_insurance_vault: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const LENDING_ACCOUNT_LIQUIDATE_IX_DISCM: [u8; 8usize] = [
    214, 169, 151, 213, 251, 167, 86, 219,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountLiquidateIxArgs {
    pub asset_amount: u64,
    pub liquidatee_accounts: u8,
    pub liquidator_accounts: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountLiquidateIxData(pub LendingAccountLiquidateIxArgs);
impl From<LendingAccountLiquidateIxArgs> for LendingAccountLiquidateIxData {
    fn from(args: LendingAccountLiquidateIxArgs) -> Self {
        Self(args)
    }
}
impl LendingAccountLiquidateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_LIQUIDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let asset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidatee_accounts: u8 = crate::borsh_de_or_default(&mut reader)?;
        let liquidator_accounts: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingAccountLiquidateIxArgs {
                asset_amount,
                liquidatee_accounts,
                liquidator_accounts,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_LIQUIDATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.asset_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidatee_accounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.liquidator_accounts, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_liquidate_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountLiquidateKeys,
    args: LendingAccountLiquidateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_LIQUIDATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingAccountLiquidateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_account_liquidate_ix(
    keys: LendingAccountLiquidateKeys,
    args: LendingAccountLiquidateIxArgs,
) -> std::io::Result<Instruction> {
    lending_account_liquidate_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_account_liquidate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountLiquidateAccounts<'_, '_>,
    args: LendingAccountLiquidateIxArgs,
) -> ProgramResult {
    let keys: LendingAccountLiquidateKeys = accounts.into();
    let ix = lending_account_liquidate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_liquidate_invoke(
    accounts: LendingAccountLiquidateAccounts<'_, '_>,
    args: LendingAccountLiquidateIxArgs,
) -> ProgramResult {
    lending_account_liquidate_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn lending_account_liquidate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountLiquidateAccounts<'_, '_>,
    args: LendingAccountLiquidateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountLiquidateKeys = accounts.into();
    let ix = lending_account_liquidate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_liquidate_invoke_signed(
    accounts: LendingAccountLiquidateAccounts<'_, '_>,
    args: LendingAccountLiquidateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_liquidate_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_account_liquidate_verify_account_keys(
    accounts: LendingAccountLiquidateAccounts<'_, '_>,
    keys: LendingAccountLiquidateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.asset_bank.key, keys.asset_bank),
        (*accounts.liab_bank.key, keys.liab_bank),
        (*accounts.liquidator_marginfi_account.key, keys.liquidator_marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.liquidatee_marginfi_account.key, keys.liquidatee_marginfi_account),
        (
            *accounts.bank_liquidity_vault_authority.key,
            keys.bank_liquidity_vault_authority,
        ),
        (*accounts.bank_liquidity_vault.key, keys.bank_liquidity_vault),
        (*accounts.bank_insurance_vault.key, keys.bank_insurance_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_liquidate_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountLiquidateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.asset_bank,
        accounts.liab_bank,
        accounts.liquidator_marginfi_account,
        accounts.liquidatee_marginfi_account,
        accounts.bank_liquidity_vault,
        accounts.bank_insurance_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_liquidate_verify_signer_privileges<'me, 'info>(
    accounts: LendingAccountLiquidateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_account_liquidate_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountLiquidateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_liquidate_verify_writable_privileges(accounts)?;
    lending_account_liquidate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_PULSE_HEALTH_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountPulseHealthAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountPulseHealthKeys {
    pub marginfi_account: Pubkey,
}
impl From<LendingAccountPulseHealthAccounts<'_, '_>> for LendingAccountPulseHealthKeys {
    fn from(accounts: LendingAccountPulseHealthAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
        }
    }
}
impl From<LendingAccountPulseHealthKeys>
for [AccountMeta; LENDING_ACCOUNT_PULSE_HEALTH_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountPulseHealthKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_ACCOUNT_PULSE_HEALTH_IX_ACCOUNTS_LEN]>
for LendingAccountPulseHealthKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_PULSE_HEALTH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
        }
    }
}
impl<'info> From<LendingAccountPulseHealthAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_PULSE_HEALTH_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountPulseHealthAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_account.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_PULSE_HEALTH_IX_ACCOUNTS_LEN]>
for LendingAccountPulseHealthAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_PULSE_HEALTH_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { marginfi_account: &arr[0] }
    }
}
pub const LENDING_ACCOUNT_PULSE_HEALTH_IX_DISCM: [u8; 8usize] = [
    186, 52, 117, 97, 34, 74, 39, 253,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountPulseHealthIxData;
impl LendingAccountPulseHealthIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_PULSE_HEALTH_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_PULSE_HEALTH_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_pulse_health_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountPulseHealthKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_PULSE_HEALTH_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingAccountPulseHealthIxData.try_to_vec()?,
    })
}
pub fn lending_account_pulse_health_ix(
    keys: LendingAccountPulseHealthKeys,
) -> std::io::Result<Instruction> {
    lending_account_pulse_health_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_account_pulse_health_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountPulseHealthAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingAccountPulseHealthKeys = accounts.into();
    let ix = lending_account_pulse_health_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_pulse_health_invoke(
    accounts: LendingAccountPulseHealthAccounts<'_, '_>,
) -> ProgramResult {
    lending_account_pulse_health_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn lending_account_pulse_health_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountPulseHealthAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountPulseHealthKeys = accounts.into();
    let ix = lending_account_pulse_health_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_pulse_health_invoke_signed(
    accounts: LendingAccountPulseHealthAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_pulse_health_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_account_pulse_health_verify_account_keys(
    accounts: LendingAccountPulseHealthAccounts<'_, '_>,
    keys: LendingAccountPulseHealthKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.marginfi_account.key, keys.marginfi_account)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_pulse_health_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountPulseHealthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_pulse_health_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountPulseHealthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_pulse_health_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_REPAY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountRepayAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountRepayKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub signer_token_account: Pubkey,
    pub liquidity_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingAccountRepayAccounts<'_, '_>> for LendingAccountRepayKeys {
    fn from(accounts: LendingAccountRepayAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            signer_token_account: *accounts.signer_token_account.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingAccountRepayKeys>
for [AccountMeta; LENDING_ACCOUNT_REPAY_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountRepayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
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
impl From<[Pubkey; LENDING_ACCOUNT_REPAY_IX_ACCOUNTS_LEN]> for LendingAccountRepayKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_REPAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            signer_token_account: pubkeys[4],
            liquidity_vault: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<LendingAccountRepayAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_REPAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountRepayAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.signer_token_account.clone(),
            accounts.liquidity_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_REPAY_IX_ACCOUNTS_LEN]>
for LendingAccountRepayAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_REPAY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            signer_token_account: &arr[4],
            liquidity_vault: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const LENDING_ACCOUNT_REPAY_IX_DISCM: [u8; 8usize] = [
    79, 209, 172, 177, 222, 51, 173, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountRepayIxArgs {
    pub amount: u64,
    pub repay_all: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountRepayIxData(pub LendingAccountRepayIxArgs);
impl From<LendingAccountRepayIxArgs> for LendingAccountRepayIxData {
    fn from(args: LendingAccountRepayIxArgs) -> Self {
        Self(args)
    }
}
impl LendingAccountRepayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_REPAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let repay_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingAccountRepayIxArgs {
                amount,
                repay_all,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_REPAY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.repay_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_repay_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountRepayKeys,
    args: LendingAccountRepayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_REPAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingAccountRepayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_account_repay_ix(
    keys: LendingAccountRepayKeys,
    args: LendingAccountRepayIxArgs,
) -> std::io::Result<Instruction> {
    lending_account_repay_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_account_repay_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountRepayAccounts<'_, '_>,
    args: LendingAccountRepayIxArgs,
) -> ProgramResult {
    let keys: LendingAccountRepayKeys = accounts.into();
    let ix = lending_account_repay_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_repay_invoke(
    accounts: LendingAccountRepayAccounts<'_, '_>,
    args: LendingAccountRepayIxArgs,
) -> ProgramResult {
    lending_account_repay_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn lending_account_repay_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountRepayAccounts<'_, '_>,
    args: LendingAccountRepayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountRepayKeys = accounts.into();
    let ix = lending_account_repay_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_repay_invoke_signed(
    accounts: LendingAccountRepayAccounts<'_, '_>,
    args: LendingAccountRepayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_repay_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_account_repay_verify_account_keys(
    accounts: LendingAccountRepayAccounts<'_, '_>,
    keys: LendingAccountRepayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_repay_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountRepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.signer_token_account,
        accounts.liquidity_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_repay_verify_signer_privileges<'me, 'info>(
    accounts: LendingAccountRepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_account_repay_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountRepayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_repay_verify_writable_privileges(accounts)?;
    lending_account_repay_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_START_FLASHLOAN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountStartFlashloanAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub ixs_sysvar: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountStartFlashloanKeys {
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub ixs_sysvar: Pubkey,
}
impl From<LendingAccountStartFlashloanAccounts<'_, '_>>
for LendingAccountStartFlashloanKeys {
    fn from(accounts: LendingAccountStartFlashloanAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            ixs_sysvar: *accounts.ixs_sysvar.key,
        }
    }
}
impl From<LendingAccountStartFlashloanKeys>
for [AccountMeta; LENDING_ACCOUNT_START_FLASHLOAN_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountStartFlashloanKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ixs_sysvar,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_ACCOUNT_START_FLASHLOAN_IX_ACCOUNTS_LEN]>
for LendingAccountStartFlashloanKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_START_FLASHLOAN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            authority: pubkeys[1],
            ixs_sysvar: pubkeys[2],
        }
    }
}
impl<'info> From<LendingAccountStartFlashloanAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_START_FLASHLOAN_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountStartFlashloanAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.ixs_sysvar.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_START_FLASHLOAN_IX_ACCOUNTS_LEN]>
for LendingAccountStartFlashloanAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_START_FLASHLOAN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            authority: &arr[1],
            ixs_sysvar: &arr[2],
        }
    }
}
pub const LENDING_ACCOUNT_START_FLASHLOAN_IX_DISCM: [u8; 8usize] = [
    14, 131, 33, 220, 81, 186, 180, 107,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountStartFlashloanIxArgs {
    pub end_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountStartFlashloanIxData(pub LendingAccountStartFlashloanIxArgs);
impl From<LendingAccountStartFlashloanIxArgs> for LendingAccountStartFlashloanIxData {
    fn from(args: LendingAccountStartFlashloanIxArgs) -> Self {
        Self(args)
    }
}
impl LendingAccountStartFlashloanIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_START_FLASHLOAN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let end_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingAccountStartFlashloanIxArgs {
                end_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_START_FLASHLOAN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.end_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_start_flashloan_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountStartFlashloanKeys,
    args: LendingAccountStartFlashloanIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_START_FLASHLOAN_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingAccountStartFlashloanIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_account_start_flashloan_ix(
    keys: LendingAccountStartFlashloanKeys,
    args: LendingAccountStartFlashloanIxArgs,
) -> std::io::Result<Instruction> {
    lending_account_start_flashloan_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_account_start_flashloan_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountStartFlashloanAccounts<'_, '_>,
    args: LendingAccountStartFlashloanIxArgs,
) -> ProgramResult {
    let keys: LendingAccountStartFlashloanKeys = accounts.into();
    let ix = lending_account_start_flashloan_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_start_flashloan_invoke(
    accounts: LendingAccountStartFlashloanAccounts<'_, '_>,
    args: LendingAccountStartFlashloanIxArgs,
) -> ProgramResult {
    lending_account_start_flashloan_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_account_start_flashloan_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountStartFlashloanAccounts<'_, '_>,
    args: LendingAccountStartFlashloanIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountStartFlashloanKeys = accounts.into();
    let ix = lending_account_start_flashloan_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_start_flashloan_invoke_signed(
    accounts: LendingAccountStartFlashloanAccounts<'_, '_>,
    args: LendingAccountStartFlashloanIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_start_flashloan_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_account_start_flashloan_verify_account_keys(
    accounts: LendingAccountStartFlashloanAccounts<'_, '_>,
    keys: LendingAccountStartFlashloanKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.ixs_sysvar.key, keys.ixs_sysvar),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_start_flashloan_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountStartFlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_start_flashloan_verify_signer_privileges<'me, 'info>(
    accounts: LendingAccountStartFlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_account_start_flashloan_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountStartFlashloanAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_start_flashloan_verify_writable_privileges(accounts)?;
    lending_account_start_flashloan_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_ACCOUNT_WITHDRAW_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct LendingAccountWithdrawAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub bank_liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingAccountWithdrawKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub destination_token_account: Pubkey,
    pub bank_liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingAccountWithdrawAccounts<'_, '_>> for LendingAccountWithdrawKeys {
    fn from(accounts: LendingAccountWithdrawAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            destination_token_account: *accounts.destination_token_account.key,
            bank_liquidity_vault_authority: *accounts.bank_liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingAccountWithdrawKeys>
for [AccountMeta; LENDING_ACCOUNT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingAccountWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
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
impl From<[Pubkey; LENDING_ACCOUNT_WITHDRAW_IX_ACCOUNTS_LEN]>
for LendingAccountWithdrawKeys {
    fn from(pubkeys: [Pubkey; LENDING_ACCOUNT_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            destination_token_account: pubkeys[4],
            bank_liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<LendingAccountWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_ACCOUNT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingAccountWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.destination_token_account.clone(),
            accounts.bank_liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_ACCOUNT_WITHDRAW_IX_ACCOUNTS_LEN]>
for LendingAccountWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_ACCOUNT_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            destination_token_account: &arr[4],
            bank_liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const LENDING_ACCOUNT_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    36, 72, 74, 19, 210, 210, 192, 192,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountWithdrawIxArgs {
    pub amount: u64,
    pub withdraw_all: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountWithdrawIxData(pub LendingAccountWithdrawIxArgs);
impl From<LendingAccountWithdrawIxArgs> for LendingAccountWithdrawIxData {
    fn from(args: LendingAccountWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl LendingAccountWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingAccountWithdrawIxArgs {
                amount,
                withdraw_all,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_account_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingAccountWithdrawKeys,
    args: LendingAccountWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_ACCOUNT_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingAccountWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_account_withdraw_ix(
    keys: LendingAccountWithdrawKeys,
    args: LendingAccountWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    lending_account_withdraw_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_account_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountWithdrawAccounts<'_, '_>,
    args: LendingAccountWithdrawIxArgs,
) -> ProgramResult {
    let keys: LendingAccountWithdrawKeys = accounts.into();
    let ix = lending_account_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_account_withdraw_invoke(
    accounts: LendingAccountWithdrawAccounts<'_, '_>,
    args: LendingAccountWithdrawIxArgs,
) -> ProgramResult {
    lending_account_withdraw_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn lending_account_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingAccountWithdrawAccounts<'_, '_>,
    args: LendingAccountWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingAccountWithdrawKeys = accounts.into();
    let ix = lending_account_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_account_withdraw_invoke_signed(
    accounts: LendingAccountWithdrawAccounts<'_, '_>,
    args: LendingAccountWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_account_withdraw_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_account_withdraw_verify_account_keys(
    accounts: LendingAccountWithdrawAccounts<'_, '_>,
    keys: LendingAccountWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (
            *accounts.bank_liquidity_vault_authority.key,
            keys.bank_liquidity_vault_authority,
        ),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_account_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: LendingAccountWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.destination_token_account,
        accounts.liquidity_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_account_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: LendingAccountWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_account_withdraw_verify_account_privileges<'me, 'info>(
    accounts: LendingAccountWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_account_withdraw_verify_writable_privileges(accounts)?;
    lending_account_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_ACCRUE_BANK_INTEREST_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolAccrueBankInterestAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolAccrueBankInterestKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolAccrueBankInterestAccounts<'_, '_>>
for LendingPoolAccrueBankInterestKeys {
    fn from(accounts: LendingPoolAccrueBankInterestAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolAccrueBankInterestKeys>
for [AccountMeta; LENDING_POOL_ACCRUE_BANK_INTEREST_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolAccrueBankInterestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_ACCRUE_BANK_INTEREST_IX_ACCOUNTS_LEN]>
for LendingPoolAccrueBankInterestKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_ACCRUE_BANK_INTEREST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
        }
    }
}
impl<'info> From<LendingPoolAccrueBankInterestAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_ACCRUE_BANK_INTEREST_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolAccrueBankInterestAccounts<'_, 'info>) -> Self {
        [accounts.group.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_ACCRUE_BANK_INTEREST_IX_ACCOUNTS_LEN]>
for LendingPoolAccrueBankInterestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_ACCRUE_BANK_INTEREST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
        }
    }
}
pub const LENDING_POOL_ACCRUE_BANK_INTEREST_IX_DISCM: [u8; 8usize] = [
    108, 201, 30, 87, 47, 65, 97, 188,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAccrueBankInterestIxData;
impl LendingPoolAccrueBankInterestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ACCRUE_BANK_INTEREST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ACCRUE_BANK_INTEREST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_accrue_bank_interest_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolAccrueBankInterestKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_ACCRUE_BANK_INTEREST_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolAccrueBankInterestIxData.try_to_vec()?,
    })
}
pub fn lending_pool_accrue_bank_interest_ix(
    keys: LendingPoolAccrueBankInterestKeys,
) -> std::io::Result<Instruction> {
    lending_pool_accrue_bank_interest_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_pool_accrue_bank_interest_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAccrueBankInterestAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolAccrueBankInterestKeys = accounts.into();
    let ix = lending_pool_accrue_bank_interest_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_accrue_bank_interest_invoke(
    accounts: LendingPoolAccrueBankInterestAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_accrue_bank_interest_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn lending_pool_accrue_bank_interest_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAccrueBankInterestAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolAccrueBankInterestKeys = accounts.into();
    let ix = lending_pool_accrue_bank_interest_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_accrue_bank_interest_invoke_signed(
    accounts: LendingPoolAccrueBankInterestAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_accrue_bank_interest_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_accrue_bank_interest_verify_account_keys(
    accounts: LendingPoolAccrueBankInterestAccounts<'_, '_>,
    keys: LendingPoolAccrueBankInterestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_accrue_bank_interest_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolAccrueBankInterestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_accrue_bank_interest_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolAccrueBankInterestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_accrue_bank_interest_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_ADD_BANK_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolAddBankAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub global_fee_wallet: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
    pub fee_payer: Pubkey,
    pub fee_state: Pubkey,
    pub global_fee_wallet: Pubkey,
    pub bank_mint: Pubkey,
    pub bank: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LendingPoolAddBankAccounts<'_, '_>> for LendingPoolAddBankKeys {
    fn from(accounts: LendingPoolAddBankAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
            fee_payer: *accounts.fee_payer.key,
            fee_state: *accounts.fee_state.key,
            global_fee_wallet: *accounts.global_fee_wallet.key,
            bank_mint: *accounts.bank_mint.key,
            bank: *accounts.bank.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fee_vault: *accounts.fee_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LendingPoolAddBankKeys>
for [AccountMeta; LENDING_POOL_ADD_BANK_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolAddBankKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_fee_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
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
impl From<[Pubkey; LENDING_POOL_ADD_BANK_IX_ACCOUNTS_LEN]> for LendingPoolAddBankKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_ADD_BANK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
            fee_payer: pubkeys[2],
            fee_state: pubkeys[3],
            global_fee_wallet: pubkeys[4],
            bank_mint: pubkeys[5],
            bank: pubkeys[6],
            liquidity_vault_authority: pubkeys[7],
            liquidity_vault: pubkeys[8],
            insurance_vault_authority: pubkeys[9],
            insurance_vault: pubkeys[10],
            fee_vault_authority: pubkeys[11],
            fee_vault: pubkeys[12],
            token_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<LendingPoolAddBankAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_ADD_BANK_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolAddBankAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.admin.clone(),
            accounts.fee_payer.clone(),
            accounts.fee_state.clone(),
            accounts.global_fee_wallet.clone(),
            accounts.bank_mint.clone(),
            accounts.bank.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
            fee_payer: &arr[2],
            fee_state: &arr[3],
            global_fee_wallet: &arr[4],
            bank_mint: &arr[5],
            bank: &arr[6],
            liquidity_vault_authority: &arr[7],
            liquidity_vault: &arr[8],
            insurance_vault_authority: &arr[9],
            insurance_vault: &arr[10],
            fee_vault_authority: &arr[11],
            fee_vault: &arr[12],
            token_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const LENDING_POOL_ADD_BANK_IX_DISCM: [u8; 8usize] = [
    215, 68, 72, 78, 208, 218, 103, 182,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAddBankIxArgs {
    pub bank_config: BankConfigCompact,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankIxData(pub LendingPoolAddBankIxArgs);
impl From<LendingPoolAddBankIxArgs> for LendingPoolAddBankIxData {
    fn from(args: LendingPoolAddBankIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolAddBankIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ADD_BANK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_config = if reader.is_empty() {
            Default::default()
        } else {
            <BankConfigCompact>::deserialize(&mut reader)?
        };
        Ok(
            Self(LendingPoolAddBankIxArgs {
                bank_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ADD_BANK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_add_bank_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolAddBankKeys,
    args: LendingPoolAddBankIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_ADD_BANK_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingPoolAddBankIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_add_bank_ix(
    keys: LendingPoolAddBankKeys,
    args: LendingPoolAddBankIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_add_bank_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_add_bank_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankAccounts<'_, '_>,
    args: LendingPoolAddBankIxArgs,
) -> ProgramResult {
    let keys: LendingPoolAddBankKeys = accounts.into();
    let ix = lending_pool_add_bank_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_add_bank_invoke(
    accounts: LendingPoolAddBankAccounts<'_, '_>,
    args: LendingPoolAddBankIxArgs,
) -> ProgramResult {
    lending_pool_add_bank_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn lending_pool_add_bank_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankAccounts<'_, '_>,
    args: LendingPoolAddBankIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolAddBankKeys = accounts.into();
    let ix = lending_pool_add_bank_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_add_bank_invoke_signed(
    accounts: LendingPoolAddBankAccounts<'_, '_>,
    args: LendingPoolAddBankIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_add_bank_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_add_bank_verify_account_keys(
    accounts: LendingPoolAddBankAccounts<'_, '_>,
    keys: LendingPoolAddBankKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.global_fee_wallet.key, keys.global_fee_wallet),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.bank.key, keys.bank),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolAddBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_group,
        accounts.fee_payer,
        accounts.global_fee_wallet,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolAddBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.fee_payer, accounts.bank] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolAddBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_add_bank_verify_writable_privileges(accounts)?;
    lending_pool_add_bank_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_ADD_BANK_DRIFT_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolAddBankDriftAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub integration_acc_3: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankDriftKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub fee_payer: Pubkey,
    pub bank_mint: Pubkey,
    pub bank: Pubkey,
    pub integration_acc_1: Pubkey,
    pub integration_acc_2: Pubkey,
    pub integration_acc_3: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LendingPoolAddBankDriftAccounts<'_, '_>> for LendingPoolAddBankDriftKeys {
    fn from(accounts: LendingPoolAddBankDriftAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            fee_payer: *accounts.fee_payer.key,
            bank_mint: *accounts.bank_mint.key,
            bank: *accounts.bank.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            integration_acc_3: *accounts.integration_acc_3.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fee_vault: *accounts.fee_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LendingPoolAddBankDriftKeys>
for [AccountMeta; LENDING_POOL_ADD_BANK_DRIFT_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolAddBankDriftKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_3,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
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
impl From<[Pubkey; LENDING_POOL_ADD_BANK_DRIFT_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankDriftKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_ADD_BANK_DRIFT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            fee_payer: pubkeys[2],
            bank_mint: pubkeys[3],
            bank: pubkeys[4],
            integration_acc_1: pubkeys[5],
            integration_acc_2: pubkeys[6],
            integration_acc_3: pubkeys[7],
            liquidity_vault_authority: pubkeys[8],
            liquidity_vault: pubkeys[9],
            insurance_vault_authority: pubkeys[10],
            insurance_vault: pubkeys[11],
            fee_vault_authority: pubkeys[12],
            fee_vault: pubkeys[13],
            token_program: pubkeys[14],
            system_program: pubkeys[15],
        }
    }
}
impl<'info> From<LendingPoolAddBankDriftAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_ADD_BANK_DRIFT_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolAddBankDriftAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.admin.clone(),
            accounts.fee_payer.clone(),
            accounts.bank_mint.clone(),
            accounts.bank.clone(),
            accounts.integration_acc_1.clone(),
            accounts.integration_acc_2.clone(),
            accounts.integration_acc_3.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_DRIFT_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankDriftAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_DRIFT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            fee_payer: &arr[2],
            bank_mint: &arr[3],
            bank: &arr[4],
            integration_acc_1: &arr[5],
            integration_acc_2: &arr[6],
            integration_acc_3: &arr[7],
            liquidity_vault_authority: &arr[8],
            liquidity_vault: &arr[9],
            insurance_vault_authority: &arr[10],
            insurance_vault: &arr[11],
            fee_vault_authority: &arr[12],
            fee_vault: &arr[13],
            token_program: &arr[14],
            system_program: &arr[15],
        }
    }
}
pub const LENDING_POOL_ADD_BANK_DRIFT_IX_DISCM: [u8; 8usize] = [
    62, 63, 49, 48, 76, 55, 108, 155,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAddBankDriftIxArgs {
    pub bank_config: DriftConfigCompact,
    pub bank_seed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankDriftIxData(pub LendingPoolAddBankDriftIxArgs);
impl From<LendingPoolAddBankDriftIxArgs> for LendingPoolAddBankDriftIxData {
    fn from(args: LendingPoolAddBankDriftIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolAddBankDriftIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ADD_BANK_DRIFT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_config = if reader.is_empty() {
            Default::default()
        } else {
            <DriftConfigCompact>::deserialize(&mut reader)?
        };
        let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolAddBankDriftIxArgs {
                bank_config,
                bank_seed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ADD_BANK_DRIFT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bank_seed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_add_bank_drift_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolAddBankDriftKeys,
    args: LendingPoolAddBankDriftIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_ADD_BANK_DRIFT_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingPoolAddBankDriftIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_add_bank_drift_ix(
    keys: LendingPoolAddBankDriftKeys,
    args: LendingPoolAddBankDriftIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_add_bank_drift_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_add_bank_drift_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankDriftAccounts<'_, '_>,
    args: LendingPoolAddBankDriftIxArgs,
) -> ProgramResult {
    let keys: LendingPoolAddBankDriftKeys = accounts.into();
    let ix = lending_pool_add_bank_drift_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_add_bank_drift_invoke(
    accounts: LendingPoolAddBankDriftAccounts<'_, '_>,
    args: LendingPoolAddBankDriftIxArgs,
) -> ProgramResult {
    lending_pool_add_bank_drift_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_add_bank_drift_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankDriftAccounts<'_, '_>,
    args: LendingPoolAddBankDriftIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolAddBankDriftKeys = accounts.into();
    let ix = lending_pool_add_bank_drift_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_add_bank_drift_invoke_signed(
    accounts: LendingPoolAddBankDriftAccounts<'_, '_>,
    args: LendingPoolAddBankDriftIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_add_bank_drift_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_add_bank_drift_verify_account_keys(
    accounts: LendingPoolAddBankDriftAccounts<'_, '_>,
    keys: LendingPoolAddBankDriftKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.bank.key, keys.bank),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.integration_acc_3.key, keys.integration_acc_3),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_drift_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolAddBankDriftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.group,
        accounts.fee_payer,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_drift_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolAddBankDriftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_drift_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolAddBankDriftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_add_bank_drift_verify_writable_privileges(accounts)?;
    lending_pool_add_bank_drift_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_ADD_BANK_JUPLEND_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolAddBankJuplendAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub f_token_mint: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankJuplendKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub fee_payer: Pubkey,
    pub bank_mint: Pubkey,
    pub bank: Pubkey,
    pub integration_acc_1: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub f_token_mint: Pubkey,
    pub integration_acc_2: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LendingPoolAddBankJuplendAccounts<'_, '_>> for LendingPoolAddBankJuplendKeys {
    fn from(accounts: LendingPoolAddBankJuplendAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            fee_payer: *accounts.fee_payer.key,
            bank_mint: *accounts.bank_mint.key,
            bank: *accounts.bank.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fee_vault: *accounts.fee_vault.key,
            f_token_mint: *accounts.f_token_mint.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LendingPoolAddBankJuplendKeys>
for [AccountMeta; LENDING_POOL_ADD_BANK_JUPLEND_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolAddBankJuplendKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.f_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
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
impl From<[Pubkey; LENDING_POOL_ADD_BANK_JUPLEND_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankJuplendKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_ADD_BANK_JUPLEND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            fee_payer: pubkeys[2],
            bank_mint: pubkeys[3],
            bank: pubkeys[4],
            integration_acc_1: pubkeys[5],
            liquidity_vault_authority: pubkeys[6],
            liquidity_vault: pubkeys[7],
            insurance_vault_authority: pubkeys[8],
            insurance_vault: pubkeys[9],
            fee_vault_authority: pubkeys[10],
            fee_vault: pubkeys[11],
            f_token_mint: pubkeys[12],
            integration_acc_2: pubkeys[13],
            token_program: pubkeys[14],
            system_program: pubkeys[15],
        }
    }
}
impl<'info> From<LendingPoolAddBankJuplendAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_ADD_BANK_JUPLEND_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolAddBankJuplendAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.admin.clone(),
            accounts.fee_payer.clone(),
            accounts.bank_mint.clone(),
            accounts.bank.clone(),
            accounts.integration_acc_1.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.f_token_mint.clone(),
            accounts.integration_acc_2.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_JUPLEND_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankJuplendAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_JUPLEND_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            fee_payer: &arr[2],
            bank_mint: &arr[3],
            bank: &arr[4],
            integration_acc_1: &arr[5],
            liquidity_vault_authority: &arr[6],
            liquidity_vault: &arr[7],
            insurance_vault_authority: &arr[8],
            insurance_vault: &arr[9],
            fee_vault_authority: &arr[10],
            fee_vault: &arr[11],
            f_token_mint: &arr[12],
            integration_acc_2: &arr[13],
            token_program: &arr[14],
            system_program: &arr[15],
        }
    }
}
pub const LENDING_POOL_ADD_BANK_JUPLEND_IX_DISCM: [u8; 8usize] = [
    18, 208, 117, 90, 53, 111, 195, 41,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAddBankJuplendIxArgs {
    pub bank_config: JuplendConfigCompact,
    pub bank_seed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankJuplendIxData(pub LendingPoolAddBankJuplendIxArgs);
impl From<LendingPoolAddBankJuplendIxArgs> for LendingPoolAddBankJuplendIxData {
    fn from(args: LendingPoolAddBankJuplendIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolAddBankJuplendIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ADD_BANK_JUPLEND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_config = if reader.is_empty() {
            Default::default()
        } else {
            <JuplendConfigCompact>::deserialize(&mut reader)?
        };
        let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolAddBankJuplendIxArgs {
                bank_config,
                bank_seed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ADD_BANK_JUPLEND_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bank_seed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_add_bank_juplend_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolAddBankJuplendKeys,
    args: LendingPoolAddBankJuplendIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_ADD_BANK_JUPLEND_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolAddBankJuplendIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_add_bank_juplend_ix(
    keys: LendingPoolAddBankJuplendKeys,
    args: LendingPoolAddBankJuplendIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_add_bank_juplend_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_add_bank_juplend_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankJuplendAccounts<'_, '_>,
    args: LendingPoolAddBankJuplendIxArgs,
) -> ProgramResult {
    let keys: LendingPoolAddBankJuplendKeys = accounts.into();
    let ix = lending_pool_add_bank_juplend_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_add_bank_juplend_invoke(
    accounts: LendingPoolAddBankJuplendAccounts<'_, '_>,
    args: LendingPoolAddBankJuplendIxArgs,
) -> ProgramResult {
    lending_pool_add_bank_juplend_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_add_bank_juplend_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankJuplendAccounts<'_, '_>,
    args: LendingPoolAddBankJuplendIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolAddBankJuplendKeys = accounts.into();
    let ix = lending_pool_add_bank_juplend_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_add_bank_juplend_invoke_signed(
    accounts: LendingPoolAddBankJuplendAccounts<'_, '_>,
    args: LendingPoolAddBankJuplendIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_add_bank_juplend_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_add_bank_juplend_verify_account_keys(
    accounts: LendingPoolAddBankJuplendAccounts<'_, '_>,
    keys: LendingPoolAddBankJuplendKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.bank.key, keys.bank),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.f_token_mint.key, keys.f_token_mint),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_juplend_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolAddBankJuplendAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.group,
        accounts.fee_payer,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
        accounts.integration_acc_2,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_juplend_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolAddBankJuplendAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_juplend_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolAddBankJuplendAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_add_bank_juplend_verify_writable_privileges(accounts)?;
    lending_pool_add_bank_juplend_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_ADD_BANK_KAMINO_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolAddBankKaminoAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankKaminoKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub fee_payer: Pubkey,
    pub bank_mint: Pubkey,
    pub bank: Pubkey,
    pub integration_acc_1: Pubkey,
    pub integration_acc_2: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LendingPoolAddBankKaminoAccounts<'_, '_>> for LendingPoolAddBankKaminoKeys {
    fn from(accounts: LendingPoolAddBankKaminoAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            fee_payer: *accounts.fee_payer.key,
            bank_mint: *accounts.bank_mint.key,
            bank: *accounts.bank.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fee_vault: *accounts.fee_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LendingPoolAddBankKaminoKeys>
for [AccountMeta; LENDING_POOL_ADD_BANK_KAMINO_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolAddBankKaminoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
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
impl From<[Pubkey; LENDING_POOL_ADD_BANK_KAMINO_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankKaminoKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_ADD_BANK_KAMINO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            fee_payer: pubkeys[2],
            bank_mint: pubkeys[3],
            bank: pubkeys[4],
            integration_acc_1: pubkeys[5],
            integration_acc_2: pubkeys[6],
            liquidity_vault_authority: pubkeys[7],
            liquidity_vault: pubkeys[8],
            insurance_vault_authority: pubkeys[9],
            insurance_vault: pubkeys[10],
            fee_vault_authority: pubkeys[11],
            fee_vault: pubkeys[12],
            token_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<LendingPoolAddBankKaminoAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_ADD_BANK_KAMINO_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolAddBankKaminoAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.admin.clone(),
            accounts.fee_payer.clone(),
            accounts.bank_mint.clone(),
            accounts.bank.clone(),
            accounts.integration_acc_1.clone(),
            accounts.integration_acc_2.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_KAMINO_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankKaminoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_KAMINO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            fee_payer: &arr[2],
            bank_mint: &arr[3],
            bank: &arr[4],
            integration_acc_1: &arr[5],
            integration_acc_2: &arr[6],
            liquidity_vault_authority: &arr[7],
            liquidity_vault: &arr[8],
            insurance_vault_authority: &arr[9],
            insurance_vault: &arr[10],
            fee_vault_authority: &arr[11],
            fee_vault: &arr[12],
            token_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const LENDING_POOL_ADD_BANK_KAMINO_IX_DISCM: [u8; 8usize] = [
    118, 53, 16, 243, 255, 245, 149, 241,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAddBankKaminoIxArgs {
    pub bank_config: KaminoConfigCompact,
    pub bank_seed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankKaminoIxData(pub LendingPoolAddBankKaminoIxArgs);
impl From<LendingPoolAddBankKaminoIxArgs> for LendingPoolAddBankKaminoIxData {
    fn from(args: LendingPoolAddBankKaminoIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolAddBankKaminoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ADD_BANK_KAMINO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_config = if reader.is_empty() {
            Default::default()
        } else {
            <KaminoConfigCompact>::deserialize(&mut reader)?
        };
        let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolAddBankKaminoIxArgs {
                bank_config,
                bank_seed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ADD_BANK_KAMINO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bank_seed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_add_bank_kamino_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolAddBankKaminoKeys,
    args: LendingPoolAddBankKaminoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_ADD_BANK_KAMINO_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingPoolAddBankKaminoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_add_bank_kamino_ix(
    keys: LendingPoolAddBankKaminoKeys,
    args: LendingPoolAddBankKaminoIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_add_bank_kamino_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_add_bank_kamino_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankKaminoAccounts<'_, '_>,
    args: LendingPoolAddBankKaminoIxArgs,
) -> ProgramResult {
    let keys: LendingPoolAddBankKaminoKeys = accounts.into();
    let ix = lending_pool_add_bank_kamino_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_add_bank_kamino_invoke(
    accounts: LendingPoolAddBankKaminoAccounts<'_, '_>,
    args: LendingPoolAddBankKaminoIxArgs,
) -> ProgramResult {
    lending_pool_add_bank_kamino_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_add_bank_kamino_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankKaminoAccounts<'_, '_>,
    args: LendingPoolAddBankKaminoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolAddBankKaminoKeys = accounts.into();
    let ix = lending_pool_add_bank_kamino_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_add_bank_kamino_invoke_signed(
    accounts: LendingPoolAddBankKaminoAccounts<'_, '_>,
    args: LendingPoolAddBankKaminoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_add_bank_kamino_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_add_bank_kamino_verify_account_keys(
    accounts: LendingPoolAddBankKaminoAccounts<'_, '_>,
    keys: LendingPoolAddBankKaminoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.bank.key, keys.bank),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_kamino_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolAddBankKaminoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.group,
        accounts.fee_payer,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_kamino_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolAddBankKaminoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_kamino_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolAddBankKaminoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_add_bank_kamino_verify_writable_privileges(accounts)?;
    lending_pool_add_bank_kamino_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolAddBankPermissionlessAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub staked_settings: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub sol_pool: &'me AccountInfo<'info>,
    pub stake_pool: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankPermissionlessKeys {
    pub marginfi_group: Pubkey,
    pub staked_settings: Pubkey,
    pub fee_payer: Pubkey,
    pub bank_mint: Pubkey,
    pub sol_pool: Pubkey,
    pub stake_pool: Pubkey,
    pub bank: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LendingPoolAddBankPermissionlessAccounts<'_, '_>>
for LendingPoolAddBankPermissionlessKeys {
    fn from(accounts: LendingPoolAddBankPermissionlessAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            staked_settings: *accounts.staked_settings.key,
            fee_payer: *accounts.fee_payer.key,
            bank_mint: *accounts.bank_mint.key,
            sol_pool: *accounts.sol_pool.key,
            stake_pool: *accounts.stake_pool.key,
            bank: *accounts.bank.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fee_vault: *accounts.fee_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LendingPoolAddBankPermissionlessKeys>
for [AccountMeta; LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolAddBankPermissionlessKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staked_settings,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sol_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
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
impl From<[Pubkey; LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankPermissionlessKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            staked_settings: pubkeys[1],
            fee_payer: pubkeys[2],
            bank_mint: pubkeys[3],
            sol_pool: pubkeys[4],
            stake_pool: pubkeys[5],
            bank: pubkeys[6],
            liquidity_vault_authority: pubkeys[7],
            liquidity_vault: pubkeys[8],
            insurance_vault_authority: pubkeys[9],
            insurance_vault: pubkeys[10],
            fee_vault_authority: pubkeys[11],
            fee_vault: pubkeys[12],
            token_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<LendingPoolAddBankPermissionlessAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolAddBankPermissionlessAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.staked_settings.clone(),
            accounts.fee_payer.clone(),
            accounts.bank_mint.clone(),
            accounts.sol_pool.clone(),
            accounts.stake_pool.clone(),
            accounts.bank.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankPermissionlessAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            staked_settings: &arr[1],
            fee_payer: &arr[2],
            bank_mint: &arr[3],
            sol_pool: &arr[4],
            stake_pool: &arr[5],
            bank: &arr[6],
            liquidity_vault_authority: &arr[7],
            liquidity_vault: &arr[8],
            insurance_vault_authority: &arr[9],
            insurance_vault: &arr[10],
            fee_vault_authority: &arr[11],
            fee_vault: &arr[12],
            token_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_DISCM: [u8; 8usize] = [
    127, 187, 121, 34, 187, 167, 238, 102,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAddBankPermissionlessIxArgs {
    pub bank_seed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankPermissionlessIxData(
    pub LendingPoolAddBankPermissionlessIxArgs,
);
impl From<LendingPoolAddBankPermissionlessIxArgs>
for LendingPoolAddBankPermissionlessIxData {
    fn from(args: LendingPoolAddBankPermissionlessIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolAddBankPermissionlessIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolAddBankPermissionlessIxArgs {
                bank_seed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_seed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_add_bank_permissionless_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolAddBankPermissionlessKeys,
    args: LendingPoolAddBankPermissionlessIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_ADD_BANK_PERMISSIONLESS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolAddBankPermissionlessIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_add_bank_permissionless_ix(
    keys: LendingPoolAddBankPermissionlessKeys,
    args: LendingPoolAddBankPermissionlessIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_add_bank_permissionless_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn lending_pool_add_bank_permissionless_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankPermissionlessAccounts<'_, '_>,
    args: LendingPoolAddBankPermissionlessIxArgs,
) -> ProgramResult {
    let keys: LendingPoolAddBankPermissionlessKeys = accounts.into();
    let ix = lending_pool_add_bank_permissionless_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_add_bank_permissionless_invoke(
    accounts: LendingPoolAddBankPermissionlessAccounts<'_, '_>,
    args: LendingPoolAddBankPermissionlessIxArgs,
) -> ProgramResult {
    lending_pool_add_bank_permissionless_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_add_bank_permissionless_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankPermissionlessAccounts<'_, '_>,
    args: LendingPoolAddBankPermissionlessIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolAddBankPermissionlessKeys = accounts.into();
    let ix = lending_pool_add_bank_permissionless_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_add_bank_permissionless_invoke_signed(
    accounts: LendingPoolAddBankPermissionlessAccounts<'_, '_>,
    args: LendingPoolAddBankPermissionlessIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_add_bank_permissionless_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_add_bank_permissionless_verify_account_keys(
    accounts: LendingPoolAddBankPermissionlessAccounts<'_, '_>,
    keys: LendingPoolAddBankPermissionlessKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.staked_settings.key, keys.staked_settings),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.sol_pool.key, keys.sol_pool),
        (*accounts.stake_pool.key, keys.stake_pool),
        (*accounts.bank.key, keys.bank),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_permissionless_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolAddBankPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_group,
        accounts.fee_payer,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_permissionless_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolAddBankPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_permissionless_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolAddBankPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_add_bank_permissionless_verify_writable_privileges(accounts)?;
    lending_pool_add_bank_permissionless_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_ADD_BANK_SOLEND_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolAddBankSolendAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankSolendKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub fee_payer: Pubkey,
    pub bank_mint: Pubkey,
    pub bank: Pubkey,
    pub integration_acc_1: Pubkey,
    pub integration_acc_2: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LendingPoolAddBankSolendAccounts<'_, '_>> for LendingPoolAddBankSolendKeys {
    fn from(accounts: LendingPoolAddBankSolendAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            fee_payer: *accounts.fee_payer.key,
            bank_mint: *accounts.bank_mint.key,
            bank: *accounts.bank.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fee_vault: *accounts.fee_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LendingPoolAddBankSolendKeys>
for [AccountMeta; LENDING_POOL_ADD_BANK_SOLEND_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolAddBankSolendKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
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
impl From<[Pubkey; LENDING_POOL_ADD_BANK_SOLEND_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankSolendKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_ADD_BANK_SOLEND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            fee_payer: pubkeys[2],
            bank_mint: pubkeys[3],
            bank: pubkeys[4],
            integration_acc_1: pubkeys[5],
            integration_acc_2: pubkeys[6],
            liquidity_vault_authority: pubkeys[7],
            liquidity_vault: pubkeys[8],
            insurance_vault_authority: pubkeys[9],
            insurance_vault: pubkeys[10],
            fee_vault_authority: pubkeys[11],
            fee_vault: pubkeys[12],
            token_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<LendingPoolAddBankSolendAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_ADD_BANK_SOLEND_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolAddBankSolendAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.admin.clone(),
            accounts.fee_payer.clone(),
            accounts.bank_mint.clone(),
            accounts.bank.clone(),
            accounts.integration_acc_1.clone(),
            accounts.integration_acc_2.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_SOLEND_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankSolendAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_SOLEND_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            fee_payer: &arr[2],
            bank_mint: &arr[3],
            bank: &arr[4],
            integration_acc_1: &arr[5],
            integration_acc_2: &arr[6],
            liquidity_vault_authority: &arr[7],
            liquidity_vault: &arr[8],
            insurance_vault_authority: &arr[9],
            insurance_vault: &arr[10],
            fee_vault_authority: &arr[11],
            fee_vault: &arr[12],
            token_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const LENDING_POOL_ADD_BANK_SOLEND_IX_DISCM: [u8; 8usize] = [
    81, 233, 203, 199, 47, 226, 0, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAddBankSolendIxArgs {
    pub bank_config: SolendConfigCompact,
    pub bank_seed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankSolendIxData(pub LendingPoolAddBankSolendIxArgs);
impl From<LendingPoolAddBankSolendIxArgs> for LendingPoolAddBankSolendIxData {
    fn from(args: LendingPoolAddBankSolendIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolAddBankSolendIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ADD_BANK_SOLEND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_config = if reader.is_empty() {
            Default::default()
        } else {
            <SolendConfigCompact>::deserialize(&mut reader)?
        };
        let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolAddBankSolendIxArgs {
                bank_config,
                bank_seed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ADD_BANK_SOLEND_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bank_seed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_add_bank_solend_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolAddBankSolendKeys,
    args: LendingPoolAddBankSolendIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_ADD_BANK_SOLEND_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingPoolAddBankSolendIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_add_bank_solend_ix(
    keys: LendingPoolAddBankSolendKeys,
    args: LendingPoolAddBankSolendIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_add_bank_solend_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_add_bank_solend_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankSolendAccounts<'_, '_>,
    args: LendingPoolAddBankSolendIxArgs,
) -> ProgramResult {
    let keys: LendingPoolAddBankSolendKeys = accounts.into();
    let ix = lending_pool_add_bank_solend_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_add_bank_solend_invoke(
    accounts: LendingPoolAddBankSolendAccounts<'_, '_>,
    args: LendingPoolAddBankSolendIxArgs,
) -> ProgramResult {
    lending_pool_add_bank_solend_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_add_bank_solend_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankSolendAccounts<'_, '_>,
    args: LendingPoolAddBankSolendIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolAddBankSolendKeys = accounts.into();
    let ix = lending_pool_add_bank_solend_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_add_bank_solend_invoke_signed(
    accounts: LendingPoolAddBankSolendAccounts<'_, '_>,
    args: LendingPoolAddBankSolendIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_add_bank_solend_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_add_bank_solend_verify_account_keys(
    accounts: LendingPoolAddBankSolendAccounts<'_, '_>,
    keys: LendingPoolAddBankSolendKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.bank.key, keys.bank),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_solend_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolAddBankSolendAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.group,
        accounts.fee_payer,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_solend_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolAddBankSolendAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_solend_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolAddBankSolendAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_add_bank_solend_verify_writable_privileges(accounts)?;
    lending_pool_add_bank_solend_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_ADD_BANK_WITH_SEED_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolAddBankWithSeedAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub global_fee_wallet: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankWithSeedKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
    pub fee_payer: Pubkey,
    pub fee_state: Pubkey,
    pub global_fee_wallet: Pubkey,
    pub bank_mint: Pubkey,
    pub bank: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LendingPoolAddBankWithSeedAccounts<'_, '_>>
for LendingPoolAddBankWithSeedKeys {
    fn from(accounts: LendingPoolAddBankWithSeedAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
            fee_payer: *accounts.fee_payer.key,
            fee_state: *accounts.fee_state.key,
            global_fee_wallet: *accounts.global_fee_wallet.key,
            bank_mint: *accounts.bank_mint.key,
            bank: *accounts.bank.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fee_vault: *accounts.fee_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LendingPoolAddBankWithSeedKeys>
for [AccountMeta; LENDING_POOL_ADD_BANK_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolAddBankWithSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_fee_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
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
impl From<[Pubkey; LENDING_POOL_ADD_BANK_WITH_SEED_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankWithSeedKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_ADD_BANK_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
            fee_payer: pubkeys[2],
            fee_state: pubkeys[3],
            global_fee_wallet: pubkeys[4],
            bank_mint: pubkeys[5],
            bank: pubkeys[6],
            liquidity_vault_authority: pubkeys[7],
            liquidity_vault: pubkeys[8],
            insurance_vault_authority: pubkeys[9],
            insurance_vault: pubkeys[10],
            fee_vault_authority: pubkeys[11],
            fee_vault: pubkeys[12],
            token_program: pubkeys[13],
            system_program: pubkeys[14],
        }
    }
}
impl<'info> From<LendingPoolAddBankWithSeedAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_ADD_BANK_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolAddBankWithSeedAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.admin.clone(),
            accounts.fee_payer.clone(),
            accounts.fee_state.clone(),
            accounts.global_fee_wallet.clone(),
            accounts.bank_mint.clone(),
            accounts.bank.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_WITH_SEED_IX_ACCOUNTS_LEN]>
for LendingPoolAddBankWithSeedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_ADD_BANK_WITH_SEED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
            fee_payer: &arr[2],
            fee_state: &arr[3],
            global_fee_wallet: &arr[4],
            bank_mint: &arr[5],
            bank: &arr[6],
            liquidity_vault_authority: &arr[7],
            liquidity_vault: &arr[8],
            insurance_vault_authority: &arr[9],
            insurance_vault: &arr[10],
            fee_vault_authority: &arr[11],
            fee_vault: &arr[12],
            token_program: &arr[13],
            system_program: &arr[14],
        }
    }
}
pub const LENDING_POOL_ADD_BANK_WITH_SEED_IX_DISCM: [u8; 8usize] = [
    76, 211, 213, 171, 117, 78, 158, 76,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolAddBankWithSeedIxArgs {
    pub bank_config: BankConfigCompact,
    pub bank_seed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAddBankWithSeedIxData(pub LendingPoolAddBankWithSeedIxArgs);
impl From<LendingPoolAddBankWithSeedIxArgs> for LendingPoolAddBankWithSeedIxData {
    fn from(args: LendingPoolAddBankWithSeedIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolAddBankWithSeedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ADD_BANK_WITH_SEED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_config = if reader.is_empty() {
            Default::default()
        } else {
            <BankConfigCompact>::deserialize(&mut reader)?
        };
        let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolAddBankWithSeedIxArgs {
                bank_config,
                bank_seed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ADD_BANK_WITH_SEED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bank_seed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_add_bank_with_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolAddBankWithSeedKeys,
    args: LendingPoolAddBankWithSeedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_ADD_BANK_WITH_SEED_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolAddBankWithSeedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_add_bank_with_seed_ix(
    keys: LendingPoolAddBankWithSeedKeys,
    args: LendingPoolAddBankWithSeedIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_add_bank_with_seed_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_add_bank_with_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankWithSeedAccounts<'_, '_>,
    args: LendingPoolAddBankWithSeedIxArgs,
) -> ProgramResult {
    let keys: LendingPoolAddBankWithSeedKeys = accounts.into();
    let ix = lending_pool_add_bank_with_seed_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_add_bank_with_seed_invoke(
    accounts: LendingPoolAddBankWithSeedAccounts<'_, '_>,
    args: LendingPoolAddBankWithSeedIxArgs,
) -> ProgramResult {
    lending_pool_add_bank_with_seed_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_add_bank_with_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolAddBankWithSeedAccounts<'_, '_>,
    args: LendingPoolAddBankWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolAddBankWithSeedKeys = accounts.into();
    let ix = lending_pool_add_bank_with_seed_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_add_bank_with_seed_invoke_signed(
    accounts: LendingPoolAddBankWithSeedAccounts<'_, '_>,
    args: LendingPoolAddBankWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_add_bank_with_seed_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_add_bank_with_seed_verify_account_keys(
    accounts: LendingPoolAddBankWithSeedAccounts<'_, '_>,
    keys: LendingPoolAddBankWithSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.global_fee_wallet.key, keys.global_fee_wallet),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.bank.key, keys.bank),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_with_seed_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolAddBankWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_group,
        accounts.fee_payer,
        accounts.global_fee_wallet,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_with_seed_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolAddBankWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_add_bank_with_seed_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolAddBankWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_add_bank_with_seed_verify_writable_privileges(accounts)?;
    lending_pool_add_bank_with_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_CLONE_BANK_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolCloneBankAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub bank_mint: &'me AccountInfo<'info>,
    pub source_bank: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolCloneBankKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
    pub fee_payer: Pubkey,
    pub bank_mint: Pubkey,
    pub source_bank: Pubkey,
    pub bank: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LendingPoolCloneBankAccounts<'_, '_>> for LendingPoolCloneBankKeys {
    fn from(accounts: LendingPoolCloneBankAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
            fee_payer: *accounts.fee_payer.key,
            bank_mint: *accounts.bank_mint.key,
            source_bank: *accounts.source_bank.key,
            bank: *accounts.bank.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fee_vault: *accounts.fee_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LendingPoolCloneBankKeys>
for [AccountMeta; LENDING_POOL_CLONE_BANK_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolCloneBankKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
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
impl From<[Pubkey; LENDING_POOL_CLONE_BANK_IX_ACCOUNTS_LEN]>
for LendingPoolCloneBankKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_CLONE_BANK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
            fee_payer: pubkeys[2],
            bank_mint: pubkeys[3],
            source_bank: pubkeys[4],
            bank: pubkeys[5],
            liquidity_vault_authority: pubkeys[6],
            liquidity_vault: pubkeys[7],
            insurance_vault_authority: pubkeys[8],
            insurance_vault: pubkeys[9],
            fee_vault_authority: pubkeys[10],
            fee_vault: pubkeys[11],
            token_program: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<LendingPoolCloneBankAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_CLONE_BANK_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolCloneBankAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.admin.clone(),
            accounts.fee_payer.clone(),
            accounts.bank_mint.clone(),
            accounts.source_bank.clone(),
            accounts.bank.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LENDING_POOL_CLONE_BANK_IX_ACCOUNTS_LEN]>
for LendingPoolCloneBankAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_CLONE_BANK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
            fee_payer: &arr[2],
            bank_mint: &arr[3],
            source_bank: &arr[4],
            bank: &arr[5],
            liquidity_vault_authority: &arr[6],
            liquidity_vault: &arr[7],
            insurance_vault_authority: &arr[8],
            insurance_vault: &arr[9],
            fee_vault_authority: &arr[10],
            fee_vault: &arr[11],
            token_program: &arr[12],
            system_program: &arr[13],
        }
    }
}
pub const LENDING_POOL_CLONE_BANK_IX_DISCM: [u8; 8usize] = [
    214, 93, 17, 236, 177, 228, 78, 17,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolCloneBankIxArgs {
    pub bank_seed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolCloneBankIxData(pub LendingPoolCloneBankIxArgs);
impl From<LendingPoolCloneBankIxArgs> for LendingPoolCloneBankIxData {
    fn from(args: LendingPoolCloneBankIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolCloneBankIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_CLONE_BANK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolCloneBankIxArgs {
                bank_seed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_CLONE_BANK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_seed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_clone_bank_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolCloneBankKeys,
    args: LendingPoolCloneBankIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_CLONE_BANK_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingPoolCloneBankIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_clone_bank_ix(
    keys: LendingPoolCloneBankKeys,
    args: LendingPoolCloneBankIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_clone_bank_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_clone_bank_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolCloneBankAccounts<'_, '_>,
    args: LendingPoolCloneBankIxArgs,
) -> ProgramResult {
    let keys: LendingPoolCloneBankKeys = accounts.into();
    let ix = lending_pool_clone_bank_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_clone_bank_invoke(
    accounts: LendingPoolCloneBankAccounts<'_, '_>,
    args: LendingPoolCloneBankIxArgs,
) -> ProgramResult {
    lending_pool_clone_bank_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn lending_pool_clone_bank_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolCloneBankAccounts<'_, '_>,
    args: LendingPoolCloneBankIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolCloneBankKeys = accounts.into();
    let ix = lending_pool_clone_bank_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_clone_bank_invoke_signed(
    accounts: LendingPoolCloneBankAccounts<'_, '_>,
    args: LendingPoolCloneBankIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_clone_bank_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_clone_bank_verify_account_keys(
    accounts: LendingPoolCloneBankAccounts<'_, '_>,
    keys: LendingPoolCloneBankKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.bank_mint.key, keys.bank_mint),
        (*accounts.source_bank.key, keys.source_bank),
        (*accounts.bank.key, keys.bank),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_clone_bank_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolCloneBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_group,
        accounts.admin,
        accounts.fee_payer,
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_clone_bank_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolCloneBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_clone_bank_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolCloneBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_clone_bank_verify_writable_privileges(accounts)?;
    lending_pool_clone_bank_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_CLONE_EMODE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolCloneEmodeAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub copy_from_bank: &'me AccountInfo<'info>,
    pub copy_to_bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolCloneEmodeKeys {
    pub group: Pubkey,
    pub signer: Pubkey,
    pub copy_from_bank: Pubkey,
    pub copy_to_bank: Pubkey,
}
impl From<LendingPoolCloneEmodeAccounts<'_, '_>> for LendingPoolCloneEmodeKeys {
    fn from(accounts: LendingPoolCloneEmodeAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            signer: *accounts.signer.key,
            copy_from_bank: *accounts.copy_from_bank.key,
            copy_to_bank: *accounts.copy_to_bank.key,
        }
    }
}
impl From<LendingPoolCloneEmodeKeys>
for [AccountMeta; LENDING_POOL_CLONE_EMODE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolCloneEmodeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.copy_from_bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.copy_to_bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_CLONE_EMODE_IX_ACCOUNTS_LEN]>
for LendingPoolCloneEmodeKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_CLONE_EMODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            signer: pubkeys[1],
            copy_from_bank: pubkeys[2],
            copy_to_bank: pubkeys[3],
        }
    }
}
impl<'info> From<LendingPoolCloneEmodeAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_CLONE_EMODE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolCloneEmodeAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.signer.clone(),
            accounts.copy_from_bank.clone(),
            accounts.copy_to_bank.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_CLONE_EMODE_IX_ACCOUNTS_LEN]>
for LendingPoolCloneEmodeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_CLONE_EMODE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            signer: &arr[1],
            copy_from_bank: &arr[2],
            copy_to_bank: &arr[3],
        }
    }
}
pub const LENDING_POOL_CLONE_EMODE_IX_DISCM: [u8; 8usize] = [
    146, 167, 94, 106, 184, 202, 15, 10,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolCloneEmodeIxData;
impl LendingPoolCloneEmodeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_CLONE_EMODE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_CLONE_EMODE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_clone_emode_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolCloneEmodeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_CLONE_EMODE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolCloneEmodeIxData.try_to_vec()?,
    })
}
pub fn lending_pool_clone_emode_ix(
    keys: LendingPoolCloneEmodeKeys,
) -> std::io::Result<Instruction> {
    lending_pool_clone_emode_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_pool_clone_emode_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolCloneEmodeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolCloneEmodeKeys = accounts.into();
    let ix = lending_pool_clone_emode_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_clone_emode_invoke(
    accounts: LendingPoolCloneEmodeAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_clone_emode_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn lending_pool_clone_emode_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolCloneEmodeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolCloneEmodeKeys = accounts.into();
    let ix = lending_pool_clone_emode_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_clone_emode_invoke_signed(
    accounts: LendingPoolCloneEmodeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_clone_emode_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_clone_emode_verify_account_keys(
    accounts: LendingPoolCloneEmodeAccounts<'_, '_>,
    keys: LendingPoolCloneEmodeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.signer.key, keys.signer),
        (*accounts.copy_from_bank.key, keys.copy_from_bank),
        (*accounts.copy_to_bank.key, keys.copy_to_bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_clone_emode_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolCloneEmodeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.copy_to_bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_clone_emode_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolCloneEmodeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_clone_emode_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolCloneEmodeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_clone_emode_verify_writable_privileges(accounts)?;
    lending_pool_clone_emode_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_CLOSE_BANK_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolCloseBankAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolCloseBankKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub admin: Pubkey,
}
impl From<LendingPoolCloseBankAccounts<'_, '_>> for LendingPoolCloseBankKeys {
    fn from(accounts: LendingPoolCloseBankAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<LendingPoolCloseBankKeys>
for [AccountMeta; LENDING_POOL_CLOSE_BANK_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolCloseBankKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_CLOSE_BANK_IX_ACCOUNTS_LEN]>
for LendingPoolCloseBankKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_CLOSE_BANK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<LendingPoolCloseBankAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_CLOSE_BANK_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolCloseBankAccounts<'_, 'info>) -> Self {
        [accounts.group.clone(), accounts.bank.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LENDING_POOL_CLOSE_BANK_IX_ACCOUNTS_LEN]>
for LendingPoolCloseBankAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_CLOSE_BANK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const LENDING_POOL_CLOSE_BANK_IX_DISCM: [u8; 8usize] = [
    22, 115, 7, 130, 227, 85, 0, 47,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolCloseBankIxData;
impl LendingPoolCloseBankIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_CLOSE_BANK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_CLOSE_BANK_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_close_bank_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolCloseBankKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_CLOSE_BANK_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolCloseBankIxData.try_to_vec()?,
    })
}
pub fn lending_pool_close_bank_ix(
    keys: LendingPoolCloseBankKeys,
) -> std::io::Result<Instruction> {
    lending_pool_close_bank_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_pool_close_bank_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolCloseBankAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolCloseBankKeys = accounts.into();
    let ix = lending_pool_close_bank_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_close_bank_invoke(
    accounts: LendingPoolCloseBankAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_close_bank_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn lending_pool_close_bank_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolCloseBankAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolCloseBankKeys = accounts.into();
    let ix = lending_pool_close_bank_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_close_bank_invoke_signed(
    accounts: LendingPoolCloseBankAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_close_bank_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_close_bank_verify_account_keys(
    accounts: LendingPoolCloseBankAccounts<'_, '_>,
    keys: LendingPoolCloseBankKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_close_bank_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolCloseBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.group, accounts.bank, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_close_bank_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolCloseBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_close_bank_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolCloseBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_close_bank_verify_writable_privileges(accounts)?;
    lending_pool_close_bank_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_COLLECT_BANK_FEES_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolCollectBankFeesAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub fee_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolCollectBankFeesKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_state: Pubkey,
    pub fee_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingPoolCollectBankFeesAccounts<'_, '_>>
for LendingPoolCollectBankFeesKeys {
    fn from(accounts: LendingPoolCollectBankFeesAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault: *accounts.insurance_vault.key,
            fee_vault: *accounts.fee_vault.key,
            fee_state: *accounts.fee_state.key,
            fee_ata: *accounts.fee_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingPoolCollectBankFeesKeys>
for [AccountMeta; LENDING_POOL_COLLECT_BANK_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolCollectBankFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_ata,
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
impl From<[Pubkey; LENDING_POOL_COLLECT_BANK_FEES_IX_ACCOUNTS_LEN]>
for LendingPoolCollectBankFeesKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_COLLECT_BANK_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
            liquidity_vault_authority: pubkeys[2],
            liquidity_vault: pubkeys[3],
            insurance_vault: pubkeys[4],
            fee_vault: pubkeys[5],
            fee_state: pubkeys[6],
            fee_ata: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<LendingPoolCollectBankFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_COLLECT_BANK_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolCollectBankFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.bank.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_state.clone(),
            accounts.fee_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_COLLECT_BANK_FEES_IX_ACCOUNTS_LEN]>
for LendingPoolCollectBankFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_COLLECT_BANK_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
            liquidity_vault_authority: &arr[2],
            liquidity_vault: &arr[3],
            insurance_vault: &arr[4],
            fee_vault: &arr[5],
            fee_state: &arr[6],
            fee_ata: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const LENDING_POOL_COLLECT_BANK_FEES_IX_DISCM: [u8; 8usize] = [
    201, 5, 215, 116, 230, 92, 75, 150,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolCollectBankFeesIxData;
impl LendingPoolCollectBankFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_COLLECT_BANK_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_COLLECT_BANK_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_collect_bank_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolCollectBankFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_COLLECT_BANK_FEES_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolCollectBankFeesIxData.try_to_vec()?,
    })
}
pub fn lending_pool_collect_bank_fees_ix(
    keys: LendingPoolCollectBankFeesKeys,
) -> std::io::Result<Instruction> {
    lending_pool_collect_bank_fees_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_pool_collect_bank_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolCollectBankFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolCollectBankFeesKeys = accounts.into();
    let ix = lending_pool_collect_bank_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_collect_bank_fees_invoke(
    accounts: LendingPoolCollectBankFeesAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_collect_bank_fees_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn lending_pool_collect_bank_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolCollectBankFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolCollectBankFeesKeys = accounts.into();
    let ix = lending_pool_collect_bank_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_collect_bank_fees_invoke_signed(
    accounts: LendingPoolCollectBankFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_collect_bank_fees_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_collect_bank_fees_verify_account_keys(
    accounts: LendingPoolCollectBankFeesAccounts<'_, '_>,
    keys: LendingPoolCollectBankFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.fee_ata.key, keys.fee_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_collect_bank_fees_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolCollectBankFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank,
        accounts.liquidity_vault,
        accounts.insurance_vault,
        accounts.fee_vault,
        accounts.fee_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_collect_bank_fees_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolCollectBankFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_collect_bank_fees_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_CONFIGURE_BANK_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolConfigureBankAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolConfigureBankAccounts<'_, '_>> for LendingPoolConfigureBankKeys {
    fn from(accounts: LendingPoolConfigureBankAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolConfigureBankKeys>
for [AccountMeta; LENDING_POOL_CONFIGURE_BANK_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolConfigureBankKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_CONFIGURE_BANK_IX_ACCOUNTS_LEN]>
for LendingPoolConfigureBankKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_CONFIGURE_BANK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<LendingPoolConfigureBankAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolConfigureBankAccounts<'_, 'info>) -> Self {
        [accounts.group.clone(), accounts.admin.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_IX_ACCOUNTS_LEN]>
for LendingPoolConfigureBankAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const LENDING_POOL_CONFIGURE_BANK_IX_DISCM: [u8; 8usize] = [
    121, 173, 156, 40, 93, 148, 56, 237,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolConfigureBankIxArgs {
    pub bank_config_opt: BankConfigOpt,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankIxData(pub LendingPoolConfigureBankIxArgs);
impl From<LendingPoolConfigureBankIxArgs> for LendingPoolConfigureBankIxData {
    fn from(args: LendingPoolConfigureBankIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolConfigureBankIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_CONFIGURE_BANK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_config_opt = if reader.is_empty() {
            Default::default()
        } else {
            <BankConfigOpt>::deserialize(&mut reader)?
        };
        Ok(
            Self(LendingPoolConfigureBankIxArgs {
                bank_config_opt,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_CONFIGURE_BANK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_config_opt, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_configure_bank_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolConfigureBankKeys,
    args: LendingPoolConfigureBankIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_CONFIGURE_BANK_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingPoolConfigureBankIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_configure_bank_ix(
    keys: LendingPoolConfigureBankKeys,
    args: LendingPoolConfigureBankIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_configure_bank_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_configure_bank_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankAccounts<'_, '_>,
    args: LendingPoolConfigureBankIxArgs,
) -> ProgramResult {
    let keys: LendingPoolConfigureBankKeys = accounts.into();
    let ix = lending_pool_configure_bank_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_configure_bank_invoke(
    accounts: LendingPoolConfigureBankAccounts<'_, '_>,
    args: LendingPoolConfigureBankIxArgs,
) -> ProgramResult {
    lending_pool_configure_bank_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_configure_bank_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankAccounts<'_, '_>,
    args: LendingPoolConfigureBankIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolConfigureBankKeys = accounts.into();
    let ix = lending_pool_configure_bank_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_configure_bank_invoke_signed(
    accounts: LendingPoolConfigureBankAccounts<'_, '_>,
    args: LendingPoolConfigureBankIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_configure_bank_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_configure_bank_verify_account_keys(
    accounts: LendingPoolConfigureBankAccounts<'_, '_>,
    keys: LendingPoolConfigureBankKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_configure_bank_verify_writable_privileges(accounts)?;
    lending_pool_configure_bank_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_CONFIGURE_BANK_EMODE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolConfigureBankEmodeAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub emode_admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankEmodeKeys {
    pub group: Pubkey,
    pub emode_admin: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolConfigureBankEmodeAccounts<'_, '_>>
for LendingPoolConfigureBankEmodeKeys {
    fn from(accounts: LendingPoolConfigureBankEmodeAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            emode_admin: *accounts.emode_admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolConfigureBankEmodeKeys>
for [AccountMeta; LENDING_POOL_CONFIGURE_BANK_EMODE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolConfigureBankEmodeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.emode_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_CONFIGURE_BANK_EMODE_IX_ACCOUNTS_LEN]>
for LendingPoolConfigureBankEmodeKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_CONFIGURE_BANK_EMODE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            emode_admin: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<LendingPoolConfigureBankEmodeAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_EMODE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolConfigureBankEmodeAccounts<'_, 'info>) -> Self {
        [accounts.group.clone(), accounts.emode_admin.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_EMODE_IX_ACCOUNTS_LEN]>
for LendingPoolConfigureBankEmodeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_EMODE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            emode_admin: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const LENDING_POOL_CONFIGURE_BANK_EMODE_IX_DISCM: [u8; 8usize] = [
    17, 175, 91, 57, 239, 86, 49, 71,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolConfigureBankEmodeIxArgs {
    pub emode_tag: u16,
    pub entries: [EmodeEntry; 10],
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankEmodeIxData(pub LendingPoolConfigureBankEmodeIxArgs);
impl From<LendingPoolConfigureBankEmodeIxArgs> for LendingPoolConfigureBankEmodeIxData {
    fn from(args: LendingPoolConfigureBankEmodeIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolConfigureBankEmodeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_CONFIGURE_BANK_EMODE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let emode_tag: u16 = crate::borsh_de_or_default(&mut reader)?;
        let entries: [EmodeEntry; 10] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolConfigureBankEmodeIxArgs {
                emode_tag,
                entries,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_CONFIGURE_BANK_EMODE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.emode_tag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.entries, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_configure_bank_emode_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolConfigureBankEmodeKeys,
    args: LendingPoolConfigureBankEmodeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_CONFIGURE_BANK_EMODE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolConfigureBankEmodeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_configure_bank_emode_ix(
    keys: LendingPoolConfigureBankEmodeKeys,
    args: LendingPoolConfigureBankEmodeIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_configure_bank_emode_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_configure_bank_emode_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankEmodeAccounts<'_, '_>,
    args: LendingPoolConfigureBankEmodeIxArgs,
) -> ProgramResult {
    let keys: LendingPoolConfigureBankEmodeKeys = accounts.into();
    let ix = lending_pool_configure_bank_emode_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_configure_bank_emode_invoke(
    accounts: LendingPoolConfigureBankEmodeAccounts<'_, '_>,
    args: LendingPoolConfigureBankEmodeIxArgs,
) -> ProgramResult {
    lending_pool_configure_bank_emode_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_configure_bank_emode_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankEmodeAccounts<'_, '_>,
    args: LendingPoolConfigureBankEmodeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolConfigureBankEmodeKeys = accounts.into();
    let ix = lending_pool_configure_bank_emode_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_configure_bank_emode_invoke_signed(
    accounts: LendingPoolConfigureBankEmodeAccounts<'_, '_>,
    args: LendingPoolConfigureBankEmodeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_configure_bank_emode_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_configure_bank_emode_verify_account_keys(
    accounts: LendingPoolConfigureBankEmodeAccounts<'_, '_>,
    keys: LendingPoolConfigureBankEmodeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.emode_admin.key, keys.emode_admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_emode_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankEmodeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_emode_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankEmodeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.emode_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_emode_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankEmodeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_configure_bank_emode_verify_writable_privileges(accounts)?;
    lending_pool_configure_bank_emode_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolConfigureBankInterestOnlyAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub delegate_curve_admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankInterestOnlyKeys {
    pub group: Pubkey,
    pub delegate_curve_admin: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolConfigureBankInterestOnlyAccounts<'_, '_>>
for LendingPoolConfigureBankInterestOnlyKeys {
    fn from(accounts: LendingPoolConfigureBankInterestOnlyAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            delegate_curve_admin: *accounts.delegate_curve_admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolConfigureBankInterestOnlyKeys>
for [AccountMeta; LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolConfigureBankInterestOnlyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegate_curve_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_ACCOUNTS_LEN]>
for LendingPoolConfigureBankInterestOnlyKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            delegate_curve_admin: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<LendingPoolConfigureBankInterestOnlyAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolConfigureBankInterestOnlyAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.delegate_curve_admin.clone(),
            accounts.bank.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_ACCOUNTS_LEN],
> for LendingPoolConfigureBankInterestOnlyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            delegate_curve_admin: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_DISCM: [u8; 8usize] = [
    245, 107, 83, 38, 103, 219, 163, 241,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolConfigureBankInterestOnlyIxArgs {
    pub interest_rate_config: InterestRateConfigOpt,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankInterestOnlyIxData(
    pub LendingPoolConfigureBankInterestOnlyIxArgs,
);
impl From<LendingPoolConfigureBankInterestOnlyIxArgs>
for LendingPoolConfigureBankInterestOnlyIxData {
    fn from(args: LendingPoolConfigureBankInterestOnlyIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolConfigureBankInterestOnlyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let interest_rate_config = if reader.is_empty() {
            Default::default()
        } else {
            <InterestRateConfigOpt>::deserialize(&mut reader)?
        };
        Ok(
            Self(LendingPoolConfigureBankInterestOnlyIxArgs {
                interest_rate_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.interest_rate_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_configure_bank_interest_only_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolConfigureBankInterestOnlyKeys,
    args: LendingPoolConfigureBankInterestOnlyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_CONFIGURE_BANK_INTEREST_ONLY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolConfigureBankInterestOnlyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_configure_bank_interest_only_ix(
    keys: LendingPoolConfigureBankInterestOnlyKeys,
    args: LendingPoolConfigureBankInterestOnlyIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_configure_bank_interest_only_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn lending_pool_configure_bank_interest_only_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankInterestOnlyAccounts<'_, '_>,
    args: LendingPoolConfigureBankInterestOnlyIxArgs,
) -> ProgramResult {
    let keys: LendingPoolConfigureBankInterestOnlyKeys = accounts.into();
    let ix = lending_pool_configure_bank_interest_only_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_configure_bank_interest_only_invoke(
    accounts: LendingPoolConfigureBankInterestOnlyAccounts<'_, '_>,
    args: LendingPoolConfigureBankInterestOnlyIxArgs,
) -> ProgramResult {
    lending_pool_configure_bank_interest_only_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_configure_bank_interest_only_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankInterestOnlyAccounts<'_, '_>,
    args: LendingPoolConfigureBankInterestOnlyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolConfigureBankInterestOnlyKeys = accounts.into();
    let ix = lending_pool_configure_bank_interest_only_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_configure_bank_interest_only_invoke_signed(
    accounts: LendingPoolConfigureBankInterestOnlyAccounts<'_, '_>,
    args: LendingPoolConfigureBankInterestOnlyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_configure_bank_interest_only_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_configure_bank_interest_only_verify_account_keys(
    accounts: LendingPoolConfigureBankInterestOnlyAccounts<'_, '_>,
    keys: LendingPoolConfigureBankInterestOnlyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.delegate_curve_admin.key, keys.delegate_curve_admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_interest_only_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankInterestOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_interest_only_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankInterestOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_curve_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_interest_only_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankInterestOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_configure_bank_interest_only_verify_writable_privileges(accounts)?;
    lending_pool_configure_bank_interest_only_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolConfigureBankLimitsOnlyAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub delegate_limit_admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankLimitsOnlyKeys {
    pub group: Pubkey,
    pub delegate_limit_admin: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolConfigureBankLimitsOnlyAccounts<'_, '_>>
for LendingPoolConfigureBankLimitsOnlyKeys {
    fn from(accounts: LendingPoolConfigureBankLimitsOnlyAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            delegate_limit_admin: *accounts.delegate_limit_admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolConfigureBankLimitsOnlyKeys>
for [AccountMeta; LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolConfigureBankLimitsOnlyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegate_limit_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_ACCOUNTS_LEN]>
for LendingPoolConfigureBankLimitsOnlyKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            delegate_limit_admin: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<LendingPoolConfigureBankLimitsOnlyAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.delegate_limit_admin.clone(),
            accounts.bank.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_ACCOUNTS_LEN],
> for LendingPoolConfigureBankLimitsOnlyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            delegate_limit_admin: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_DISCM: [u8; 8usize] = [
    157, 196, 221, 200, 202, 62, 84, 21,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolConfigureBankLimitsOnlyIxArgs {
    pub deposit_limit: Option<u64>,
    pub borrow_limit: Option<u64>,
    pub total_asset_value_init_limit: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankLimitsOnlyIxData(
    pub LendingPoolConfigureBankLimitsOnlyIxArgs,
);
impl From<LendingPoolConfigureBankLimitsOnlyIxArgs>
for LendingPoolConfigureBankLimitsOnlyIxData {
    fn from(args: LendingPoolConfigureBankLimitsOnlyIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolConfigureBankLimitsOnlyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let deposit_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let borrow_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(LendingPoolConfigureBankLimitsOnlyIxArgs {
                deposit_limit,
                borrow_limit,
                total_asset_value_init_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.deposit_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.borrow_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.total_asset_value_init_limit,
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
pub fn lending_pool_configure_bank_limits_only_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolConfigureBankLimitsOnlyKeys,
    args: LendingPoolConfigureBankLimitsOnlyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_CONFIGURE_BANK_LIMITS_ONLY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolConfigureBankLimitsOnlyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_configure_bank_limits_only_ix(
    keys: LendingPoolConfigureBankLimitsOnlyKeys,
    args: LendingPoolConfigureBankLimitsOnlyIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_configure_bank_limits_only_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn lending_pool_configure_bank_limits_only_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'_, '_>,
    args: LendingPoolConfigureBankLimitsOnlyIxArgs,
) -> ProgramResult {
    let keys: LendingPoolConfigureBankLimitsOnlyKeys = accounts.into();
    let ix = lending_pool_configure_bank_limits_only_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_configure_bank_limits_only_invoke(
    accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'_, '_>,
    args: LendingPoolConfigureBankLimitsOnlyIxArgs,
) -> ProgramResult {
    lending_pool_configure_bank_limits_only_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_configure_bank_limits_only_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'_, '_>,
    args: LendingPoolConfigureBankLimitsOnlyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolConfigureBankLimitsOnlyKeys = accounts.into();
    let ix = lending_pool_configure_bank_limits_only_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_configure_bank_limits_only_invoke_signed(
    accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'_, '_>,
    args: LendingPoolConfigureBankLimitsOnlyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_configure_bank_limits_only_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_configure_bank_limits_only_verify_account_keys(
    accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'_, '_>,
    keys: LendingPoolConfigureBankLimitsOnlyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.delegate_limit_admin.key, keys.delegate_limit_admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_limits_only_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_limits_only_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_limit_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_limits_only_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankLimitsOnlyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_configure_bank_limits_only_verify_writable_privileges(accounts)?;
    lending_pool_configure_bank_limits_only_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolConfigureBankOracleAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankOracleKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolConfigureBankOracleAccounts<'_, '_>>
for LendingPoolConfigureBankOracleKeys {
    fn from(accounts: LendingPoolConfigureBankOracleAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolConfigureBankOracleKeys>
for [AccountMeta; LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolConfigureBankOracleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_ACCOUNTS_LEN]>
for LendingPoolConfigureBankOracleKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<LendingPoolConfigureBankOracleAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolConfigureBankOracleAccounts<'_, 'info>) -> Self {
        [accounts.group.clone(), accounts.admin.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_ACCOUNTS_LEN]>
for LendingPoolConfigureBankOracleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_DISCM: [u8; 8usize] = [
    209, 82, 255, 171, 124, 21, 71, 81,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolConfigureBankOracleIxArgs {
    pub setup: u8,
    pub oracle: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolConfigureBankOracleIxData(
    pub LendingPoolConfigureBankOracleIxArgs,
);
impl From<LendingPoolConfigureBankOracleIxArgs>
for LendingPoolConfigureBankOracleIxData {
    fn from(args: LendingPoolConfigureBankOracleIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolConfigureBankOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let setup: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolConfigureBankOracleIxArgs {
                setup,
                oracle,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.setup, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_configure_bank_oracle_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolConfigureBankOracleKeys,
    args: LendingPoolConfigureBankOracleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_CONFIGURE_BANK_ORACLE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolConfigureBankOracleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_configure_bank_oracle_ix(
    keys: LendingPoolConfigureBankOracleKeys,
    args: LendingPoolConfigureBankOracleIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_configure_bank_oracle_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn lending_pool_configure_bank_oracle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankOracleAccounts<'_, '_>,
    args: LendingPoolConfigureBankOracleIxArgs,
) -> ProgramResult {
    let keys: LendingPoolConfigureBankOracleKeys = accounts.into();
    let ix = lending_pool_configure_bank_oracle_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_configure_bank_oracle_invoke(
    accounts: LendingPoolConfigureBankOracleAccounts<'_, '_>,
    args: LendingPoolConfigureBankOracleIxArgs,
) -> ProgramResult {
    lending_pool_configure_bank_oracle_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_configure_bank_oracle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolConfigureBankOracleAccounts<'_, '_>,
    args: LendingPoolConfigureBankOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolConfigureBankOracleKeys = accounts.into();
    let ix = lending_pool_configure_bank_oracle_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_configure_bank_oracle_invoke_signed(
    accounts: LendingPoolConfigureBankOracleAccounts<'_, '_>,
    args: LendingPoolConfigureBankOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_configure_bank_oracle_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_configure_bank_oracle_verify_account_keys(
    accounts: LendingPoolConfigureBankOracleAccounts<'_, '_>,
    keys: LendingPoolConfigureBankOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_oracle_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_oracle_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_configure_bank_oracle_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolConfigureBankOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_configure_bank_oracle_verify_writable_privileges(accounts)?;
    lending_pool_configure_bank_oracle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolForceTokenlessRepayCompleteAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub risk_admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolForceTokenlessRepayCompleteKeys {
    pub group: Pubkey,
    pub risk_admin: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolForceTokenlessRepayCompleteAccounts<'_, '_>>
for LendingPoolForceTokenlessRepayCompleteKeys {
    fn from(accounts: LendingPoolForceTokenlessRepayCompleteAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            risk_admin: *accounts.risk_admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolForceTokenlessRepayCompleteKeys>
for [AccountMeta; LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolForceTokenlessRepayCompleteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.risk_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_ACCOUNTS_LEN]>
for LendingPoolForceTokenlessRepayCompleteKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            risk_admin: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<LendingPoolForceTokenlessRepayCompleteAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'_, 'info>,
    ) -> Self {
        [accounts.group.clone(), accounts.risk_admin.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_ACCOUNTS_LEN],
> for LendingPoolForceTokenlessRepayCompleteAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            risk_admin: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_DISCM: [u8; 8usize] = [
    15, 203, 147, 232, 199, 14, 231, 37,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolForceTokenlessRepayCompleteIxData;
impl LendingPoolForceTokenlessRepayCompleteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_force_tokenless_repay_complete_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolForceTokenlessRepayCompleteKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_FORCE_TOKENLESS_REPAY_COMPLETE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolForceTokenlessRepayCompleteIxData.try_to_vec()?,
    })
}
pub fn lending_pool_force_tokenless_repay_complete_ix(
    keys: LendingPoolForceTokenlessRepayCompleteKeys,
) -> std::io::Result<Instruction> {
    lending_pool_force_tokenless_repay_complete_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
    )
}
pub fn lending_pool_force_tokenless_repay_complete_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolForceTokenlessRepayCompleteKeys = accounts.into();
    let ix = lending_pool_force_tokenless_repay_complete_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_force_tokenless_repay_complete_invoke(
    accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_force_tokenless_repay_complete_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn lending_pool_force_tokenless_repay_complete_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolForceTokenlessRepayCompleteKeys = accounts.into();
    let ix = lending_pool_force_tokenless_repay_complete_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_force_tokenless_repay_complete_invoke_signed(
    accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_force_tokenless_repay_complete_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_force_tokenless_repay_complete_verify_account_keys(
    accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'_, '_>,
    keys: LendingPoolForceTokenlessRepayCompleteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.risk_admin.key, keys.risk_admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_force_tokenless_repay_complete_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_force_tokenless_repay_complete_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.risk_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_force_tokenless_repay_complete_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolForceTokenlessRepayCompleteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_force_tokenless_repay_complete_verify_writable_privileges(accounts)?;
    lending_pool_force_tokenless_repay_complete_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_HANDLE_BANKRUPTCY_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolHandleBankruptcyAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolHandleBankruptcyKeys {
    pub group: Pubkey,
    pub signer: Pubkey,
    pub bank: Pubkey,
    pub marginfi_account: Pubkey,
    pub liquidity_vault: Pubkey,
    pub insurance_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingPoolHandleBankruptcyAccounts<'_, '_>>
for LendingPoolHandleBankruptcyKeys {
    fn from(accounts: LendingPoolHandleBankruptcyAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            signer: *accounts.signer.key,
            bank: *accounts.bank.key,
            marginfi_account: *accounts.marginfi_account.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            insurance_vault: *accounts.insurance_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingPoolHandleBankruptcyKeys>
for [AccountMeta; LENDING_POOL_HANDLE_BANKRUPTCY_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolHandleBankruptcyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
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
impl From<[Pubkey; LENDING_POOL_HANDLE_BANKRUPTCY_IX_ACCOUNTS_LEN]>
for LendingPoolHandleBankruptcyKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_HANDLE_BANKRUPTCY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            signer: pubkeys[1],
            bank: pubkeys[2],
            marginfi_account: pubkeys[3],
            liquidity_vault: pubkeys[4],
            insurance_vault: pubkeys[5],
            insurance_vault_authority: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<LendingPoolHandleBankruptcyAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_HANDLE_BANKRUPTCY_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolHandleBankruptcyAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.signer.clone(),
            accounts.bank.clone(),
            accounts.marginfi_account.clone(),
            accounts.liquidity_vault.clone(),
            accounts.insurance_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_HANDLE_BANKRUPTCY_IX_ACCOUNTS_LEN]>
for LendingPoolHandleBankruptcyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_HANDLE_BANKRUPTCY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            signer: &arr[1],
            bank: &arr[2],
            marginfi_account: &arr[3],
            liquidity_vault: &arr[4],
            insurance_vault: &arr[5],
            insurance_vault_authority: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const LENDING_POOL_HANDLE_BANKRUPTCY_IX_DISCM: [u8; 8usize] = [
    162, 11, 56, 139, 90, 128, 70, 173,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolHandleBankruptcyIxData;
impl LendingPoolHandleBankruptcyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_HANDLE_BANKRUPTCY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_HANDLE_BANKRUPTCY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_handle_bankruptcy_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolHandleBankruptcyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_HANDLE_BANKRUPTCY_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolHandleBankruptcyIxData.try_to_vec()?,
    })
}
pub fn lending_pool_handle_bankruptcy_ix(
    keys: LendingPoolHandleBankruptcyKeys,
) -> std::io::Result<Instruction> {
    lending_pool_handle_bankruptcy_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_pool_handle_bankruptcy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolHandleBankruptcyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolHandleBankruptcyKeys = accounts.into();
    let ix = lending_pool_handle_bankruptcy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_handle_bankruptcy_invoke(
    accounts: LendingPoolHandleBankruptcyAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_handle_bankruptcy_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn lending_pool_handle_bankruptcy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolHandleBankruptcyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolHandleBankruptcyKeys = accounts.into();
    let ix = lending_pool_handle_bankruptcy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_handle_bankruptcy_invoke_signed(
    accounts: LendingPoolHandleBankruptcyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_handle_bankruptcy_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_handle_bankruptcy_verify_account_keys(
    accounts: LendingPoolHandleBankruptcyAccounts<'_, '_>,
    keys: LendingPoolHandleBankruptcyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.signer.key, keys.signer),
        (*accounts.bank.key, keys.bank),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_handle_bankruptcy_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolHandleBankruptcyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank,
        accounts.marginfi_account,
        accounts.liquidity_vault,
        accounts.insurance_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_handle_bankruptcy_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolHandleBankruptcyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_handle_bankruptcy_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolHandleBankruptcyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_handle_bankruptcy_verify_writable_privileges(accounts)?;
    lending_pool_handle_bankruptcy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolPulseBankPriceCacheAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolPulseBankPriceCacheKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolPulseBankPriceCacheAccounts<'_, '_>>
for LendingPoolPulseBankPriceCacheKeys {
    fn from(accounts: LendingPoolPulseBankPriceCacheAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolPulseBankPriceCacheKeys>
for [AccountMeta; LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolPulseBankPriceCacheKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_ACCOUNTS_LEN]>
for LendingPoolPulseBankPriceCacheKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
        }
    }
}
impl<'info> From<LendingPoolPulseBankPriceCacheAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolPulseBankPriceCacheAccounts<'_, 'info>) -> Self {
        [accounts.group.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_ACCOUNTS_LEN]>
for LendingPoolPulseBankPriceCacheAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
        }
    }
}
pub const LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_DISCM: [u8; 8usize] = [
    192, 19, 201, 135, 105, 203, 32, 222,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolPulseBankPriceCacheIxData;
impl LendingPoolPulseBankPriceCacheIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_pulse_bank_price_cache_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolPulseBankPriceCacheKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_PULSE_BANK_PRICE_CACHE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolPulseBankPriceCacheIxData.try_to_vec()?,
    })
}
pub fn lending_pool_pulse_bank_price_cache_ix(
    keys: LendingPoolPulseBankPriceCacheKeys,
) -> std::io::Result<Instruction> {
    lending_pool_pulse_bank_price_cache_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_pool_pulse_bank_price_cache_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolPulseBankPriceCacheAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolPulseBankPriceCacheKeys = accounts.into();
    let ix = lending_pool_pulse_bank_price_cache_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_pulse_bank_price_cache_invoke(
    accounts: LendingPoolPulseBankPriceCacheAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_pulse_bank_price_cache_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn lending_pool_pulse_bank_price_cache_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolPulseBankPriceCacheAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolPulseBankPriceCacheKeys = accounts.into();
    let ix = lending_pool_pulse_bank_price_cache_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_pulse_bank_price_cache_invoke_signed(
    accounts: LendingPoolPulseBankPriceCacheAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_pulse_bank_price_cache_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_pulse_bank_price_cache_verify_account_keys(
    accounts: LendingPoolPulseBankPriceCacheAccounts<'_, '_>,
    keys: LendingPoolPulseBankPriceCacheKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_pulse_bank_price_cache_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolPulseBankPriceCacheAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_pulse_bank_price_cache_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolPulseBankPriceCacheAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_pulse_bank_price_cache_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolReclaimEmissionsVaultAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub emissions_mint: &'me AccountInfo<'info>,
    pub emissions_auth: &'me AccountInfo<'info>,
    pub emissions_vault: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub destination_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolReclaimEmissionsVaultKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub emissions_mint: Pubkey,
    pub emissions_auth: Pubkey,
    pub emissions_vault: Pubkey,
    pub fee_state: Pubkey,
    pub destination_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingPoolReclaimEmissionsVaultAccounts<'_, '_>>
for LendingPoolReclaimEmissionsVaultKeys {
    fn from(accounts: LendingPoolReclaimEmissionsVaultAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
            emissions_mint: *accounts.emissions_mint.key,
            emissions_auth: *accounts.emissions_auth.key,
            emissions_vault: *accounts.emissions_vault.key,
            fee_state: *accounts.fee_state.key,
            destination_account: *accounts.destination_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingPoolReclaimEmissionsVaultKeys>
for [AccountMeta; LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolReclaimEmissionsVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.emissions_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.emissions_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.emissions_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_account,
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
impl From<[Pubkey; LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_ACCOUNTS_LEN]>
for LendingPoolReclaimEmissionsVaultKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
            emissions_mint: pubkeys[2],
            emissions_auth: pubkeys[3],
            emissions_vault: pubkeys[4],
            fee_state: pubkeys[5],
            destination_account: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<LendingPoolReclaimEmissionsVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolReclaimEmissionsVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.bank.clone(),
            accounts.emissions_mint.clone(),
            accounts.emissions_auth.clone(),
            accounts.emissions_vault.clone(),
            accounts.fee_state.clone(),
            accounts.destination_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_ACCOUNTS_LEN]>
for LendingPoolReclaimEmissionsVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
            emissions_mint: &arr[2],
            emissions_auth: &arr[3],
            emissions_vault: &arr[4],
            fee_state: &arr[5],
            destination_account: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_DISCM: [u8; 8usize] = [
    206, 67, 186, 225, 41, 30, 95, 216,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolReclaimEmissionsVaultIxData;
impl LendingPoolReclaimEmissionsVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_reclaim_emissions_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolReclaimEmissionsVaultKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_RECLAIM_EMISSIONS_VAULT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolReclaimEmissionsVaultIxData.try_to_vec()?,
    })
}
pub fn lending_pool_reclaim_emissions_vault_ix(
    keys: LendingPoolReclaimEmissionsVaultKeys,
) -> std::io::Result<Instruction> {
    lending_pool_reclaim_emissions_vault_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn lending_pool_reclaim_emissions_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolReclaimEmissionsVaultAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolReclaimEmissionsVaultKeys = accounts.into();
    let ix = lending_pool_reclaim_emissions_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_reclaim_emissions_vault_invoke(
    accounts: LendingPoolReclaimEmissionsVaultAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_reclaim_emissions_vault_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn lending_pool_reclaim_emissions_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolReclaimEmissionsVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolReclaimEmissionsVaultKeys = accounts.into();
    let ix = lending_pool_reclaim_emissions_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_reclaim_emissions_vault_invoke_signed(
    accounts: LendingPoolReclaimEmissionsVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_reclaim_emissions_vault_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_reclaim_emissions_vault_verify_account_keys(
    accounts: LendingPoolReclaimEmissionsVaultAccounts<'_, '_>,
    keys: LendingPoolReclaimEmissionsVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
        (*accounts.emissions_mint.key, keys.emissions_mint),
        (*accounts.emissions_auth.key, keys.emissions_auth),
        (*accounts.emissions_vault.key, keys.emissions_vault),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.destination_account.key, keys.destination_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_reclaim_emissions_vault_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolReclaimEmissionsVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank,
        accounts.emissions_vault,
        accounts.destination_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_reclaim_emissions_vault_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolReclaimEmissionsVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_reclaim_emissions_vault_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolSetFixedOraclePriceAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolSetFixedOraclePriceKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub bank: Pubkey,
}
impl From<LendingPoolSetFixedOraclePriceAccounts<'_, '_>>
for LendingPoolSetFixedOraclePriceKeys {
    fn from(accounts: LendingPoolSetFixedOraclePriceAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<LendingPoolSetFixedOraclePriceKeys>
for [AccountMeta; LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolSetFixedOraclePriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_ACCOUNTS_LEN]>
for LendingPoolSetFixedOraclePriceKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<LendingPoolSetFixedOraclePriceAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolSetFixedOraclePriceAccounts<'_, 'info>) -> Self {
        [accounts.group.clone(), accounts.admin.clone(), accounts.bank.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_ACCOUNTS_LEN]>
for LendingPoolSetFixedOraclePriceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_DISCM: [u8; 8usize] = [
    28, 126, 127, 127, 60, 37, 211, 125,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolSetFixedOraclePriceIxArgs {
    pub price: WrappedI80F48,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolSetFixedOraclePriceIxData(
    pub LendingPoolSetFixedOraclePriceIxArgs,
);
impl From<LendingPoolSetFixedOraclePriceIxArgs>
for LendingPoolSetFixedOraclePriceIxData {
    fn from(args: LendingPoolSetFixedOraclePriceIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolSetFixedOraclePriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let price = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        Ok(
            Self(LendingPoolSetFixedOraclePriceIxArgs {
                price,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_set_fixed_oracle_price_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolSetFixedOraclePriceKeys,
    args: LendingPoolSetFixedOraclePriceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_SET_FIXED_ORACLE_PRICE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolSetFixedOraclePriceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_set_fixed_oracle_price_ix(
    keys: LendingPoolSetFixedOraclePriceKeys,
    args: LendingPoolSetFixedOraclePriceIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_set_fixed_oracle_price_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn lending_pool_set_fixed_oracle_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolSetFixedOraclePriceAccounts<'_, '_>,
    args: LendingPoolSetFixedOraclePriceIxArgs,
) -> ProgramResult {
    let keys: LendingPoolSetFixedOraclePriceKeys = accounts.into();
    let ix = lending_pool_set_fixed_oracle_price_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_set_fixed_oracle_price_invoke(
    accounts: LendingPoolSetFixedOraclePriceAccounts<'_, '_>,
    args: LendingPoolSetFixedOraclePriceIxArgs,
) -> ProgramResult {
    lending_pool_set_fixed_oracle_price_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_set_fixed_oracle_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolSetFixedOraclePriceAccounts<'_, '_>,
    args: LendingPoolSetFixedOraclePriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolSetFixedOraclePriceKeys = accounts.into();
    let ix = lending_pool_set_fixed_oracle_price_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_set_fixed_oracle_price_invoke_signed(
    accounts: LendingPoolSetFixedOraclePriceAccounts<'_, '_>,
    args: LendingPoolSetFixedOraclePriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_set_fixed_oracle_price_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_set_fixed_oracle_price_verify_account_keys(
    accounts: LendingPoolSetFixedOraclePriceAccounts<'_, '_>,
    keys: LendingPoolSetFixedOraclePriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_set_fixed_oracle_price_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolSetFixedOraclePriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_set_fixed_oracle_price_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolSetFixedOraclePriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_set_fixed_oracle_price_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolSetFixedOraclePriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_set_fixed_oracle_price_verify_writable_privileges(accounts)?;
    lending_pool_set_fixed_oracle_price_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolUpdateFeesDestinationAccountAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub destination_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolUpdateFeesDestinationAccountKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub admin: Pubkey,
    pub destination_account: Pubkey,
}
impl From<LendingPoolUpdateFeesDestinationAccountAccounts<'_, '_>>
for LendingPoolUpdateFeesDestinationAccountKeys {
    fn from(accounts: LendingPoolUpdateFeesDestinationAccountAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
            admin: *accounts.admin.key,
            destination_account: *accounts.destination_account.key,
        }
    }
}
impl From<LendingPoolUpdateFeesDestinationAccountKeys>
for [AccountMeta; LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolUpdateFeesDestinationAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for LendingPoolUpdateFeesDestinationAccountKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
            admin: pubkeys[2],
            destination_account: pubkeys[3],
        }
    }
}
impl<'info> From<LendingPoolUpdateFeesDestinationAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.group.clone(),
            accounts.bank.clone(),
            accounts.admin.clone(),
            accounts.destination_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN],
> for LendingPoolUpdateFeesDestinationAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
            admin: &arr[2],
            destination_account: &arr[3],
        }
    }
}
pub const LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    102, 4, 121, 243, 237, 110, 95, 13,
];
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolUpdateFeesDestinationAccountIxData;
impl LendingPoolUpdateFeesDestinationAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_update_fees_destination_account_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolUpdateFeesDestinationAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_UPDATE_FEES_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LendingPoolUpdateFeesDestinationAccountIxData.try_to_vec()?,
    })
}
pub fn lending_pool_update_fees_destination_account_ix(
    keys: LendingPoolUpdateFeesDestinationAccountKeys,
) -> std::io::Result<Instruction> {
    lending_pool_update_fees_destination_account_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
    )
}
pub fn lending_pool_update_fees_destination_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LendingPoolUpdateFeesDestinationAccountKeys = accounts.into();
    let ix = lending_pool_update_fees_destination_account_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_update_fees_destination_account_invoke(
    accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'_, '_>,
) -> ProgramResult {
    lending_pool_update_fees_destination_account_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn lending_pool_update_fees_destination_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolUpdateFeesDestinationAccountKeys = accounts.into();
    let ix = lending_pool_update_fees_destination_account_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_update_fees_destination_account_invoke_signed(
    accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_update_fees_destination_account_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn lending_pool_update_fees_destination_account_verify_account_keys(
    accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'_, '_>,
    keys: LendingPoolUpdateFeesDestinationAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
        (*accounts.admin.key, keys.admin),
        (*accounts.destination_account.key, keys.destination_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_update_fees_destination_account_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_update_fees_destination_account_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_update_fees_destination_account_verify_account_privileges<
    'me,
    'info,
>(
    accounts: LendingPoolUpdateFeesDestinationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_update_fees_destination_account_verify_writable_privileges(accounts)?;
    lending_pool_update_fees_destination_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_WITHDRAW_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolWithdrawFeesAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub dst_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolWithdrawFeesKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub admin: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub dst_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingPoolWithdrawFeesAccounts<'_, '_>> for LendingPoolWithdrawFeesKeys {
    fn from(accounts: LendingPoolWithdrawFeesAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
            admin: *accounts.admin.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            dst_token_account: *accounts.dst_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingPoolWithdrawFeesKeys>
for [AccountMeta; LENDING_POOL_WITHDRAW_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolWithdrawFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dst_token_account,
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
impl From<[Pubkey; LENDING_POOL_WITHDRAW_FEES_IX_ACCOUNTS_LEN]>
for LendingPoolWithdrawFeesKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_WITHDRAW_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
            admin: pubkeys[2],
            fee_vault: pubkeys[3],
            fee_vault_authority: pubkeys[4],
            dst_token_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<LendingPoolWithdrawFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_WITHDRAW_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolWithdrawFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.bank.clone(),
            accounts.admin.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.dst_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_WITHDRAW_FEES_IX_ACCOUNTS_LEN]>
for LendingPoolWithdrawFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_WITHDRAW_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
            admin: &arr[2],
            fee_vault: &arr[3],
            fee_vault_authority: &arr[4],
            dst_token_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const LENDING_POOL_WITHDRAW_FEES_IX_DISCM: [u8; 8usize] = [
    92, 140, 215, 254, 170, 0, 83, 174,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolWithdrawFeesIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolWithdrawFeesIxData(pub LendingPoolWithdrawFeesIxArgs);
impl From<LendingPoolWithdrawFeesIxArgs> for LendingPoolWithdrawFeesIxData {
    fn from(args: LendingPoolWithdrawFeesIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolWithdrawFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_WITHDRAW_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolWithdrawFeesIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_WITHDRAW_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_withdraw_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolWithdrawFeesKeys,
    args: LendingPoolWithdrawFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_WITHDRAW_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: LendingPoolWithdrawFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_withdraw_fees_ix(
    keys: LendingPoolWithdrawFeesKeys,
    args: LendingPoolWithdrawFeesIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_withdraw_fees_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_withdraw_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolWithdrawFeesAccounts<'_, '_>,
    args: LendingPoolWithdrawFeesIxArgs,
) -> ProgramResult {
    let keys: LendingPoolWithdrawFeesKeys = accounts.into();
    let ix = lending_pool_withdraw_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_withdraw_fees_invoke(
    accounts: LendingPoolWithdrawFeesAccounts<'_, '_>,
    args: LendingPoolWithdrawFeesIxArgs,
) -> ProgramResult {
    lending_pool_withdraw_fees_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_withdraw_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolWithdrawFeesAccounts<'_, '_>,
    args: LendingPoolWithdrawFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolWithdrawFeesKeys = accounts.into();
    let ix = lending_pool_withdraw_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_withdraw_fees_invoke_signed(
    accounts: LendingPoolWithdrawFeesAccounts<'_, '_>,
    args: LendingPoolWithdrawFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_withdraw_fees_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_withdraw_fees_verify_account_keys(
    accounts: LendingPoolWithdrawFeesAccounts<'_, '_>,
    keys: LendingPoolWithdrawFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.dst_token_account.key, keys.dst_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_withdraw_fees_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolWithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_vault, accounts.dst_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_withdraw_fees_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolWithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_withdraw_fees_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolWithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_withdraw_fees_verify_writable_privileges(accounts)?;
    lending_pool_withdraw_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolWithdrawFeesPermissionlessAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub fees_destination_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolWithdrawFeesPermissionlessKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub fee_vault: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub fees_destination_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingPoolWithdrawFeesPermissionlessAccounts<'_, '_>>
for LendingPoolWithdrawFeesPermissionlessKeys {
    fn from(accounts: LendingPoolWithdrawFeesPermissionlessAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
            fee_vault: *accounts.fee_vault.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            fees_destination_account: *accounts.fees_destination_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingPoolWithdrawFeesPermissionlessKeys>
for [AccountMeta; LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolWithdrawFeesPermissionlessKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fees_destination_account,
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
impl From<[Pubkey; LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for LendingPoolWithdrawFeesPermissionlessKeys {
    fn from(
        pubkeys: [Pubkey; LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
            fee_vault: pubkeys[2],
            fee_vault_authority: pubkeys[3],
            fees_destination_account: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<LendingPoolWithdrawFeesPermissionlessAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolWithdrawFeesPermissionlessAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.bank.clone(),
            accounts.fee_vault.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.fees_destination_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_ACCOUNTS_LEN],
> for LendingPoolWithdrawFeesPermissionlessAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
            fee_vault: &arr[2],
            fee_vault_authority: &arr[3],
            fees_destination_account: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_DISCM: [u8; 8usize] = [
    57, 245, 1, 208, 130, 18, 145, 113,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolWithdrawFeesPermissionlessIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolWithdrawFeesPermissionlessIxData(
    pub LendingPoolWithdrawFeesPermissionlessIxArgs,
);
impl From<LendingPoolWithdrawFeesPermissionlessIxArgs>
for LendingPoolWithdrawFeesPermissionlessIxData {
    fn from(args: LendingPoolWithdrawFeesPermissionlessIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolWithdrawFeesPermissionlessIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolWithdrawFeesPermissionlessIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_withdraw_fees_permissionless_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolWithdrawFeesPermissionlessKeys,
    args: LendingPoolWithdrawFeesPermissionlessIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_WITHDRAW_FEES_PERMISSIONLESS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolWithdrawFeesPermissionlessIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_withdraw_fees_permissionless_ix(
    keys: LendingPoolWithdrawFeesPermissionlessKeys,
    args: LendingPoolWithdrawFeesPermissionlessIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_withdraw_fees_permissionless_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn lending_pool_withdraw_fees_permissionless_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolWithdrawFeesPermissionlessAccounts<'_, '_>,
    args: LendingPoolWithdrawFeesPermissionlessIxArgs,
) -> ProgramResult {
    let keys: LendingPoolWithdrawFeesPermissionlessKeys = accounts.into();
    let ix = lending_pool_withdraw_fees_permissionless_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_withdraw_fees_permissionless_invoke(
    accounts: LendingPoolWithdrawFeesPermissionlessAccounts<'_, '_>,
    args: LendingPoolWithdrawFeesPermissionlessIxArgs,
) -> ProgramResult {
    lending_pool_withdraw_fees_permissionless_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_withdraw_fees_permissionless_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolWithdrawFeesPermissionlessAccounts<'_, '_>,
    args: LendingPoolWithdrawFeesPermissionlessIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolWithdrawFeesPermissionlessKeys = accounts.into();
    let ix = lending_pool_withdraw_fees_permissionless_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_withdraw_fees_permissionless_invoke_signed(
    accounts: LendingPoolWithdrawFeesPermissionlessAccounts<'_, '_>,
    args: LendingPoolWithdrawFeesPermissionlessIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_withdraw_fees_permissionless_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_withdraw_fees_permissionless_verify_account_keys(
    accounts: LendingPoolWithdrawFeesPermissionlessAccounts<'_, '_>,
    keys: LendingPoolWithdrawFeesPermissionlessKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.fees_destination_account.key, keys.fees_destination_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_withdraw_fees_permissionless_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolWithdrawFeesPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_vault, accounts.fees_destination_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_withdraw_fees_permissionless_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolWithdrawFeesPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_withdraw_fees_permissionless_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LENDING_POOL_WITHDRAW_INSURANCE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct LendingPoolWithdrawInsuranceAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub insurance_vault: &'me AccountInfo<'info>,
    pub insurance_vault_authority: &'me AccountInfo<'info>,
    pub dst_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LendingPoolWithdrawInsuranceKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub admin: Pubkey,
    pub insurance_vault: Pubkey,
    pub insurance_vault_authority: Pubkey,
    pub dst_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<LendingPoolWithdrawInsuranceAccounts<'_, '_>>
for LendingPoolWithdrawInsuranceKeys {
    fn from(accounts: LendingPoolWithdrawInsuranceAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
            admin: *accounts.admin.key,
            insurance_vault: *accounts.insurance_vault.key,
            insurance_vault_authority: *accounts.insurance_vault_authority.key,
            dst_token_account: *accounts.dst_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LendingPoolWithdrawInsuranceKeys>
for [AccountMeta; LENDING_POOL_WITHDRAW_INSURANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: LendingPoolWithdrawInsuranceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.insurance_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.insurance_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dst_token_account,
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
impl From<[Pubkey; LENDING_POOL_WITHDRAW_INSURANCE_IX_ACCOUNTS_LEN]>
for LendingPoolWithdrawInsuranceKeys {
    fn from(pubkeys: [Pubkey; LENDING_POOL_WITHDRAW_INSURANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
            admin: pubkeys[2],
            insurance_vault: pubkeys[3],
            insurance_vault_authority: pubkeys[4],
            dst_token_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<LendingPoolWithdrawInsuranceAccounts<'_, 'info>>
for [AccountInfo<'info>; LENDING_POOL_WITHDRAW_INSURANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: LendingPoolWithdrawInsuranceAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.bank.clone(),
            accounts.admin.clone(),
            accounts.insurance_vault.clone(),
            accounts.insurance_vault_authority.clone(),
            accounts.dst_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LENDING_POOL_WITHDRAW_INSURANCE_IX_ACCOUNTS_LEN]>
for LendingPoolWithdrawInsuranceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LENDING_POOL_WITHDRAW_INSURANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
            admin: &arr[2],
            insurance_vault: &arr[3],
            insurance_vault_authority: &arr[4],
            dst_token_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const LENDING_POOL_WITHDRAW_INSURANCE_IX_DISCM: [u8; 8usize] = [
    108, 60, 60, 246, 104, 79, 159, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolWithdrawInsuranceIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolWithdrawInsuranceIxData(pub LendingPoolWithdrawInsuranceIxArgs);
impl From<LendingPoolWithdrawInsuranceIxArgs> for LendingPoolWithdrawInsuranceIxData {
    fn from(args: LendingPoolWithdrawInsuranceIxArgs) -> Self {
        Self(args)
    }
}
impl LendingPoolWithdrawInsuranceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_WITHDRAW_INSURANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LendingPoolWithdrawInsuranceIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_WITHDRAW_INSURANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lending_pool_withdraw_insurance_ix_with_program_id(
    program_id: Pubkey,
    keys: LendingPoolWithdrawInsuranceKeys,
    args: LendingPoolWithdrawInsuranceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LENDING_POOL_WITHDRAW_INSURANCE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LendingPoolWithdrawInsuranceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lending_pool_withdraw_insurance_ix(
    keys: LendingPoolWithdrawInsuranceKeys,
    args: LendingPoolWithdrawInsuranceIxArgs,
) -> std::io::Result<Instruction> {
    lending_pool_withdraw_insurance_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn lending_pool_withdraw_insurance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolWithdrawInsuranceAccounts<'_, '_>,
    args: LendingPoolWithdrawInsuranceIxArgs,
) -> ProgramResult {
    let keys: LendingPoolWithdrawInsuranceKeys = accounts.into();
    let ix = lending_pool_withdraw_insurance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lending_pool_withdraw_insurance_invoke(
    accounts: LendingPoolWithdrawInsuranceAccounts<'_, '_>,
    args: LendingPoolWithdrawInsuranceIxArgs,
) -> ProgramResult {
    lending_pool_withdraw_insurance_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn lending_pool_withdraw_insurance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LendingPoolWithdrawInsuranceAccounts<'_, '_>,
    args: LendingPoolWithdrawInsuranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LendingPoolWithdrawInsuranceKeys = accounts.into();
    let ix = lending_pool_withdraw_insurance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lending_pool_withdraw_insurance_invoke_signed(
    accounts: LendingPoolWithdrawInsuranceAccounts<'_, '_>,
    args: LendingPoolWithdrawInsuranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lending_pool_withdraw_insurance_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lending_pool_withdraw_insurance_verify_account_keys(
    accounts: LendingPoolWithdrawInsuranceAccounts<'_, '_>,
    keys: LendingPoolWithdrawInsuranceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
        (*accounts.admin.key, keys.admin),
        (*accounts.insurance_vault.key, keys.insurance_vault),
        (*accounts.insurance_vault_authority.key, keys.insurance_vault_authority),
        (*accounts.dst_token_account.key, keys.dst_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lending_pool_withdraw_insurance_verify_writable_privileges<'me, 'info>(
    accounts: LendingPoolWithdrawInsuranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.insurance_vault, accounts.dst_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lending_pool_withdraw_insurance_verify_signer_privileges<'me, 'info>(
    accounts: LendingPoolWithdrawInsuranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lending_pool_withdraw_insurance_verify_account_privileges<'me, 'info>(
    accounts: LendingPoolWithdrawInsuranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lending_pool_withdraw_insurance_verify_writable_privileges(accounts)?;
    lending_pool_withdraw_insurance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_CLOSE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountCloseAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountCloseKeys {
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub fee_payer: Pubkey,
}
impl From<MarginfiAccountCloseAccounts<'_, '_>> for MarginfiAccountCloseKeys {
    fn from(accounts: MarginfiAccountCloseAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            fee_payer: *accounts.fee_payer.key,
        }
    }
}
impl From<MarginfiAccountCloseKeys>
for [AccountMeta; MARGINFI_ACCOUNT_CLOSE_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountCloseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MARGINFI_ACCOUNT_CLOSE_IX_ACCOUNTS_LEN]>
for MarginfiAccountCloseKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_ACCOUNT_CLOSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            authority: pubkeys[1],
            fee_payer: pubkeys[2],
        }
    }
}
impl<'info> From<MarginfiAccountCloseAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_CLOSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountCloseAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.fee_payer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_CLOSE_IX_ACCOUNTS_LEN]>
for MarginfiAccountCloseAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_ACCOUNT_CLOSE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            authority: &arr[1],
            fee_payer: &arr[2],
        }
    }
}
pub const MARGINFI_ACCOUNT_CLOSE_IX_DISCM: [u8; 8usize] = [
    186, 221, 93, 34, 50, 97, 194, 241,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountCloseIxData;
impl MarginfiAccountCloseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_CLOSE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_CLOSE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_close_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountCloseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_CLOSE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiAccountCloseIxData.try_to_vec()?,
    })
}
pub fn marginfi_account_close_ix(
    keys: MarginfiAccountCloseKeys,
) -> std::io::Result<Instruction> {
    marginfi_account_close_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn marginfi_account_close_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountCloseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiAccountCloseKeys = accounts.into();
    let ix = marginfi_account_close_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_close_invoke(
    accounts: MarginfiAccountCloseAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_account_close_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn marginfi_account_close_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountCloseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountCloseKeys = accounts.into();
    let ix = marginfi_account_close_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_close_invoke_signed(
    accounts: MarginfiAccountCloseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_close_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_account_close_verify_account_keys(
    accounts: MarginfiAccountCloseAccounts<'_, '_>,
    keys: MarginfiAccountCloseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_payer.key, keys.fee_payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_close_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account, accounts.fee_payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_close_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_close_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountCloseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_close_verify_writable_privileges(accounts)?;
    marginfi_account_close_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_CLOSE_ORDER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountCloseOrderAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountCloseOrderKeys {
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub order: Pubkey,
    pub fee_recipient: Pubkey,
    pub system_program: Pubkey,
}
impl From<MarginfiAccountCloseOrderAccounts<'_, '_>> for MarginfiAccountCloseOrderKeys {
    fn from(accounts: MarginfiAccountCloseOrderAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            order: *accounts.order.key,
            fee_recipient: *accounts.fee_recipient.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MarginfiAccountCloseOrderKeys>
for [AccountMeta; MARGINFI_ACCOUNT_CLOSE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountCloseOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
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
impl From<[Pubkey; MARGINFI_ACCOUNT_CLOSE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountCloseOrderKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_ACCOUNT_CLOSE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            authority: pubkeys[1],
            order: pubkeys[2],
            fee_recipient: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<MarginfiAccountCloseOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_CLOSE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountCloseOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.order.clone(),
            accounts.fee_recipient.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_CLOSE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountCloseOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_ACCOUNT_CLOSE_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            authority: &arr[1],
            order: &arr[2],
            fee_recipient: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const MARGINFI_ACCOUNT_CLOSE_ORDER_IX_DISCM: [u8; 8usize] = [
    212, 223, 79, 182, 172, 183, 205, 237,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountCloseOrderIxData;
impl MarginfiAccountCloseOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_CLOSE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_CLOSE_ORDER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_close_order_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountCloseOrderKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_CLOSE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiAccountCloseOrderIxData.try_to_vec()?,
    })
}
pub fn marginfi_account_close_order_ix(
    keys: MarginfiAccountCloseOrderKeys,
) -> std::io::Result<Instruction> {
    marginfi_account_close_order_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn marginfi_account_close_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountCloseOrderAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiAccountCloseOrderKeys = accounts.into();
    let ix = marginfi_account_close_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_close_order_invoke(
    accounts: MarginfiAccountCloseOrderAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_account_close_order_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn marginfi_account_close_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountCloseOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountCloseOrderKeys = accounts.into();
    let ix = marginfi_account_close_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_close_order_invoke_signed(
    accounts: MarginfiAccountCloseOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_close_order_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_account_close_order_verify_account_keys(
    accounts: MarginfiAccountCloseOrderAccounts<'_, '_>,
    keys: MarginfiAccountCloseOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.order.key, keys.order),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_close_order_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountCloseOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.order,
        accounts.fee_recipient,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_close_order_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountCloseOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_close_order_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountCloseOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_close_order_verify_writable_privileges(accounts)?;
    marginfi_account_close_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountEndExecuteOrderAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub executor: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub execute_record: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountEndExecuteOrderKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub executor: Pubkey,
    pub fee_recipient: Pubkey,
    pub order: Pubkey,
    pub execute_record: Pubkey,
    pub fee_state: Pubkey,
}
impl From<MarginfiAccountEndExecuteOrderAccounts<'_, '_>>
for MarginfiAccountEndExecuteOrderKeys {
    fn from(accounts: MarginfiAccountEndExecuteOrderAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            executor: *accounts.executor.key,
            fee_recipient: *accounts.fee_recipient.key,
            order: *accounts.order.key,
            execute_record: *accounts.execute_record.key,
            fee_state: *accounts.fee_state.key,
        }
    }
}
impl From<MarginfiAccountEndExecuteOrderKeys>
for [AccountMeta; MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountEndExecuteOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.executor,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.execute_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountEndExecuteOrderKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            executor: pubkeys[2],
            fee_recipient: pubkeys[3],
            order: pubkeys[4],
            execute_record: pubkeys[5],
            fee_state: pubkeys[6],
        }
    }
}
impl<'info> From<MarginfiAccountEndExecuteOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountEndExecuteOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.executor.clone(),
            accounts.fee_recipient.clone(),
            accounts.order.clone(),
            accounts.execute_record.clone(),
            accounts.fee_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountEndExecuteOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            executor: &arr[2],
            fee_recipient: &arr[3],
            order: &arr[4],
            execute_record: &arr[5],
            fee_state: &arr[6],
        }
    }
}
pub const MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_DISCM: [u8; 8usize] = [
    115, 42, 20, 93, 121, 84, 178, 83,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountEndExecuteOrderIxData;
impl MarginfiAccountEndExecuteOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_end_execute_order_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountEndExecuteOrderKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_END_EXECUTE_ORDER_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiAccountEndExecuteOrderIxData.try_to_vec()?,
    })
}
pub fn marginfi_account_end_execute_order_ix(
    keys: MarginfiAccountEndExecuteOrderKeys,
) -> std::io::Result<Instruction> {
    marginfi_account_end_execute_order_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn marginfi_account_end_execute_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountEndExecuteOrderAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiAccountEndExecuteOrderKeys = accounts.into();
    let ix = marginfi_account_end_execute_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_end_execute_order_invoke(
    accounts: MarginfiAccountEndExecuteOrderAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_account_end_execute_order_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn marginfi_account_end_execute_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountEndExecuteOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountEndExecuteOrderKeys = accounts.into();
    let ix = marginfi_account_end_execute_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_end_execute_order_invoke_signed(
    accounts: MarginfiAccountEndExecuteOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_end_execute_order_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_account_end_execute_order_verify_account_keys(
    accounts: MarginfiAccountEndExecuteOrderAccounts<'_, '_>,
    keys: MarginfiAccountEndExecuteOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.executor.key, keys.executor),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.order.key, keys.order),
        (*accounts.execute_record.key, keys.execute_record),
        (*accounts.fee_state.key, keys.fee_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_end_execute_order_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountEndExecuteOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.fee_recipient,
        accounts.order,
        accounts.execute_record,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_end_execute_order_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountEndExecuteOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.executor] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_end_execute_order_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountEndExecuteOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_end_execute_order_verify_writable_privileges(accounts)?;
    marginfi_account_end_execute_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountInitLiqRecordAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub liquidation_record: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountInitLiqRecordKeys {
    pub marginfi_account: Pubkey,
    pub fee_payer: Pubkey,
    pub liquidation_record: Pubkey,
    pub system_program: Pubkey,
}
impl From<MarginfiAccountInitLiqRecordAccounts<'_, '_>>
for MarginfiAccountInitLiqRecordKeys {
    fn from(accounts: MarginfiAccountInitLiqRecordAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            fee_payer: *accounts.fee_payer.key,
            liquidation_record: *accounts.liquidation_record.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MarginfiAccountInitLiqRecordKeys>
for [AccountMeta; MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountInitLiqRecordKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_record,
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
impl From<[Pubkey; MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_ACCOUNTS_LEN]>
for MarginfiAccountInitLiqRecordKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            fee_payer: pubkeys[1],
            liquidation_record: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<MarginfiAccountInitLiqRecordAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountInitLiqRecordAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.fee_payer.clone(),
            accounts.liquidation_record.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_ACCOUNTS_LEN]>
for MarginfiAccountInitLiqRecordAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            fee_payer: &arr[1],
            liquidation_record: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_DISCM: [u8; 8usize] = [
    236, 213, 238, 126, 147, 251, 164, 8,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountInitLiqRecordIxData;
impl MarginfiAccountInitLiqRecordIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_init_liq_record_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountInitLiqRecordKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_INIT_LIQ_RECORD_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiAccountInitLiqRecordIxData.try_to_vec()?,
    })
}
pub fn marginfi_account_init_liq_record_ix(
    keys: MarginfiAccountInitLiqRecordKeys,
) -> std::io::Result<Instruction> {
    marginfi_account_init_liq_record_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn marginfi_account_init_liq_record_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountInitLiqRecordAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiAccountInitLiqRecordKeys = accounts.into();
    let ix = marginfi_account_init_liq_record_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_init_liq_record_invoke(
    accounts: MarginfiAccountInitLiqRecordAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_account_init_liq_record_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn marginfi_account_init_liq_record_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountInitLiqRecordAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountInitLiqRecordKeys = accounts.into();
    let ix = marginfi_account_init_liq_record_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_init_liq_record_invoke_signed(
    accounts: MarginfiAccountInitLiqRecordAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_init_liq_record_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_account_init_liq_record_verify_account_keys(
    accounts: MarginfiAccountInitLiqRecordAccounts<'_, '_>,
    keys: MarginfiAccountInitLiqRecordKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.liquidation_record.key, keys.liquidation_record),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_init_liq_record_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountInitLiqRecordAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.fee_payer,
        accounts.liquidation_record,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_init_liq_record_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountInitLiqRecordAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_init_liq_record_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountInitLiqRecordAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_init_liq_record_verify_writable_privileges(accounts)?;
    marginfi_account_init_liq_record_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_INITIALIZE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountInitializeAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountInitializeKeys {
    pub marginfi_group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub fee_payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<MarginfiAccountInitializeAccounts<'_, '_>> for MarginfiAccountInitializeKeys {
    fn from(accounts: MarginfiAccountInitializeAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            fee_payer: *accounts.fee_payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MarginfiAccountInitializeKeys>
for [AccountMeta; MARGINFI_ACCOUNT_INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountInitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
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
impl From<[Pubkey; MARGINFI_ACCOUNT_INITIALIZE_IX_ACCOUNTS_LEN]>
for MarginfiAccountInitializeKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_ACCOUNT_INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            fee_payer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<MarginfiAccountInitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountInitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.fee_payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_INITIALIZE_IX_ACCOUNTS_LEN]>
for MarginfiAccountInitializeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_ACCOUNT_INITIALIZE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            fee_payer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const MARGINFI_ACCOUNT_INITIALIZE_IX_DISCM: [u8; 8usize] = [
    43, 78, 61, 255, 148, 52, 249, 154,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountInitializeIxData;
impl MarginfiAccountInitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_INITIALIZE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountInitializeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiAccountInitializeIxData.try_to_vec()?,
    })
}
pub fn marginfi_account_initialize_ix(
    keys: MarginfiAccountInitializeKeys,
) -> std::io::Result<Instruction> {
    marginfi_account_initialize_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn marginfi_account_initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountInitializeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiAccountInitializeKeys = accounts.into();
    let ix = marginfi_account_initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_initialize_invoke(
    accounts: MarginfiAccountInitializeAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_account_initialize_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn marginfi_account_initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountInitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountInitializeKeys = accounts.into();
    let ix = marginfi_account_initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_initialize_invoke_signed(
    accounts: MarginfiAccountInitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_initialize_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_account_initialize_verify_account_keys(
    accounts: MarginfiAccountInitializeAccounts<'_, '_>,
    keys: MarginfiAccountInitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_initialize_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account, accounts.fee_payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_initialize_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.marginfi_account,
        accounts.authority,
        accounts.fee_payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_initialize_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_initialize_verify_writable_privileges(accounts)?;
    marginfi_account_initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountInitializePdaAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountInitializePdaKeys {
    pub marginfi_group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub fee_payer: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub system_program: Pubkey,
}
impl From<MarginfiAccountInitializePdaAccounts<'_, '_>>
for MarginfiAccountInitializePdaKeys {
    fn from(accounts: MarginfiAccountInitializePdaAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            fee_payer: *accounts.fee_payer.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MarginfiAccountInitializePdaKeys>
for [AccountMeta; MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountInitializePdaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
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
impl From<[Pubkey; MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_ACCOUNTS_LEN]>
for MarginfiAccountInitializePdaKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            fee_payer: pubkeys[3],
            instructions_sysvar: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<MarginfiAccountInitializePdaAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountInitializePdaAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.fee_payer.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_ACCOUNTS_LEN]>
for MarginfiAccountInitializePdaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            fee_payer: &arr[3],
            instructions_sysvar: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_DISCM: [u8; 8usize] = [
    87, 177, 91, 80, 218, 119, 245, 31,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountInitializePdaIxArgs {
    pub account_index: u16,
    pub third_party_id: Option<u16>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountInitializePdaIxData(pub MarginfiAccountInitializePdaIxArgs);
impl From<MarginfiAccountInitializePdaIxArgs> for MarginfiAccountInitializePdaIxData {
    fn from(args: MarginfiAccountInitializePdaIxArgs) -> Self {
        Self(args)
    }
}
impl MarginfiAccountInitializePdaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let third_party_id: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MarginfiAccountInitializePdaIxArgs {
                account_index,
                third_party_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_DISCM)?;
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
pub fn marginfi_account_initialize_pda_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountInitializePdaKeys,
    args: MarginfiAccountInitializePdaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_INITIALIZE_PDA_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: MarginfiAccountInitializePdaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marginfi_account_initialize_pda_ix(
    keys: MarginfiAccountInitializePdaKeys,
    args: MarginfiAccountInitializePdaIxArgs,
) -> std::io::Result<Instruction> {
    marginfi_account_initialize_pda_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn marginfi_account_initialize_pda_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountInitializePdaAccounts<'_, '_>,
    args: MarginfiAccountInitializePdaIxArgs,
) -> ProgramResult {
    let keys: MarginfiAccountInitializePdaKeys = accounts.into();
    let ix = marginfi_account_initialize_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_initialize_pda_invoke(
    accounts: MarginfiAccountInitializePdaAccounts<'_, '_>,
    args: MarginfiAccountInitializePdaIxArgs,
) -> ProgramResult {
    marginfi_account_initialize_pda_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn marginfi_account_initialize_pda_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountInitializePdaAccounts<'_, '_>,
    args: MarginfiAccountInitializePdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountInitializePdaKeys = accounts.into();
    let ix = marginfi_account_initialize_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_initialize_pda_invoke_signed(
    accounts: MarginfiAccountInitializePdaAccounts<'_, '_>,
    args: MarginfiAccountInitializePdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_initialize_pda_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn marginfi_account_initialize_pda_verify_account_keys(
    accounts: MarginfiAccountInitializePdaAccounts<'_, '_>,
    keys: MarginfiAccountInitializePdaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_initialize_pda_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountInitializePdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account, accounts.fee_payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_initialize_pda_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountInitializePdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_initialize_pda_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountInitializePdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_initialize_pda_verify_writable_privileges(accounts)?;
    marginfi_account_initialize_pda_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountKeeperCloseOrderAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountKeeperCloseOrderKeys {
    pub marginfi_account: Pubkey,
    pub fee_recipient: Pubkey,
    pub order: Pubkey,
}
impl From<MarginfiAccountKeeperCloseOrderAccounts<'_, '_>>
for MarginfiAccountKeeperCloseOrderKeys {
    fn from(accounts: MarginfiAccountKeeperCloseOrderAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            fee_recipient: *accounts.fee_recipient.key,
            order: *accounts.order.key,
        }
    }
}
impl From<MarginfiAccountKeeperCloseOrderKeys>
for [AccountMeta; MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountKeeperCloseOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountKeeperCloseOrderKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            fee_recipient: pubkeys[1],
            order: pubkeys[2],
        }
    }
}
impl<'info> From<MarginfiAccountKeeperCloseOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountKeeperCloseOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.fee_recipient.clone(),
            accounts.order.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountKeeperCloseOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            fee_recipient: &arr[1],
            order: &arr[2],
        }
    }
}
pub const MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_DISCM: [u8; 8usize] = [
    128, 114, 71, 46, 194, 71, 186, 106,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountKeeperCloseOrderIxData;
impl MarginfiAccountKeeperCloseOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_keeper_close_order_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountKeeperCloseOrderKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_KEEPER_CLOSE_ORDER_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiAccountKeeperCloseOrderIxData.try_to_vec()?,
    })
}
pub fn marginfi_account_keeper_close_order_ix(
    keys: MarginfiAccountKeeperCloseOrderKeys,
) -> std::io::Result<Instruction> {
    marginfi_account_keeper_close_order_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn marginfi_account_keeper_close_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountKeeperCloseOrderAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiAccountKeeperCloseOrderKeys = accounts.into();
    let ix = marginfi_account_keeper_close_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_keeper_close_order_invoke(
    accounts: MarginfiAccountKeeperCloseOrderAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_account_keeper_close_order_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn marginfi_account_keeper_close_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountKeeperCloseOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountKeeperCloseOrderKeys = accounts.into();
    let ix = marginfi_account_keeper_close_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_keeper_close_order_invoke_signed(
    accounts: MarginfiAccountKeeperCloseOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_keeper_close_order_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_account_keeper_close_order_verify_account_keys(
    accounts: MarginfiAccountKeeperCloseOrderAccounts<'_, '_>,
    keys: MarginfiAccountKeeperCloseOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.order.key, keys.order),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_keeper_close_order_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountKeeperCloseOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_recipient, accounts.order] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_keeper_close_order_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountKeeperCloseOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_keeper_close_order_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_PLACE_ORDER_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountPlaceOrderAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub global_fee_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountPlaceOrderKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub fee_payer: Pubkey,
    pub authority: Pubkey,
    pub order: Pubkey,
    pub fee_state: Pubkey,
    pub global_fee_wallet: Pubkey,
    pub system_program: Pubkey,
}
impl From<MarginfiAccountPlaceOrderAccounts<'_, '_>> for MarginfiAccountPlaceOrderKeys {
    fn from(accounts: MarginfiAccountPlaceOrderAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            fee_payer: *accounts.fee_payer.key,
            authority: *accounts.authority.key,
            order: *accounts.order.key,
            fee_state: *accounts.fee_state.key,
            global_fee_wallet: *accounts.global_fee_wallet.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MarginfiAccountPlaceOrderKeys>
for [AccountMeta; MARGINFI_ACCOUNT_PLACE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountPlaceOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_fee_wallet,
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
impl From<[Pubkey; MARGINFI_ACCOUNT_PLACE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountPlaceOrderKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_ACCOUNT_PLACE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            fee_payer: pubkeys[2],
            authority: pubkeys[3],
            order: pubkeys[4],
            fee_state: pubkeys[5],
            global_fee_wallet: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<MarginfiAccountPlaceOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_PLACE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountPlaceOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.fee_payer.clone(),
            accounts.authority.clone(),
            accounts.order.clone(),
            accounts.fee_state.clone(),
            accounts.global_fee_wallet.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_PLACE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountPlaceOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_ACCOUNT_PLACE_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            fee_payer: &arr[2],
            authority: &arr[3],
            order: &arr[4],
            fee_state: &arr[5],
            global_fee_wallet: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const MARGINFI_ACCOUNT_PLACE_ORDER_IX_DISCM: [u8; 8usize] = [
    244, 112, 75, 138, 143, 108, 7, 186,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountPlaceOrderIxArgs {
    pub bank_keys: Vec<Pubkey>,
    pub trigger: OrderTrigger,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountPlaceOrderIxData(pub MarginfiAccountPlaceOrderIxArgs);
impl From<MarginfiAccountPlaceOrderIxArgs> for MarginfiAccountPlaceOrderIxData {
    fn from(args: MarginfiAccountPlaceOrderIxArgs) -> Self {
        Self(args)
    }
}
impl MarginfiAccountPlaceOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_PLACE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_keys: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let trigger = <OrderTrigger as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(MarginfiAccountPlaceOrderIxArgs {
                bank_keys,
                trigger,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_PLACE_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_keys, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.trigger, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_place_order_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountPlaceOrderKeys,
    args: MarginfiAccountPlaceOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_PLACE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: MarginfiAccountPlaceOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marginfi_account_place_order_ix(
    keys: MarginfiAccountPlaceOrderKeys,
    args: MarginfiAccountPlaceOrderIxArgs,
) -> std::io::Result<Instruction> {
    marginfi_account_place_order_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn marginfi_account_place_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountPlaceOrderAccounts<'_, '_>,
    args: MarginfiAccountPlaceOrderIxArgs,
) -> ProgramResult {
    let keys: MarginfiAccountPlaceOrderKeys = accounts.into();
    let ix = marginfi_account_place_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_place_order_invoke(
    accounts: MarginfiAccountPlaceOrderAccounts<'_, '_>,
    args: MarginfiAccountPlaceOrderIxArgs,
) -> ProgramResult {
    marginfi_account_place_order_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn marginfi_account_place_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountPlaceOrderAccounts<'_, '_>,
    args: MarginfiAccountPlaceOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountPlaceOrderKeys = accounts.into();
    let ix = marginfi_account_place_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_place_order_invoke_signed(
    accounts: MarginfiAccountPlaceOrderAccounts<'_, '_>,
    args: MarginfiAccountPlaceOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_place_order_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn marginfi_account_place_order_verify_account_keys(
    accounts: MarginfiAccountPlaceOrderAccounts<'_, '_>,
    keys: MarginfiAccountPlaceOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.order.key, keys.order),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.global_fee_wallet.key, keys.global_fee_wallet),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_place_order_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountPlaceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.fee_payer,
        accounts.order,
        accounts.global_fee_wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_place_order_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountPlaceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_place_order_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountPlaceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_place_order_verify_writable_privileges(accounts)?;
    marginfi_account_place_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_SET_FREEZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountSetFreezeAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountSetFreezeKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub admin: Pubkey,
}
impl From<MarginfiAccountSetFreezeAccounts<'_, '_>> for MarginfiAccountSetFreezeKeys {
    fn from(accounts: MarginfiAccountSetFreezeAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<MarginfiAccountSetFreezeKeys>
for [AccountMeta; MARGINFI_ACCOUNT_SET_FREEZE_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountSetFreezeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
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
impl From<[Pubkey; MARGINFI_ACCOUNT_SET_FREEZE_IX_ACCOUNTS_LEN]>
for MarginfiAccountSetFreezeKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_ACCOUNT_SET_FREEZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<MarginfiAccountSetFreezeAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_SET_FREEZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountSetFreezeAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_SET_FREEZE_IX_ACCOUNTS_LEN]>
for MarginfiAccountSetFreezeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_ACCOUNT_SET_FREEZE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const MARGINFI_ACCOUNT_SET_FREEZE_IX_DISCM: [u8; 8usize] = [
    199, 179, 231, 30, 138, 247, 110, 227,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountSetFreezeIxArgs {
    pub frozen: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountSetFreezeIxData(pub MarginfiAccountSetFreezeIxArgs);
impl From<MarginfiAccountSetFreezeIxArgs> for MarginfiAccountSetFreezeIxData {
    fn from(args: MarginfiAccountSetFreezeIxArgs) -> Self {
        Self(args)
    }
}
impl MarginfiAccountSetFreezeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_SET_FREEZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let frozen: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MarginfiAccountSetFreezeIxArgs {
                frozen,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_SET_FREEZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.frozen, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_set_freeze_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountSetFreezeKeys,
    args: MarginfiAccountSetFreezeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_SET_FREEZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: MarginfiAccountSetFreezeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marginfi_account_set_freeze_ix(
    keys: MarginfiAccountSetFreezeKeys,
    args: MarginfiAccountSetFreezeIxArgs,
) -> std::io::Result<Instruction> {
    marginfi_account_set_freeze_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn marginfi_account_set_freeze_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountSetFreezeAccounts<'_, '_>,
    args: MarginfiAccountSetFreezeIxArgs,
) -> ProgramResult {
    let keys: MarginfiAccountSetFreezeKeys = accounts.into();
    let ix = marginfi_account_set_freeze_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_set_freeze_invoke(
    accounts: MarginfiAccountSetFreezeAccounts<'_, '_>,
    args: MarginfiAccountSetFreezeIxArgs,
) -> ProgramResult {
    marginfi_account_set_freeze_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn marginfi_account_set_freeze_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountSetFreezeAccounts<'_, '_>,
    args: MarginfiAccountSetFreezeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountSetFreezeKeys = accounts.into();
    let ix = marginfi_account_set_freeze_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_set_freeze_invoke_signed(
    accounts: MarginfiAccountSetFreezeAccounts<'_, '_>,
    args: MarginfiAccountSetFreezeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_set_freeze_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn marginfi_account_set_freeze_verify_account_keys(
    accounts: MarginfiAccountSetFreezeAccounts<'_, '_>,
    keys: MarginfiAccountSetFreezeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_set_freeze_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountSetFreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_set_freeze_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountSetFreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_set_freeze_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountSetFreezeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_set_freeze_verify_writable_privileges(accounts)?;
    marginfi_account_set_freeze_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountSetKeeperCloseFlagsAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountSetKeeperCloseFlagsKeys {
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
}
impl From<MarginfiAccountSetKeeperCloseFlagsAccounts<'_, '_>>
for MarginfiAccountSetKeeperCloseFlagsKeys {
    fn from(accounts: MarginfiAccountSetKeeperCloseFlagsAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<MarginfiAccountSetKeeperCloseFlagsKeys>
for [AccountMeta; MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountSetKeeperCloseFlagsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_ACCOUNTS_LEN]>
for MarginfiAccountSetKeeperCloseFlagsKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<MarginfiAccountSetKeeperCloseFlagsAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_account.clone(), accounts.authority.clone()]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_ACCOUNTS_LEN],
> for MarginfiAccountSetKeeperCloseFlagsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_DISCM: [u8; 8usize] = [
    82, 163, 165, 222, 212, 255, 33, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountSetKeeperCloseFlagsIxArgs {
    pub bank_keys_opt: Option<Vec<Pubkey>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountSetKeeperCloseFlagsIxData(
    pub MarginfiAccountSetKeeperCloseFlagsIxArgs,
);
impl From<MarginfiAccountSetKeeperCloseFlagsIxArgs>
for MarginfiAccountSetKeeperCloseFlagsIxData {
    fn from(args: MarginfiAccountSetKeeperCloseFlagsIxArgs) -> Self {
        Self(args)
    }
}
impl MarginfiAccountSetKeeperCloseFlagsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bank_keys_opt: Option<Vec<Pubkey>> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(MarginfiAccountSetKeeperCloseFlagsIxArgs {
                bank_keys_opt,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bank_keys_opt, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_set_keeper_close_flags_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountSetKeeperCloseFlagsKeys,
    args: MarginfiAccountSetKeeperCloseFlagsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_SET_KEEPER_CLOSE_FLAGS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: MarginfiAccountSetKeeperCloseFlagsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marginfi_account_set_keeper_close_flags_ix(
    keys: MarginfiAccountSetKeeperCloseFlagsKeys,
    args: MarginfiAccountSetKeeperCloseFlagsIxArgs,
) -> std::io::Result<Instruction> {
    marginfi_account_set_keeper_close_flags_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn marginfi_account_set_keeper_close_flags_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'_, '_>,
    args: MarginfiAccountSetKeeperCloseFlagsIxArgs,
) -> ProgramResult {
    let keys: MarginfiAccountSetKeeperCloseFlagsKeys = accounts.into();
    let ix = marginfi_account_set_keeper_close_flags_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_set_keeper_close_flags_invoke(
    accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'_, '_>,
    args: MarginfiAccountSetKeeperCloseFlagsIxArgs,
) -> ProgramResult {
    marginfi_account_set_keeper_close_flags_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn marginfi_account_set_keeper_close_flags_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'_, '_>,
    args: MarginfiAccountSetKeeperCloseFlagsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountSetKeeperCloseFlagsKeys = accounts.into();
    let ix = marginfi_account_set_keeper_close_flags_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_set_keeper_close_flags_invoke_signed(
    accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'_, '_>,
    args: MarginfiAccountSetKeeperCloseFlagsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_set_keeper_close_flags_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn marginfi_account_set_keeper_close_flags_verify_account_keys(
    accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'_, '_>,
    keys: MarginfiAccountSetKeeperCloseFlagsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_set_keeper_close_flags_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_set_keeper_close_flags_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_set_keeper_close_flags_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountSetKeeperCloseFlagsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_set_keeper_close_flags_verify_writable_privileges(accounts)?;
    marginfi_account_set_keeper_close_flags_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountStartExecuteOrderAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub executor: &'me AccountInfo<'info>,
    pub order: &'me AccountInfo<'info>,
    pub execute_record: &'me AccountInfo<'info>,
    pub instruction_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountStartExecuteOrderKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub fee_payer: Pubkey,
    pub executor: Pubkey,
    pub order: Pubkey,
    pub execute_record: Pubkey,
    pub instruction_sysvar: Pubkey,
    pub system_program: Pubkey,
}
impl From<MarginfiAccountStartExecuteOrderAccounts<'_, '_>>
for MarginfiAccountStartExecuteOrderKeys {
    fn from(accounts: MarginfiAccountStartExecuteOrderAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            fee_payer: *accounts.fee_payer.key,
            executor: *accounts.executor.key,
            order: *accounts.order.key,
            execute_record: *accounts.execute_record.key,
            instruction_sysvar: *accounts.instruction_sysvar.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MarginfiAccountStartExecuteOrderKeys>
for [AccountMeta; MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountStartExecuteOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.executor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.execute_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar,
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
impl From<[Pubkey; MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountStartExecuteOrderKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            fee_payer: pubkeys[2],
            executor: pubkeys[3],
            order: pubkeys[4],
            execute_record: pubkeys[5],
            instruction_sysvar: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<MarginfiAccountStartExecuteOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiAccountStartExecuteOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.fee_payer.clone(),
            accounts.executor.clone(),
            accounts.order.clone(),
            accounts.execute_record.clone(),
            accounts.instruction_sysvar.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_ACCOUNTS_LEN]>
for MarginfiAccountStartExecuteOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            fee_payer: &arr[2],
            executor: &arr[3],
            order: &arr[4],
            execute_record: &arr[5],
            instruction_sysvar: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_DISCM: [u8; 8usize] = [
    1, 70, 140, 134, 183, 29, 208, 224,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountStartExecuteOrderIxData;
impl MarginfiAccountStartExecuteOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_start_execute_order_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountStartExecuteOrderKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_START_EXECUTE_ORDER_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiAccountStartExecuteOrderIxData.try_to_vec()?,
    })
}
pub fn marginfi_account_start_execute_order_ix(
    keys: MarginfiAccountStartExecuteOrderKeys,
) -> std::io::Result<Instruction> {
    marginfi_account_start_execute_order_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn marginfi_account_start_execute_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountStartExecuteOrderAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiAccountStartExecuteOrderKeys = accounts.into();
    let ix = marginfi_account_start_execute_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_start_execute_order_invoke(
    accounts: MarginfiAccountStartExecuteOrderAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_account_start_execute_order_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn marginfi_account_start_execute_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountStartExecuteOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountStartExecuteOrderKeys = accounts.into();
    let ix = marginfi_account_start_execute_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_start_execute_order_invoke_signed(
    accounts: MarginfiAccountStartExecuteOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_start_execute_order_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_account_start_execute_order_verify_account_keys(
    accounts: MarginfiAccountStartExecuteOrderAccounts<'_, '_>,
    keys: MarginfiAccountStartExecuteOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.executor.key, keys.executor),
        (*accounts.order.key, keys.order),
        (*accounts.execute_record.key, keys.execute_record),
        (*accounts.instruction_sysvar.key, keys.instruction_sysvar),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_start_execute_order_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiAccountStartExecuteOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.fee_payer,
        accounts.order,
        accounts.execute_record,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_start_execute_order_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiAccountStartExecuteOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_start_execute_order_verify_account_privileges<'me, 'info>(
    accounts: MarginfiAccountStartExecuteOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_start_execute_order_verify_writable_privileges(accounts)?;
    marginfi_account_start_execute_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub destination_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiAccountUpdateEmissionsDestinationAccountKeys {
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub destination_account: Pubkey,
}
impl From<MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'_, '_>>
for MarginfiAccountUpdateEmissionsDestinationAccountKeys {
    fn from(accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            destination_account: *accounts.destination_account.key,
        }
    }
}
impl From<MarginfiAccountUpdateEmissionsDestinationAccountKeys>
for [AccountMeta; MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiAccountUpdateEmissionsDestinationAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<
    [Pubkey; MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN],
> for MarginfiAccountUpdateEmissionsDestinationAccountKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            authority: pubkeys[1],
            destination_account: pubkeys[2],
        }
    }
}
impl<'info> From<MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'_, 'info>>
for [AccountInfo<
    'info,
>; MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.destination_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN],
> for MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_account: &arr[0],
            authority: &arr[1],
            destination_account: &arr[2],
        }
    }
}
pub const MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    73, 185, 162, 201, 111, 24, 116, 185,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountUpdateEmissionsDestinationAccountIxData;
impl MarginfiAccountUpdateEmissionsDestinationAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_DISCM
        {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_account_update_emissions_destination_account_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiAccountUpdateEmissionsDestinationAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_ACCOUNT_UPDATE_EMISSIONS_DESTINATION_ACCOUNT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiAccountUpdateEmissionsDestinationAccountIxData.try_to_vec()?,
    })
}
pub fn marginfi_account_update_emissions_destination_account_ix(
    keys: MarginfiAccountUpdateEmissionsDestinationAccountKeys,
) -> std::io::Result<Instruction> {
    marginfi_account_update_emissions_destination_account_ix_with_program_id(
        MARGINFI_PROGRAM_ID,
        keys,
    )
}
pub fn marginfi_account_update_emissions_destination_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiAccountUpdateEmissionsDestinationAccountKeys = accounts.into();
    let ix = marginfi_account_update_emissions_destination_account_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_account_update_emissions_destination_account_invoke(
    accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_account_update_emissions_destination_account_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
    )
}
pub fn marginfi_account_update_emissions_destination_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiAccountUpdateEmissionsDestinationAccountKeys = accounts.into();
    let ix = marginfi_account_update_emissions_destination_account_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_account_update_emissions_destination_account_invoke_signed(
    accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_account_update_emissions_destination_account_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_account_update_emissions_destination_account_verify_account_keys(
    accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'_, '_>,
    keys: MarginfiAccountUpdateEmissionsDestinationAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.destination_account.key, keys.destination_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_account_update_emissions_destination_account_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_account_update_emissions_destination_account_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_account_update_emissions_destination_account_verify_account_privileges<
    'me,
    'info,
>(
    accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_account_update_emissions_destination_account_verify_writable_privileges(
        accounts,
    )?;
    marginfi_account_update_emissions_destination_account_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const MARGINFI_GROUP_CONFIGURE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiGroupConfigureAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiGroupConfigureKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
}
impl From<MarginfiGroupConfigureAccounts<'_, '_>> for MarginfiGroupConfigureKeys {
    fn from(accounts: MarginfiGroupConfigureAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<MarginfiGroupConfigureKeys>
for [AccountMeta; MARGINFI_GROUP_CONFIGURE_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiGroupConfigureKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
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
impl From<[Pubkey; MARGINFI_GROUP_CONFIGURE_IX_ACCOUNTS_LEN]>
for MarginfiGroupConfigureKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_GROUP_CONFIGURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<MarginfiGroupConfigureAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_GROUP_CONFIGURE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiGroupConfigureAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_group.clone(), accounts.admin.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_GROUP_CONFIGURE_IX_ACCOUNTS_LEN]>
for MarginfiGroupConfigureAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_GROUP_CONFIGURE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const MARGINFI_GROUP_CONFIGURE_IX_DISCM: [u8; 8usize] = [
    62, 199, 81, 78, 33, 13, 236, 61,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiGroupConfigureIxArgs {
    pub new_admin: Option<Pubkey>,
    pub new_emode_admin: Option<Pubkey>,
    pub new_curve_admin: Option<Pubkey>,
    pub new_limit_admin: Option<Pubkey>,
    pub new_flow_admin: Option<Pubkey>,
    pub new_emissions_admin: Option<Pubkey>,
    pub new_metadata_admin: Option<Pubkey>,
    pub new_risk_admin: Option<Pubkey>,
    pub emode_max_init_leverage: Option<WrappedI80F48>,
    pub emode_max_maint_leverage: Option<WrappedI80F48>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiGroupConfigureIxData(pub MarginfiGroupConfigureIxArgs);
impl From<MarginfiGroupConfigureIxArgs> for MarginfiGroupConfigureIxData {
    fn from(args: MarginfiGroupConfigureIxArgs) -> Self {
        Self(args)
    }
}
impl MarginfiGroupConfigureIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_GROUP_CONFIGURE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let new_emode_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let new_curve_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let new_limit_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let new_flow_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let new_emissions_admin: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_metadata_admin: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_risk_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let emode_max_init_leverage: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let emode_max_maint_leverage: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(MarginfiGroupConfigureIxArgs {
                new_admin,
                new_emode_admin,
                new_curve_admin,
                new_limit_admin,
                new_flow_admin,
                new_emissions_admin,
                new_metadata_admin,
                new_risk_admin,
                emode_max_init_leverage,
                emode_max_maint_leverage,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_GROUP_CONFIGURE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_emode_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_curve_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_limit_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_flow_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_emissions_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_metadata_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_risk_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.emode_max_init_leverage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.emode_max_maint_leverage, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_group_configure_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiGroupConfigureKeys,
    args: MarginfiGroupConfigureIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_GROUP_CONFIGURE_IX_ACCOUNTS_LEN] = keys.into();
    let data: MarginfiGroupConfigureIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marginfi_group_configure_ix(
    keys: MarginfiGroupConfigureKeys,
    args: MarginfiGroupConfigureIxArgs,
) -> std::io::Result<Instruction> {
    marginfi_group_configure_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn marginfi_group_configure_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiGroupConfigureAccounts<'_, '_>,
    args: MarginfiGroupConfigureIxArgs,
) -> ProgramResult {
    let keys: MarginfiGroupConfigureKeys = accounts.into();
    let ix = marginfi_group_configure_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_group_configure_invoke(
    accounts: MarginfiGroupConfigureAccounts<'_, '_>,
    args: MarginfiGroupConfigureIxArgs,
) -> ProgramResult {
    marginfi_group_configure_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn marginfi_group_configure_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiGroupConfigureAccounts<'_, '_>,
    args: MarginfiGroupConfigureIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiGroupConfigureKeys = accounts.into();
    let ix = marginfi_group_configure_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_group_configure_invoke_signed(
    accounts: MarginfiGroupConfigureAccounts<'_, '_>,
    args: MarginfiGroupConfigureIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_group_configure_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn marginfi_group_configure_verify_account_keys(
    accounts: MarginfiGroupConfigureAccounts<'_, '_>,
    keys: MarginfiGroupConfigureKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_group_configure_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiGroupConfigureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_group] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_group_configure_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiGroupConfigureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_group_configure_verify_account_privileges<'me, 'info>(
    accounts: MarginfiGroupConfigureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_group_configure_verify_writable_privileges(accounts)?;
    marginfi_group_configure_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_GROUP_INITIALIZE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiGroupInitializeAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiGroupInitializeKeys {
    pub marginfi_group: Pubkey,
    pub admin: Pubkey,
    pub fee_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<MarginfiGroupInitializeAccounts<'_, '_>> for MarginfiGroupInitializeKeys {
    fn from(accounts: MarginfiGroupInitializeAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            admin: *accounts.admin.key,
            fee_state: *accounts.fee_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MarginfiGroupInitializeKeys>
for [AccountMeta; MARGINFI_GROUP_INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiGroupInitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
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
impl From<[Pubkey; MARGINFI_GROUP_INITIALIZE_IX_ACCOUNTS_LEN]>
for MarginfiGroupInitializeKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_GROUP_INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            admin: pubkeys[1],
            fee_state: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<MarginfiGroupInitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_GROUP_INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiGroupInitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.admin.clone(),
            accounts.fee_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_GROUP_INITIALIZE_IX_ACCOUNTS_LEN]>
for MarginfiGroupInitializeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_GROUP_INITIALIZE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            admin: &arr[1],
            fee_state: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const MARGINFI_GROUP_INITIALIZE_IX_DISCM: [u8; 8usize] = [
    255, 67, 67, 26, 94, 31, 34, 20,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiGroupInitializeIxData;
impl MarginfiGroupInitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_GROUP_INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_GROUP_INITIALIZE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_group_initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiGroupInitializeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_GROUP_INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiGroupInitializeIxData.try_to_vec()?,
    })
}
pub fn marginfi_group_initialize_ix(
    keys: MarginfiGroupInitializeKeys,
) -> std::io::Result<Instruction> {
    marginfi_group_initialize_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn marginfi_group_initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiGroupInitializeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiGroupInitializeKeys = accounts.into();
    let ix = marginfi_group_initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_group_initialize_invoke(
    accounts: MarginfiGroupInitializeAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_group_initialize_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn marginfi_group_initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiGroupInitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiGroupInitializeKeys = accounts.into();
    let ix = marginfi_group_initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_group_initialize_invoke_signed(
    accounts: MarginfiGroupInitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_group_initialize_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_group_initialize_verify_account_keys(
    accounts: MarginfiGroupInitializeAccounts<'_, '_>,
    keys: MarginfiGroupInitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_group_initialize_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiGroupInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_group, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_group_initialize_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiGroupInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.marginfi_group, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_group_initialize_verify_account_privileges<'me, 'info>(
    accounts: MarginfiGroupInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_group_initialize_verify_writable_privileges(accounts)?;
    marginfi_group_initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_CURVE_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct MigrateCurveAccounts<'me, 'info> {
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateCurveKeys {
    pub bank: Pubkey,
}
impl From<MigrateCurveAccounts<'_, '_>> for MigrateCurveKeys {
    fn from(accounts: MigrateCurveAccounts) -> Self {
        Self { bank: *accounts.bank.key }
    }
}
impl From<MigrateCurveKeys> for [AccountMeta; MIGRATE_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateCurveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_CURVE_IX_ACCOUNTS_LEN]> for MigrateCurveKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self { bank: pubkeys[0] }
    }
}
impl<'info> From<MigrateCurveAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_CURVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateCurveAccounts<'_, 'info>) -> Self {
        [accounts.bank.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_CURVE_IX_ACCOUNTS_LEN]>
for MigrateCurveAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self { bank: &arr[0] }
    }
}
pub const MIGRATE_CURVE_IX_DISCM: [u8; 8usize] = [151, 254, 50, 13, 112, 235, 152, 72];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateCurveIxData;
impl MigrateCurveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_CURVE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_CURVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_curve_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateCurveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_CURVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateCurveIxData.try_to_vec()?,
    })
}
pub fn migrate_curve_ix(keys: MigrateCurveKeys) -> std::io::Result<Instruction> {
    migrate_curve_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn migrate_curve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateCurveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateCurveKeys = accounts.into();
    let ix = migrate_curve_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_curve_invoke(accounts: MigrateCurveAccounts<'_, '_>) -> ProgramResult {
    migrate_curve_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn migrate_curve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateCurveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateCurveKeys = accounts.into();
    let ix = migrate_curve_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_curve_invoke_signed(
    accounts: MigrateCurveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_curve_invoke_signed_with_program_id(MARGINFI_PROGRAM_ID, accounts, seeds)
}
pub fn migrate_curve_verify_account_keys(
    accounts: MigrateCurveAccounts<'_, '_>,
    keys: MigrateCurveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.bank.key, keys.bank)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_curve_verify_writable_privileges<'me, 'info>(
    accounts: MigrateCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_curve_verify_account_privileges<'me, 'info>(
    accounts: MigrateCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_curve_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PANIC_PAUSE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PanicPauseAccounts<'me, 'info> {
    pub global_fee_admin: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PanicPauseKeys {
    pub global_fee_admin: Pubkey,
    pub fee_state: Pubkey,
}
impl From<PanicPauseAccounts<'_, '_>> for PanicPauseKeys {
    fn from(accounts: PanicPauseAccounts) -> Self {
        Self {
            global_fee_admin: *accounts.global_fee_admin.key,
            fee_state: *accounts.fee_state.key,
        }
    }
}
impl From<PanicPauseKeys> for [AccountMeta; PANIC_PAUSE_IX_ACCOUNTS_LEN] {
    fn from(keys: PanicPauseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_fee_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PANIC_PAUSE_IX_ACCOUNTS_LEN]> for PanicPauseKeys {
    fn from(pubkeys: [Pubkey; PANIC_PAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_fee_admin: pubkeys[0],
            fee_state: pubkeys[1],
        }
    }
}
impl<'info> From<PanicPauseAccounts<'_, 'info>>
for [AccountInfo<'info>; PANIC_PAUSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PanicPauseAccounts<'_, 'info>) -> Self {
        [accounts.global_fee_admin.clone(), accounts.fee_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PANIC_PAUSE_IX_ACCOUNTS_LEN]>
for PanicPauseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PANIC_PAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_fee_admin: &arr[0],
            fee_state: &arr[1],
        }
    }
}
pub const PANIC_PAUSE_IX_DISCM: [u8; 8usize] = [76, 164, 123, 25, 4, 43, 79, 165];
#[derive(Clone, Debug, PartialEq)]
pub struct PanicPauseIxData;
impl PanicPauseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PANIC_PAUSE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PANIC_PAUSE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn panic_pause_ix_with_program_id(
    program_id: Pubkey,
    keys: PanicPauseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PANIC_PAUSE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PanicPauseIxData.try_to_vec()?,
    })
}
pub fn panic_pause_ix(keys: PanicPauseKeys) -> std::io::Result<Instruction> {
    panic_pause_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn panic_pause_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PanicPauseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PanicPauseKeys = accounts.into();
    let ix = panic_pause_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn panic_pause_invoke(accounts: PanicPauseAccounts<'_, '_>) -> ProgramResult {
    panic_pause_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn panic_pause_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PanicPauseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PanicPauseKeys = accounts.into();
    let ix = panic_pause_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn panic_pause_invoke_signed(
    accounts: PanicPauseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    panic_pause_invoke_signed_with_program_id(MARGINFI_PROGRAM_ID, accounts, seeds)
}
pub fn panic_pause_verify_account_keys(
    accounts: PanicPauseAccounts<'_, '_>,
    keys: PanicPauseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global_fee_admin.key, keys.global_fee_admin),
        (*accounts.fee_state.key, keys.fee_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn panic_pause_verify_writable_privileges<'me, 'info>(
    accounts: PanicPauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn panic_pause_verify_signer_privileges<'me, 'info>(
    accounts: PanicPauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_fee_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn panic_pause_verify_account_privileges<'me, 'info>(
    accounts: PanicPauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    panic_pause_verify_writable_privileges(accounts)?;
    panic_pause_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PANIC_UNPAUSE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PanicUnpauseAccounts<'me, 'info> {
    pub global_fee_admin: &'me AccountInfo<'info>,
    pub fee_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PanicUnpauseKeys {
    pub global_fee_admin: Pubkey,
    pub fee_state: Pubkey,
}
impl From<PanicUnpauseAccounts<'_, '_>> for PanicUnpauseKeys {
    fn from(accounts: PanicUnpauseAccounts) -> Self {
        Self {
            global_fee_admin: *accounts.global_fee_admin.key,
            fee_state: *accounts.fee_state.key,
        }
    }
}
impl From<PanicUnpauseKeys> for [AccountMeta; PANIC_UNPAUSE_IX_ACCOUNTS_LEN] {
    fn from(keys: PanicUnpauseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global_fee_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PANIC_UNPAUSE_IX_ACCOUNTS_LEN]> for PanicUnpauseKeys {
    fn from(pubkeys: [Pubkey; PANIC_UNPAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_fee_admin: pubkeys[0],
            fee_state: pubkeys[1],
        }
    }
}
impl<'info> From<PanicUnpauseAccounts<'_, 'info>>
for [AccountInfo<'info>; PANIC_UNPAUSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PanicUnpauseAccounts<'_, 'info>) -> Self {
        [accounts.global_fee_admin.clone(), accounts.fee_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PANIC_UNPAUSE_IX_ACCOUNTS_LEN]>
for PanicUnpauseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PANIC_UNPAUSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global_fee_admin: &arr[0],
            fee_state: &arr[1],
        }
    }
}
pub const PANIC_UNPAUSE_IX_DISCM: [u8; 8usize] = [236, 107, 194, 242, 99, 51, 121, 128];
#[derive(Clone, Debug, PartialEq)]
pub struct PanicUnpauseIxData;
impl PanicUnpauseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PANIC_UNPAUSE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PANIC_UNPAUSE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn panic_unpause_ix_with_program_id(
    program_id: Pubkey,
    keys: PanicUnpauseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PANIC_UNPAUSE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PanicUnpauseIxData.try_to_vec()?,
    })
}
pub fn panic_unpause_ix(keys: PanicUnpauseKeys) -> std::io::Result<Instruction> {
    panic_unpause_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn panic_unpause_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PanicUnpauseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PanicUnpauseKeys = accounts.into();
    let ix = panic_unpause_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn panic_unpause_invoke(accounts: PanicUnpauseAccounts<'_, '_>) -> ProgramResult {
    panic_unpause_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn panic_unpause_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PanicUnpauseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PanicUnpauseKeys = accounts.into();
    let ix = panic_unpause_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn panic_unpause_invoke_signed(
    accounts: PanicUnpauseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    panic_unpause_invoke_signed_with_program_id(MARGINFI_PROGRAM_ID, accounts, seeds)
}
pub fn panic_unpause_verify_account_keys(
    accounts: PanicUnpauseAccounts<'_, '_>,
    keys: PanicUnpauseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global_fee_admin.key, keys.global_fee_admin),
        (*accounts.fee_state.key, keys.fee_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn panic_unpause_verify_writable_privileges<'me, 'info>(
    accounts: PanicUnpauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_fee_admin, accounts.fee_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn panic_unpause_verify_signer_privileges<'me, 'info>(
    accounts: PanicUnpauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.global_fee_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn panic_unpause_verify_account_privileges<'me, 'info>(
    accounts: PanicUnpauseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    panic_unpause_verify_writable_privileges(accounts)?;
    panic_unpause_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PANIC_UNPAUSE_PERMISSIONLESS_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct PanicUnpausePermissionlessAccounts<'me, 'info> {
    pub fee_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PanicUnpausePermissionlessKeys {
    pub fee_state: Pubkey,
}
impl From<PanicUnpausePermissionlessAccounts<'_, '_>>
for PanicUnpausePermissionlessKeys {
    fn from(accounts: PanicUnpausePermissionlessAccounts) -> Self {
        Self {
            fee_state: *accounts.fee_state.key,
        }
    }
}
impl From<PanicUnpausePermissionlessKeys>
for [AccountMeta; PANIC_UNPAUSE_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(keys: PanicUnpausePermissionlessKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PANIC_UNPAUSE_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for PanicUnpausePermissionlessKeys {
    fn from(pubkeys: [Pubkey; PANIC_UNPAUSE_PERMISSIONLESS_IX_ACCOUNTS_LEN]) -> Self {
        Self { fee_state: pubkeys[0] }
    }
}
impl<'info> From<PanicUnpausePermissionlessAccounts<'_, 'info>>
for [AccountInfo<'info>; PANIC_UNPAUSE_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: PanicUnpausePermissionlessAccounts<'_, 'info>) -> Self {
        [accounts.fee_state.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PANIC_UNPAUSE_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for PanicUnpausePermissionlessAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PANIC_UNPAUSE_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { fee_state: &arr[0] }
    }
}
pub const PANIC_UNPAUSE_PERMISSIONLESS_IX_DISCM: [u8; 8usize] = [
    245, 139, 50, 159, 213, 62, 91, 248,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PanicUnpausePermissionlessIxData;
impl PanicUnpausePermissionlessIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PANIC_UNPAUSE_PERMISSIONLESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PANIC_UNPAUSE_PERMISSIONLESS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn panic_unpause_permissionless_ix_with_program_id(
    program_id: Pubkey,
    keys: PanicUnpausePermissionlessKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PANIC_UNPAUSE_PERMISSIONLESS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PanicUnpausePermissionlessIxData.try_to_vec()?,
    })
}
pub fn panic_unpause_permissionless_ix(
    keys: PanicUnpausePermissionlessKeys,
) -> std::io::Result<Instruction> {
    panic_unpause_permissionless_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn panic_unpause_permissionless_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PanicUnpausePermissionlessAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PanicUnpausePermissionlessKeys = accounts.into();
    let ix = panic_unpause_permissionless_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn panic_unpause_permissionless_invoke(
    accounts: PanicUnpausePermissionlessAccounts<'_, '_>,
) -> ProgramResult {
    panic_unpause_permissionless_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn panic_unpause_permissionless_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PanicUnpausePermissionlessAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PanicUnpausePermissionlessKeys = accounts.into();
    let ix = panic_unpause_permissionless_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn panic_unpause_permissionless_invoke_signed(
    accounts: PanicUnpausePermissionlessAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    panic_unpause_permissionless_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn panic_unpause_permissionless_verify_account_keys(
    accounts: PanicUnpausePermissionlessAccounts<'_, '_>,
    keys: PanicUnpausePermissionlessKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.fee_state.key, keys.fee_state)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn panic_unpause_permissionless_verify_writable_privileges<'me, 'info>(
    accounts: PanicUnpausePermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn panic_unpause_permissionless_verify_account_privileges<'me, 'info>(
    accounts: PanicUnpausePermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    panic_unpause_permissionless_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PROPAGATE_FEE_STATE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PropagateFeeStateAccounts<'me, 'info> {
    pub fee_state: &'me AccountInfo<'info>,
    pub marginfi_group: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PropagateFeeStateKeys {
    pub fee_state: Pubkey,
    pub marginfi_group: Pubkey,
}
impl From<PropagateFeeStateAccounts<'_, '_>> for PropagateFeeStateKeys {
    fn from(accounts: PropagateFeeStateAccounts) -> Self {
        Self {
            fee_state: *accounts.fee_state.key,
            marginfi_group: *accounts.marginfi_group.key,
        }
    }
}
impl From<PropagateFeeStateKeys> for [AccountMeta; PROPAGATE_FEE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: PropagateFeeStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PROPAGATE_FEE_STATE_IX_ACCOUNTS_LEN]> for PropagateFeeStateKeys {
    fn from(pubkeys: [Pubkey; PROPAGATE_FEE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_state: pubkeys[0],
            marginfi_group: pubkeys[1],
        }
    }
}
impl<'info> From<PropagateFeeStateAccounts<'_, 'info>>
for [AccountInfo<'info>; PROPAGATE_FEE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PropagateFeeStateAccounts<'_, 'info>) -> Self {
        [accounts.fee_state.clone(), accounts.marginfi_group.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PROPAGATE_FEE_STATE_IX_ACCOUNTS_LEN]>
for PropagateFeeStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PROPAGATE_FEE_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fee_state: &arr[0],
            marginfi_group: &arr[1],
        }
    }
}
pub const PROPAGATE_FEE_STATE_IX_DISCM: [u8; 8usize] = [
    64, 3, 166, 194, 129, 21, 101, 155,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PropagateFeeStateIxData;
impl PropagateFeeStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPAGATE_FEE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPAGATE_FEE_STATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn propagate_fee_state_ix_with_program_id(
    program_id: Pubkey,
    keys: PropagateFeeStateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROPAGATE_FEE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PropagateFeeStateIxData.try_to_vec()?,
    })
}
pub fn propagate_fee_state_ix(
    keys: PropagateFeeStateKeys,
) -> std::io::Result<Instruction> {
    propagate_fee_state_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn propagate_fee_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PropagateFeeStateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PropagateFeeStateKeys = accounts.into();
    let ix = propagate_fee_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn propagate_fee_state_invoke(
    accounts: PropagateFeeStateAccounts<'_, '_>,
) -> ProgramResult {
    propagate_fee_state_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn propagate_fee_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PropagateFeeStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PropagateFeeStateKeys = accounts.into();
    let ix = propagate_fee_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn propagate_fee_state_invoke_signed(
    accounts: PropagateFeeStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    propagate_fee_state_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn propagate_fee_state_verify_account_keys(
    accounts: PropagateFeeStateAccounts<'_, '_>,
    keys: PropagateFeeStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_state.key, keys.fee_state),
        (*accounts.marginfi_group.key, keys.marginfi_group),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn propagate_fee_state_verify_writable_privileges<'me, 'info>(
    accounts: PropagateFeeStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_group] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn propagate_fee_state_verify_account_privileges<'me, 'info>(
    accounts: PropagateFeeStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    propagate_fee_state_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PROPAGATE_STAKED_SETTINGS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PropagateStakedSettingsAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub staked_settings: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PropagateStakedSettingsKeys {
    pub marginfi_group: Pubkey,
    pub staked_settings: Pubkey,
    pub bank: Pubkey,
}
impl From<PropagateStakedSettingsAccounts<'_, '_>> for PropagateStakedSettingsKeys {
    fn from(accounts: PropagateStakedSettingsAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            staked_settings: *accounts.staked_settings.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<PropagateStakedSettingsKeys>
for [AccountMeta; PROPAGATE_STAKED_SETTINGS_IX_ACCOUNTS_LEN] {
    fn from(keys: PropagateStakedSettingsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staked_settings,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PROPAGATE_STAKED_SETTINGS_IX_ACCOUNTS_LEN]>
for PropagateStakedSettingsKeys {
    fn from(pubkeys: [Pubkey; PROPAGATE_STAKED_SETTINGS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            staked_settings: pubkeys[1],
            bank: pubkeys[2],
        }
    }
}
impl<'info> From<PropagateStakedSettingsAccounts<'_, 'info>>
for [AccountInfo<'info>; PROPAGATE_STAKED_SETTINGS_IX_ACCOUNTS_LEN] {
    fn from(accounts: PropagateStakedSettingsAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_group.clone(),
            accounts.staked_settings.clone(),
            accounts.bank.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PROPAGATE_STAKED_SETTINGS_IX_ACCOUNTS_LEN]>
for PropagateStakedSettingsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PROPAGATE_STAKED_SETTINGS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            staked_settings: &arr[1],
            bank: &arr[2],
        }
    }
}
pub const PROPAGATE_STAKED_SETTINGS_IX_DISCM: [u8; 8usize] = [
    210, 30, 152, 69, 130, 99, 222, 170,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PropagateStakedSettingsIxData;
impl PropagateStakedSettingsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPAGATE_STAKED_SETTINGS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPAGATE_STAKED_SETTINGS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn propagate_staked_settings_ix_with_program_id(
    program_id: Pubkey,
    keys: PropagateStakedSettingsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROPAGATE_STAKED_SETTINGS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PropagateStakedSettingsIxData.try_to_vec()?,
    })
}
pub fn propagate_staked_settings_ix(
    keys: PropagateStakedSettingsKeys,
) -> std::io::Result<Instruction> {
    propagate_staked_settings_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn propagate_staked_settings_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PropagateStakedSettingsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PropagateStakedSettingsKeys = accounts.into();
    let ix = propagate_staked_settings_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn propagate_staked_settings_invoke(
    accounts: PropagateStakedSettingsAccounts<'_, '_>,
) -> ProgramResult {
    propagate_staked_settings_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn propagate_staked_settings_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PropagateStakedSettingsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PropagateStakedSettingsKeys = accounts.into();
    let ix = propagate_staked_settings_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn propagate_staked_settings_invoke_signed(
    accounts: PropagateStakedSettingsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    propagate_staked_settings_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn propagate_staked_settings_verify_account_keys(
    accounts: PropagateStakedSettingsAccounts<'_, '_>,
    keys: PropagateStakedSettingsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.staked_settings.key, keys.staked_settings),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn propagate_staked_settings_verify_writable_privileges<'me, 'info>(
    accounts: PropagateStakedSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn propagate_staked_settings_verify_account_privileges<'me, 'info>(
    accounts: PropagateStakedSettingsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    propagate_staked_settings_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PURGE_DELEVERAGE_BALANCE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PurgeDeleverageBalanceAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub risk_admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PurgeDeleverageBalanceKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub risk_admin: Pubkey,
    pub bank: Pubkey,
}
impl From<PurgeDeleverageBalanceAccounts<'_, '_>> for PurgeDeleverageBalanceKeys {
    fn from(accounts: PurgeDeleverageBalanceAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            risk_admin: *accounts.risk_admin.key,
            bank: *accounts.bank.key,
        }
    }
}
impl From<PurgeDeleverageBalanceKeys>
for [AccountMeta; PURGE_DELEVERAGE_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: PurgeDeleverageBalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.risk_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PURGE_DELEVERAGE_BALANCE_IX_ACCOUNTS_LEN]>
for PurgeDeleverageBalanceKeys {
    fn from(pubkeys: [Pubkey; PURGE_DELEVERAGE_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            risk_admin: pubkeys[2],
            bank: pubkeys[3],
        }
    }
}
impl<'info> From<PurgeDeleverageBalanceAccounts<'_, 'info>>
for [AccountInfo<'info>; PURGE_DELEVERAGE_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PurgeDeleverageBalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.risk_admin.clone(),
            accounts.bank.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PURGE_DELEVERAGE_BALANCE_IX_ACCOUNTS_LEN]>
for PurgeDeleverageBalanceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PURGE_DELEVERAGE_BALANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            risk_admin: &arr[2],
            bank: &arr[3],
        }
    }
}
pub const PURGE_DELEVERAGE_BALANCE_IX_DISCM: [u8; 8usize] = [
    132, 187, 25, 149, 181, 59, 253, 136,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PurgeDeleverageBalanceIxData;
impl PurgeDeleverageBalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PURGE_DELEVERAGE_BALANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PURGE_DELEVERAGE_BALANCE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn purge_deleverage_balance_ix_with_program_id(
    program_id: Pubkey,
    keys: PurgeDeleverageBalanceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PURGE_DELEVERAGE_BALANCE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PurgeDeleverageBalanceIxData.try_to_vec()?,
    })
}
pub fn purge_deleverage_balance_ix(
    keys: PurgeDeleverageBalanceKeys,
) -> std::io::Result<Instruction> {
    purge_deleverage_balance_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn purge_deleverage_balance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PurgeDeleverageBalanceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PurgeDeleverageBalanceKeys = accounts.into();
    let ix = purge_deleverage_balance_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn purge_deleverage_balance_invoke(
    accounts: PurgeDeleverageBalanceAccounts<'_, '_>,
) -> ProgramResult {
    purge_deleverage_balance_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn purge_deleverage_balance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PurgeDeleverageBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PurgeDeleverageBalanceKeys = accounts.into();
    let ix = purge_deleverage_balance_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn purge_deleverage_balance_invoke_signed(
    accounts: PurgeDeleverageBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    purge_deleverage_balance_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn purge_deleverage_balance_verify_account_keys(
    accounts: PurgeDeleverageBalanceAccounts<'_, '_>,
    keys: PurgeDeleverageBalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.risk_admin.key, keys.risk_admin),
        (*accounts.bank.key, keys.bank),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn purge_deleverage_balance_verify_writable_privileges<'me, 'info>(
    accounts: PurgeDeleverageBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account, accounts.bank] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn purge_deleverage_balance_verify_signer_privileges<'me, 'info>(
    accounts: PurgeDeleverageBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.risk_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn purge_deleverage_balance_verify_account_privileges<'me, 'info>(
    accounts: PurgeDeleverageBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    purge_deleverage_balance_verify_writable_privileges(accounts)?;
    purge_deleverage_balance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SOLEND_DEPOSIT_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct SolendDepositAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_collateral_supply: &'me AccountInfo<'info>,
    pub user_collateral: &'me AccountInfo<'info>,
    pub pyth_price: &'me AccountInfo<'info>,
    pub switchboard_feed: &'me AccountInfo<'info>,
    pub solend_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SolendDepositKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub signer_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub integration_acc_2: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub integration_acc_1: Pubkey,
    pub mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_collateral_supply: Pubkey,
    pub user_collateral: Pubkey,
    pub pyth_price: Pubkey,
    pub switchboard_feed: Pubkey,
    pub solend_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<SolendDepositAccounts<'_, '_>> for SolendDepositKeys {
    fn from(accounts: SolendDepositAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            signer_token_account: *accounts.signer_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            mint: *accounts.mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_collateral_supply: *accounts.reserve_collateral_supply.key,
            user_collateral: *accounts.user_collateral.key,
            pyth_price: *accounts.pyth_price.key,
            switchboard_feed: *accounts.switchboard_feed.key,
            solend_program: *accounts.solend_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SolendDepositKeys> for [AccountMeta; SOLEND_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SolendDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.solend_program,
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
impl From<[Pubkey; SOLEND_DEPOSIT_IX_ACCOUNTS_LEN]> for SolendDepositKeys {
    fn from(pubkeys: [Pubkey; SOLEND_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            signer_token_account: pubkeys[4],
            liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            integration_acc_2: pubkeys[7],
            lending_market: pubkeys[8],
            lending_market_authority: pubkeys[9],
            integration_acc_1: pubkeys[10],
            mint: pubkeys[11],
            reserve_liquidity_supply: pubkeys[12],
            reserve_collateral_mint: pubkeys[13],
            reserve_collateral_supply: pubkeys[14],
            user_collateral: pubkeys[15],
            pyth_price: pubkeys[16],
            switchboard_feed: pubkeys[17],
            solend_program: pubkeys[18],
            token_program: pubkeys[19],
        }
    }
}
impl<'info> From<SolendDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; SOLEND_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SolendDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.signer_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.integration_acc_2.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.integration_acc_1.clone(),
            accounts.mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_collateral_supply.clone(),
            accounts.user_collateral.clone(),
            accounts.pyth_price.clone(),
            accounts.switchboard_feed.clone(),
            accounts.solend_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOLEND_DEPOSIT_IX_ACCOUNTS_LEN]>
for SolendDepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SOLEND_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            signer_token_account: &arr[4],
            liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            integration_acc_2: &arr[7],
            lending_market: &arr[8],
            lending_market_authority: &arr[9],
            integration_acc_1: &arr[10],
            mint: &arr[11],
            reserve_liquidity_supply: &arr[12],
            reserve_collateral_mint: &arr[13],
            reserve_collateral_supply: &arr[14],
            user_collateral: &arr[15],
            pyth_price: &arr[16],
            switchboard_feed: &arr[17],
            solend_program: &arr[18],
            token_program: &arr[19],
        }
    }
}
pub const SOLEND_DEPOSIT_IX_DISCM: [u8; 8usize] = [56, 127, 176, 148, 12, 25, 3, 24];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SolendDepositIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SolendDepositIxData(pub SolendDepositIxArgs);
impl From<SolendDepositIxArgs> for SolendDepositIxData {
    fn from(args: SolendDepositIxArgs) -> Self {
        Self(args)
    }
}
impl SolendDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SOLEND_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SolendDepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SOLEND_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn solend_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: SolendDepositKeys,
    args: SolendDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOLEND_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SolendDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn solend_deposit_ix(
    keys: SolendDepositKeys,
    args: SolendDepositIxArgs,
) -> std::io::Result<Instruction> {
    solend_deposit_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn solend_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SolendDepositAccounts<'_, '_>,
    args: SolendDepositIxArgs,
) -> ProgramResult {
    let keys: SolendDepositKeys = accounts.into();
    let ix = solend_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn solend_deposit_invoke(
    accounts: SolendDepositAccounts<'_, '_>,
    args: SolendDepositIxArgs,
) -> ProgramResult {
    solend_deposit_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn solend_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SolendDepositAccounts<'_, '_>,
    args: SolendDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SolendDepositKeys = accounts.into();
    let ix = solend_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn solend_deposit_invoke_signed(
    accounts: SolendDepositAccounts<'_, '_>,
    args: SolendDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    solend_deposit_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn solend_deposit_verify_account_keys(
    accounts: SolendDepositAccounts<'_, '_>,
    keys: SolendDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.mint.key, keys.mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_collateral_supply.key, keys.reserve_collateral_supply),
        (*accounts.user_collateral.key, keys.user_collateral),
        (*accounts.pyth_price.key, keys.pyth_price),
        (*accounts.switchboard_feed.key, keys.switchboard_feed),
        (*accounts.solend_program.key, keys.solend_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn solend_deposit_verify_writable_privileges<'me, 'info>(
    accounts: SolendDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.signer_token_account,
        accounts.liquidity_vault,
        accounts.integration_acc_2,
        accounts.integration_acc_1,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_collateral_supply,
        accounts.user_collateral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn solend_deposit_verify_signer_privileges<'me, 'info>(
    accounts: SolendDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn solend_deposit_verify_account_privileges<'me, 'info>(
    accounts: SolendDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    solend_deposit_verify_writable_privileges(accounts)?;
    solend_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SOLEND_INIT_OBLIGATION_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct SolendInitObligationAccounts<'me, 'info> {
    pub fee_payer: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub signer_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_collateral_supply: &'me AccountInfo<'info>,
    pub user_collateral: &'me AccountInfo<'info>,
    pub pyth_price: &'me AccountInfo<'info>,
    pub switchboard_feed: &'me AccountInfo<'info>,
    pub solend_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SolendInitObligationKeys {
    pub fee_payer: Pubkey,
    pub bank: Pubkey,
    pub signer_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub integration_acc_2: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub integration_acc_1: Pubkey,
    pub mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_collateral_supply: Pubkey,
    pub user_collateral: Pubkey,
    pub pyth_price: Pubkey,
    pub switchboard_feed: Pubkey,
    pub solend_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<SolendInitObligationAccounts<'_, '_>> for SolendInitObligationKeys {
    fn from(accounts: SolendInitObligationAccounts) -> Self {
        Self {
            fee_payer: *accounts.fee_payer.key,
            bank: *accounts.bank.key,
            signer_token_account: *accounts.signer_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            mint: *accounts.mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_collateral_supply: *accounts.reserve_collateral_supply.key,
            user_collateral: *accounts.user_collateral.key,
            pyth_price: *accounts.pyth_price.key,
            switchboard_feed: *accounts.switchboard_feed.key,
            solend_program: *accounts.solend_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SolendInitObligationKeys>
for [AccountMeta; SOLEND_INIT_OBLIGATION_IX_ACCOUNTS_LEN] {
    fn from(keys: SolendInitObligationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.solend_program,
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
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOLEND_INIT_OBLIGATION_IX_ACCOUNTS_LEN]>
for SolendInitObligationKeys {
    fn from(pubkeys: [Pubkey; SOLEND_INIT_OBLIGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_payer: pubkeys[0],
            bank: pubkeys[1],
            signer_token_account: pubkeys[2],
            liquidity_vault_authority: pubkeys[3],
            liquidity_vault: pubkeys[4],
            integration_acc_2: pubkeys[5],
            lending_market: pubkeys[6],
            lending_market_authority: pubkeys[7],
            integration_acc_1: pubkeys[8],
            mint: pubkeys[9],
            reserve_liquidity_supply: pubkeys[10],
            reserve_collateral_mint: pubkeys[11],
            reserve_collateral_supply: pubkeys[12],
            user_collateral: pubkeys[13],
            pyth_price: pubkeys[14],
            switchboard_feed: pubkeys[15],
            solend_program: pubkeys[16],
            token_program: pubkeys[17],
            rent: pubkeys[18],
            system_program: pubkeys[19],
        }
    }
}
impl<'info> From<SolendInitObligationAccounts<'_, 'info>>
for [AccountInfo<'info>; SOLEND_INIT_OBLIGATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: SolendInitObligationAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_payer.clone(),
            accounts.bank.clone(),
            accounts.signer_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.integration_acc_2.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.integration_acc_1.clone(),
            accounts.mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_collateral_supply.clone(),
            accounts.user_collateral.clone(),
            accounts.pyth_price.clone(),
            accounts.switchboard_feed.clone(),
            accounts.solend_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOLEND_INIT_OBLIGATION_IX_ACCOUNTS_LEN]>
for SolendInitObligationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SOLEND_INIT_OBLIGATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fee_payer: &arr[0],
            bank: &arr[1],
            signer_token_account: &arr[2],
            liquidity_vault_authority: &arr[3],
            liquidity_vault: &arr[4],
            integration_acc_2: &arr[5],
            lending_market: &arr[6],
            lending_market_authority: &arr[7],
            integration_acc_1: &arr[8],
            mint: &arr[9],
            reserve_liquidity_supply: &arr[10],
            reserve_collateral_mint: &arr[11],
            reserve_collateral_supply: &arr[12],
            user_collateral: &arr[13],
            pyth_price: &arr[14],
            switchboard_feed: &arr[15],
            solend_program: &arr[16],
            token_program: &arr[17],
            rent: &arr[18],
            system_program: &arr[19],
        }
    }
}
pub const SOLEND_INIT_OBLIGATION_IX_DISCM: [u8; 8usize] = [
    81, 96, 123, 149, 218, 116, 235, 196,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SolendInitObligationIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SolendInitObligationIxData(pub SolendInitObligationIxArgs);
impl From<SolendInitObligationIxArgs> for SolendInitObligationIxData {
    fn from(args: SolendInitObligationIxArgs) -> Self {
        Self(args)
    }
}
impl SolendInitObligationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SOLEND_INIT_OBLIGATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SolendInitObligationIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SOLEND_INIT_OBLIGATION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn solend_init_obligation_ix_with_program_id(
    program_id: Pubkey,
    keys: SolendInitObligationKeys,
    args: SolendInitObligationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOLEND_INIT_OBLIGATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: SolendInitObligationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn solend_init_obligation_ix(
    keys: SolendInitObligationKeys,
    args: SolendInitObligationIxArgs,
) -> std::io::Result<Instruction> {
    solend_init_obligation_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn solend_init_obligation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SolendInitObligationAccounts<'_, '_>,
    args: SolendInitObligationIxArgs,
) -> ProgramResult {
    let keys: SolendInitObligationKeys = accounts.into();
    let ix = solend_init_obligation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn solend_init_obligation_invoke(
    accounts: SolendInitObligationAccounts<'_, '_>,
    args: SolendInitObligationIxArgs,
) -> ProgramResult {
    solend_init_obligation_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn solend_init_obligation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SolendInitObligationAccounts<'_, '_>,
    args: SolendInitObligationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SolendInitObligationKeys = accounts.into();
    let ix = solend_init_obligation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn solend_init_obligation_invoke_signed(
    accounts: SolendInitObligationAccounts<'_, '_>,
    args: SolendInitObligationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    solend_init_obligation_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn solend_init_obligation_verify_account_keys(
    accounts: SolendInitObligationAccounts<'_, '_>,
    keys: SolendInitObligationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.bank.key, keys.bank),
        (*accounts.signer_token_account.key, keys.signer_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.mint.key, keys.mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_collateral_supply.key, keys.reserve_collateral_supply),
        (*accounts.user_collateral.key, keys.user_collateral),
        (*accounts.pyth_price.key, keys.pyth_price),
        (*accounts.switchboard_feed.key, keys.switchboard_feed),
        (*accounts.solend_program.key, keys.solend_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn solend_init_obligation_verify_writable_privileges<'me, 'info>(
    accounts: SolendInitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_payer,
        accounts.signer_token_account,
        accounts.liquidity_vault,
        accounts.integration_acc_2,
        accounts.integration_acc_1,
        accounts.mint,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_collateral_supply,
        accounts.user_collateral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn solend_init_obligation_verify_signer_privileges<'me, 'info>(
    accounts: SolendInitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn solend_init_obligation_verify_account_privileges<'me, 'info>(
    accounts: SolendInitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    solend_init_obligation_verify_writable_privileges(accounts)?;
    solend_init_obligation_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SOLEND_WITHDRAW_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct SolendWithdrawAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub integration_acc_2: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub integration_acc_1: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_collateral_supply: &'me AccountInfo<'info>,
    pub user_collateral: &'me AccountInfo<'info>,
    pub solend_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SolendWithdrawKeys {
    pub group: Pubkey,
    pub marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub bank: Pubkey,
    pub destination_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub integration_acc_2: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub integration_acc_1: Pubkey,
    pub mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_collateral_supply: Pubkey,
    pub user_collateral: Pubkey,
    pub solend_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<SolendWithdrawAccounts<'_, '_>> for SolendWithdrawKeys {
    fn from(accounts: SolendWithdrawAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            marginfi_account: *accounts.marginfi_account.key,
            authority: *accounts.authority.key,
            bank: *accounts.bank.key,
            destination_token_account: *accounts.destination_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            integration_acc_2: *accounts.integration_acc_2.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            integration_acc_1: *accounts.integration_acc_1.key,
            mint: *accounts.mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_collateral_supply: *accounts.reserve_collateral_supply.key,
            user_collateral: *accounts.user_collateral.key,
            solend_program: *accounts.solend_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SolendWithdrawKeys> for [AccountMeta; SOLEND_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: SolendWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.integration_acc_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.integration_acc_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.solend_program,
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
impl From<[Pubkey; SOLEND_WITHDRAW_IX_ACCOUNTS_LEN]> for SolendWithdrawKeys {
    fn from(pubkeys: [Pubkey; SOLEND_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            marginfi_account: pubkeys[1],
            authority: pubkeys[2],
            bank: pubkeys[3],
            destination_token_account: pubkeys[4],
            liquidity_vault_authority: pubkeys[5],
            liquidity_vault: pubkeys[6],
            integration_acc_2: pubkeys[7],
            lending_market: pubkeys[8],
            lending_market_authority: pubkeys[9],
            integration_acc_1: pubkeys[10],
            mint: pubkeys[11],
            reserve_liquidity_supply: pubkeys[12],
            reserve_collateral_mint: pubkeys[13],
            reserve_collateral_supply: pubkeys[14],
            user_collateral: pubkeys[15],
            solend_program: pubkeys[16],
            token_program: pubkeys[17],
        }
    }
}
impl<'info> From<SolendWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; SOLEND_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: SolendWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.bank.clone(),
            accounts.destination_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.integration_acc_2.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.integration_acc_1.clone(),
            accounts.mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_collateral_supply.clone(),
            accounts.user_collateral.clone(),
            accounts.solend_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOLEND_WITHDRAW_IX_ACCOUNTS_LEN]>
for SolendWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SOLEND_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: &arr[0],
            marginfi_account: &arr[1],
            authority: &arr[2],
            bank: &arr[3],
            destination_token_account: &arr[4],
            liquidity_vault_authority: &arr[5],
            liquidity_vault: &arr[6],
            integration_acc_2: &arr[7],
            lending_market: &arr[8],
            lending_market_authority: &arr[9],
            integration_acc_1: &arr[10],
            mint: &arr[11],
            reserve_liquidity_supply: &arr[12],
            reserve_collateral_mint: &arr[13],
            reserve_collateral_supply: &arr[14],
            user_collateral: &arr[15],
            solend_program: &arr[16],
            token_program: &arr[17],
        }
    }
}
pub const SOLEND_WITHDRAW_IX_DISCM: [u8; 8usize] = [238, 144, 170, 199, 21, 72, 155, 36];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SolendWithdrawIxArgs {
    pub amount: u64,
    pub withdraw_all: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SolendWithdrawIxData(pub SolendWithdrawIxArgs);
impl From<SolendWithdrawIxArgs> for SolendWithdrawIxData {
    fn from(args: SolendWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl SolendWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SOLEND_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_all: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SolendWithdrawIxArgs {
                amount,
                withdraw_all,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SOLEND_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_all, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn solend_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: SolendWithdrawKeys,
    args: SolendWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOLEND_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: SolendWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn solend_withdraw_ix(
    keys: SolendWithdrawKeys,
    args: SolendWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    solend_withdraw_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn solend_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SolendWithdrawAccounts<'_, '_>,
    args: SolendWithdrawIxArgs,
) -> ProgramResult {
    let keys: SolendWithdrawKeys = accounts.into();
    let ix = solend_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn solend_withdraw_invoke(
    accounts: SolendWithdrawAccounts<'_, '_>,
    args: SolendWithdrawIxArgs,
) -> ProgramResult {
    solend_withdraw_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn solend_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SolendWithdrawAccounts<'_, '_>,
    args: SolendWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SolendWithdrawKeys = accounts.into();
    let ix = solend_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn solend_withdraw_invoke_signed(
    accounts: SolendWithdrawAccounts<'_, '_>,
    args: SolendWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    solend_withdraw_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn solend_withdraw_verify_account_keys(
    accounts: SolendWithdrawAccounts<'_, '_>,
    keys: SolendWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.bank.key, keys.bank),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.integration_acc_2.key, keys.integration_acc_2),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.integration_acc_1.key, keys.integration_acc_1),
        (*accounts.mint.key, keys.mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_collateral_supply.key, keys.reserve_collateral_supply),
        (*accounts.user_collateral.key, keys.user_collateral),
        (*accounts.solend_program.key, keys.solend_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn solend_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: SolendWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.marginfi_account,
        accounts.bank,
        accounts.destination_token_account,
        accounts.liquidity_vault_authority,
        accounts.liquidity_vault,
        accounts.integration_acc_2,
        accounts.lending_market,
        accounts.integration_acc_1,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_collateral_supply,
        accounts.user_collateral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn solend_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: SolendWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn solend_withdraw_verify_account_privileges<'me, 'info>(
    accounts: SolendWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    solend_withdraw_verify_writable_privileges(accounts)?;
    solend_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const START_DELEVERAGE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct StartDeleverageAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub liquidation_record: &'me AccountInfo<'info>,
    pub group: &'me AccountInfo<'info>,
    pub risk_admin: &'me AccountInfo<'info>,
    pub instruction_sysvar: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StartDeleverageKeys {
    pub marginfi_account: Pubkey,
    pub liquidation_record: Pubkey,
    pub group: Pubkey,
    pub risk_admin: Pubkey,
    pub instruction_sysvar: Pubkey,
}
impl From<StartDeleverageAccounts<'_, '_>> for StartDeleverageKeys {
    fn from(accounts: StartDeleverageAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            liquidation_record: *accounts.liquidation_record.key,
            group: *accounts.group.key,
            risk_admin: *accounts.risk_admin.key,
            instruction_sysvar: *accounts.instruction_sysvar.key,
        }
    }
}
impl From<StartDeleverageKeys> for [AccountMeta; START_DELEVERAGE_IX_ACCOUNTS_LEN] {
    fn from(keys: StartDeleverageKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.risk_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; START_DELEVERAGE_IX_ACCOUNTS_LEN]> for StartDeleverageKeys {
    fn from(pubkeys: [Pubkey; START_DELEVERAGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            liquidation_record: pubkeys[1],
            group: pubkeys[2],
            risk_admin: pubkeys[3],
            instruction_sysvar: pubkeys[4],
        }
    }
}
impl<'info> From<StartDeleverageAccounts<'_, 'info>>
for [AccountInfo<'info>; START_DELEVERAGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: StartDeleverageAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.liquidation_record.clone(),
            accounts.group.clone(),
            accounts.risk_admin.clone(),
            accounts.instruction_sysvar.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; START_DELEVERAGE_IX_ACCOUNTS_LEN]>
for StartDeleverageAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; START_DELEVERAGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: &arr[0],
            liquidation_record: &arr[1],
            group: &arr[2],
            risk_admin: &arr[3],
            instruction_sysvar: &arr[4],
        }
    }
}
pub const START_DELEVERAGE_IX_DISCM: [u8; 8usize] = [10, 138, 10, 57, 40, 232, 182, 193];
#[derive(Clone, Debug, PartialEq)]
pub struct StartDeleverageIxData;
impl StartDeleverageIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != START_DELEVERAGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&START_DELEVERAGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn start_deleverage_ix_with_program_id(
    program_id: Pubkey,
    keys: StartDeleverageKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; START_DELEVERAGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: StartDeleverageIxData.try_to_vec()?,
    })
}
pub fn start_deleverage_ix(keys: StartDeleverageKeys) -> std::io::Result<Instruction> {
    start_deleverage_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn start_deleverage_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StartDeleverageAccounts<'_, '_>,
) -> ProgramResult {
    let keys: StartDeleverageKeys = accounts.into();
    let ix = start_deleverage_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn start_deleverage_invoke(
    accounts: StartDeleverageAccounts<'_, '_>,
) -> ProgramResult {
    start_deleverage_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn start_deleverage_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StartDeleverageAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StartDeleverageKeys = accounts.into();
    let ix = start_deleverage_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn start_deleverage_invoke_signed(
    accounts: StartDeleverageAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    start_deleverage_invoke_signed_with_program_id(MARGINFI_PROGRAM_ID, accounts, seeds)
}
pub fn start_deleverage_verify_account_keys(
    accounts: StartDeleverageAccounts<'_, '_>,
    keys: StartDeleverageKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.liquidation_record.key, keys.liquidation_record),
        (*accounts.group.key, keys.group),
        (*accounts.risk_admin.key, keys.risk_admin),
        (*accounts.instruction_sysvar.key, keys.instruction_sysvar),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn start_deleverage_verify_writable_privileges<'me, 'info>(
    accounts: StartDeleverageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account, accounts.liquidation_record] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn start_deleverage_verify_signer_privileges<'me, 'info>(
    accounts: StartDeleverageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.risk_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn start_deleverage_verify_account_privileges<'me, 'info>(
    accounts: StartDeleverageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    start_deleverage_verify_writable_privileges(accounts)?;
    start_deleverage_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const START_LIQUIDATION_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct StartLiquidationAccounts<'me, 'info> {
    pub marginfi_account: &'me AccountInfo<'info>,
    pub liquidation_record: &'me AccountInfo<'info>,
    pub liquidation_receiver: &'me AccountInfo<'info>,
    pub instruction_sysvar: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StartLiquidationKeys {
    pub marginfi_account: Pubkey,
    pub liquidation_record: Pubkey,
    pub liquidation_receiver: Pubkey,
    pub instruction_sysvar: Pubkey,
}
impl From<StartLiquidationAccounts<'_, '_>> for StartLiquidationKeys {
    fn from(accounts: StartLiquidationAccounts) -> Self {
        Self {
            marginfi_account: *accounts.marginfi_account.key,
            liquidation_record: *accounts.liquidation_record.key,
            liquidation_receiver: *accounts.liquidation_receiver.key,
            instruction_sysvar: *accounts.instruction_sysvar.key,
        }
    }
}
impl From<StartLiquidationKeys> for [AccountMeta; START_LIQUIDATION_IX_ACCOUNTS_LEN] {
    fn from(keys: StartLiquidationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_receiver,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; START_LIQUIDATION_IX_ACCOUNTS_LEN]> for StartLiquidationKeys {
    fn from(pubkeys: [Pubkey; START_LIQUIDATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: pubkeys[0],
            liquidation_record: pubkeys[1],
            liquidation_receiver: pubkeys[2],
            instruction_sysvar: pubkeys[3],
        }
    }
}
impl<'info> From<StartLiquidationAccounts<'_, 'info>>
for [AccountInfo<'info>; START_LIQUIDATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: StartLiquidationAccounts<'_, 'info>) -> Self {
        [
            accounts.marginfi_account.clone(),
            accounts.liquidation_record.clone(),
            accounts.liquidation_receiver.clone(),
            accounts.instruction_sysvar.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; START_LIQUIDATION_IX_ACCOUNTS_LEN]>
for StartLiquidationAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; START_LIQUIDATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_account: &arr[0],
            liquidation_record: &arr[1],
            liquidation_receiver: &arr[2],
            instruction_sysvar: &arr[3],
        }
    }
}
pub const START_LIQUIDATION_IX_DISCM: [u8; 8usize] = [
    244, 93, 90, 214, 192, 166, 191, 21,
];
#[derive(Clone, Debug, PartialEq)]
pub struct StartLiquidationIxData;
impl StartLiquidationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != START_LIQUIDATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&START_LIQUIDATION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn start_liquidation_ix_with_program_id(
    program_id: Pubkey,
    keys: StartLiquidationKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; START_LIQUIDATION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: StartLiquidationIxData.try_to_vec()?,
    })
}
pub fn start_liquidation_ix(keys: StartLiquidationKeys) -> std::io::Result<Instruction> {
    start_liquidation_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn start_liquidation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StartLiquidationAccounts<'_, '_>,
) -> ProgramResult {
    let keys: StartLiquidationKeys = accounts.into();
    let ix = start_liquidation_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn start_liquidation_invoke(
    accounts: StartLiquidationAccounts<'_, '_>,
) -> ProgramResult {
    start_liquidation_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn start_liquidation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StartLiquidationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StartLiquidationKeys = accounts.into();
    let ix = start_liquidation_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn start_liquidation_invoke_signed(
    accounts: StartLiquidationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    start_liquidation_invoke_signed_with_program_id(MARGINFI_PROGRAM_ID, accounts, seeds)
}
pub fn start_liquidation_verify_account_keys(
    accounts: StartLiquidationAccounts<'_, '_>,
    keys: StartLiquidationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.liquidation_record.key, keys.liquidation_record),
        (*accounts.liquidation_receiver.key, keys.liquidation_receiver),
        (*accounts.instruction_sysvar.key, keys.instruction_sysvar),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn start_liquidation_verify_writable_privileges<'me, 'info>(
    accounts: StartLiquidationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_account, accounts.liquidation_record] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn start_liquidation_verify_account_privileges<'me, 'info>(
    accounts: StartLiquidationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    start_liquidation_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SUPER_ADMIN_DEPOSIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SuperAdminDepositAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub admin_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SuperAdminDepositKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub bank: Pubkey,
    pub admin_token_account: Pubkey,
    pub liquidity_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<SuperAdminDepositAccounts<'_, '_>> for SuperAdminDepositKeys {
    fn from(accounts: SuperAdminDepositAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            bank: *accounts.bank.key,
            admin_token_account: *accounts.admin_token_account.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SuperAdminDepositKeys> for [AccountMeta; SUPER_ADMIN_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SuperAdminDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
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
impl From<[Pubkey; SUPER_ADMIN_DEPOSIT_IX_ACCOUNTS_LEN]> for SuperAdminDepositKeys {
    fn from(pubkeys: [Pubkey; SUPER_ADMIN_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            bank: pubkeys[2],
            admin_token_account: pubkeys[3],
            liquidity_vault: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<SuperAdminDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; SUPER_ADMIN_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SuperAdminDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.admin.clone(),
            accounts.bank.clone(),
            accounts.admin_token_account.clone(),
            accounts.liquidity_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SUPER_ADMIN_DEPOSIT_IX_ACCOUNTS_LEN]>
for SuperAdminDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SUPER_ADMIN_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            bank: &arr[2],
            admin_token_account: &arr[3],
            liquidity_vault: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const SUPER_ADMIN_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    241, 189, 199, 17, 207, 225, 64, 75,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SuperAdminDepositIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SuperAdminDepositIxData(pub SuperAdminDepositIxArgs);
impl From<SuperAdminDepositIxArgs> for SuperAdminDepositIxData {
    fn from(args: SuperAdminDepositIxArgs) -> Self {
        Self(args)
    }
}
impl SuperAdminDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SUPER_ADMIN_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SuperAdminDepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SUPER_ADMIN_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn super_admin_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: SuperAdminDepositKeys,
    args: SuperAdminDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SUPER_ADMIN_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SuperAdminDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn super_admin_deposit_ix(
    keys: SuperAdminDepositKeys,
    args: SuperAdminDepositIxArgs,
) -> std::io::Result<Instruction> {
    super_admin_deposit_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn super_admin_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SuperAdminDepositAccounts<'_, '_>,
    args: SuperAdminDepositIxArgs,
) -> ProgramResult {
    let keys: SuperAdminDepositKeys = accounts.into();
    let ix = super_admin_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn super_admin_deposit_invoke(
    accounts: SuperAdminDepositAccounts<'_, '_>,
    args: SuperAdminDepositIxArgs,
) -> ProgramResult {
    super_admin_deposit_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn super_admin_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SuperAdminDepositAccounts<'_, '_>,
    args: SuperAdminDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SuperAdminDepositKeys = accounts.into();
    let ix = super_admin_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn super_admin_deposit_invoke_signed(
    accounts: SuperAdminDepositAccounts<'_, '_>,
    args: SuperAdminDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    super_admin_deposit_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn super_admin_deposit_verify_account_keys(
    accounts: SuperAdminDepositAccounts<'_, '_>,
    keys: SuperAdminDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.bank.key, keys.bank),
        (*accounts.admin_token_account.key, keys.admin_token_account),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn super_admin_deposit_verify_writable_privileges<'me, 'info>(
    accounts: SuperAdminDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank,
        accounts.admin_token_account,
        accounts.liquidity_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn super_admin_deposit_verify_signer_privileges<'me, 'info>(
    accounts: SuperAdminDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn super_admin_deposit_verify_account_privileges<'me, 'info>(
    accounts: SuperAdminDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    super_admin_deposit_verify_writable_privileges(accounts)?;
    super_admin_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SUPER_ADMIN_WITHDRAW_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct SuperAdminWithdrawAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub liquidity_vault_authority: &'me AccountInfo<'info>,
    pub liquidity_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SuperAdminWithdrawKeys {
    pub group: Pubkey,
    pub admin: Pubkey,
    pub bank: Pubkey,
    pub destination_token_account: Pubkey,
    pub liquidity_vault_authority: Pubkey,
    pub liquidity_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<SuperAdminWithdrawAccounts<'_, '_>> for SuperAdminWithdrawKeys {
    fn from(accounts: SuperAdminWithdrawAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            admin: *accounts.admin.key,
            bank: *accounts.bank.key,
            destination_token_account: *accounts.destination_token_account.key,
            liquidity_vault_authority: *accounts.liquidity_vault_authority.key,
            liquidity_vault: *accounts.liquidity_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SuperAdminWithdrawKeys>
for [AccountMeta; SUPER_ADMIN_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: SuperAdminWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_vault,
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
impl From<[Pubkey; SUPER_ADMIN_WITHDRAW_IX_ACCOUNTS_LEN]> for SuperAdminWithdrawKeys {
    fn from(pubkeys: [Pubkey; SUPER_ADMIN_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            admin: pubkeys[1],
            bank: pubkeys[2],
            destination_token_account: pubkeys[3],
            liquidity_vault_authority: pubkeys[4],
            liquidity_vault: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<SuperAdminWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; SUPER_ADMIN_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: SuperAdminWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.admin.clone(),
            accounts.bank.clone(),
            accounts.destination_token_account.clone(),
            accounts.liquidity_vault_authority.clone(),
            accounts.liquidity_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SUPER_ADMIN_WITHDRAW_IX_ACCOUNTS_LEN]>
for SuperAdminWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SUPER_ADMIN_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            admin: &arr[1],
            bank: &arr[2],
            destination_token_account: &arr[3],
            liquidity_vault_authority: &arr[4],
            liquidity_vault: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const SUPER_ADMIN_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    202, 67, 85, 126, 104, 138, 79, 197,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SuperAdminWithdrawIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SuperAdminWithdrawIxData(pub SuperAdminWithdrawIxArgs);
impl From<SuperAdminWithdrawIxArgs> for SuperAdminWithdrawIxData {
    fn from(args: SuperAdminWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl SuperAdminWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SUPER_ADMIN_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SuperAdminWithdrawIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SUPER_ADMIN_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn super_admin_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: SuperAdminWithdrawKeys,
    args: SuperAdminWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SUPER_ADMIN_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: SuperAdminWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn super_admin_withdraw_ix(
    keys: SuperAdminWithdrawKeys,
    args: SuperAdminWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    super_admin_withdraw_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn super_admin_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SuperAdminWithdrawAccounts<'_, '_>,
    args: SuperAdminWithdrawIxArgs,
) -> ProgramResult {
    let keys: SuperAdminWithdrawKeys = accounts.into();
    let ix = super_admin_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn super_admin_withdraw_invoke(
    accounts: SuperAdminWithdrawAccounts<'_, '_>,
    args: SuperAdminWithdrawIxArgs,
) -> ProgramResult {
    super_admin_withdraw_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn super_admin_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SuperAdminWithdrawAccounts<'_, '_>,
    args: SuperAdminWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SuperAdminWithdrawKeys = accounts.into();
    let ix = super_admin_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn super_admin_withdraw_invoke_signed(
    accounts: SuperAdminWithdrawAccounts<'_, '_>,
    args: SuperAdminWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    super_admin_withdraw_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn super_admin_withdraw_verify_account_keys(
    accounts: SuperAdminWithdrawAccounts<'_, '_>,
    keys: SuperAdminWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.admin.key, keys.admin),
        (*accounts.bank.key, keys.bank),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.liquidity_vault_authority.key, keys.liquidity_vault_authority),
        (*accounts.liquidity_vault.key, keys.liquidity_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn super_admin_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: SuperAdminWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bank,
        accounts.destination_token_account,
        accounts.liquidity_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn super_admin_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: SuperAdminWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn super_admin_withdraw_verify_account_privileges<'me, 'info>(
    accounts: SuperAdminWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    super_admin_withdraw_verify_writable_privileges(accounts)?;
    super_admin_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_TO_NEW_ACCOUNT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct TransferToNewAccountAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub old_marginfi_account: &'me AccountInfo<'info>,
    pub new_marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
    pub global_fee_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferToNewAccountKeys {
    pub group: Pubkey,
    pub old_marginfi_account: Pubkey,
    pub new_marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub fee_payer: Pubkey,
    pub new_authority: Pubkey,
    pub global_fee_wallet: Pubkey,
    pub system_program: Pubkey,
}
impl From<TransferToNewAccountAccounts<'_, '_>> for TransferToNewAccountKeys {
    fn from(accounts: TransferToNewAccountAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            old_marginfi_account: *accounts.old_marginfi_account.key,
            new_marginfi_account: *accounts.new_marginfi_account.key,
            authority: *accounts.authority.key,
            fee_payer: *accounts.fee_payer.key,
            new_authority: *accounts.new_authority.key,
            global_fee_wallet: *accounts.global_fee_wallet.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TransferToNewAccountKeys>
for [AccountMeta; TRANSFER_TO_NEW_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferToNewAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_marginfi_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_fee_wallet,
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
impl From<[Pubkey; TRANSFER_TO_NEW_ACCOUNT_IX_ACCOUNTS_LEN]>
for TransferToNewAccountKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_TO_NEW_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            old_marginfi_account: pubkeys[1],
            new_marginfi_account: pubkeys[2],
            authority: pubkeys[3],
            fee_payer: pubkeys[4],
            new_authority: pubkeys[5],
            global_fee_wallet: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<TransferToNewAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_TO_NEW_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferToNewAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.old_marginfi_account.clone(),
            accounts.new_marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.fee_payer.clone(),
            accounts.new_authority.clone(),
            accounts.global_fee_wallet.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_TO_NEW_ACCOUNT_IX_ACCOUNTS_LEN]>
for TransferToNewAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_TO_NEW_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            old_marginfi_account: &arr[1],
            new_marginfi_account: &arr[2],
            authority: &arr[3],
            fee_payer: &arr[4],
            new_authority: &arr[5],
            global_fee_wallet: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const TRANSFER_TO_NEW_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    28, 79, 129, 231, 169, 69, 69, 65,
];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferToNewAccountIxData;
impl TransferToNewAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_TO_NEW_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_TO_NEW_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_to_new_account_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferToNewAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_TO_NEW_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferToNewAccountIxData.try_to_vec()?,
    })
}
pub fn transfer_to_new_account_ix(
    keys: TransferToNewAccountKeys,
) -> std::io::Result<Instruction> {
    transfer_to_new_account_ix_with_program_id(MARGINFI_PROGRAM_ID, keys)
}
pub fn transfer_to_new_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferToNewAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferToNewAccountKeys = accounts.into();
    let ix = transfer_to_new_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_to_new_account_invoke(
    accounts: TransferToNewAccountAccounts<'_, '_>,
) -> ProgramResult {
    transfer_to_new_account_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts)
}
pub fn transfer_to_new_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferToNewAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferToNewAccountKeys = accounts.into();
    let ix = transfer_to_new_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_to_new_account_invoke_signed(
    accounts: TransferToNewAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_to_new_account_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn transfer_to_new_account_verify_account_keys(
    accounts: TransferToNewAccountAccounts<'_, '_>,
    keys: TransferToNewAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.old_marginfi_account.key, keys.old_marginfi_account),
        (*accounts.new_marginfi_account.key, keys.new_marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.global_fee_wallet.key, keys.global_fee_wallet),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_to_new_account_verify_writable_privileges<'me, 'info>(
    accounts: TransferToNewAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.old_marginfi_account,
        accounts.new_marginfi_account,
        accounts.fee_payer,
        accounts.global_fee_wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_to_new_account_verify_signer_privileges<'me, 'info>(
    accounts: TransferToNewAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.new_marginfi_account,
        accounts.authority,
        accounts.fee_payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_to_new_account_verify_account_privileges<'me, 'info>(
    accounts: TransferToNewAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_to_new_account_verify_writable_privileges(accounts)?;
    transfer_to_new_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_TO_NEW_ACCOUNT_PDA_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct TransferToNewAccountPdaAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub old_marginfi_account: &'me AccountInfo<'info>,
    pub new_marginfi_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
    pub global_fee_wallet: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferToNewAccountPdaKeys {
    pub group: Pubkey,
    pub old_marginfi_account: Pubkey,
    pub new_marginfi_account: Pubkey,
    pub authority: Pubkey,
    pub fee_payer: Pubkey,
    pub new_authority: Pubkey,
    pub global_fee_wallet: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub system_program: Pubkey,
}
impl From<TransferToNewAccountPdaAccounts<'_, '_>> for TransferToNewAccountPdaKeys {
    fn from(accounts: TransferToNewAccountPdaAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            old_marginfi_account: *accounts.old_marginfi_account.key,
            new_marginfi_account: *accounts.new_marginfi_account.key,
            authority: *accounts.authority.key,
            fee_payer: *accounts.fee_payer.key,
            new_authority: *accounts.new_authority.key,
            global_fee_wallet: *accounts.global_fee_wallet.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TransferToNewAccountPdaKeys>
for [AccountMeta; TRANSFER_TO_NEW_ACCOUNT_PDA_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferToNewAccountPdaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_fee_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
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
impl From<[Pubkey; TRANSFER_TO_NEW_ACCOUNT_PDA_IX_ACCOUNTS_LEN]>
for TransferToNewAccountPdaKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_TO_NEW_ACCOUNT_PDA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            old_marginfi_account: pubkeys[1],
            new_marginfi_account: pubkeys[2],
            authority: pubkeys[3],
            fee_payer: pubkeys[4],
            new_authority: pubkeys[5],
            global_fee_wallet: pubkeys[6],
            instructions_sysvar: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<TransferToNewAccountPdaAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_TO_NEW_ACCOUNT_PDA_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferToNewAccountPdaAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.old_marginfi_account.clone(),
            accounts.new_marginfi_account.clone(),
            accounts.authority.clone(),
            accounts.fee_payer.clone(),
            accounts.new_authority.clone(),
            accounts.global_fee_wallet.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_TO_NEW_ACCOUNT_PDA_IX_ACCOUNTS_LEN]>
for TransferToNewAccountPdaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_TO_NEW_ACCOUNT_PDA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            old_marginfi_account: &arr[1],
            new_marginfi_account: &arr[2],
            authority: &arr[3],
            fee_payer: &arr[4],
            new_authority: &arr[5],
            global_fee_wallet: &arr[6],
            instructions_sysvar: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const TRANSFER_TO_NEW_ACCOUNT_PDA_IX_DISCM: [u8; 8usize] = [
    172, 210, 224, 220, 146, 212, 253, 49,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferToNewAccountPdaIxArgs {
    pub account_index: u16,
    pub third_party_id: Option<u16>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferToNewAccountPdaIxData(pub TransferToNewAccountPdaIxArgs);
impl From<TransferToNewAccountPdaIxArgs> for TransferToNewAccountPdaIxData {
    fn from(args: TransferToNewAccountPdaIxArgs) -> Self {
        Self(args)
    }
}
impl TransferToNewAccountPdaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_TO_NEW_ACCOUNT_PDA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let third_party_id: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TransferToNewAccountPdaIxArgs {
                account_index,
                third_party_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_TO_NEW_ACCOUNT_PDA_IX_DISCM)?;
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
pub fn transfer_to_new_account_pda_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferToNewAccountPdaKeys,
    args: TransferToNewAccountPdaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_TO_NEW_ACCOUNT_PDA_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferToNewAccountPdaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_to_new_account_pda_ix(
    keys: TransferToNewAccountPdaKeys,
    args: TransferToNewAccountPdaIxArgs,
) -> std::io::Result<Instruction> {
    transfer_to_new_account_pda_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn transfer_to_new_account_pda_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferToNewAccountPdaAccounts<'_, '_>,
    args: TransferToNewAccountPdaIxArgs,
) -> ProgramResult {
    let keys: TransferToNewAccountPdaKeys = accounts.into();
    let ix = transfer_to_new_account_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_to_new_account_pda_invoke(
    accounts: TransferToNewAccountPdaAccounts<'_, '_>,
    args: TransferToNewAccountPdaIxArgs,
) -> ProgramResult {
    transfer_to_new_account_pda_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn transfer_to_new_account_pda_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferToNewAccountPdaAccounts<'_, '_>,
    args: TransferToNewAccountPdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferToNewAccountPdaKeys = accounts.into();
    let ix = transfer_to_new_account_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_to_new_account_pda_invoke_signed(
    accounts: TransferToNewAccountPdaAccounts<'_, '_>,
    args: TransferToNewAccountPdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_to_new_account_pda_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_to_new_account_pda_verify_account_keys(
    accounts: TransferToNewAccountPdaAccounts<'_, '_>,
    keys: TransferToNewAccountPdaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.old_marginfi_account.key, keys.old_marginfi_account),
        (*accounts.new_marginfi_account.key, keys.new_marginfi_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.global_fee_wallet.key, keys.global_fee_wallet),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_to_new_account_pda_verify_writable_privileges<'me, 'info>(
    accounts: TransferToNewAccountPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.old_marginfi_account,
        accounts.new_marginfi_account,
        accounts.fee_payer,
        accounts.global_fee_wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_to_new_account_pda_verify_signer_privileges<'me, 'info>(
    accounts: TransferToNewAccountPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_to_new_account_pda_verify_account_privileges<'me, 'info>(
    accounts: TransferToNewAccountPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_to_new_account_pda_verify_writable_privileges(accounts)?;
    transfer_to_new_account_pda_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_DELEVERAGE_WITHDRAWALS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDeleverageWithdrawalsAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub delegate_flow_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDeleverageWithdrawalsKeys {
    pub marginfi_group: Pubkey,
    pub delegate_flow_admin: Pubkey,
}
impl From<UpdateDeleverageWithdrawalsAccounts<'_, '_>>
for UpdateDeleverageWithdrawalsKeys {
    fn from(accounts: UpdateDeleverageWithdrawalsAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            delegate_flow_admin: *accounts.delegate_flow_admin.key,
        }
    }
}
impl From<UpdateDeleverageWithdrawalsKeys>
for [AccountMeta; UPDATE_DELEVERAGE_WITHDRAWALS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateDeleverageWithdrawalsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegate_flow_admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_DELEVERAGE_WITHDRAWALS_IX_ACCOUNTS_LEN]>
for UpdateDeleverageWithdrawalsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_DELEVERAGE_WITHDRAWALS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            delegate_flow_admin: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateDeleverageWithdrawalsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_DELEVERAGE_WITHDRAWALS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateDeleverageWithdrawalsAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_group.clone(), accounts.delegate_flow_admin.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_DELEVERAGE_WITHDRAWALS_IX_ACCOUNTS_LEN]>
for UpdateDeleverageWithdrawalsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_DELEVERAGE_WITHDRAWALS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            delegate_flow_admin: &arr[1],
        }
    }
}
pub const UPDATE_DELEVERAGE_WITHDRAWALS_IX_DISCM: [u8; 8usize] = [
    56, 3, 181, 118, 27, 247, 207, 227,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDeleverageWithdrawalsIxArgs {
    pub outflow_usd: u32,
    pub update_seq: u64,
    pub event_start_slot: u64,
    pub event_end_slot: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDeleverageWithdrawalsIxData(pub UpdateDeleverageWithdrawalsIxArgs);
impl From<UpdateDeleverageWithdrawalsIxArgs> for UpdateDeleverageWithdrawalsIxData {
    fn from(args: UpdateDeleverageWithdrawalsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateDeleverageWithdrawalsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DELEVERAGE_WITHDRAWALS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let outflow_usd: u32 = crate::borsh_de_or_default(&mut reader)?;
        let update_seq: u64 = crate::borsh_de_or_default(&mut reader)?;
        let event_start_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let event_end_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateDeleverageWithdrawalsIxArgs {
                outflow_usd,
                update_seq,
                event_start_slot,
                event_end_slot,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DELEVERAGE_WITHDRAWALS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.outflow_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.update_seq, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.event_start_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.event_end_slot, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_deleverage_withdrawals_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateDeleverageWithdrawalsKeys,
    args: UpdateDeleverageWithdrawalsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_DELEVERAGE_WITHDRAWALS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateDeleverageWithdrawalsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_deleverage_withdrawals_ix(
    keys: UpdateDeleverageWithdrawalsKeys,
    args: UpdateDeleverageWithdrawalsIxArgs,
) -> std::io::Result<Instruction> {
    update_deleverage_withdrawals_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn update_deleverage_withdrawals_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDeleverageWithdrawalsAccounts<'_, '_>,
    args: UpdateDeleverageWithdrawalsIxArgs,
) -> ProgramResult {
    let keys: UpdateDeleverageWithdrawalsKeys = accounts.into();
    let ix = update_deleverage_withdrawals_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_deleverage_withdrawals_invoke(
    accounts: UpdateDeleverageWithdrawalsAccounts<'_, '_>,
    args: UpdateDeleverageWithdrawalsIxArgs,
) -> ProgramResult {
    update_deleverage_withdrawals_invoke_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_deleverage_withdrawals_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDeleverageWithdrawalsAccounts<'_, '_>,
    args: UpdateDeleverageWithdrawalsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateDeleverageWithdrawalsKeys = accounts.into();
    let ix = update_deleverage_withdrawals_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_deleverage_withdrawals_invoke_signed(
    accounts: UpdateDeleverageWithdrawalsAccounts<'_, '_>,
    args: UpdateDeleverageWithdrawalsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_deleverage_withdrawals_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_deleverage_withdrawals_verify_account_keys(
    accounts: UpdateDeleverageWithdrawalsAccounts<'_, '_>,
    keys: UpdateDeleverageWithdrawalsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.delegate_flow_admin.key, keys.delegate_flow_admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_deleverage_withdrawals_verify_writable_privileges<'me, 'info>(
    accounts: UpdateDeleverageWithdrawalsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_group] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_deleverage_withdrawals_verify_signer_privileges<'me, 'info>(
    accounts: UpdateDeleverageWithdrawalsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_flow_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_deleverage_withdrawals_verify_account_privileges<'me, 'info>(
    accounts: UpdateDeleverageWithdrawalsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_deleverage_withdrawals_verify_writable_privileges(accounts)?;
    update_deleverage_withdrawals_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_GROUP_RATE_LIMITER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateGroupRateLimiterAccounts<'me, 'info> {
    pub marginfi_group: &'me AccountInfo<'info>,
    pub delegate_flow_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateGroupRateLimiterKeys {
    pub marginfi_group: Pubkey,
    pub delegate_flow_admin: Pubkey,
}
impl From<UpdateGroupRateLimiterAccounts<'_, '_>> for UpdateGroupRateLimiterKeys {
    fn from(accounts: UpdateGroupRateLimiterAccounts) -> Self {
        Self {
            marginfi_group: *accounts.marginfi_group.key,
            delegate_flow_admin: *accounts.delegate_flow_admin.key,
        }
    }
}
impl From<UpdateGroupRateLimiterKeys>
for [AccountMeta; UPDATE_GROUP_RATE_LIMITER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateGroupRateLimiterKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegate_flow_admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_GROUP_RATE_LIMITER_IX_ACCOUNTS_LEN]>
for UpdateGroupRateLimiterKeys {
    fn from(pubkeys: [Pubkey; UPDATE_GROUP_RATE_LIMITER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marginfi_group: pubkeys[0],
            delegate_flow_admin: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateGroupRateLimiterAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_GROUP_RATE_LIMITER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateGroupRateLimiterAccounts<'_, 'info>) -> Self {
        [accounts.marginfi_group.clone(), accounts.delegate_flow_admin.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_GROUP_RATE_LIMITER_IX_ACCOUNTS_LEN]>
for UpdateGroupRateLimiterAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_GROUP_RATE_LIMITER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            marginfi_group: &arr[0],
            delegate_flow_admin: &arr[1],
        }
    }
}
pub const UPDATE_GROUP_RATE_LIMITER_IX_DISCM: [u8; 8usize] = [
    23, 78, 60, 139, 187, 44, 129, 37,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateGroupRateLimiterIxArgs {
    pub outflow_usd: Option<u64>,
    pub inflow_usd: Option<u64>,
    pub update_seq: u64,
    pub event_start_slot: u64,
    pub event_end_slot: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGroupRateLimiterIxData(pub UpdateGroupRateLimiterIxArgs);
impl From<UpdateGroupRateLimiterIxArgs> for UpdateGroupRateLimiterIxData {
    fn from(args: UpdateGroupRateLimiterIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateGroupRateLimiterIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_GROUP_RATE_LIMITER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let outflow_usd: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let inflow_usd: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let update_seq: u64 = crate::borsh_de_or_default(&mut reader)?;
        let event_start_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let event_end_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateGroupRateLimiterIxArgs {
                outflow_usd,
                inflow_usd,
                update_seq,
                event_start_slot,
                event_end_slot,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_GROUP_RATE_LIMITER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.outflow_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.inflow_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.update_seq, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.event_start_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.event_end_slot, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_group_rate_limiter_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateGroupRateLimiterKeys,
    args: UpdateGroupRateLimiterIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_GROUP_RATE_LIMITER_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateGroupRateLimiterIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_group_rate_limiter_ix(
    keys: UpdateGroupRateLimiterKeys,
    args: UpdateGroupRateLimiterIxArgs,
) -> std::io::Result<Instruction> {
    update_group_rate_limiter_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn update_group_rate_limiter_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGroupRateLimiterAccounts<'_, '_>,
    args: UpdateGroupRateLimiterIxArgs,
) -> ProgramResult {
    let keys: UpdateGroupRateLimiterKeys = accounts.into();
    let ix = update_group_rate_limiter_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_group_rate_limiter_invoke(
    accounts: UpdateGroupRateLimiterAccounts<'_, '_>,
    args: UpdateGroupRateLimiterIxArgs,
) -> ProgramResult {
    update_group_rate_limiter_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn update_group_rate_limiter_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGroupRateLimiterAccounts<'_, '_>,
    args: UpdateGroupRateLimiterIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateGroupRateLimiterKeys = accounts.into();
    let ix = update_group_rate_limiter_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_group_rate_limiter_invoke_signed(
    accounts: UpdateGroupRateLimiterAccounts<'_, '_>,
    args: UpdateGroupRateLimiterIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_group_rate_limiter_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_group_rate_limiter_verify_account_keys(
    accounts: UpdateGroupRateLimiterAccounts<'_, '_>,
    keys: UpdateGroupRateLimiterKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.delegate_flow_admin.key, keys.delegate_flow_admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_group_rate_limiter_verify_writable_privileges<'me, 'info>(
    accounts: UpdateGroupRateLimiterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.marginfi_group] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_group_rate_limiter_verify_signer_privileges<'me, 'info>(
    accounts: UpdateGroupRateLimiterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.delegate_flow_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_group_rate_limiter_verify_account_privileges<'me, 'info>(
    accounts: UpdateGroupRateLimiterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_group_rate_limiter_verify_writable_privileges(accounts)?;
    update_group_rate_limiter_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WRITE_BANK_METADATA_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct WriteBankMetadataAccounts<'me, 'info> {
    pub group: &'me AccountInfo<'info>,
    pub bank: &'me AccountInfo<'info>,
    pub metadata_admin: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WriteBankMetadataKeys {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub metadata_admin: Pubkey,
    pub metadata: Pubkey,
}
impl From<WriteBankMetadataAccounts<'_, '_>> for WriteBankMetadataKeys {
    fn from(accounts: WriteBankMetadataAccounts) -> Self {
        Self {
            group: *accounts.group.key,
            bank: *accounts.bank.key,
            metadata_admin: *accounts.metadata_admin.key,
            metadata: *accounts.metadata.key,
        }
    }
}
impl From<WriteBankMetadataKeys> for [AccountMeta; WRITE_BANK_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: WriteBankMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; WRITE_BANK_METADATA_IX_ACCOUNTS_LEN]> for WriteBankMetadataKeys {
    fn from(pubkeys: [Pubkey; WRITE_BANK_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            group: pubkeys[0],
            bank: pubkeys[1],
            metadata_admin: pubkeys[2],
            metadata: pubkeys[3],
        }
    }
}
impl<'info> From<WriteBankMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; WRITE_BANK_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: WriteBankMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.group.clone(),
            accounts.bank.clone(),
            accounts.metadata_admin.clone(),
            accounts.metadata.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WRITE_BANK_METADATA_IX_ACCOUNTS_LEN]>
for WriteBankMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WRITE_BANK_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            group: &arr[0],
            bank: &arr[1],
            metadata_admin: &arr[2],
            metadata: &arr[3],
        }
    }
}
pub const WRITE_BANK_METADATA_IX_DISCM: [u8; 8usize] = [
    147, 78, 81, 133, 129, 138, 233, 59,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WriteBankMetadataIxArgs {
    pub ticker: Option<Vec<u8>>,
    pub description: Option<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WriteBankMetadataIxData(pub WriteBankMetadataIxArgs);
impl From<WriteBankMetadataIxArgs> for WriteBankMetadataIxData {
    fn from(args: WriteBankMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl WriteBankMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WRITE_BANK_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let ticker: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        let description: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WriteBankMetadataIxArgs {
                ticker,
                description,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WRITE_BANK_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.ticker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.description, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn write_bank_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: WriteBankMetadataKeys,
    args: WriteBankMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WRITE_BANK_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: WriteBankMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn write_bank_metadata_ix(
    keys: WriteBankMetadataKeys,
    args: WriteBankMetadataIxArgs,
) -> std::io::Result<Instruction> {
    write_bank_metadata_ix_with_program_id(MARGINFI_PROGRAM_ID, keys, args)
}
pub fn write_bank_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WriteBankMetadataAccounts<'_, '_>,
    args: WriteBankMetadataIxArgs,
) -> ProgramResult {
    let keys: WriteBankMetadataKeys = accounts.into();
    let ix = write_bank_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn write_bank_metadata_invoke(
    accounts: WriteBankMetadataAccounts<'_, '_>,
    args: WriteBankMetadataIxArgs,
) -> ProgramResult {
    write_bank_metadata_invoke_with_program_id(MARGINFI_PROGRAM_ID, accounts, args)
}
pub fn write_bank_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WriteBankMetadataAccounts<'_, '_>,
    args: WriteBankMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WriteBankMetadataKeys = accounts.into();
    let ix = write_bank_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn write_bank_metadata_invoke_signed(
    accounts: WriteBankMetadataAccounts<'_, '_>,
    args: WriteBankMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    write_bank_metadata_invoke_signed_with_program_id(
        MARGINFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn write_bank_metadata_verify_account_keys(
    accounts: WriteBankMetadataAccounts<'_, '_>,
    keys: WriteBankMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.group.key, keys.group),
        (*accounts.bank.key, keys.bank),
        (*accounts.metadata_admin.key, keys.metadata_admin),
        (*accounts.metadata.key, keys.metadata),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn write_bank_metadata_verify_writable_privileges<'me, 'info>(
    accounts: WriteBankMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata_admin, accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn write_bank_metadata_verify_signer_privileges<'me, 'info>(
    accounts: WriteBankMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.metadata_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn write_bank_metadata_verify_account_privileges<'me, 'info>(
    accounts: WriteBankMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    write_bank_metadata_verify_writable_privileges(accounts)?;
    write_bank_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
