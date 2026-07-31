use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum HumaProgramIx {
    AddAsyncDeploymentTarget(AddAsyncDeploymentTargetIxArgs),
    AddDeploymentTarget(AddDeploymentTargetIxArgs),
    AddInstantWithdrawalLender(AddInstantWithdrawalLenderIxArgs),
    AddMode(AddModeIxArgs),
    AddPoolOperator(AddPoolOperatorIxArgs),
    AddRedemptionRequestV2(AddRedemptionRequestV2IxArgs),
    AddStrategyManagerWallet(AddStrategyManagerWalletIxArgs),
    ApproveManualStrategyManager,
    CancelManualStrategyManagerProposal,
    CancelRedemptionRequest(CancelRedemptionRequestIxArgs),
    ChangePoolOwner(ChangePoolOwnerIxArgs),
    ClearInstantWithdrawalLiquiditySource,
    CloseLenderAccounts,
    ClosePool,
    CompleteLiquidityPaybackAsync(CompleteLiquidityPaybackAsyncIxArgs),
    CreateLenderAccountsV2,
    CreatePool(CreatePoolIxArgs),
    DeclareLoss(DeclareLossIxArgs),
    DeployLiquidity(DeployLiquidityIxArgs),
    DeployLiquidityAsync(DeployLiquidityAsyncIxArgs),
    DeployLiquidityManually(DeployLiquidityManuallyIxArgs),
    Deposit(DepositIxArgs),
    DisablePool,
    Disburse,
    EnablePool,
    EnterPreClosure,
    InitiateLiquidityPaybackAsync(InitiateLiquidityPaybackAsyncIxArgs),
    InstantWithdraw(InstantWithdrawIxArgs),
    InstantWithdrawPrivileged(InstantWithdrawPrivilegedIxArgs),
    MakeInitialDeposit(MakeInitialDepositIxArgs),
    PayBackLiquidity(PayBackLiquidityIxArgs),
    PayBackLiquidityManually(PayBackLiquidityManuallyIxArgs),
    ProcessRedemptionRequest,
    ProposeManualStrategyManager(ProposeManualStrategyManagerIxArgs),
    RecoverLoss(RecoverLossIxArgs),
    RefreshModeAssets,
    RefreshPoolAssets,
    RemoveAsyncDeploymentTarget,
    RemoveDeploymentTarget,
    RemoveInstantWithdrawalLender(RemoveInstantWithdrawalLenderIxArgs),
    RemoveManualStrategyManager,
    RemoveMode(RemoveModeIxArgs),
    RemovePoolOperator(RemovePoolOperatorIxArgs),
    RemoveStrategyManagerWallet,
    SetHumaConfig,
    SetInstantWithdrawalFeeConfigs(SetInstantWithdrawalFeeConfigsIxArgs),
    SetInstantWithdrawalLiquiditySource,
    SetInstantWithdrawalReserveLimit(SetInstantWithdrawalReserveLimitIxArgs),
    SetLiquidAssetsDeployed(SetLiquidAssetsDeployedIxArgs),
    SetLossAuthority(SetLossAuthorityIxArgs),
    SetLpConfig(SetLpConfigIxArgs),
    SetManualDeploymentLimits(SetManualDeploymentLimitsIxArgs),
    SetPoolOwnerTreasury(SetPoolOwnerTreasuryIxArgs),
    SetupDeploymentTarget(SetupDeploymentTargetIxArgs),
    SwitchMode(SwitchModeIxArgs),
    UpdateModeApy(UpdateModeApyIxArgs),
    UpdateModeName(UpdateModeNameIxArgs),
    UpdateModeTokenMetadata(UpdateModeTokenMetadataIxArgs),
    UpdatePoolName(UpdatePoolNameIxArgs),
    WithdrawAfterPoolClosure,
    WithdrawLiquidityAfterTargetClosure(WithdrawLiquidityAfterTargetClosureIxArgs),
}
impl HumaProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_ASYNC_DEPLOYMENT_TARGET_IX_DISCM) {
            let mut reader = &buf[ADD_ASYNC_DEPLOYMENT_TARGET_IX_DISCM.len()..];
            let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddAsyncDeploymentTarget(AddAsyncDeploymentTargetIxArgs {
                    strategy_type,
                    target_key,
                }),
            );
        }
        if buf.starts_with(&ADD_DEPLOYMENT_TARGET_IX_DISCM) {
            let mut reader = &buf[ADD_DEPLOYMENT_TARGET_IX_DISCM.len()..];
            let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddDeploymentTarget(AddDeploymentTargetIxArgs {
                    strategy_type,
                    target_key,
                }),
            );
        }
        if buf.starts_with(&ADD_INSTANT_WITHDRAWAL_LENDER_IX_DISCM) {
            let mut reader = &buf[ADD_INSTANT_WITHDRAWAL_LENDER_IX_DISCM.len()..];
            let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddInstantWithdrawalLender(AddInstantWithdrawalLenderIxArgs {
                    lender,
                }),
            );
        }
        if buf.starts_with(&ADD_MODE_IX_DISCM) {
            let mut reader = &buf[ADD_MODE_IX_DISCM.len()..];
            let mode_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let mode_name: String = crate::borsh_de_or_default(&mut reader)?;
            let target_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let token_metadata_args = if reader.is_empty() {
                Default::default()
            } else {
                <ManageModeTokenMetadataArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddMode(AddModeIxArgs {
                    mode_id,
                    mode_name,
                    target_apy_bps,
                    token_metadata_args,
                }),
            );
        }
        if buf.starts_with(&ADD_POOL_OPERATOR_IX_DISCM) {
            let mut reader = &buf[ADD_POOL_OPERATOR_IX_DISCM.len()..];
            let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::AddPoolOperator(AddPoolOperatorIxArgs { operator }));
        }
        if buf.starts_with(&ADD_REDEMPTION_REQUEST_V2_IX_DISCM) {
            let mut reader = &buf[ADD_REDEMPTION_REQUEST_V2_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddRedemptionRequestV2(AddRedemptionRequestV2IxArgs {
                    shares,
                }),
            );
        }
        if buf.starts_with(&ADD_STRATEGY_MANAGER_WALLET_IX_DISCM) {
            let mut reader = &buf[ADD_STRATEGY_MANAGER_WALLET_IX_DISCM.len()..];
            let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddStrategyManagerWallet(AddStrategyManagerWalletIxArgs {
                    wallet,
                }),
            );
        }
        if buf.starts_with(&APPROVE_MANUAL_STRATEGY_MANAGER_IX_DISCM) {
            return Ok(Self::ApproveManualStrategyManager);
        }
        if buf.starts_with(&CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_DISCM) {
            return Ok(Self::CancelManualStrategyManagerProposal);
        }
        if buf.starts_with(&CANCEL_REDEMPTION_REQUEST_IX_DISCM) {
            let mut reader = &buf[CANCEL_REDEMPTION_REQUEST_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CancelRedemptionRequest(CancelRedemptionRequestIxArgs {
                    shares,
                }),
            );
        }
        if buf.starts_with(&CHANGE_POOL_OWNER_IX_DISCM) {
            let mut reader = &buf[CHANGE_POOL_OWNER_IX_DISCM.len()..];
            let new_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ChangePoolOwner(ChangePoolOwnerIxArgs { new_owner }));
        }
        if buf.starts_with(&CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM) {
            return Ok(Self::ClearInstantWithdrawalLiquiditySource);
        }
        if buf.starts_with(&CLOSE_LENDER_ACCOUNTS_IX_DISCM) {
            return Ok(Self::CloseLenderAccounts);
        }
        if buf.starts_with(&CLOSE_POOL_IX_DISCM) {
            return Ok(Self::ClosePool);
        }
        if buf.starts_with(&COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM) {
            let mut reader = &buf[COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM.len()..];
            let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::CompleteLiquidityPaybackAsync(CompleteLiquidityPaybackAsyncIxArgs {
                    strategy_type,
                }),
            );
        }
        if buf.starts_with(&CREATE_LENDER_ACCOUNTS_V2_IX_DISCM) {
            return Ok(Self::CreateLenderAccountsV2);
        }
        if buf.starts_with(&CREATE_POOL_IX_DISCM) {
            let mut reader = &buf[CREATE_POOL_IX_DISCM.len()..];
            let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let pool_name: String = crate::borsh_de_or_default(&mut reader)?;
            let pool_owner_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreatePool(CreatePoolIxArgs {
                    pool_id,
                    pool_name,
                    pool_owner_treasury,
                    loss_authority,
                }),
            );
        }
        if buf.starts_with(&DECLARE_LOSS_IX_DISCM) {
            let mut reader = &buf[DECLARE_LOSS_IX_DISCM.len()..];
            let loss: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::DeclareLoss(DeclareLossIxArgs { loss }));
        }
        if buf.starts_with(&DEPLOY_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[DEPLOY_LIQUIDITY_IX_DISCM.len()..];
            let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DeployLiquidity(DeployLiquidityIxArgs {
                    strategy_type,
                    amount,
                }),
            );
        }
        if buf.starts_with(&DEPLOY_LIQUIDITY_ASYNC_IX_DISCM) {
            let mut reader = &buf[DEPLOY_LIQUIDITY_ASYNC_IX_DISCM.len()..];
            let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DeployLiquidityAsync(DeployLiquidityAsyncIxArgs {
                    strategy_type,
                    amount,
                }),
            );
        }
        if buf.starts_with(&DEPLOY_LIQUIDITY_MANUALLY_IX_DISCM) {
            let mut reader = &buf[DEPLOY_LIQUIDITY_MANUALLY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DeployLiquidityManually(DeployLiquidityManuallyIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
            let commitment: String = crate::borsh_de_or_default(&mut reader)?;
            let commitment_auto_renewal: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Deposit(DepositIxArgs {
                    assets,
                    commitment,
                    commitment_auto_renewal,
                }),
            );
        }
        if buf.starts_with(&DISABLE_POOL_IX_DISCM) {
            return Ok(Self::DisablePool);
        }
        if buf.starts_with(&DISBURSE_IX_DISCM) {
            return Ok(Self::Disburse);
        }
        if buf.starts_with(&ENABLE_POOL_IX_DISCM) {
            return Ok(Self::EnablePool);
        }
        if buf.starts_with(&ENTER_PRE_CLOSURE_IX_DISCM) {
            return Ok(Self::EnterPreClosure);
        }
        if buf.starts_with(&INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM) {
            let mut reader = &buf[INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM.len()..];
            let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitiateLiquidityPaybackAsync(InitiateLiquidityPaybackAsyncIxArgs {
                    strategy_type,
                    shares,
                }),
            );
        }
        if buf.starts_with(&INSTANT_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[INSTANT_WITHDRAW_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InstantWithdraw(InstantWithdrawIxArgs {
                    shares,
                    max_fee,
                }),
            );
        }
        if buf.starts_with(&INSTANT_WITHDRAW_PRIVILEGED_IX_DISCM) {
            let mut reader = &buf[INSTANT_WITHDRAW_PRIVILEGED_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InstantWithdrawPrivileged(InstantWithdrawPrivilegedIxArgs {
                    shares,
                }),
            );
        }
        if buf.starts_with(&MAKE_INITIAL_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[MAKE_INITIAL_DEPOSIT_IX_DISCM.len()..];
            let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
            let commitment: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MakeInitialDeposit(MakeInitialDepositIxArgs {
                    assets,
                    commitment,
                }),
            );
        }
        if buf.starts_with(&PAY_BACK_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[PAY_BACK_LIQUIDITY_IX_DISCM.len()..];
            let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PayBackLiquidity(PayBackLiquidityIxArgs {
                    strategy_type,
                    amount,
                }),
            );
        }
        if buf.starts_with(&PAY_BACK_LIQUIDITY_MANUALLY_IX_DISCM) {
            let mut reader = &buf[PAY_BACK_LIQUIDITY_MANUALLY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PayBackLiquidityManually(PayBackLiquidityManuallyIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&PROCESS_REDEMPTION_REQUEST_IX_DISCM) {
            return Ok(Self::ProcessRedemptionRequest);
        }
        if buf.starts_with(&PROPOSE_MANUAL_STRATEGY_MANAGER_IX_DISCM) {
            let mut reader = &buf[PROPOSE_MANUAL_STRATEGY_MANAGER_IX_DISCM.len()..];
            let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ProposeManualStrategyManager(ProposeManualStrategyManagerIxArgs {
                    wallet,
                }),
            );
        }
        if buf.starts_with(&RECOVER_LOSS_IX_DISCM) {
            let mut reader = &buf[RECOVER_LOSS_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RecoverLoss(RecoverLossIxArgs { amount }));
        }
        if buf.starts_with(&REFRESH_MODE_ASSETS_IX_DISCM) {
            return Ok(Self::RefreshModeAssets);
        }
        if buf.starts_with(&REFRESH_POOL_ASSETS_IX_DISCM) {
            return Ok(Self::RefreshPoolAssets);
        }
        if buf.starts_with(&REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_DISCM) {
            return Ok(Self::RemoveAsyncDeploymentTarget);
        }
        if buf.starts_with(&REMOVE_DEPLOYMENT_TARGET_IX_DISCM) {
            return Ok(Self::RemoveDeploymentTarget);
        }
        if buf.starts_with(&REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_DISCM) {
            let mut reader = &buf[REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_DISCM.len()..];
            let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveInstantWithdrawalLender(RemoveInstantWithdrawalLenderIxArgs {
                    lender,
                }),
            );
        }
        if buf.starts_with(&REMOVE_MANUAL_STRATEGY_MANAGER_IX_DISCM) {
            return Ok(Self::RemoveManualStrategyManager);
        }
        if buf.starts_with(&REMOVE_MODE_IX_DISCM) {
            let mut reader = &buf[REMOVE_MODE_IX_DISCM.len()..];
            let mode_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RemoveMode(RemoveModeIxArgs { mode_id }));
        }
        if buf.starts_with(&REMOVE_POOL_OPERATOR_IX_DISCM) {
            let mut reader = &buf[REMOVE_POOL_OPERATOR_IX_DISCM.len()..];
            let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemovePoolOperator(RemovePoolOperatorIxArgs {
                    operator,
                }),
            );
        }
        if buf.starts_with(&REMOVE_STRATEGY_MANAGER_WALLET_IX_DISCM) {
            return Ok(Self::RemoveStrategyManagerWallet);
        }
        if buf.starts_with(&SET_HUMA_CONFIG_IX_DISCM) {
            return Ok(Self::SetHumaConfig);
        }
        if buf.starts_with(&SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_DISCM) {
            let mut reader = &buf[SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_DISCM.len()..];
            let fee_configs: Vec<InstantWithdrawalFeeConfigInput> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetInstantWithdrawalFeeConfigs(SetInstantWithdrawalFeeConfigsIxArgs {
                    fee_configs,
                }),
            );
        }
        if buf.starts_with(&SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM) {
            return Ok(Self::SetInstantWithdrawalLiquiditySource);
        }
        if buf.starts_with(&SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_DISCM) {
            let mut reader = &buf[SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_DISCM.len()..];
            let new_reserve_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetInstantWithdrawalReserveLimit(SetInstantWithdrawalReserveLimitIxArgs {
                    new_reserve_limit,
                }),
            );
        }
        if buf.starts_with(&SET_LIQUID_ASSETS_DEPLOYED_IX_DISCM) {
            let mut reader = &buf[SET_LIQUID_ASSETS_DEPLOYED_IX_DISCM.len()..];
            let liquid_assets_deployed: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetLiquidAssetsDeployed(SetLiquidAssetsDeployedIxArgs {
                    liquid_assets_deployed,
                }),
            );
        }
        if buf.starts_with(&SET_LOSS_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[SET_LOSS_AUTHORITY_IX_DISCM.len()..];
            let new_loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetLossAuthority(SetLossAuthorityIxArgs {
                    new_loss_authority,
                }),
            );
        }
        if buf.starts_with(&SET_LP_CONFIG_IX_DISCM) {
            let mut reader = &buf[SET_LP_CONFIG_IX_DISCM.len()..];
            let configs = <LPConfig>::deserialize(&mut reader)?;
            return Ok(Self::SetLpConfig(SetLpConfigIxArgs { configs }));
        }
        if buf.starts_with(&SET_MANUAL_DEPLOYMENT_LIMITS_IX_DISCM) {
            let mut reader = &buf[SET_MANUAL_DEPLOYMENT_LIMITS_IX_DISCM.len()..];
            let daily_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
            let per_wallet_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetManualDeploymentLimits(SetManualDeploymentLimitsIxArgs {
                    daily_limit,
                    per_wallet_limit,
                }),
            );
        }
        if buf.starts_with(&SET_POOL_OWNER_TREASURY_IX_DISCM) {
            let mut reader = &buf[SET_POOL_OWNER_TREASURY_IX_DISCM.len()..];
            let new_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPoolOwnerTreasury(SetPoolOwnerTreasuryIxArgs {
                    new_treasury,
                }),
            );
        }
        if buf.starts_with(&SETUP_DEPLOYMENT_TARGET_IX_DISCM) {
            let mut reader = &buf[SETUP_DEPLOYMENT_TARGET_IX_DISCM.len()..];
            let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetupDeploymentTarget(SetupDeploymentTargetIxArgs {
                    strategy_type,
                }),
            );
        }
        if buf.starts_with(&SWITCH_MODE_IX_DISCM) {
            let mut reader = &buf[SWITCH_MODE_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let investment_id: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwitchMode(SwitchModeIxArgs {
                    shares,
                    investment_id,
                }),
            );
        }
        if buf.starts_with(&UPDATE_MODE_APY_IX_DISCM) {
            let mut reader = &buf[UPDATE_MODE_APY_IX_DISCM.len()..];
            let new_target_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateModeApy(UpdateModeApyIxArgs {
                    new_target_apy_bps,
                }),
            );
        }
        if buf.starts_with(&UPDATE_MODE_NAME_IX_DISCM) {
            let mut reader = &buf[UPDATE_MODE_NAME_IX_DISCM.len()..];
            let new_name: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdateModeName(UpdateModeNameIxArgs { new_name }));
        }
        if buf.starts_with(&UPDATE_MODE_TOKEN_METADATA_IX_DISCM) {
            let mut reader = &buf[UPDATE_MODE_TOKEN_METADATA_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ManageModeTokenMetadataArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateModeTokenMetadata(UpdateModeTokenMetadataIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&UPDATE_POOL_NAME_IX_DISCM) {
            let mut reader = &buf[UPDATE_POOL_NAME_IX_DISCM.len()..];
            let new_name: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdatePoolName(UpdatePoolNameIxArgs { new_name }));
        }
        if buf.starts_with(&WITHDRAW_AFTER_POOL_CLOSURE_IX_DISCM) {
            return Ok(Self::WithdrawAfterPoolClosure);
        }
        if buf.starts_with(&WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_DISCM
                .len()..];
            let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::WithdrawLiquidityAfterTargetClosure(WithdrawLiquidityAfterTargetClosureIxArgs {
                    strategy_type,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddAsyncDeploymentTarget(args) => {
                writer.write_all(&ADD_ASYNC_DEPLOYMENT_TARGET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.target_key, &mut writer)?;
                Ok(())
            }
            Self::AddDeploymentTarget(args) => {
                writer.write_all(&ADD_DEPLOYMENT_TARGET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.target_key, &mut writer)?;
                Ok(())
            }
            Self::AddInstantWithdrawalLender(args) => {
                writer.write_all(&ADD_INSTANT_WITHDRAWAL_LENDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lender, &mut writer)?;
                Ok(())
            }
            Self::AddMode(args) => {
                writer.write_all(&ADD_MODE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mode_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.mode_name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.target_apy_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.token_metadata_args,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddPoolOperator(args) => {
                writer.write_all(&ADD_POOL_OPERATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.operator, &mut writer)?;
                Ok(())
            }
            Self::AddRedemptionRequestV2(args) => {
                writer.write_all(&ADD_REDEMPTION_REQUEST_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::AddStrategyManagerWallet(args) => {
                writer.write_all(&ADD_STRATEGY_MANAGER_WALLET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.wallet, &mut writer)?;
                Ok(())
            }
            Self::ApproveManualStrategyManager => {
                writer.write_all(&APPROVE_MANUAL_STRATEGY_MANAGER_IX_DISCM)
            }
            Self::CancelManualStrategyManagerProposal => {
                writer.write_all(&CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_DISCM)
            }
            Self::CancelRedemptionRequest(args) => {
                writer.write_all(&CANCEL_REDEMPTION_REQUEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::ChangePoolOwner(args) => {
                writer.write_all(&CHANGE_POOL_OWNER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_owner, &mut writer)?;
                Ok(())
            }
            Self::ClearInstantWithdrawalLiquiditySource => {
                writer.write_all(&CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM)
            }
            Self::CloseLenderAccounts => {
                writer.write_all(&CLOSE_LENDER_ACCOUNTS_IX_DISCM)
            }
            Self::ClosePool => writer.write_all(&CLOSE_POOL_IX_DISCM),
            Self::CompleteLiquidityPaybackAsync(args) => {
                writer.write_all(&COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                Ok(())
            }
            Self::CreateLenderAccountsV2 => {
                writer.write_all(&CREATE_LENDER_ACCOUNTS_V2_IX_DISCM)
            }
            Self::CreatePool(args) => {
                writer.write_all(&CREATE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pool_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pool_name, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.pool_owner_treasury,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.loss_authority, &mut writer)?;
                Ok(())
            }
            Self::DeclareLoss(args) => {
                writer.write_all(&DECLARE_LOSS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.loss, &mut writer)?;
                Ok(())
            }
            Self::DeployLiquidity(args) => {
                writer.write_all(&DEPLOY_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::DeployLiquidityAsync(args) => {
                writer.write_all(&DEPLOY_LIQUIDITY_ASYNC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::DeployLiquidityManually(args) => {
                writer.write_all(&DEPLOY_LIQUIDITY_MANUALLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.assets, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.commitment, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.commitment_auto_renewal,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::DisablePool => writer.write_all(&DISABLE_POOL_IX_DISCM),
            Self::Disburse => writer.write_all(&DISBURSE_IX_DISCM),
            Self::EnablePool => writer.write_all(&ENABLE_POOL_IX_DISCM),
            Self::EnterPreClosure => writer.write_all(&ENTER_PRE_CLOSURE_IX_DISCM),
            Self::InitiateLiquidityPaybackAsync(args) => {
                writer.write_all(&INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::InstantWithdraw(args) => {
                writer.write_all(&INSTANT_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_fee, &mut writer)?;
                Ok(())
            }
            Self::InstantWithdrawPrivileged(args) => {
                writer.write_all(&INSTANT_WITHDRAW_PRIVILEGED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                Ok(())
            }
            Self::MakeInitialDeposit(args) => {
                writer.write_all(&MAKE_INITIAL_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.assets, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.commitment, &mut writer)?;
                Ok(())
            }
            Self::PayBackLiquidity(args) => {
                writer.write_all(&PAY_BACK_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::PayBackLiquidityManually(args) => {
                writer.write_all(&PAY_BACK_LIQUIDITY_MANUALLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::ProcessRedemptionRequest => {
                writer.write_all(&PROCESS_REDEMPTION_REQUEST_IX_DISCM)
            }
            Self::ProposeManualStrategyManager(args) => {
                writer.write_all(&PROPOSE_MANUAL_STRATEGY_MANAGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.wallet, &mut writer)?;
                Ok(())
            }
            Self::RecoverLoss(args) => {
                writer.write_all(&RECOVER_LOSS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::RefreshModeAssets => writer.write_all(&REFRESH_MODE_ASSETS_IX_DISCM),
            Self::RefreshPoolAssets => writer.write_all(&REFRESH_POOL_ASSETS_IX_DISCM),
            Self::RemoveAsyncDeploymentTarget => {
                writer.write_all(&REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_DISCM)
            }
            Self::RemoveDeploymentTarget => {
                writer.write_all(&REMOVE_DEPLOYMENT_TARGET_IX_DISCM)
            }
            Self::RemoveInstantWithdrawalLender(args) => {
                writer.write_all(&REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lender, &mut writer)?;
                Ok(())
            }
            Self::RemoveManualStrategyManager => {
                writer.write_all(&REMOVE_MANUAL_STRATEGY_MANAGER_IX_DISCM)
            }
            Self::RemoveMode(args) => {
                writer.write_all(&REMOVE_MODE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mode_id, &mut writer)?;
                Ok(())
            }
            Self::RemovePoolOperator(args) => {
                writer.write_all(&REMOVE_POOL_OPERATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.operator, &mut writer)?;
                Ok(())
            }
            Self::RemoveStrategyManagerWallet => {
                writer.write_all(&REMOVE_STRATEGY_MANAGER_WALLET_IX_DISCM)
            }
            Self::SetHumaConfig => writer.write_all(&SET_HUMA_CONFIG_IX_DISCM),
            Self::SetInstantWithdrawalFeeConfigs(args) => {
                writer.write_all(&SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_configs, &mut writer)?;
                Ok(())
            }
            Self::SetInstantWithdrawalLiquiditySource => {
                writer.write_all(&SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM)
            }
            Self::SetInstantWithdrawalReserveLimit(args) => {
                writer.write_all(&SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_reserve_limit, &mut writer)?;
                Ok(())
            }
            Self::SetLiquidAssetsDeployed(args) => {
                writer.write_all(&SET_LIQUID_ASSETS_DEPLOYED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquid_assets_deployed,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetLossAuthority(args) => {
                writer.write_all(&SET_LOSS_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_loss_authority, &mut writer)?;
                Ok(())
            }
            Self::SetLpConfig(args) => {
                writer.write_all(&SET_LP_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.configs, &mut writer)?;
                Ok(())
            }
            Self::SetManualDeploymentLimits(args) => {
                writer.write_all(&SET_MANUAL_DEPLOYMENT_LIMITS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.daily_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.per_wallet_limit, &mut writer)?;
                Ok(())
            }
            Self::SetPoolOwnerTreasury(args) => {
                writer.write_all(&SET_POOL_OWNER_TREASURY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_treasury, &mut writer)?;
                Ok(())
            }
            Self::SetupDeploymentTarget(args) => {
                writer.write_all(&SETUP_DEPLOYMENT_TARGET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                Ok(())
            }
            Self::SwitchMode(args) => {
                writer.write_all(&SWITCH_MODE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.investment_id, &mut writer)?;
                Ok(())
            }
            Self::UpdateModeApy(args) => {
                writer.write_all(&UPDATE_MODE_APY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_target_apy_bps, &mut writer)?;
                Ok(())
            }
            Self::UpdateModeName(args) => {
                writer.write_all(&UPDATE_MODE_NAME_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_name, &mut writer)?;
                Ok(())
            }
            Self::UpdateModeTokenMetadata(args) => {
                writer.write_all(&UPDATE_MODE_TOKEN_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdatePoolName(args) => {
                writer.write_all(&UPDATE_POOL_NAME_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_name, &mut writer)?;
                Ok(())
            }
            Self::WithdrawAfterPoolClosure => {
                writer.write_all(&WITHDRAW_AFTER_POOL_CLOSURE_IX_DISCM)
            }
            Self::WithdrawLiquidityAfterTargetClosure(args) => {
                writer.write_all(&WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
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
pub const ADD_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AddAsyncDeploymentTargetAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub async_deployment_config: &'me AccountInfo<'info>,
    pub async_deployment_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddAsyncDeploymentTargetKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub async_deployment_config: Pubkey,
    pub async_deployment_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddAsyncDeploymentTargetAccounts<'_, '_>> for AddAsyncDeploymentTargetKeys {
    fn from(accounts: AddAsyncDeploymentTargetAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            async_deployment_config: *accounts.async_deployment_config.key,
            async_deployment_state: *accounts.async_deployment_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddAsyncDeploymentTargetKeys>
for [AccountMeta; ADD_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(keys: AddAsyncDeploymentTargetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.async_deployment_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.async_deployment_state,
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
impl From<[Pubkey; ADD_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for AddAsyncDeploymentTargetKeys {
    fn from(pubkeys: [Pubkey; ADD_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            async_deployment_config: pubkeys[4],
            async_deployment_state: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<AddAsyncDeploymentTargetAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddAsyncDeploymentTargetAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.async_deployment_config.clone(),
            accounts.async_deployment_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for AddAsyncDeploymentTargetAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            async_deployment_config: &arr[4],
            async_deployment_state: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const ADD_ASYNC_DEPLOYMENT_TARGET_IX_DISCM: [u8; 8usize] = [
    21, 128, 4, 227, 182, 90, 20, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddAsyncDeploymentTargetIxArgs {
    pub strategy_type: AsyncDeploymentStrategyType,
    pub target_key: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddAsyncDeploymentTargetIxData(pub AddAsyncDeploymentTargetIxArgs);
impl From<AddAsyncDeploymentTargetIxArgs> for AddAsyncDeploymentTargetIxData {
    fn from(args: AddAsyncDeploymentTargetIxArgs) -> Self {
        Self(args)
    }
}
impl AddAsyncDeploymentTargetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_ASYNC_DEPLOYMENT_TARGET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddAsyncDeploymentTargetIxArgs {
                strategy_type,
                target_key,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_ASYNC_DEPLOYMENT_TARGET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.target_key, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_async_deployment_target_ix_with_program_id(
    program_id: Pubkey,
    keys: AddAsyncDeploymentTargetKeys,
    args: AddAsyncDeploymentTargetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddAsyncDeploymentTargetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_async_deployment_target_ix(
    keys: AddAsyncDeploymentTargetKeys,
    args: AddAsyncDeploymentTargetIxArgs,
) -> std::io::Result<Instruction> {
    add_async_deployment_target_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn add_async_deployment_target_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddAsyncDeploymentTargetAccounts<'_, '_>,
    args: AddAsyncDeploymentTargetIxArgs,
) -> ProgramResult {
    let keys: AddAsyncDeploymentTargetKeys = accounts.into();
    let ix = add_async_deployment_target_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_async_deployment_target_invoke(
    accounts: AddAsyncDeploymentTargetAccounts<'_, '_>,
    args: AddAsyncDeploymentTargetIxArgs,
) -> ProgramResult {
    add_async_deployment_target_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn add_async_deployment_target_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddAsyncDeploymentTargetAccounts<'_, '_>,
    args: AddAsyncDeploymentTargetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddAsyncDeploymentTargetKeys = accounts.into();
    let ix = add_async_deployment_target_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_async_deployment_target_invoke_signed(
    accounts: AddAsyncDeploymentTargetAccounts<'_, '_>,
    args: AddAsyncDeploymentTargetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_async_deployment_target_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_async_deployment_target_verify_account_keys(
    accounts: AddAsyncDeploymentTargetAccounts<'_, '_>,
    keys: AddAsyncDeploymentTargetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.async_deployment_config.key, keys.async_deployment_config),
        (*accounts.async_deployment_state.key, keys.async_deployment_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_async_deployment_target_verify_writable_privileges<'me, 'info>(
    accounts: AddAsyncDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.async_deployment_config,
        accounts.async_deployment_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_async_deployment_target_verify_signer_privileges<'me, 'info>(
    accounts: AddAsyncDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_async_deployment_target_verify_account_privileges<'me, 'info>(
    accounts: AddAsyncDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_async_deployment_target_verify_writable_privileges(accounts)?;
    add_async_deployment_target_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AddDeploymentTargetAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub deployment_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddDeploymentTargetKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub deployment_config: Pubkey,
    pub deployment_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddDeploymentTargetAccounts<'_, '_>> for AddDeploymentTargetKeys {
    fn from(accounts: AddDeploymentTargetAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            deployment_config: *accounts.deployment_config.key,
            deployment_state: *accounts.deployment_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddDeploymentTargetKeys>
for [AccountMeta; ADD_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(keys: AddDeploymentTargetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployment_state,
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
impl From<[Pubkey; ADD_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]> for AddDeploymentTargetKeys {
    fn from(pubkeys: [Pubkey; ADD_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            deployment_config: pubkeys[4],
            deployment_state: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<AddDeploymentTargetAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddDeploymentTargetAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.deployment_config.clone(),
            accounts.deployment_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for AddDeploymentTargetAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            deployment_config: &arr[4],
            deployment_state: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const ADD_DEPLOYMENT_TARGET_IX_DISCM: [u8; 8usize] = [
    231, 191, 129, 183, 23, 79, 165, 130,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddDeploymentTargetIxArgs {
    pub strategy_type: DeploymentStrategyType,
    pub target_key: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddDeploymentTargetIxData(pub AddDeploymentTargetIxArgs);
impl From<AddDeploymentTargetIxArgs> for AddDeploymentTargetIxData {
    fn from(args: AddDeploymentTargetIxArgs) -> Self {
        Self(args)
    }
}
impl AddDeploymentTargetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_DEPLOYMENT_TARGET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddDeploymentTargetIxArgs {
                strategy_type,
                target_key,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_DEPLOYMENT_TARGET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.target_key, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_deployment_target_ix_with_program_id(
    program_id: Pubkey,
    keys: AddDeploymentTargetKeys,
    args: AddDeploymentTargetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddDeploymentTargetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_deployment_target_ix(
    keys: AddDeploymentTargetKeys,
    args: AddDeploymentTargetIxArgs,
) -> std::io::Result<Instruction> {
    add_deployment_target_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn add_deployment_target_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddDeploymentTargetAccounts<'_, '_>,
    args: AddDeploymentTargetIxArgs,
) -> ProgramResult {
    let keys: AddDeploymentTargetKeys = accounts.into();
    let ix = add_deployment_target_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_deployment_target_invoke(
    accounts: AddDeploymentTargetAccounts<'_, '_>,
    args: AddDeploymentTargetIxArgs,
) -> ProgramResult {
    add_deployment_target_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn add_deployment_target_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddDeploymentTargetAccounts<'_, '_>,
    args: AddDeploymentTargetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddDeploymentTargetKeys = accounts.into();
    let ix = add_deployment_target_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_deployment_target_invoke_signed(
    accounts: AddDeploymentTargetAccounts<'_, '_>,
    args: AddDeploymentTargetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_deployment_target_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_deployment_target_verify_account_keys(
    accounts: AddDeploymentTargetAccounts<'_, '_>,
    keys: AddDeploymentTargetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.deployment_state.key, keys.deployment_state),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_deployment_target_verify_writable_privileges<'me, 'info>(
    accounts: AddDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.deployment_config,
        accounts.deployment_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_deployment_target_verify_signer_privileges<'me, 'info>(
    accounts: AddDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_deployment_target_verify_account_privileges<'me, 'info>(
    accounts: AddDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_deployment_target_verify_writable_privileges(accounts)?;
    add_deployment_target_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddInstantWithdrawalLenderAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub instant_withdrawal_lender_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddInstantWithdrawalLenderKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub instant_withdrawal_lender_config: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddInstantWithdrawalLenderAccounts<'_, '_>>
for AddInstantWithdrawalLenderKeys {
    fn from(accounts: AddInstantWithdrawalLenderAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            instant_withdrawal_lender_config: *accounts
                .instant_withdrawal_lender_config
                .key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddInstantWithdrawalLenderKeys>
for [AccountMeta; ADD_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN] {
    fn from(keys: AddInstantWithdrawalLenderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instant_withdrawal_lender_config,
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
impl From<[Pubkey; ADD_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN]>
for AddInstantWithdrawalLenderKeys {
    fn from(pubkeys: [Pubkey; ADD_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            instant_withdrawal_lender_config: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<AddInstantWithdrawalLenderAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddInstantWithdrawalLenderAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.instant_withdrawal_lender_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN]>
for AddInstantWithdrawalLenderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            instant_withdrawal_lender_config: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const ADD_INSTANT_WITHDRAWAL_LENDER_IX_DISCM: [u8; 8usize] = [
    202, 77, 67, 93, 143, 238, 172, 10,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddInstantWithdrawalLenderIxArgs {
    pub lender: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddInstantWithdrawalLenderIxData(pub AddInstantWithdrawalLenderIxArgs);
impl From<AddInstantWithdrawalLenderIxArgs> for AddInstantWithdrawalLenderIxData {
    fn from(args: AddInstantWithdrawalLenderIxArgs) -> Self {
        Self(args)
    }
}
impl AddInstantWithdrawalLenderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_INSTANT_WITHDRAWAL_LENDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddInstantWithdrawalLenderIxArgs {
                lender,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_INSTANT_WITHDRAWAL_LENDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lender, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_instant_withdrawal_lender_ix_with_program_id(
    program_id: Pubkey,
    keys: AddInstantWithdrawalLenderKeys,
    args: AddInstantWithdrawalLenderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AddInstantWithdrawalLenderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_instant_withdrawal_lender_ix(
    keys: AddInstantWithdrawalLenderKeys,
    args: AddInstantWithdrawalLenderIxArgs,
) -> std::io::Result<Instruction> {
    add_instant_withdrawal_lender_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn add_instant_withdrawal_lender_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddInstantWithdrawalLenderAccounts<'_, '_>,
    args: AddInstantWithdrawalLenderIxArgs,
) -> ProgramResult {
    let keys: AddInstantWithdrawalLenderKeys = accounts.into();
    let ix = add_instant_withdrawal_lender_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_instant_withdrawal_lender_invoke(
    accounts: AddInstantWithdrawalLenderAccounts<'_, '_>,
    args: AddInstantWithdrawalLenderIxArgs,
) -> ProgramResult {
    add_instant_withdrawal_lender_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn add_instant_withdrawal_lender_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddInstantWithdrawalLenderAccounts<'_, '_>,
    args: AddInstantWithdrawalLenderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddInstantWithdrawalLenderKeys = accounts.into();
    let ix = add_instant_withdrawal_lender_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_instant_withdrawal_lender_invoke_signed(
    accounts: AddInstantWithdrawalLenderAccounts<'_, '_>,
    args: AddInstantWithdrawalLenderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_instant_withdrawal_lender_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_instant_withdrawal_lender_verify_account_keys(
    accounts: AddInstantWithdrawalLenderAccounts<'_, '_>,
    keys: AddInstantWithdrawalLenderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (
            *accounts.instant_withdrawal_lender_config.key,
            keys.instant_withdrawal_lender_config,
        ),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_instant_withdrawal_lender_verify_writable_privileges<'me, 'info>(
    accounts: AddInstantWithdrawalLenderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.instant_withdrawal_lender_config,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_instant_withdrawal_lender_verify_signer_privileges<'me, 'info>(
    accounts: AddInstantWithdrawalLenderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_instant_withdrawal_lender_verify_account_privileges<'me, 'info>(
    accounts: AddInstantWithdrawalLenderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_instant_withdrawal_lender_verify_writable_privileges(accounts)?;
    add_instant_withdrawal_lender_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_MODE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct AddModeAccounts<'me, 'info> {
    pub pool_owner: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub token_metadata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub mpl_token_metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub sysvar_instructions_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddModeKeys {
    pub pool_owner: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub token_metadata: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub mpl_token_metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub sysvar_instructions_program: Pubkey,
}
impl From<AddModeAccounts<'_, '_>> for AddModeKeys {
    fn from(accounts: AddModeAccounts) -> Self {
        Self {
            pool_owner: *accounts.pool_owner.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            token_metadata: *accounts.token_metadata.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            mpl_token_metadata_program: *accounts.mpl_token_metadata_program.key,
            system_program: *accounts.system_program.key,
            sysvar_instructions_program: *accounts.sysvar_instructions_program.key,
        }
    }
}
impl From<AddModeKeys> for [AccountMeta; ADD_MODE_IX_ACCOUNTS_LEN] {
    fn from(keys: AddModeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_metadata,
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
                pubkey: keys.mpl_token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sysvar_instructions_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_MODE_IX_ACCOUNTS_LEN]> for AddModeKeys {
    fn from(pubkeys: [Pubkey; ADD_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner: pubkeys[0],
            pool_config: pubkeys[1],
            pool_state: pubkeys[2],
            pool_authority: pubkeys[3],
            underlying_mint: pubkeys[4],
            mode_config: pubkeys[5],
            mode_mint: pubkeys[6],
            token_metadata: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            mpl_token_metadata_program: pubkeys[10],
            system_program: pubkeys[11],
            sysvar_instructions_program: pubkeys[12],
        }
    }
}
impl<'info> From<AddModeAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_MODE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddModeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_owner.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.token_metadata.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.mpl_token_metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.sysvar_instructions_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_MODE_IX_ACCOUNTS_LEN]>
for AddModeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner: &arr[0],
            pool_config: &arr[1],
            pool_state: &arr[2],
            pool_authority: &arr[3],
            underlying_mint: &arr[4],
            mode_config: &arr[5],
            mode_mint: &arr[6],
            token_metadata: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            mpl_token_metadata_program: &arr[10],
            system_program: &arr[11],
            sysvar_instructions_program: &arr[12],
        }
    }
}
pub const ADD_MODE_IX_DISCM: [u8; 8usize] = [136, 187, 228, 193, 168, 107, 113, 144];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddModeIxArgs {
    pub mode_id: Pubkey,
    pub mode_name: String,
    pub target_apy_bps: u16,
    pub token_metadata_args: ManageModeTokenMetadataArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddModeIxData(pub AddModeIxArgs);
impl From<AddModeIxArgs> for AddModeIxData {
    fn from(args: AddModeIxArgs) -> Self {
        Self(args)
    }
}
impl AddModeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_MODE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mode_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mode_name: String = crate::borsh_de_or_default(&mut reader)?;
        let target_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_metadata_args = if reader.is_empty() {
            Default::default()
        } else {
            <ManageModeTokenMetadataArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddModeIxArgs {
                mode_id,
                mode_name,
                target_apy_bps,
                token_metadata_args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_MODE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mode_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.mode_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.target_apy_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_metadata_args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_mode_ix_with_program_id(
    program_id: Pubkey,
    keys: AddModeKeys,
    args: AddModeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_MODE_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddModeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_mode_ix(
    keys: AddModeKeys,
    args: AddModeIxArgs,
) -> std::io::Result<Instruction> {
    add_mode_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn add_mode_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddModeAccounts<'_, '_>,
    args: AddModeIxArgs,
) -> ProgramResult {
    let keys: AddModeKeys = accounts.into();
    let ix = add_mode_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_mode_invoke(
    accounts: AddModeAccounts<'_, '_>,
    args: AddModeIxArgs,
) -> ProgramResult {
    add_mode_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn add_mode_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddModeAccounts<'_, '_>,
    args: AddModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddModeKeys = accounts.into();
    let ix = add_mode_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_mode_invoke_signed(
    accounts: AddModeAccounts<'_, '_>,
    args: AddModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_mode_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_mode_verify_account_keys(
    accounts: AddModeAccounts<'_, '_>,
    keys: AddModeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_owner.key, keys.pool_owner),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.token_metadata.key, keys.token_metadata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.mpl_token_metadata_program.key, keys.mpl_token_metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.sysvar_instructions_program.key, keys.sysvar_instructions_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_mode_verify_writable_privileges<'me, 'info>(
    accounts: AddModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_owner,
        accounts.pool_state,
        accounts.mode_config,
        accounts.mode_mint,
        accounts.token_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_mode_verify_signer_privileges<'me, 'info>(
    accounts: AddModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_mode_verify_account_privileges<'me, 'info>(
    accounts: AddModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_mode_verify_writable_privileges(accounts)?;
    add_mode_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_POOL_OPERATOR_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddPoolOperatorAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_operator_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddPoolOperatorKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_operator_config: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddPoolOperatorAccounts<'_, '_>> for AddPoolOperatorKeys {
    fn from(accounts: AddPoolOperatorAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_operator_config: *accounts.pool_operator_config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddPoolOperatorKeys> for [AccountMeta; ADD_POOL_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: AddPoolOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_operator_config,
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
impl From<[Pubkey; ADD_POOL_OPERATOR_IX_ACCOUNTS_LEN]> for AddPoolOperatorKeys {
    fn from(pubkeys: [Pubkey; ADD_POOL_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            pool_operator_config: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<AddPoolOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_POOL_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddPoolOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_operator_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_POOL_OPERATOR_IX_ACCOUNTS_LEN]>
for AddPoolOperatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_POOL_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            pool_operator_config: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const ADD_POOL_OPERATOR_IX_DISCM: [u8; 8usize] = [
    87, 245, 32, 78, 182, 157, 163, 249,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddPoolOperatorIxArgs {
    pub operator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddPoolOperatorIxData(pub AddPoolOperatorIxArgs);
impl From<AddPoolOperatorIxArgs> for AddPoolOperatorIxData {
    fn from(args: AddPoolOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl AddPoolOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_POOL_OPERATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(AddPoolOperatorIxArgs { operator }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_POOL_OPERATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.operator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_pool_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: AddPoolOperatorKeys,
    args: AddPoolOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_POOL_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddPoolOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_pool_operator_ix(
    keys: AddPoolOperatorKeys,
    args: AddPoolOperatorIxArgs,
) -> std::io::Result<Instruction> {
    add_pool_operator_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn add_pool_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddPoolOperatorAccounts<'_, '_>,
    args: AddPoolOperatorIxArgs,
) -> ProgramResult {
    let keys: AddPoolOperatorKeys = accounts.into();
    let ix = add_pool_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_pool_operator_invoke(
    accounts: AddPoolOperatorAccounts<'_, '_>,
    args: AddPoolOperatorIxArgs,
) -> ProgramResult {
    add_pool_operator_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn add_pool_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddPoolOperatorAccounts<'_, '_>,
    args: AddPoolOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddPoolOperatorKeys = accounts.into();
    let ix = add_pool_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_pool_operator_invoke_signed(
    accounts: AddPoolOperatorAccounts<'_, '_>,
    args: AddPoolOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_pool_operator_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_pool_operator_verify_account_keys(
    accounts: AddPoolOperatorAccounts<'_, '_>,
    keys: AddPoolOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_operator_config.key, keys.pool_operator_config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_pool_operator_verify_writable_privileges<'me, 'info>(
    accounts: AddPoolOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.pool_operator_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_pool_operator_verify_signer_privileges<'me, 'info>(
    accounts: AddPoolOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_pool_operator_verify_account_privileges<'me, 'info>(
    accounts: AddPoolOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_pool_operator_verify_writable_privileges(accounts)?;
    add_pool_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_REDEMPTION_REQUEST_V2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct AddRedemptionRequestV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub lender: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub redemption_request: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_mode_token: &'me AccountInfo<'info>,
    pub lender_mode_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddRedemptionRequestV2Keys {
    pub payer: Pubkey,
    pub lender: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub redemption_request: Pubkey,
    pub lender_state: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_mode_token: Pubkey,
    pub lender_mode_token: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddRedemptionRequestV2Accounts<'_, '_>> for AddRedemptionRequestV2Keys {
    fn from(accounts: AddRedemptionRequestV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            lender: *accounts.lender.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            redemption_request: *accounts.redemption_request.key,
            lender_state: *accounts.lender_state.key,
            pool_authority: *accounts.pool_authority.key,
            pool_mode_token: *accounts.pool_mode_token.key,
            lender_mode_token: *accounts.lender_mode_token.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddRedemptionRequestV2Keys>
for [AccountMeta; ADD_REDEMPTION_REQUEST_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: AddRedemptionRequestV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_mode_token,
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
impl From<[Pubkey; ADD_REDEMPTION_REQUEST_V2_IX_ACCOUNTS_LEN]>
for AddRedemptionRequestV2Keys {
    fn from(pubkeys: [Pubkey; ADD_REDEMPTION_REQUEST_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            lender: pubkeys[1],
            huma_config: pubkeys[2],
            pool_config: pubkeys[3],
            pool_state: pubkeys[4],
            mode_config: pubkeys[5],
            mode_mint: pubkeys[6],
            redemption_request: pubkeys[7],
            lender_state: pubkeys[8],
            pool_authority: pubkeys[9],
            pool_mode_token: pubkeys[10],
            lender_mode_token: pubkeys[11],
            token_program: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<AddRedemptionRequestV2Accounts<'_, 'info>>
for [AccountInfo<'info>; ADD_REDEMPTION_REQUEST_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddRedemptionRequestV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.lender.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.redemption_request.clone(),
            accounts.lender_state.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_mode_token.clone(),
            accounts.lender_mode_token.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_REDEMPTION_REQUEST_V2_IX_ACCOUNTS_LEN]>
for AddRedemptionRequestV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_REDEMPTION_REQUEST_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            lender: &arr[1],
            huma_config: &arr[2],
            pool_config: &arr[3],
            pool_state: &arr[4],
            mode_config: &arr[5],
            mode_mint: &arr[6],
            redemption_request: &arr[7],
            lender_state: &arr[8],
            pool_authority: &arr[9],
            pool_mode_token: &arr[10],
            lender_mode_token: &arr[11],
            token_program: &arr[12],
            system_program: &arr[13],
        }
    }
}
pub const ADD_REDEMPTION_REQUEST_V2_IX_DISCM: [u8; 8usize] = [
    96, 173, 49, 36, 201, 46, 244, 189,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddRedemptionRequestV2IxArgs {
    pub shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddRedemptionRequestV2IxData(pub AddRedemptionRequestV2IxArgs);
impl From<AddRedemptionRequestV2IxArgs> for AddRedemptionRequestV2IxData {
    fn from(args: AddRedemptionRequestV2IxArgs) -> Self {
        Self(args)
    }
}
impl AddRedemptionRequestV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_REDEMPTION_REQUEST_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddRedemptionRequestV2IxArgs {
                shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_REDEMPTION_REQUEST_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_redemption_request_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: AddRedemptionRequestV2Keys,
    args: AddRedemptionRequestV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_REDEMPTION_REQUEST_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddRedemptionRequestV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_redemption_request_v2_ix(
    keys: AddRedemptionRequestV2Keys,
    args: AddRedemptionRequestV2IxArgs,
) -> std::io::Result<Instruction> {
    add_redemption_request_v2_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn add_redemption_request_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddRedemptionRequestV2Accounts<'_, '_>,
    args: AddRedemptionRequestV2IxArgs,
) -> ProgramResult {
    let keys: AddRedemptionRequestV2Keys = accounts.into();
    let ix = add_redemption_request_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_redemption_request_v2_invoke(
    accounts: AddRedemptionRequestV2Accounts<'_, '_>,
    args: AddRedemptionRequestV2IxArgs,
) -> ProgramResult {
    add_redemption_request_v2_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn add_redemption_request_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddRedemptionRequestV2Accounts<'_, '_>,
    args: AddRedemptionRequestV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddRedemptionRequestV2Keys = accounts.into();
    let ix = add_redemption_request_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_redemption_request_v2_invoke_signed(
    accounts: AddRedemptionRequestV2Accounts<'_, '_>,
    args: AddRedemptionRequestV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_redemption_request_v2_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_redemption_request_v2_verify_account_keys(
    accounts: AddRedemptionRequestV2Accounts<'_, '_>,
    keys: AddRedemptionRequestV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.lender.key, keys.lender),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.redemption_request.key, keys.redemption_request),
        (*accounts.lender_state.key, keys.lender_state),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_mode_token.key, keys.pool_mode_token),
        (*accounts.lender_mode_token.key, keys.lender_mode_token),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_redemption_request_v2_verify_writable_privileges<'me, 'info>(
    accounts: AddRedemptionRequestV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.redemption_request,
        accounts.lender_state,
        accounts.pool_mode_token,
        accounts.lender_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_redemption_request_v2_verify_signer_privileges<'me, 'info>(
    accounts: AddRedemptionRequestV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.lender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_redemption_request_v2_verify_account_privileges<'me, 'info>(
    accounts: AddRedemptionRequestV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_redemption_request_v2_verify_writable_privileges(accounts)?;
    add_redemption_request_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddStrategyManagerWalletAccounts<'me, 'info> {
    pub pool_owner: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddStrategyManagerWalletKeys {
    pub pool_owner: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddStrategyManagerWalletAccounts<'_, '_>> for AddStrategyManagerWalletKeys {
    fn from(accounts: AddStrategyManagerWalletAccounts) -> Self {
        Self {
            pool_owner: *accounts.pool_owner.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddStrategyManagerWalletKeys>
for [AccountMeta; ADD_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN] {
    fn from(keys: AddStrategyManagerWalletKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
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
impl From<[Pubkey; ADD_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN]>
for AddStrategyManagerWalletKeys {
    fn from(pubkeys: [Pubkey; ADD_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            strategy_manager_wallet: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<AddStrategyManagerWalletAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddStrategyManagerWalletAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_owner.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN]>
for AddStrategyManagerWalletAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_owner: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            strategy_manager_wallet: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const ADD_STRATEGY_MANAGER_WALLET_IX_DISCM: [u8; 8usize] = [
    195, 173, 154, 48, 102, 125, 181, 237,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddStrategyManagerWalletIxArgs {
    pub wallet: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddStrategyManagerWalletIxData(pub AddStrategyManagerWalletIxArgs);
impl From<AddStrategyManagerWalletIxArgs> for AddStrategyManagerWalletIxData {
    fn from(args: AddStrategyManagerWalletIxArgs) -> Self {
        Self(args)
    }
}
impl AddStrategyManagerWalletIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_STRATEGY_MANAGER_WALLET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddStrategyManagerWalletIxArgs {
                wallet,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_STRATEGY_MANAGER_WALLET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.wallet, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_strategy_manager_wallet_ix_with_program_id(
    program_id: Pubkey,
    keys: AddStrategyManagerWalletKeys,
    args: AddStrategyManagerWalletIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddStrategyManagerWalletIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_strategy_manager_wallet_ix(
    keys: AddStrategyManagerWalletKeys,
    args: AddStrategyManagerWalletIxArgs,
) -> std::io::Result<Instruction> {
    add_strategy_manager_wallet_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn add_strategy_manager_wallet_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddStrategyManagerWalletAccounts<'_, '_>,
    args: AddStrategyManagerWalletIxArgs,
) -> ProgramResult {
    let keys: AddStrategyManagerWalletKeys = accounts.into();
    let ix = add_strategy_manager_wallet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_strategy_manager_wallet_invoke(
    accounts: AddStrategyManagerWalletAccounts<'_, '_>,
    args: AddStrategyManagerWalletIxArgs,
) -> ProgramResult {
    add_strategy_manager_wallet_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn add_strategy_manager_wallet_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddStrategyManagerWalletAccounts<'_, '_>,
    args: AddStrategyManagerWalletIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddStrategyManagerWalletKeys = accounts.into();
    let ix = add_strategy_manager_wallet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_strategy_manager_wallet_invoke_signed(
    accounts: AddStrategyManagerWalletAccounts<'_, '_>,
    args: AddStrategyManagerWalletIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_strategy_manager_wallet_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_strategy_manager_wallet_verify_account_keys(
    accounts: AddStrategyManagerWalletAccounts<'_, '_>,
    keys: AddStrategyManagerWalletKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_owner.key, keys.pool_owner),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_strategy_manager_wallet_verify_writable_privileges<'me, 'info>(
    accounts: AddStrategyManagerWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_owner, accounts.strategy_manager_wallet] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_strategy_manager_wallet_verify_signer_privileges<'me, 'info>(
    accounts: AddStrategyManagerWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_strategy_manager_wallet_verify_account_privileges<'me, 'info>(
    accounts: AddStrategyManagerWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_strategy_manager_wallet_verify_writable_privileges(accounts)?;
    add_strategy_manager_wallet_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const APPROVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ApproveManualStrategyManagerAccounts<'me, 'info> {
    pub huma_owner: &'me AccountInfo<'info>,
    pub pool_owner: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub manual_strategy_manager: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ApproveManualStrategyManagerKeys {
    pub huma_owner: Pubkey,
    pub pool_owner: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub proposal: Pubkey,
    pub manual_strategy_manager: Pubkey,
    pub system_program: Pubkey,
}
impl From<ApproveManualStrategyManagerAccounts<'_, '_>>
for ApproveManualStrategyManagerKeys {
    fn from(accounts: ApproveManualStrategyManagerAccounts) -> Self {
        Self {
            huma_owner: *accounts.huma_owner.key,
            pool_owner: *accounts.pool_owner.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            proposal: *accounts.proposal.key,
            manual_strategy_manager: *accounts.manual_strategy_manager.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ApproveManualStrategyManagerKeys>
for [AccountMeta; APPROVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: ApproveManualStrategyManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.huma_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manual_strategy_manager,
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
impl From<[Pubkey; APPROVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]>
for ApproveManualStrategyManagerKeys {
    fn from(pubkeys: [Pubkey; APPROVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            huma_owner: pubkeys[0],
            pool_owner: pubkeys[1],
            huma_config: pubkeys[2],
            pool_config: pubkeys[3],
            pool_state: pubkeys[4],
            proposal: pubkeys[5],
            manual_strategy_manager: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<ApproveManualStrategyManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; APPROVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ApproveManualStrategyManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.huma_owner.clone(),
            accounts.pool_owner.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.proposal.clone(),
            accounts.manual_strategy_manager.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; APPROVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]>
for ApproveManualStrategyManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; APPROVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            huma_owner: &arr[0],
            pool_owner: &arr[1],
            huma_config: &arr[2],
            pool_config: &arr[3],
            pool_state: &arr[4],
            proposal: &arr[5],
            manual_strategy_manager: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const APPROVE_MANUAL_STRATEGY_MANAGER_IX_DISCM: [u8; 8usize] = [
    250, 162, 53, 178, 184, 236, 106, 167,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveManualStrategyManagerIxData;
impl ApproveManualStrategyManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != APPROVE_MANUAL_STRATEGY_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&APPROVE_MANUAL_STRATEGY_MANAGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn approve_manual_strategy_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: ApproveManualStrategyManagerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; APPROVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ApproveManualStrategyManagerIxData.try_to_vec()?,
    })
}
pub fn approve_manual_strategy_manager_ix(
    keys: ApproveManualStrategyManagerKeys,
) -> std::io::Result<Instruction> {
    approve_manual_strategy_manager_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn approve_manual_strategy_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ApproveManualStrategyManagerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ApproveManualStrategyManagerKeys = accounts.into();
    let ix = approve_manual_strategy_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn approve_manual_strategy_manager_invoke(
    accounts: ApproveManualStrategyManagerAccounts<'_, '_>,
) -> ProgramResult {
    approve_manual_strategy_manager_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn approve_manual_strategy_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ApproveManualStrategyManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ApproveManualStrategyManagerKeys = accounts.into();
    let ix = approve_manual_strategy_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn approve_manual_strategy_manager_invoke_signed(
    accounts: ApproveManualStrategyManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    approve_manual_strategy_manager_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn approve_manual_strategy_manager_verify_account_keys(
    accounts: ApproveManualStrategyManagerAccounts<'_, '_>,
    keys: ApproveManualStrategyManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.huma_owner.key, keys.huma_owner),
        (*accounts.pool_owner.key, keys.pool_owner),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.manual_strategy_manager.key, keys.manual_strategy_manager),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn approve_manual_strategy_manager_verify_writable_privileges<'me, 'info>(
    accounts: ApproveManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.huma_owner,
        accounts.pool_owner,
        accounts.proposal,
        accounts.manual_strategy_manager,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn approve_manual_strategy_manager_verify_signer_privileges<'me, 'info>(
    accounts: ApproveManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.huma_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn approve_manual_strategy_manager_verify_account_privileges<'me, 'info>(
    accounts: ApproveManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    approve_manual_strategy_manager_verify_writable_privileges(accounts)?;
    approve_manual_strategy_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CancelManualStrategyManagerProposalAccounts<'me, 'info> {
    pub pool_owner: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelManualStrategyManagerProposalKeys {
    pub pool_owner: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub proposal: Pubkey,
}
impl From<CancelManualStrategyManagerProposalAccounts<'_, '_>>
for CancelManualStrategyManagerProposalKeys {
    fn from(accounts: CancelManualStrategyManagerProposalAccounts) -> Self {
        Self {
            pool_owner: *accounts.pool_owner.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            proposal: *accounts.proposal.key,
        }
    }
}
impl From<CancelManualStrategyManagerProposalKeys>
for [AccountMeta; CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelManualStrategyManagerProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_ACCOUNTS_LEN]>
for CancelManualStrategyManagerProposalKeys {
    fn from(
        pubkeys: [Pubkey; CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_owner: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            proposal: pubkeys[4],
        }
    }
}
impl<'info> From<CancelManualStrategyManagerProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelManualStrategyManagerProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_owner.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.proposal.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_ACCOUNTS_LEN],
> for CancelManualStrategyManagerProposalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_owner: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            proposal: &arr[4],
        }
    }
}
pub const CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_DISCM: [u8; 8usize] = [
    164, 185, 150, 19, 234, 253, 40, 69,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelManualStrategyManagerProposalIxData;
impl CancelManualStrategyManagerProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_manual_strategy_manager_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelManualStrategyManagerProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_MANUAL_STRATEGY_MANAGER_PROPOSAL_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelManualStrategyManagerProposalIxData.try_to_vec()?,
    })
}
pub fn cancel_manual_strategy_manager_proposal_ix(
    keys: CancelManualStrategyManagerProposalKeys,
) -> std::io::Result<Instruction> {
    cancel_manual_strategy_manager_proposal_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn cancel_manual_strategy_manager_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelManualStrategyManagerProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelManualStrategyManagerProposalKeys = accounts.into();
    let ix = cancel_manual_strategy_manager_proposal_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_manual_strategy_manager_proposal_invoke(
    accounts: CancelManualStrategyManagerProposalAccounts<'_, '_>,
) -> ProgramResult {
    cancel_manual_strategy_manager_proposal_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
    )
}
pub fn cancel_manual_strategy_manager_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelManualStrategyManagerProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelManualStrategyManagerProposalKeys = accounts.into();
    let ix = cancel_manual_strategy_manager_proposal_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_manual_strategy_manager_proposal_invoke_signed(
    accounts: CancelManualStrategyManagerProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_manual_strategy_manager_proposal_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cancel_manual_strategy_manager_proposal_verify_account_keys(
    accounts: CancelManualStrategyManagerProposalAccounts<'_, '_>,
    keys: CancelManualStrategyManagerProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_owner.key, keys.pool_owner),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.proposal.key, keys.proposal),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_manual_strategy_manager_proposal_verify_writable_privileges<'me, 'info>(
    accounts: CancelManualStrategyManagerProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_owner, accounts.proposal] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_manual_strategy_manager_proposal_verify_signer_privileges<'me, 'info>(
    accounts: CancelManualStrategyManagerProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_manual_strategy_manager_proposal_verify_account_privileges<'me, 'info>(
    accounts: CancelManualStrategyManagerProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_manual_strategy_manager_proposal_verify_writable_privileges(accounts)?;
    cancel_manual_strategy_manager_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CancelRedemptionRequestAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub redemption_request: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_mode_token: &'me AccountInfo<'info>,
    pub lender_mode_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelRedemptionRequestKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub redemption_request: Pubkey,
    pub lender_state: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_mode_token: Pubkey,
    pub lender_mode_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<CancelRedemptionRequestAccounts<'_, '_>> for CancelRedemptionRequestKeys {
    fn from(accounts: CancelRedemptionRequestAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            redemption_request: *accounts.redemption_request.key,
            lender_state: *accounts.lender_state.key,
            pool_authority: *accounts.pool_authority.key,
            pool_mode_token: *accounts.pool_mode_token.key,
            lender_mode_token: *accounts.lender_mode_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CancelRedemptionRequestKeys>
for [AccountMeta; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelRedemptionRequestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_mode_token,
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
impl From<[Pubkey; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]>
for CancelRedemptionRequestKeys {
    fn from(pubkeys: [Pubkey; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
            mode_mint: pubkeys[5],
            redemption_request: pubkeys[6],
            lender_state: pubkeys[7],
            pool_authority: pubkeys[8],
            pool_mode_token: pubkeys[9],
            lender_mode_token: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<CancelRedemptionRequestAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelRedemptionRequestAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.redemption_request.clone(),
            accounts.lender_state.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_mode_token.clone(),
            accounts.lender_mode_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]>
for CancelRedemptionRequestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
            mode_mint: &arr[5],
            redemption_request: &arr[6],
            lender_state: &arr[7],
            pool_authority: &arr[8],
            pool_mode_token: &arr[9],
            lender_mode_token: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const CANCEL_REDEMPTION_REQUEST_IX_DISCM: [u8; 8usize] = [
    77, 155, 4, 179, 114, 233, 162, 45,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelRedemptionRequestIxArgs {
    pub shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelRedemptionRequestIxData(pub CancelRedemptionRequestIxArgs);
impl From<CancelRedemptionRequestIxArgs> for CancelRedemptionRequestIxData {
    fn from(args: CancelRedemptionRequestIxArgs) -> Self {
        Self(args)
    }
}
impl CancelRedemptionRequestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_REDEMPTION_REQUEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CancelRedemptionRequestIxArgs {
                shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_REDEMPTION_REQUEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_redemption_request_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelRedemptionRequestKeys,
    args: CancelRedemptionRequestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelRedemptionRequestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_redemption_request_ix(
    keys: CancelRedemptionRequestKeys,
    args: CancelRedemptionRequestIxArgs,
) -> std::io::Result<Instruction> {
    cancel_redemption_request_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn cancel_redemption_request_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
    args: CancelRedemptionRequestIxArgs,
) -> ProgramResult {
    let keys: CancelRedemptionRequestKeys = accounts.into();
    let ix = cancel_redemption_request_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_redemption_request_invoke(
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
    args: CancelRedemptionRequestIxArgs,
) -> ProgramResult {
    cancel_redemption_request_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn cancel_redemption_request_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
    args: CancelRedemptionRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelRedemptionRequestKeys = accounts.into();
    let ix = cancel_redemption_request_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_redemption_request_invoke_signed(
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
    args: CancelRedemptionRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_redemption_request_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_redemption_request_verify_account_keys(
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
    keys: CancelRedemptionRequestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.redemption_request.key, keys.redemption_request),
        (*accounts.lender_state.key, keys.lender_state),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_mode_token.key, keys.pool_mode_token),
        (*accounts.lender_mode_token.key, keys.lender_mode_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_redemption_request_verify_writable_privileges<'me, 'info>(
    accounts: CancelRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.redemption_request,
        accounts.lender_state,
        accounts.pool_mode_token,
        accounts.lender_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_redemption_request_verify_signer_privileges<'me, 'info>(
    accounts: CancelRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_redemption_request_verify_account_privileges<'me, 'info>(
    accounts: CancelRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_redemption_request_verify_writable_privileges(accounts)?;
    cancel_redemption_request_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_POOL_OWNER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ChangePoolOwnerAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangePoolOwnerKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<ChangePoolOwnerAccounts<'_, '_>> for ChangePoolOwnerKeys {
    fn from(accounts: ChangePoolOwnerAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<ChangePoolOwnerKeys> for [AccountMeta; CHANGE_POOL_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangePoolOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_POOL_OWNER_IX_ACCOUNTS_LEN]> for ChangePoolOwnerKeys {
    fn from(pubkeys: [Pubkey; CHANGE_POOL_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
        }
    }
}
impl<'info> From<ChangePoolOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_POOL_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangePoolOwnerAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_POOL_OWNER_IX_ACCOUNTS_LEN]>
for ChangePoolOwnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_POOL_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
        }
    }
}
pub const CHANGE_POOL_OWNER_IX_DISCM: [u8; 8usize] = [
    169, 55, 183, 24, 152, 180, 167, 11,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangePoolOwnerIxArgs {
    pub new_owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangePoolOwnerIxData(pub ChangePoolOwnerIxArgs);
impl From<ChangePoolOwnerIxArgs> for ChangePoolOwnerIxData {
    fn from(args: ChangePoolOwnerIxArgs) -> Self {
        Self(args)
    }
}
impl ChangePoolOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_POOL_OWNER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ChangePoolOwnerIxArgs { new_owner }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_POOL_OWNER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_owner, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_pool_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangePoolOwnerKeys,
    args: ChangePoolOwnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_POOL_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangePoolOwnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_pool_owner_ix(
    keys: ChangePoolOwnerKeys,
    args: ChangePoolOwnerIxArgs,
) -> std::io::Result<Instruction> {
    change_pool_owner_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn change_pool_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangePoolOwnerAccounts<'_, '_>,
    args: ChangePoolOwnerIxArgs,
) -> ProgramResult {
    let keys: ChangePoolOwnerKeys = accounts.into();
    let ix = change_pool_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_pool_owner_invoke(
    accounts: ChangePoolOwnerAccounts<'_, '_>,
    args: ChangePoolOwnerIxArgs,
) -> ProgramResult {
    change_pool_owner_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn change_pool_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangePoolOwnerAccounts<'_, '_>,
    args: ChangePoolOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangePoolOwnerKeys = accounts.into();
    let ix = change_pool_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_pool_owner_invoke_signed(
    accounts: ChangePoolOwnerAccounts<'_, '_>,
    args: ChangePoolOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_pool_owner_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn change_pool_owner_verify_account_keys(
    accounts: ChangePoolOwnerAccounts<'_, '_>,
    keys: ChangePoolOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_pool_owner_verify_writable_privileges<'me, 'info>(
    accounts: ChangePoolOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_pool_owner_verify_signer_privileges<'me, 'info>(
    accounts: ChangePoolOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_pool_owner_verify_account_privileges<'me, 'info>(
    accounts: ChangePoolOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_pool_owner_verify_writable_privileges(accounts)?;
    change_pool_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ClearInstantWithdrawalLiquiditySourceAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClearInstantWithdrawalLiquiditySourceKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
}
impl From<ClearInstantWithdrawalLiquiditySourceAccounts<'_, '_>>
for ClearInstantWithdrawalLiquiditySourceKeys {
    fn from(accounts: ClearInstantWithdrawalLiquiditySourceAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
        }
    }
}
impl From<ClearInstantWithdrawalLiquiditySourceKeys>
for [AccountMeta; CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClearInstantWithdrawalLiquiditySourceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN]>
for ClearInstantWithdrawalLiquiditySourceKeys {
    fn from(
        pubkeys: [Pubkey; CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            strategy_manager_wallet: pubkeys[4],
        }
    }
}
impl<'info> From<ClearInstantWithdrawalLiquiditySourceAccounts<'_, 'info>>
for [AccountInfo<'info>; CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.strategy_manager_wallet.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN],
> for ClearInstantWithdrawalLiquiditySourceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            strategy_manager_wallet: &arr[4],
        }
    }
}
pub const CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM: [u8; 8usize] = [
    53, 151, 188, 231, 236, 184, 141, 85,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClearInstantWithdrawalLiquiditySourceIxData;
impl ClearInstantWithdrawalLiquiditySourceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn clear_instant_withdrawal_liquidity_source_ix_with_program_id(
    program_id: Pubkey,
    keys: ClearInstantWithdrawalLiquiditySourceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLEAR_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClearInstantWithdrawalLiquiditySourceIxData.try_to_vec()?,
    })
}
pub fn clear_instant_withdrawal_liquidity_source_ix(
    keys: ClearInstantWithdrawalLiquiditySourceKeys,
) -> std::io::Result<Instruction> {
    clear_instant_withdrawal_liquidity_source_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn clear_instant_withdrawal_liquidity_source_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClearInstantWithdrawalLiquiditySourceKeys = accounts.into();
    let ix = clear_instant_withdrawal_liquidity_source_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn clear_instant_withdrawal_liquidity_source_invoke(
    accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
) -> ProgramResult {
    clear_instant_withdrawal_liquidity_source_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
    )
}
pub fn clear_instant_withdrawal_liquidity_source_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClearInstantWithdrawalLiquiditySourceKeys = accounts.into();
    let ix = clear_instant_withdrawal_liquidity_source_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn clear_instant_withdrawal_liquidity_source_invoke_signed(
    accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    clear_instant_withdrawal_liquidity_source_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn clear_instant_withdrawal_liquidity_source_verify_account_keys(
    accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
    keys: ClearInstantWithdrawalLiquiditySourceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn clear_instant_withdrawal_liquidity_source_verify_writable_privileges<'me, 'info>(
    accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn clear_instant_withdrawal_liquidity_source_verify_signer_privileges<'me, 'info>(
    accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn clear_instant_withdrawal_liquidity_source_verify_account_privileges<'me, 'info>(
    accounts: ClearInstantWithdrawalLiquiditySourceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    clear_instant_withdrawal_liquidity_source_verify_writable_privileges(accounts)?;
    clear_instant_withdrawal_liquidity_source_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_LENDER_ACCOUNTS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CloseLenderAccountsAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub lender: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseLenderAccountsKeys {
    pub payer: Pubkey,
    pub lender: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub lender_state: Pubkey,
}
impl From<CloseLenderAccountsAccounts<'_, '_>> for CloseLenderAccountsKeys {
    fn from(accounts: CloseLenderAccountsAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            lender: *accounts.lender.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            lender_state: *accounts.lender_state.key,
        }
    }
}
impl From<CloseLenderAccountsKeys>
for [AccountMeta; CLOSE_LENDER_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseLenderAccountsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_LENDER_ACCOUNTS_IX_ACCOUNTS_LEN]> for CloseLenderAccountsKeys {
    fn from(pubkeys: [Pubkey; CLOSE_LENDER_ACCOUNTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            lender: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
            lender_state: pubkeys[5],
        }
    }
}
impl<'info> From<CloseLenderAccountsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_LENDER_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseLenderAccountsAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.lender.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.lender_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_LENDER_ACCOUNTS_IX_ACCOUNTS_LEN]>
for CloseLenderAccountsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_LENDER_ACCOUNTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            lender: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
            lender_state: &arr[5],
        }
    }
}
pub const CLOSE_LENDER_ACCOUNTS_IX_DISCM: [u8; 8usize] = [
    222, 126, 36, 205, 193, 173, 194, 224,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseLenderAccountsIxData;
impl CloseLenderAccountsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_LENDER_ACCOUNTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_LENDER_ACCOUNTS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_lender_accounts_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseLenderAccountsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_LENDER_ACCOUNTS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseLenderAccountsIxData.try_to_vec()?,
    })
}
pub fn close_lender_accounts_ix(
    keys: CloseLenderAccountsKeys,
) -> std::io::Result<Instruction> {
    close_lender_accounts_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn close_lender_accounts_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseLenderAccountsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseLenderAccountsKeys = accounts.into();
    let ix = close_lender_accounts_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_lender_accounts_invoke(
    accounts: CloseLenderAccountsAccounts<'_, '_>,
) -> ProgramResult {
    close_lender_accounts_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn close_lender_accounts_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseLenderAccountsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseLenderAccountsKeys = accounts.into();
    let ix = close_lender_accounts_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_lender_accounts_invoke_signed(
    accounts: CloseLenderAccountsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_lender_accounts_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn close_lender_accounts_verify_account_keys(
    accounts: CloseLenderAccountsAccounts<'_, '_>,
    keys: CloseLenderAccountsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.lender.key, keys.lender),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.lender_state.key, keys.lender_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_lender_accounts_verify_writable_privileges<'me, 'info>(
    accounts: CloseLenderAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.lender_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_lender_accounts_verify_signer_privileges<'me, 'info>(
    accounts: CloseLenderAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.lender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_lender_accounts_verify_account_privileges<'me, 'info>(
    accounts: CloseLenderAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_lender_accounts_verify_writable_privileges(accounts)?;
    close_lender_accounts_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ClosePoolAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePoolKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClosePoolAccounts<'_, '_>> for ClosePoolKeys {
    fn from(accounts: ClosePoolAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClosePoolKeys> for [AccountMeta; CLOSE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; CLOSE_POOL_IX_ACCOUNTS_LEN]> for ClosePoolKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            pool_authority: pubkeys[4],
            underlying_mint: pubkeys[5],
            pool_underlying_token: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<ClosePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POOL_IX_ACCOUNTS_LEN]>
for ClosePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            pool_authority: &arr[4],
            underlying_mint: &arr[5],
            pool_underlying_token: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const CLOSE_POOL_IX_DISCM: [u8; 8usize] = [140, 189, 209, 23, 239, 62, 239, 11];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePoolIxData;
impl ClosePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePoolIxData.try_to_vec()?,
    })
}
pub fn close_pool_ix(keys: ClosePoolKeys) -> std::io::Result<Instruction> {
    close_pool_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn close_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePoolKeys = accounts.into();
    let ix = close_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_pool_invoke(accounts: ClosePoolAccounts<'_, '_>) -> ProgramResult {
    close_pool_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn close_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePoolKeys = accounts.into();
    let ix = close_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_pool_invoke_signed(
    accounts: ClosePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_pool_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn close_pool_verify_account_keys(
    accounts: ClosePoolAccounts<'_, '_>,
    keys: ClosePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_pool_verify_writable_privileges<'me, 'info>(
    accounts: ClosePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_pool_verify_signer_privileges<'me, 'info>(
    accounts: ClosePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_pool_verify_account_privileges<'me, 'info>(
    accounts: ClosePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_pool_verify_writable_privileges(accounts)?;
    close_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CompleteLiquidityPaybackAsyncAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub async_deployment_config: &'me AccountInfo<'info>,
    pub async_deployment_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CompleteLiquidityPaybackAsyncKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub async_deployment_config: Pubkey,
    pub async_deployment_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<CompleteLiquidityPaybackAsyncAccounts<'_, '_>>
for CompleteLiquidityPaybackAsyncKeys {
    fn from(accounts: CompleteLiquidityPaybackAsyncAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            async_deployment_config: *accounts.async_deployment_config.key,
            async_deployment_state: *accounts.async_deployment_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CompleteLiquidityPaybackAsyncKeys>
for [AccountMeta; COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN] {
    fn from(keys: CompleteLiquidityPaybackAsyncKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.async_deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.async_deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN]>
for CompleteLiquidityPaybackAsyncKeys {
    fn from(
        pubkeys: [Pubkey; COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            async_deployment_config: pubkeys[4],
            async_deployment_state: pubkeys[5],
            strategy_manager_wallet: pubkeys[6],
            pool_authority: pubkeys[7],
            underlying_mint: pubkeys[8],
            pool_underlying_token: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<CompleteLiquidityPaybackAsyncAccounts<'_, 'info>>
for [AccountInfo<'info>; COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN] {
    fn from(accounts: CompleteLiquidityPaybackAsyncAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.async_deployment_config.clone(),
            accounts.async_deployment_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN]>
for CompleteLiquidityPaybackAsyncAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            async_deployment_config: &arr[4],
            async_deployment_state: &arr[5],
            strategy_manager_wallet: &arr[6],
            pool_authority: &arr[7],
            underlying_mint: &arr[8],
            pool_underlying_token: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM: [u8; 8usize] = [
    233, 72, 152, 253, 73, 201, 108, 242,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompleteLiquidityPaybackAsyncIxArgs {
    pub strategy_type: AsyncDeploymentStrategyType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompleteLiquidityPaybackAsyncIxData(pub CompleteLiquidityPaybackAsyncIxArgs);
impl From<CompleteLiquidityPaybackAsyncIxArgs> for CompleteLiquidityPaybackAsyncIxData {
    fn from(args: CompleteLiquidityPaybackAsyncIxArgs) -> Self {
        Self(args)
    }
}
impl CompleteLiquidityPaybackAsyncIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(CompleteLiquidityPaybackAsyncIxArgs {
                strategy_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn complete_liquidity_payback_async_ix_with_program_id(
    program_id: Pubkey,
    keys: CompleteLiquidityPaybackAsyncKeys,
    args: CompleteLiquidityPaybackAsyncIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COMPLETE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CompleteLiquidityPaybackAsyncIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn complete_liquidity_payback_async_ix(
    keys: CompleteLiquidityPaybackAsyncKeys,
    args: CompleteLiquidityPaybackAsyncIxArgs,
) -> std::io::Result<Instruction> {
    complete_liquidity_payback_async_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn complete_liquidity_payback_async_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CompleteLiquidityPaybackAsyncAccounts<'_, '_>,
    args: CompleteLiquidityPaybackAsyncIxArgs,
) -> ProgramResult {
    let keys: CompleteLiquidityPaybackAsyncKeys = accounts.into();
    let ix = complete_liquidity_payback_async_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn complete_liquidity_payback_async_invoke(
    accounts: CompleteLiquidityPaybackAsyncAccounts<'_, '_>,
    args: CompleteLiquidityPaybackAsyncIxArgs,
) -> ProgramResult {
    complete_liquidity_payback_async_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn complete_liquidity_payback_async_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CompleteLiquidityPaybackAsyncAccounts<'_, '_>,
    args: CompleteLiquidityPaybackAsyncIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CompleteLiquidityPaybackAsyncKeys = accounts.into();
    let ix = complete_liquidity_payback_async_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn complete_liquidity_payback_async_invoke_signed(
    accounts: CompleteLiquidityPaybackAsyncAccounts<'_, '_>,
    args: CompleteLiquidityPaybackAsyncIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    complete_liquidity_payback_async_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn complete_liquidity_payback_async_verify_account_keys(
    accounts: CompleteLiquidityPaybackAsyncAccounts<'_, '_>,
    keys: CompleteLiquidityPaybackAsyncKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.async_deployment_config.key, keys.async_deployment_config),
        (*accounts.async_deployment_state.key, keys.async_deployment_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn complete_liquidity_payback_async_verify_writable_privileges<'me, 'info>(
    accounts: CompleteLiquidityPaybackAsyncAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.async_deployment_state,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn complete_liquidity_payback_async_verify_signer_privileges<'me, 'info>(
    accounts: CompleteLiquidityPaybackAsyncAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn complete_liquidity_payback_async_verify_account_privileges<'me, 'info>(
    accounts: CompleteLiquidityPaybackAsyncAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    complete_liquidity_payback_async_verify_writable_privileges(accounts)?;
    complete_liquidity_payback_async_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_LENDER_ACCOUNTS_V2_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CreateLenderAccountsV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub lender: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
    pub lender_mode_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateLenderAccountsV2Keys {
    pub payer: Pubkey,
    pub lender: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub lender_state: Pubkey,
    pub lender_mode_token: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateLenderAccountsV2Accounts<'_, '_>> for CreateLenderAccountsV2Keys {
    fn from(accounts: CreateLenderAccountsV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            lender: *accounts.lender.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            lender_state: *accounts.lender_state.key,
            lender_mode_token: *accounts.lender_mode_token.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateLenderAccountsV2Keys>
for [AccountMeta; CREATE_LENDER_ACCOUNTS_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateLenderAccountsV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_mode_token,
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
impl From<[Pubkey; CREATE_LENDER_ACCOUNTS_V2_IX_ACCOUNTS_LEN]>
for CreateLenderAccountsV2Keys {
    fn from(pubkeys: [Pubkey; CREATE_LENDER_ACCOUNTS_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            lender: pubkeys[1],
            huma_config: pubkeys[2],
            pool_config: pubkeys[3],
            pool_state: pubkeys[4],
            mode_config: pubkeys[5],
            mode_mint: pubkeys[6],
            lender_state: pubkeys[7],
            lender_mode_token: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<CreateLenderAccountsV2Accounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_LENDER_ACCOUNTS_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateLenderAccountsV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.lender.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.lender_state.clone(),
            accounts.lender_mode_token.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_LENDER_ACCOUNTS_V2_IX_ACCOUNTS_LEN]>
for CreateLenderAccountsV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_LENDER_ACCOUNTS_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            lender: &arr[1],
            huma_config: &arr[2],
            pool_config: &arr[3],
            pool_state: &arr[4],
            mode_config: &arr[5],
            mode_mint: &arr[6],
            lender_state: &arr[7],
            lender_mode_token: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const CREATE_LENDER_ACCOUNTS_V2_IX_DISCM: [u8; 8usize] = [
    203, 52, 185, 231, 192, 74, 121, 108,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateLenderAccountsV2IxData;
impl CreateLenderAccountsV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_LENDER_ACCOUNTS_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_LENDER_ACCOUNTS_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_lender_accounts_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateLenderAccountsV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_LENDER_ACCOUNTS_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateLenderAccountsV2IxData.try_to_vec()?,
    })
}
pub fn create_lender_accounts_v2_ix(
    keys: CreateLenderAccountsV2Keys,
) -> std::io::Result<Instruction> {
    create_lender_accounts_v2_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn create_lender_accounts_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateLenderAccountsV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateLenderAccountsV2Keys = accounts.into();
    let ix = create_lender_accounts_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_lender_accounts_v2_invoke(
    accounts: CreateLenderAccountsV2Accounts<'_, '_>,
) -> ProgramResult {
    create_lender_accounts_v2_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn create_lender_accounts_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateLenderAccountsV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateLenderAccountsV2Keys = accounts.into();
    let ix = create_lender_accounts_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_lender_accounts_v2_invoke_signed(
    accounts: CreateLenderAccountsV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_lender_accounts_v2_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_lender_accounts_v2_verify_account_keys(
    accounts: CreateLenderAccountsV2Accounts<'_, '_>,
    keys: CreateLenderAccountsV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.lender.key, keys.lender),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.lender_state.key, keys.lender_state),
        (*accounts.lender_mode_token.key, keys.lender_mode_token),
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
pub fn create_lender_accounts_v2_verify_writable_privileges<'me, 'info>(
    accounts: CreateLenderAccountsV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.lender_state,
        accounts.lender_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_lender_accounts_v2_verify_signer_privileges<'me, 'info>(
    accounts: CreateLenderAccountsV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_lender_accounts_v2_verify_account_privileges<'me, 'info>(
    accounts: CreateLenderAccountsV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_lender_accounts_v2_verify_writable_privileges(accounts)?;
    create_lender_accounts_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub liquidity_asset: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub owner: Pubkey,
    pub huma_config: Pubkey,
    pub liquidity_asset: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            huma_config: *accounts.huma_config.key,
            liquidity_asset: *accounts.liquidity_asset.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_asset,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            huma_config: pubkeys[1],
            liquidity_asset: pubkeys[2],
            pool_config: pubkeys[3],
            pool_state: pubkeys[4],
            pool_authority: pubkeys[5],
            underlying_mint: pubkeys[6],
            pool_underlying_token: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            system_program: pubkeys[10],
        }
    }
}
impl<'info> From<CreatePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.huma_config.clone(),
            accounts.liquidity_asset.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
for CreatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            huma_config: &arr[1],
            liquidity_asset: &arr[2],
            pool_config: &arr[3],
            pool_state: &arr[4],
            pool_authority: &arr[5],
            underlying_mint: &arr[6],
            pool_underlying_token: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            system_program: &arr[10],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: [u8; 8usize] = [233, 146, 209, 142, 207, 104, 64, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolIxArgs {
    pub pool_id: Pubkey,
    pub pool_name: String,
    pub pool_owner_treasury: Pubkey,
    pub loss_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolIxData(pub CreatePoolIxArgs);
impl From<CreatePoolIxArgs> for CreatePoolIxData {
    fn from(args: CreatePoolIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_name: String = crate::borsh_de_or_default(&mut reader)?;
        let pool_owner_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreatePoolIxArgs {
                pool_id,
                pool_name,
                pool_owner_treasury,
                loss_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pool_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pool_owner_treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.loss_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_pool_ix(
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    create_pool_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn create_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_pool_invoke(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    create_pool_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn create_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_pool_invoke_signed(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_pool_verify_account_keys(
    accounts: CreatePoolAccounts<'_, '_>,
    keys: CreatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.liquidity_asset.key, keys.liquidity_asset),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
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
pub fn create_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.pool_config,
        accounts.pool_state,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_pool_verify_account_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_pool_verify_writable_privileges(accounts)?;
    create_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECLARE_LOSS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct DeclareLossAccounts<'me, 'info> {
    pub loss_authority: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeclareLossKeys {
    pub loss_authority: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<DeclareLossAccounts<'_, '_>> for DeclareLossKeys {
    fn from(accounts: DeclareLossAccounts) -> Self {
        Self {
            loss_authority: *accounts.loss_authority.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DeclareLossKeys> for [AccountMeta; DECLARE_LOSS_IX_ACCOUNTS_LEN] {
    fn from(keys: DeclareLossKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.loss_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; DECLARE_LOSS_IX_ACCOUNTS_LEN]> for DeclareLossKeys {
    fn from(pubkeys: [Pubkey; DECLARE_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            loss_authority: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            pool_authority: pubkeys[4],
            underlying_mint: pubkeys[5],
            pool_underlying_token: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<DeclareLossAccounts<'_, 'info>>
for [AccountInfo<'info>; DECLARE_LOSS_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeclareLossAccounts<'_, 'info>) -> Self {
        [
            accounts.loss_authority.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECLARE_LOSS_IX_ACCOUNTS_LEN]>
for DeclareLossAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECLARE_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            loss_authority: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            pool_authority: &arr[4],
            underlying_mint: &arr[5],
            pool_underlying_token: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const DECLARE_LOSS_IX_DISCM: [u8; 8usize] = [51, 202, 154, 130, 65, 132, 242, 188];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeclareLossIxArgs {
    pub loss: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeclareLossIxData(pub DeclareLossIxArgs);
impl From<DeclareLossIxArgs> for DeclareLossIxData {
    fn from(args: DeclareLossIxArgs) -> Self {
        Self(args)
    }
}
impl DeclareLossIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECLARE_LOSS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let loss: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(DeclareLossIxArgs { loss }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECLARE_LOSS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.loss, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn declare_loss_ix_with_program_id(
    program_id: Pubkey,
    keys: DeclareLossKeys,
    args: DeclareLossIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECLARE_LOSS_IX_ACCOUNTS_LEN] = keys.into();
    let data: DeclareLossIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn declare_loss_ix(
    keys: DeclareLossKeys,
    args: DeclareLossIxArgs,
) -> std::io::Result<Instruction> {
    declare_loss_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn declare_loss_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeclareLossAccounts<'_, '_>,
    args: DeclareLossIxArgs,
) -> ProgramResult {
    let keys: DeclareLossKeys = accounts.into();
    let ix = declare_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn declare_loss_invoke(
    accounts: DeclareLossAccounts<'_, '_>,
    args: DeclareLossIxArgs,
) -> ProgramResult {
    declare_loss_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn declare_loss_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeclareLossAccounts<'_, '_>,
    args: DeclareLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeclareLossKeys = accounts.into();
    let ix = declare_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn declare_loss_invoke_signed(
    accounts: DeclareLossAccounts<'_, '_>,
    args: DeclareLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    declare_loss_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn declare_loss_verify_account_keys(
    accounts: DeclareLossAccounts<'_, '_>,
    keys: DeclareLossKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.loss_authority.key, keys.loss_authority),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn declare_loss_verify_writable_privileges<'me, 'info>(
    accounts: DeclareLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn declare_loss_verify_signer_privileges<'me, 'info>(
    accounts: DeclareLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.loss_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn declare_loss_verify_account_privileges<'me, 'info>(
    accounts: DeclareLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    declare_loss_verify_writable_privileges(accounts)?;
    declare_loss_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPLOY_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct DeployLiquidityAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub deployment_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeployLiquidityKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub deployment_config: Pubkey,
    pub deployment_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<DeployLiquidityAccounts<'_, '_>> for DeployLiquidityKeys {
    fn from(accounts: DeployLiquidityAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            deployment_config: *accounts.deployment_config.key,
            deployment_state: *accounts.deployment_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DeployLiquidityKeys> for [AccountMeta; DEPLOY_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: DeployLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; DEPLOY_LIQUIDITY_IX_ACCOUNTS_LEN]> for DeployLiquidityKeys {
    fn from(pubkeys: [Pubkey; DEPLOY_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            deployment_config: pubkeys[4],
            deployment_state: pubkeys[5],
            strategy_manager_wallet: pubkeys[6],
            pool_authority: pubkeys[7],
            underlying_mint: pubkeys[8],
            pool_underlying_token: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<DeployLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPLOY_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeployLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.deployment_config.clone(),
            accounts.deployment_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPLOY_LIQUIDITY_IX_ACCOUNTS_LEN]>
for DeployLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPLOY_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            deployment_config: &arr[4],
            deployment_state: &arr[5],
            strategy_manager_wallet: &arr[6],
            pool_authority: &arr[7],
            underlying_mint: &arr[8],
            pool_underlying_token: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const DEPLOY_LIQUIDITY_IX_DISCM: [u8; 8usize] = [49, 35, 101, 76, 93, 4, 31, 53];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeployLiquidityIxArgs {
    pub strategy_type: DeploymentStrategyType,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeployLiquidityIxData(pub DeployLiquidityIxArgs);
impl From<DeployLiquidityIxArgs> for DeployLiquidityIxData {
    fn from(args: DeployLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl DeployLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOY_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DeployLiquidityIxArgs {
                strategy_type,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOY_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deploy_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: DeployLiquidityKeys,
    args: DeployLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPLOY_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DeployLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deploy_liquidity_ix(
    keys: DeployLiquidityKeys,
    args: DeployLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    deploy_liquidity_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn deploy_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeployLiquidityAccounts<'_, '_>,
    args: DeployLiquidityIxArgs,
) -> ProgramResult {
    let keys: DeployLiquidityKeys = accounts.into();
    let ix = deploy_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deploy_liquidity_invoke(
    accounts: DeployLiquidityAccounts<'_, '_>,
    args: DeployLiquidityIxArgs,
) -> ProgramResult {
    deploy_liquidity_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn deploy_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeployLiquidityAccounts<'_, '_>,
    args: DeployLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeployLiquidityKeys = accounts.into();
    let ix = deploy_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deploy_liquidity_invoke_signed(
    accounts: DeployLiquidityAccounts<'_, '_>,
    args: DeployLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deploy_liquidity_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deploy_liquidity_verify_account_keys(
    accounts: DeployLiquidityAccounts<'_, '_>,
    keys: DeployLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.deployment_state.key, keys.deployment_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: DeployLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.wallet,
        accounts.pool_state,
        accounts.deployment_state,
        accounts.pool_authority,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: DeployLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_verify_account_privileges<'me, 'info>(
    accounts: DeployLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deploy_liquidity_verify_writable_privileges(accounts)?;
    deploy_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPLOY_LIQUIDITY_ASYNC_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct DeployLiquidityAsyncAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub async_deployment_config: &'me AccountInfo<'info>,
    pub async_deployment_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeployLiquidityAsyncKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub async_deployment_config: Pubkey,
    pub async_deployment_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<DeployLiquidityAsyncAccounts<'_, '_>> for DeployLiquidityAsyncKeys {
    fn from(accounts: DeployLiquidityAsyncAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            async_deployment_config: *accounts.async_deployment_config.key,
            async_deployment_state: *accounts.async_deployment_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DeployLiquidityAsyncKeys>
for [AccountMeta; DEPLOY_LIQUIDITY_ASYNC_IX_ACCOUNTS_LEN] {
    fn from(keys: DeployLiquidityAsyncKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.async_deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.async_deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; DEPLOY_LIQUIDITY_ASYNC_IX_ACCOUNTS_LEN]>
for DeployLiquidityAsyncKeys {
    fn from(pubkeys: [Pubkey; DEPLOY_LIQUIDITY_ASYNC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            async_deployment_config: pubkeys[4],
            async_deployment_state: pubkeys[5],
            strategy_manager_wallet: pubkeys[6],
            pool_authority: pubkeys[7],
            underlying_mint: pubkeys[8],
            pool_underlying_token: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<DeployLiquidityAsyncAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPLOY_LIQUIDITY_ASYNC_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeployLiquidityAsyncAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.async_deployment_config.clone(),
            accounts.async_deployment_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPLOY_LIQUIDITY_ASYNC_IX_ACCOUNTS_LEN]>
for DeployLiquidityAsyncAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPLOY_LIQUIDITY_ASYNC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            async_deployment_config: &arr[4],
            async_deployment_state: &arr[5],
            strategy_manager_wallet: &arr[6],
            pool_authority: &arr[7],
            underlying_mint: &arr[8],
            pool_underlying_token: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const DEPLOY_LIQUIDITY_ASYNC_IX_DISCM: [u8; 8usize] = [
    44, 179, 47, 203, 177, 83, 181, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeployLiquidityAsyncIxArgs {
    pub strategy_type: AsyncDeploymentStrategyType,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeployLiquidityAsyncIxData(pub DeployLiquidityAsyncIxArgs);
impl From<DeployLiquidityAsyncIxArgs> for DeployLiquidityAsyncIxData {
    fn from(args: DeployLiquidityAsyncIxArgs) -> Self {
        Self(args)
    }
}
impl DeployLiquidityAsyncIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOY_LIQUIDITY_ASYNC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DeployLiquidityAsyncIxArgs {
                strategy_type,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOY_LIQUIDITY_ASYNC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deploy_liquidity_async_ix_with_program_id(
    program_id: Pubkey,
    keys: DeployLiquidityAsyncKeys,
    args: DeployLiquidityAsyncIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPLOY_LIQUIDITY_ASYNC_IX_ACCOUNTS_LEN] = keys.into();
    let data: DeployLiquidityAsyncIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deploy_liquidity_async_ix(
    keys: DeployLiquidityAsyncKeys,
    args: DeployLiquidityAsyncIxArgs,
) -> std::io::Result<Instruction> {
    deploy_liquidity_async_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn deploy_liquidity_async_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeployLiquidityAsyncAccounts<'_, '_>,
    args: DeployLiquidityAsyncIxArgs,
) -> ProgramResult {
    let keys: DeployLiquidityAsyncKeys = accounts.into();
    let ix = deploy_liquidity_async_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deploy_liquidity_async_invoke(
    accounts: DeployLiquidityAsyncAccounts<'_, '_>,
    args: DeployLiquidityAsyncIxArgs,
) -> ProgramResult {
    deploy_liquidity_async_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn deploy_liquidity_async_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeployLiquidityAsyncAccounts<'_, '_>,
    args: DeployLiquidityAsyncIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeployLiquidityAsyncKeys = accounts.into();
    let ix = deploy_liquidity_async_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deploy_liquidity_async_invoke_signed(
    accounts: DeployLiquidityAsyncAccounts<'_, '_>,
    args: DeployLiquidityAsyncIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deploy_liquidity_async_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deploy_liquidity_async_verify_account_keys(
    accounts: DeployLiquidityAsyncAccounts<'_, '_>,
    keys: DeployLiquidityAsyncKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.async_deployment_config.key, keys.async_deployment_config),
        (*accounts.async_deployment_state.key, keys.async_deployment_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_async_verify_writable_privileges<'me, 'info>(
    accounts: DeployLiquidityAsyncAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.async_deployment_state,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_async_verify_signer_privileges<'me, 'info>(
    accounts: DeployLiquidityAsyncAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_async_verify_account_privileges<'me, 'info>(
    accounts: DeployLiquidityAsyncAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deploy_liquidity_async_verify_writable_privileges(accounts)?;
    deploy_liquidity_async_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPLOY_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct DeployLiquidityManuallyAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub deployment_state: &'me AccountInfo<'info>,
    pub manual_strategy_manager: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeployLiquidityManuallyKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub deployment_config: Pubkey,
    pub deployment_state: Pubkey,
    pub manual_strategy_manager: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<DeployLiquidityManuallyAccounts<'_, '_>> for DeployLiquidityManuallyKeys {
    fn from(accounts: DeployLiquidityManuallyAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            deployment_config: *accounts.deployment_config.key,
            deployment_state: *accounts.deployment_state.key,
            manual_strategy_manager: *accounts.manual_strategy_manager.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DeployLiquidityManuallyKeys>
for [AccountMeta; DEPLOY_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN] {
    fn from(keys: DeployLiquidityManuallyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manual_strategy_manager,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; DEPLOY_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN]>
for DeployLiquidityManuallyKeys {
    fn from(pubkeys: [Pubkey; DEPLOY_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            deployment_config: pubkeys[4],
            deployment_state: pubkeys[5],
            manual_strategy_manager: pubkeys[6],
            pool_authority: pubkeys[7],
            underlying_mint: pubkeys[8],
            pool_underlying_token: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<DeployLiquidityManuallyAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPLOY_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeployLiquidityManuallyAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.deployment_config.clone(),
            accounts.deployment_state.clone(),
            accounts.manual_strategy_manager.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPLOY_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN]>
for DeployLiquidityManuallyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPLOY_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            deployment_config: &arr[4],
            deployment_state: &arr[5],
            manual_strategy_manager: &arr[6],
            pool_authority: &arr[7],
            underlying_mint: &arr[8],
            pool_underlying_token: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const DEPLOY_LIQUIDITY_MANUALLY_IX_DISCM: [u8; 8usize] = [
    74, 157, 114, 153, 25, 5, 219, 43,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeployLiquidityManuallyIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeployLiquidityManuallyIxData(pub DeployLiquidityManuallyIxArgs);
impl From<DeployLiquidityManuallyIxArgs> for DeployLiquidityManuallyIxData {
    fn from(args: DeployLiquidityManuallyIxArgs) -> Self {
        Self(args)
    }
}
impl DeployLiquidityManuallyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOY_LIQUIDITY_MANUALLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DeployLiquidityManuallyIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOY_LIQUIDITY_MANUALLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deploy_liquidity_manually_ix_with_program_id(
    program_id: Pubkey,
    keys: DeployLiquidityManuallyKeys,
    args: DeployLiquidityManuallyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPLOY_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DeployLiquidityManuallyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deploy_liquidity_manually_ix(
    keys: DeployLiquidityManuallyKeys,
    args: DeployLiquidityManuallyIxArgs,
) -> std::io::Result<Instruction> {
    deploy_liquidity_manually_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn deploy_liquidity_manually_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeployLiquidityManuallyAccounts<'_, '_>,
    args: DeployLiquidityManuallyIxArgs,
) -> ProgramResult {
    let keys: DeployLiquidityManuallyKeys = accounts.into();
    let ix = deploy_liquidity_manually_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deploy_liquidity_manually_invoke(
    accounts: DeployLiquidityManuallyAccounts<'_, '_>,
    args: DeployLiquidityManuallyIxArgs,
) -> ProgramResult {
    deploy_liquidity_manually_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn deploy_liquidity_manually_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeployLiquidityManuallyAccounts<'_, '_>,
    args: DeployLiquidityManuallyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeployLiquidityManuallyKeys = accounts.into();
    let ix = deploy_liquidity_manually_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deploy_liquidity_manually_invoke_signed(
    accounts: DeployLiquidityManuallyAccounts<'_, '_>,
    args: DeployLiquidityManuallyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deploy_liquidity_manually_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deploy_liquidity_manually_verify_account_keys(
    accounts: DeployLiquidityManuallyAccounts<'_, '_>,
    keys: DeployLiquidityManuallyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.deployment_state.key, keys.deployment_state),
        (*accounts.manual_strategy_manager.key, keys.manual_strategy_manager),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_manually_verify_writable_privileges<'me, 'info>(
    accounts: DeployLiquidityManuallyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.deployment_state,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_manually_verify_signer_privileges<'me, 'info>(
    accounts: DeployLiquidityManuallyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deploy_liquidity_manually_verify_account_privileges<'me, 'info>(
    accounts: DeployLiquidityManuallyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deploy_liquidity_manually_verify_writable_privileges(accounts)?;
    deploy_liquidity_manually_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub depositor: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub depositor_underlying_token: &'me AccountInfo<'info>,
    pub depositor_mode_token: &'me AccountInfo<'info>,
    pub underlying_token_program: &'me AccountInfo<'info>,
    pub mode_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub depositor: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub depositor_underlying_token: Pubkey,
    pub depositor_mode_token: Pubkey,
    pub underlying_token_program: Pubkey,
    pub mode_token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            depositor: *accounts.depositor.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            depositor_underlying_token: *accounts.depositor_underlying_token.key,
            depositor_mode_token: *accounts.depositor_mode_token.key,
            underlying_token_program: *accounts.underlying_token_program.key,
            mode_token_program: *accounts.mode_token_program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            depositor: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
            mode_mint: pubkeys[5],
            pool_authority: pubkeys[6],
            underlying_mint: pubkeys[7],
            pool_underlying_token: pubkeys[8],
            depositor_underlying_token: pubkeys[9],
            depositor_mode_token: pubkeys[10],
            underlying_token_program: pubkeys[11],
            mode_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.depositor.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.depositor_underlying_token.clone(),
            accounts.depositor_mode_token.clone(),
            accounts.underlying_token_program.clone(),
            accounts.mode_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            depositor: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
            mode_mint: &arr[5],
            pool_authority: &arr[6],
            underlying_mint: &arr[7],
            pool_underlying_token: &arr[8],
            depositor_underlying_token: &arr[9],
            depositor_mode_token: &arr[10],
            underlying_token_program: &arr[11],
            mode_token_program: &arr[12],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub assets: u64,
    pub commitment: String,
    pub commitment_auto_renewal: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositIxData(pub DepositIxArgs);
impl From<DepositIxArgs> for DepositIxData {
    fn from(args: DepositIxArgs) -> Self {
        Self(args)
    }
}
impl DepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let commitment: String = crate::borsh_de_or_default(&mut reader)?;
        let commitment_auto_renewal: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositIxArgs {
                assets,
                commitment,
                commitment_auto_renewal,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.commitment, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.commitment_auto_renewal, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositKeys,
    args: DepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_ix(
    keys: DepositKeys,
    args: DepositIxArgs,
) -> std::io::Result<Instruction> {
    deposit_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
) -> ProgramResult {
    let keys: DepositKeys = accounts.into();
    let ix = deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_invoke(
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
) -> ProgramResult {
    deposit_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositKeys = accounts.into();
    let ix = deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_invoke_signed(
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.depositor.key, keys.depositor),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.depositor_underlying_token.key, keys.depositor_underlying_token),
        (*accounts.depositor_mode_token.key, keys.depositor_mode_token),
        (*accounts.underlying_token_program.key, keys.underlying_token_program),
        (*accounts.mode_token_program.key, keys.mode_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_verify_writable_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.mode_mint,
        accounts.pool_underlying_token,
        accounts.depositor_underlying_token,
        accounts.depositor_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_verify_signer_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.depositor] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_verify_account_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_verify_writable_privileges(accounts)?;
    deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DISABLE_POOL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct DisablePoolAccounts<'me, 'info> {
    pub pool_operator: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_operator_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DisablePoolKeys {
    pub pool_operator: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_operator_config: Pubkey,
}
impl From<DisablePoolAccounts<'_, '_>> for DisablePoolKeys {
    fn from(accounts: DisablePoolAccounts) -> Self {
        Self {
            pool_operator: *accounts.pool_operator.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_operator_config: *accounts.pool_operator_config.key,
        }
    }
}
impl From<DisablePoolKeys> for [AccountMeta; DISABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: DisablePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_operator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_operator_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DISABLE_POOL_IX_ACCOUNTS_LEN]> for DisablePoolKeys {
    fn from(pubkeys: [Pubkey; DISABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_operator: pubkeys[0],
            pool_config: pubkeys[1],
            pool_state: pubkeys[2],
            pool_operator_config: pubkeys[3],
        }
    }
}
impl<'info> From<DisablePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; DISABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: DisablePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_operator.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_operator_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISABLE_POOL_IX_ACCOUNTS_LEN]>
for DisablePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DISABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_operator: &arr[0],
            pool_config: &arr[1],
            pool_state: &arr[2],
            pool_operator_config: &arr[3],
        }
    }
}
pub const DISABLE_POOL_IX_DISCM: [u8; 8usize] = [248, 118, 211, 160, 149, 150, 135, 37];
#[derive(Clone, Debug, PartialEq)]
pub struct DisablePoolIxData;
impl DisablePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISABLE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISABLE_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn disable_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: DisablePoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISABLE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DisablePoolIxData.try_to_vec()?,
    })
}
pub fn disable_pool_ix(keys: DisablePoolKeys) -> std::io::Result<Instruction> {
    disable_pool_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn disable_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DisablePoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DisablePoolKeys = accounts.into();
    let ix = disable_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn disable_pool_invoke(accounts: DisablePoolAccounts<'_, '_>) -> ProgramResult {
    disable_pool_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn disable_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DisablePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DisablePoolKeys = accounts.into();
    let ix = disable_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn disable_pool_invoke_signed(
    accounts: DisablePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    disable_pool_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn disable_pool_verify_account_keys(
    accounts: DisablePoolAccounts<'_, '_>,
    keys: DisablePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_operator.key, keys.pool_operator),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_operator_config.key, keys.pool_operator_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn disable_pool_verify_writable_privileges<'me, 'info>(
    accounts: DisablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn disable_pool_verify_signer_privileges<'me, 'info>(
    accounts: DisablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn disable_pool_verify_account_privileges<'me, 'info>(
    accounts: DisablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    disable_pool_verify_writable_privileges(accounts)?;
    disable_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DISBURSE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct DisburseAccounts<'me, 'info> {
    pub lender: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub lender_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DisburseKeys {
    pub lender: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub lender_state: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub lender_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<DisburseAccounts<'_, '_>> for DisburseKeys {
    fn from(accounts: DisburseAccounts) -> Self {
        Self {
            lender: *accounts.lender.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            lender_state: *accounts.lender_state.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_authority: *accounts.pool_authority.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            lender_underlying_token: *accounts.lender_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DisburseKeys> for [AccountMeta; DISBURSE_IX_ACCOUNTS_LEN] {
    fn from(keys: DisburseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_underlying_token,
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
impl From<[Pubkey; DISBURSE_IX_ACCOUNTS_LEN]> for DisburseKeys {
    fn from(pubkeys: [Pubkey; DISBURSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lender: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
            lender_state: pubkeys[5],
            underlying_mint: pubkeys[6],
            pool_authority: pubkeys[7],
            pool_underlying_token: pubkeys[8],
            lender_underlying_token: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<DisburseAccounts<'_, 'info>>
for [AccountInfo<'info>; DISBURSE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DisburseAccounts<'_, 'info>) -> Self {
        [
            accounts.lender.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.lender_state.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.lender_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISBURSE_IX_ACCOUNTS_LEN]>
for DisburseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DISBURSE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lender: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
            lender_state: &arr[5],
            underlying_mint: &arr[6],
            pool_authority: &arr[7],
            pool_underlying_token: &arr[8],
            lender_underlying_token: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const DISBURSE_IX_DISCM: [u8; 8usize] = [68, 250, 205, 89, 217, 142, 13, 44];
#[derive(Clone, Debug, PartialEq)]
pub struct DisburseIxData;
impl DisburseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISBURSE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISBURSE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn disburse_ix_with_program_id(
    program_id: Pubkey,
    keys: DisburseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISBURSE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DisburseIxData.try_to_vec()?,
    })
}
pub fn disburse_ix(keys: DisburseKeys) -> std::io::Result<Instruction> {
    disburse_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn disburse_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DisburseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DisburseKeys = accounts.into();
    let ix = disburse_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn disburse_invoke(accounts: DisburseAccounts<'_, '_>) -> ProgramResult {
    disburse_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn disburse_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DisburseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DisburseKeys = accounts.into();
    let ix = disburse_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn disburse_invoke_signed(
    accounts: DisburseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    disburse_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn disburse_verify_account_keys(
    accounts: DisburseAccounts<'_, '_>,
    keys: DisburseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lender.key, keys.lender),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.lender_state.key, keys.lender_state),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.lender_underlying_token.key, keys.lender_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn disburse_verify_writable_privileges<'me, 'info>(
    accounts: DisburseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.lender_state,
        accounts.pool_underlying_token,
        accounts.lender_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn disburse_verify_signer_privileges<'me, 'info>(
    accounts: DisburseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn disburse_verify_account_privileges<'me, 'info>(
    accounts: DisburseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    disburse_verify_writable_privileges(accounts)?;
    disburse_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ENABLE_POOL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct EnablePoolAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EnablePoolKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub underlying_mint: Pubkey,
}
impl From<EnablePoolAccounts<'_, '_>> for EnablePoolKeys {
    fn from(accounts: EnablePoolAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            underlying_mint: *accounts.underlying_mint.key,
        }
    }
}
impl From<EnablePoolKeys> for [AccountMeta; ENABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: EnablePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ENABLE_POOL_IX_ACCOUNTS_LEN]> for EnablePoolKeys {
    fn from(pubkeys: [Pubkey; ENABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            underlying_mint: pubkeys[4],
        }
    }
}
impl<'info> From<EnablePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; ENABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: EnablePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.underlying_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENABLE_POOL_IX_ACCOUNTS_LEN]>
for EnablePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ENABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            underlying_mint: &arr[4],
        }
    }
}
pub const ENABLE_POOL_IX_DISCM: [u8; 8usize] = [120, 47, 0, 69, 84, 74, 16, 177];
#[derive(Clone, Debug, PartialEq)]
pub struct EnablePoolIxData;
impl EnablePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ENABLE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ENABLE_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn enable_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: EnablePoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ENABLE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: EnablePoolIxData.try_to_vec()?,
    })
}
pub fn enable_pool_ix(keys: EnablePoolKeys) -> std::io::Result<Instruction> {
    enable_pool_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn enable_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EnablePoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: EnablePoolKeys = accounts.into();
    let ix = enable_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn enable_pool_invoke(accounts: EnablePoolAccounts<'_, '_>) -> ProgramResult {
    enable_pool_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn enable_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EnablePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EnablePoolKeys = accounts.into();
    let ix = enable_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn enable_pool_invoke_signed(
    accounts: EnablePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    enable_pool_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn enable_pool_verify_account_keys(
    accounts: EnablePoolAccounts<'_, '_>,
    keys: EnablePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.underlying_mint.key, keys.underlying_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn enable_pool_verify_writable_privileges<'me, 'info>(
    accounts: EnablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn enable_pool_verify_signer_privileges<'me, 'info>(
    accounts: EnablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn enable_pool_verify_account_privileges<'me, 'info>(
    accounts: EnablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    enable_pool_verify_writable_privileges(accounts)?;
    enable_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ENTER_PRE_CLOSURE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct EnterPreClosureAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EnterPreClosureKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<EnterPreClosureAccounts<'_, '_>> for EnterPreClosureKeys {
    fn from(accounts: EnterPreClosureAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<EnterPreClosureKeys> for [AccountMeta; ENTER_PRE_CLOSURE_IX_ACCOUNTS_LEN] {
    fn from(keys: EnterPreClosureKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ENTER_PRE_CLOSURE_IX_ACCOUNTS_LEN]> for EnterPreClosureKeys {
    fn from(pubkeys: [Pubkey; ENTER_PRE_CLOSURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
        }
    }
}
impl<'info> From<EnterPreClosureAccounts<'_, 'info>>
for [AccountInfo<'info>; ENTER_PRE_CLOSURE_IX_ACCOUNTS_LEN] {
    fn from(accounts: EnterPreClosureAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENTER_PRE_CLOSURE_IX_ACCOUNTS_LEN]>
for EnterPreClosureAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ENTER_PRE_CLOSURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
        }
    }
}
pub const ENTER_PRE_CLOSURE_IX_DISCM: [u8; 8usize] = [
    236, 246, 62, 133, 60, 195, 118, 151,
];
#[derive(Clone, Debug, PartialEq)]
pub struct EnterPreClosureIxData;
impl EnterPreClosureIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ENTER_PRE_CLOSURE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ENTER_PRE_CLOSURE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn enter_pre_closure_ix_with_program_id(
    program_id: Pubkey,
    keys: EnterPreClosureKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ENTER_PRE_CLOSURE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: EnterPreClosureIxData.try_to_vec()?,
    })
}
pub fn enter_pre_closure_ix(keys: EnterPreClosureKeys) -> std::io::Result<Instruction> {
    enter_pre_closure_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn enter_pre_closure_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EnterPreClosureAccounts<'_, '_>,
) -> ProgramResult {
    let keys: EnterPreClosureKeys = accounts.into();
    let ix = enter_pre_closure_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn enter_pre_closure_invoke(
    accounts: EnterPreClosureAccounts<'_, '_>,
) -> ProgramResult {
    enter_pre_closure_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn enter_pre_closure_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EnterPreClosureAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EnterPreClosureKeys = accounts.into();
    let ix = enter_pre_closure_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn enter_pre_closure_invoke_signed(
    accounts: EnterPreClosureAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    enter_pre_closure_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn enter_pre_closure_verify_account_keys(
    accounts: EnterPreClosureAccounts<'_, '_>,
    keys: EnterPreClosureKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn enter_pre_closure_verify_writable_privileges<'me, 'info>(
    accounts: EnterPreClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn enter_pre_closure_verify_signer_privileges<'me, 'info>(
    accounts: EnterPreClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn enter_pre_closure_verify_account_privileges<'me, 'info>(
    accounts: EnterPreClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    enter_pre_closure_verify_writable_privileges(accounts)?;
    enter_pre_closure_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitiateLiquidityPaybackAsyncAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub async_deployment_config: &'me AccountInfo<'info>,
    pub async_deployment_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitiateLiquidityPaybackAsyncKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub async_deployment_config: Pubkey,
    pub async_deployment_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub pool_authority: Pubkey,
}
impl From<InitiateLiquidityPaybackAsyncAccounts<'_, '_>>
for InitiateLiquidityPaybackAsyncKeys {
    fn from(accounts: InitiateLiquidityPaybackAsyncAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            async_deployment_config: *accounts.async_deployment_config.key,
            async_deployment_state: *accounts.async_deployment_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            pool_authority: *accounts.pool_authority.key,
        }
    }
}
impl From<InitiateLiquidityPaybackAsyncKeys>
for [AccountMeta; INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN] {
    fn from(keys: InitiateLiquidityPaybackAsyncKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.async_deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.async_deployment_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN]>
for InitiateLiquidityPaybackAsyncKeys {
    fn from(
        pubkeys: [Pubkey; INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            async_deployment_config: pubkeys[4],
            async_deployment_state: pubkeys[5],
            strategy_manager_wallet: pubkeys[6],
            pool_authority: pubkeys[7],
        }
    }
}
impl<'info> From<InitiateLiquidityPaybackAsyncAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitiateLiquidityPaybackAsyncAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.async_deployment_config.clone(),
            accounts.async_deployment_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.pool_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN]>
for InitiateLiquidityPaybackAsyncAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            async_deployment_config: &arr[4],
            async_deployment_state: &arr[5],
            strategy_manager_wallet: &arr[6],
            pool_authority: &arr[7],
        }
    }
}
pub const INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM: [u8; 8usize] = [
    14, 191, 236, 248, 247, 101, 232, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitiateLiquidityPaybackAsyncIxArgs {
    pub strategy_type: AsyncDeploymentStrategyType,
    pub shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitiateLiquidityPaybackAsyncIxData(pub InitiateLiquidityPaybackAsyncIxArgs);
impl From<InitiateLiquidityPaybackAsyncIxArgs> for InitiateLiquidityPaybackAsyncIxData {
    fn from(args: InitiateLiquidityPaybackAsyncIxArgs) -> Self {
        Self(args)
    }
}
impl InitiateLiquidityPaybackAsyncIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitiateLiquidityPaybackAsyncIxArgs {
                strategy_type,
                shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initiate_liquidity_payback_async_ix_with_program_id(
    program_id: Pubkey,
    keys: InitiateLiquidityPaybackAsyncKeys,
    args: InitiateLiquidityPaybackAsyncIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIATE_LIQUIDITY_PAYBACK_ASYNC_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitiateLiquidityPaybackAsyncIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initiate_liquidity_payback_async_ix(
    keys: InitiateLiquidityPaybackAsyncKeys,
    args: InitiateLiquidityPaybackAsyncIxArgs,
) -> std::io::Result<Instruction> {
    initiate_liquidity_payback_async_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn initiate_liquidity_payback_async_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitiateLiquidityPaybackAsyncAccounts<'_, '_>,
    args: InitiateLiquidityPaybackAsyncIxArgs,
) -> ProgramResult {
    let keys: InitiateLiquidityPaybackAsyncKeys = accounts.into();
    let ix = initiate_liquidity_payback_async_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initiate_liquidity_payback_async_invoke(
    accounts: InitiateLiquidityPaybackAsyncAccounts<'_, '_>,
    args: InitiateLiquidityPaybackAsyncIxArgs,
) -> ProgramResult {
    initiate_liquidity_payback_async_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initiate_liquidity_payback_async_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitiateLiquidityPaybackAsyncAccounts<'_, '_>,
    args: InitiateLiquidityPaybackAsyncIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitiateLiquidityPaybackAsyncKeys = accounts.into();
    let ix = initiate_liquidity_payback_async_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initiate_liquidity_payback_async_invoke_signed(
    accounts: InitiateLiquidityPaybackAsyncAccounts<'_, '_>,
    args: InitiateLiquidityPaybackAsyncIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initiate_liquidity_payback_async_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initiate_liquidity_payback_async_verify_account_keys(
    accounts: InitiateLiquidityPaybackAsyncAccounts<'_, '_>,
    keys: InitiateLiquidityPaybackAsyncKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.async_deployment_config.key, keys.async_deployment_config),
        (*accounts.async_deployment_state.key, keys.async_deployment_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.pool_authority.key, keys.pool_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initiate_liquidity_payback_async_verify_signer_privileges<'me, 'info>(
    accounts: InitiateLiquidityPaybackAsyncAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initiate_liquidity_payback_async_verify_account_privileges<'me, 'info>(
    accounts: InitiateLiquidityPaybackAsyncAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initiate_liquidity_payback_async_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_WITHDRAW_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct InstantWithdrawAccounts<'me, 'info> {
    pub lender: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub deployment_state: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub lender_underlying_token: &'me AccountInfo<'info>,
    pub pool_owner_treasury_underlying_token: &'me AccountInfo<'info>,
    pub lender_mode_token: &'me AccountInfo<'info>,
    pub underlying_token_program: &'me AccountInfo<'info>,
    pub mode_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantWithdrawKeys {
    pub lender: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub deployment_config: Pubkey,
    pub deployment_state: Pubkey,
    pub lender_state: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub lender_underlying_token: Pubkey,
    pub pool_owner_treasury_underlying_token: Pubkey,
    pub lender_mode_token: Pubkey,
    pub underlying_token_program: Pubkey,
    pub mode_token_program: Pubkey,
}
impl From<InstantWithdrawAccounts<'_, '_>> for InstantWithdrawKeys {
    fn from(accounts: InstantWithdrawAccounts) -> Self {
        Self {
            lender: *accounts.lender.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            deployment_config: *accounts.deployment_config.key,
            deployment_state: *accounts.deployment_state.key,
            lender_state: *accounts.lender_state.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_authority: *accounts.pool_authority.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            lender_underlying_token: *accounts.lender_underlying_token.key,
            pool_owner_treasury_underlying_token: *accounts
                .pool_owner_treasury_underlying_token
                .key,
            lender_mode_token: *accounts.lender_mode_token.key,
            underlying_token_program: *accounts.underlying_token_program.key,
            mode_token_program: *accounts.mode_token_program.key,
        }
    }
}
impl From<InstantWithdrawKeys> for [AccountMeta; INSTANT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_owner_treasury_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_WITHDRAW_IX_ACCOUNTS_LEN]> for InstantWithdrawKeys {
    fn from(pubkeys: [Pubkey; INSTANT_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lender: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
            mode_mint: pubkeys[5],
            deployment_config: pubkeys[6],
            deployment_state: pubkeys[7],
            lender_state: pubkeys[8],
            underlying_mint: pubkeys[9],
            pool_authority: pubkeys[10],
            pool_underlying_token: pubkeys[11],
            lender_underlying_token: pubkeys[12],
            pool_owner_treasury_underlying_token: pubkeys[13],
            lender_mode_token: pubkeys[14],
            underlying_token_program: pubkeys[15],
            mode_token_program: pubkeys[16],
        }
    }
}
impl<'info> From<InstantWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.lender.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.deployment_config.clone(),
            accounts.deployment_state.clone(),
            accounts.lender_state.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.lender_underlying_token.clone(),
            accounts.pool_owner_treasury_underlying_token.clone(),
            accounts.lender_mode_token.clone(),
            accounts.underlying_token_program.clone(),
            accounts.mode_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INSTANT_WITHDRAW_IX_ACCOUNTS_LEN]>
for InstantWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INSTANT_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lender: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
            mode_mint: &arr[5],
            deployment_config: &arr[6],
            deployment_state: &arr[7],
            lender_state: &arr[8],
            underlying_mint: &arr[9],
            pool_authority: &arr[10],
            pool_underlying_token: &arr[11],
            lender_underlying_token: &arr[12],
            pool_owner_treasury_underlying_token: &arr[13],
            lender_mode_token: &arr[14],
            underlying_token_program: &arr[15],
            mode_token_program: &arr[16],
        }
    }
}
pub const INSTANT_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    171, 49, 145, 176, 48, 101, 112, 162,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawIxArgs {
    pub shares: u64,
    pub max_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawIxData(pub InstantWithdrawIxArgs);
impl From<InstantWithdrawIxArgs> for InstantWithdrawIxData {
    fn from(args: InstantWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl InstantWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InstantWithdrawIxArgs {
                shares,
                max_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantWithdrawKeys,
    args: InstantWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_withdraw_ix(
    keys: InstantWithdrawKeys,
    args: InstantWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    instant_withdraw_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn instant_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawAccounts<'_, '_>,
    args: InstantWithdrawIxArgs,
) -> ProgramResult {
    let keys: InstantWithdrawKeys = accounts.into();
    let ix = instant_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_withdraw_invoke(
    accounts: InstantWithdrawAccounts<'_, '_>,
    args: InstantWithdrawIxArgs,
) -> ProgramResult {
    instant_withdraw_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn instant_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawAccounts<'_, '_>,
    args: InstantWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantWithdrawKeys = accounts.into();
    let ix = instant_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_withdraw_invoke_signed(
    accounts: InstantWithdrawAccounts<'_, '_>,
    args: InstantWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_withdraw_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_withdraw_verify_account_keys(
    accounts: InstantWithdrawAccounts<'_, '_>,
    keys: InstantWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lender.key, keys.lender),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.deployment_state.key, keys.deployment_state),
        (*accounts.lender_state.key, keys.lender_state),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.lender_underlying_token.key, keys.lender_underlying_token),
        (
            *accounts.pool_owner_treasury_underlying_token.key,
            keys.pool_owner_treasury_underlying_token,
        ),
        (*accounts.lender_mode_token.key, keys.lender_mode_token),
        (*accounts.underlying_token_program.key, keys.underlying_token_program),
        (*accounts.mode_token_program.key, keys.mode_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: InstantWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.mode_mint,
        accounts.deployment_state,
        accounts.lender_state,
        accounts.pool_authority,
        accounts.pool_underlying_token,
        accounts.lender_underlying_token,
        accounts.pool_owner_treasury_underlying_token,
        accounts.lender_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: InstantWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_withdraw_verify_account_privileges<'me, 'info>(
    accounts: InstantWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_withdraw_verify_writable_privileges(accounts)?;
    instant_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_WITHDRAW_PRIVILEGED_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct InstantWithdrawPrivilegedAccounts<'me, 'info> {
    pub lender: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub instant_withdrawal_lender_config: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub deployment_state: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub lender_underlying_token: &'me AccountInfo<'info>,
    pub lender_mode_token: &'me AccountInfo<'info>,
    pub underlying_token_program: &'me AccountInfo<'info>,
    pub mode_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantWithdrawPrivilegedKeys {
    pub lender: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub instant_withdrawal_lender_config: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub deployment_config: Pubkey,
    pub deployment_state: Pubkey,
    pub lender_state: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub lender_underlying_token: Pubkey,
    pub lender_mode_token: Pubkey,
    pub underlying_token_program: Pubkey,
    pub mode_token_program: Pubkey,
}
impl From<InstantWithdrawPrivilegedAccounts<'_, '_>> for InstantWithdrawPrivilegedKeys {
    fn from(accounts: InstantWithdrawPrivilegedAccounts) -> Self {
        Self {
            lender: *accounts.lender.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            instant_withdrawal_lender_config: *accounts
                .instant_withdrawal_lender_config
                .key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            deployment_config: *accounts.deployment_config.key,
            deployment_state: *accounts.deployment_state.key,
            lender_state: *accounts.lender_state.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_authority: *accounts.pool_authority.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            lender_underlying_token: *accounts.lender_underlying_token.key,
            lender_mode_token: *accounts.lender_mode_token.key,
            underlying_token_program: *accounts.underlying_token_program.key,
            mode_token_program: *accounts.mode_token_program.key,
        }
    }
}
impl From<InstantWithdrawPrivilegedKeys>
for [AccountMeta; INSTANT_WITHDRAW_PRIVILEGED_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantWithdrawPrivilegedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instant_withdrawal_lender_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_WITHDRAW_PRIVILEGED_IX_ACCOUNTS_LEN]>
for InstantWithdrawPrivilegedKeys {
    fn from(pubkeys: [Pubkey; INSTANT_WITHDRAW_PRIVILEGED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lender: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            instant_withdrawal_lender_config: pubkeys[4],
            mode_config: pubkeys[5],
            mode_mint: pubkeys[6],
            deployment_config: pubkeys[7],
            deployment_state: pubkeys[8],
            lender_state: pubkeys[9],
            underlying_mint: pubkeys[10],
            pool_authority: pubkeys[11],
            pool_underlying_token: pubkeys[12],
            lender_underlying_token: pubkeys[13],
            lender_mode_token: pubkeys[14],
            underlying_token_program: pubkeys[15],
            mode_token_program: pubkeys[16],
        }
    }
}
impl<'info> From<InstantWithdrawPrivilegedAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_WITHDRAW_PRIVILEGED_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantWithdrawPrivilegedAccounts<'_, 'info>) -> Self {
        [
            accounts.lender.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.instant_withdrawal_lender_config.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.deployment_config.clone(),
            accounts.deployment_state.clone(),
            accounts.lender_state.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.lender_underlying_token.clone(),
            accounts.lender_mode_token.clone(),
            accounts.underlying_token_program.clone(),
            accounts.mode_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INSTANT_WITHDRAW_PRIVILEGED_IX_ACCOUNTS_LEN]>
for InstantWithdrawPrivilegedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_WITHDRAW_PRIVILEGED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lender: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            instant_withdrawal_lender_config: &arr[4],
            mode_config: &arr[5],
            mode_mint: &arr[6],
            deployment_config: &arr[7],
            deployment_state: &arr[8],
            lender_state: &arr[9],
            underlying_mint: &arr[10],
            pool_authority: &arr[11],
            pool_underlying_token: &arr[12],
            lender_underlying_token: &arr[13],
            lender_mode_token: &arr[14],
            underlying_token_program: &arr[15],
            mode_token_program: &arr[16],
        }
    }
}
pub const INSTANT_WITHDRAW_PRIVILEGED_IX_DISCM: [u8; 8usize] = [
    101, 97, 214, 222, 232, 246, 104, 48,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawPrivilegedIxArgs {
    pub shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawPrivilegedIxData(pub InstantWithdrawPrivilegedIxArgs);
impl From<InstantWithdrawPrivilegedIxArgs> for InstantWithdrawPrivilegedIxData {
    fn from(args: InstantWithdrawPrivilegedIxArgs) -> Self {
        Self(args)
    }
}
impl InstantWithdrawPrivilegedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAW_PRIVILEGED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InstantWithdrawPrivilegedIxArgs {
                shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAW_PRIVILEGED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_withdraw_privileged_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantWithdrawPrivilegedKeys,
    args: InstantWithdrawPrivilegedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_WITHDRAW_PRIVILEGED_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantWithdrawPrivilegedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_withdraw_privileged_ix(
    keys: InstantWithdrawPrivilegedKeys,
    args: InstantWithdrawPrivilegedIxArgs,
) -> std::io::Result<Instruction> {
    instant_withdraw_privileged_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn instant_withdraw_privileged_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawPrivilegedAccounts<'_, '_>,
    args: InstantWithdrawPrivilegedIxArgs,
) -> ProgramResult {
    let keys: InstantWithdrawPrivilegedKeys = accounts.into();
    let ix = instant_withdraw_privileged_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_withdraw_privileged_invoke(
    accounts: InstantWithdrawPrivilegedAccounts<'_, '_>,
    args: InstantWithdrawPrivilegedIxArgs,
) -> ProgramResult {
    instant_withdraw_privileged_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn instant_withdraw_privileged_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantWithdrawPrivilegedAccounts<'_, '_>,
    args: InstantWithdrawPrivilegedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantWithdrawPrivilegedKeys = accounts.into();
    let ix = instant_withdraw_privileged_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_withdraw_privileged_invoke_signed(
    accounts: InstantWithdrawPrivilegedAccounts<'_, '_>,
    args: InstantWithdrawPrivilegedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_withdraw_privileged_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_withdraw_privileged_verify_account_keys(
    accounts: InstantWithdrawPrivilegedAccounts<'_, '_>,
    keys: InstantWithdrawPrivilegedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lender.key, keys.lender),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (
            *accounts.instant_withdrawal_lender_config.key,
            keys.instant_withdrawal_lender_config,
        ),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.deployment_state.key, keys.deployment_state),
        (*accounts.lender_state.key, keys.lender_state),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.lender_underlying_token.key, keys.lender_underlying_token),
        (*accounts.lender_mode_token.key, keys.lender_mode_token),
        (*accounts.underlying_token_program.key, keys.underlying_token_program),
        (*accounts.mode_token_program.key, keys.mode_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_withdraw_privileged_verify_writable_privileges<'me, 'info>(
    accounts: InstantWithdrawPrivilegedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.mode_mint,
        accounts.deployment_state,
        accounts.lender_state,
        accounts.pool_authority,
        accounts.pool_underlying_token,
        accounts.lender_underlying_token,
        accounts.lender_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_withdraw_privileged_verify_signer_privileges<'me, 'info>(
    accounts: InstantWithdrawPrivilegedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_withdraw_privileged_verify_account_privileges<'me, 'info>(
    accounts: InstantWithdrawPrivilegedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_withdraw_privileged_verify_writable_privileges(accounts)?;
    instant_withdraw_privileged_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MAKE_INITIAL_DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct MakeInitialDepositAccounts<'me, 'info> {
    pub pool_owner_treasury: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub pool_owner_treasury_underlying_token: &'me AccountInfo<'info>,
    pub pool_owner_treasury_mode_token: &'me AccountInfo<'info>,
    pub underlying_token_program: &'me AccountInfo<'info>,
    pub mode_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MakeInitialDepositKeys {
    pub pool_owner_treasury: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub pool_owner_treasury_underlying_token: Pubkey,
    pub pool_owner_treasury_mode_token: Pubkey,
    pub underlying_token_program: Pubkey,
    pub mode_token_program: Pubkey,
}
impl From<MakeInitialDepositAccounts<'_, '_>> for MakeInitialDepositKeys {
    fn from(accounts: MakeInitialDepositAccounts) -> Self {
        Self {
            pool_owner_treasury: *accounts.pool_owner_treasury.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            pool_owner_treasury_underlying_token: *accounts
                .pool_owner_treasury_underlying_token
                .key,
            pool_owner_treasury_mode_token: *accounts.pool_owner_treasury_mode_token.key,
            underlying_token_program: *accounts.underlying_token_program.key,
            mode_token_program: *accounts.mode_token_program.key,
        }
    }
}
impl From<MakeInitialDepositKeys>
for [AccountMeta; MAKE_INITIAL_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: MakeInitialDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_owner_treasury,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_owner_treasury_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_owner_treasury_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MAKE_INITIAL_DEPOSIT_IX_ACCOUNTS_LEN]> for MakeInitialDepositKeys {
    fn from(pubkeys: [Pubkey; MAKE_INITIAL_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner_treasury: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
            mode_mint: pubkeys[5],
            pool_authority: pubkeys[6],
            underlying_mint: pubkeys[7],
            pool_underlying_token: pubkeys[8],
            pool_owner_treasury_underlying_token: pubkeys[9],
            pool_owner_treasury_mode_token: pubkeys[10],
            underlying_token_program: pubkeys[11],
            mode_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<MakeInitialDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; MAKE_INITIAL_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MakeInitialDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_owner_treasury.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.pool_owner_treasury_underlying_token.clone(),
            accounts.pool_owner_treasury_mode_token.clone(),
            accounts.underlying_token_program.clone(),
            accounts.mode_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MAKE_INITIAL_DEPOSIT_IX_ACCOUNTS_LEN]>
for MakeInitialDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MAKE_INITIAL_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_owner_treasury: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
            mode_mint: &arr[5],
            pool_authority: &arr[6],
            underlying_mint: &arr[7],
            pool_underlying_token: &arr[8],
            pool_owner_treasury_underlying_token: &arr[9],
            pool_owner_treasury_mode_token: &arr[10],
            underlying_token_program: &arr[11],
            mode_token_program: &arr[12],
        }
    }
}
pub const MAKE_INITIAL_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    141, 233, 75, 102, 37, 93, 94, 79,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MakeInitialDepositIxArgs {
    pub assets: u64,
    pub commitment: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MakeInitialDepositIxData(pub MakeInitialDepositIxArgs);
impl From<MakeInitialDepositIxArgs> for MakeInitialDepositIxData {
    fn from(args: MakeInitialDepositIxArgs) -> Self {
        Self(args)
    }
}
impl MakeInitialDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MAKE_INITIAL_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let commitment: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MakeInitialDepositIxArgs {
                assets,
                commitment,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MAKE_INITIAL_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.commitment, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn make_initial_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: MakeInitialDepositKeys,
    args: MakeInitialDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MAKE_INITIAL_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: MakeInitialDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn make_initial_deposit_ix(
    keys: MakeInitialDepositKeys,
    args: MakeInitialDepositIxArgs,
) -> std::io::Result<Instruction> {
    make_initial_deposit_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn make_initial_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MakeInitialDepositAccounts<'_, '_>,
    args: MakeInitialDepositIxArgs,
) -> ProgramResult {
    let keys: MakeInitialDepositKeys = accounts.into();
    let ix = make_initial_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn make_initial_deposit_invoke(
    accounts: MakeInitialDepositAccounts<'_, '_>,
    args: MakeInitialDepositIxArgs,
) -> ProgramResult {
    make_initial_deposit_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn make_initial_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MakeInitialDepositAccounts<'_, '_>,
    args: MakeInitialDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MakeInitialDepositKeys = accounts.into();
    let ix = make_initial_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn make_initial_deposit_invoke_signed(
    accounts: MakeInitialDepositAccounts<'_, '_>,
    args: MakeInitialDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    make_initial_deposit_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn make_initial_deposit_verify_account_keys(
    accounts: MakeInitialDepositAccounts<'_, '_>,
    keys: MakeInitialDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_owner_treasury.key, keys.pool_owner_treasury),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (
            *accounts.pool_owner_treasury_underlying_token.key,
            keys.pool_owner_treasury_underlying_token,
        ),
        (
            *accounts.pool_owner_treasury_mode_token.key,
            keys.pool_owner_treasury_mode_token,
        ),
        (*accounts.underlying_token_program.key, keys.underlying_token_program),
        (*accounts.mode_token_program.key, keys.mode_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn make_initial_deposit_verify_writable_privileges<'me, 'info>(
    accounts: MakeInitialDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.mode_mint,
        accounts.pool_underlying_token,
        accounts.pool_owner_treasury_underlying_token,
        accounts.pool_owner_treasury_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn make_initial_deposit_verify_signer_privileges<'me, 'info>(
    accounts: MakeInitialDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_owner_treasury] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn make_initial_deposit_verify_account_privileges<'me, 'info>(
    accounts: MakeInitialDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    make_initial_deposit_verify_writable_privileges(accounts)?;
    make_initial_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAY_BACK_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct PayBackLiquidityAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub deployment_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PayBackLiquidityKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub deployment_config: Pubkey,
    pub deployment_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<PayBackLiquidityAccounts<'_, '_>> for PayBackLiquidityKeys {
    fn from(accounts: PayBackLiquidityAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            deployment_config: *accounts.deployment_config.key,
            deployment_state: *accounts.deployment_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PayBackLiquidityKeys> for [AccountMeta; PAY_BACK_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: PayBackLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; PAY_BACK_LIQUIDITY_IX_ACCOUNTS_LEN]> for PayBackLiquidityKeys {
    fn from(pubkeys: [Pubkey; PAY_BACK_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            deployment_config: pubkeys[4],
            deployment_state: pubkeys[5],
            strategy_manager_wallet: pubkeys[6],
            pool_authority: pubkeys[7],
            underlying_mint: pubkeys[8],
            pool_underlying_token: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<PayBackLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; PAY_BACK_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: PayBackLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.deployment_config.clone(),
            accounts.deployment_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAY_BACK_LIQUIDITY_IX_ACCOUNTS_LEN]>
for PayBackLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAY_BACK_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            deployment_config: &arr[4],
            deployment_state: &arr[5],
            strategy_manager_wallet: &arr[6],
            pool_authority: &arr[7],
            underlying_mint: &arr[8],
            pool_underlying_token: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const PAY_BACK_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    97, 171, 224, 78, 162, 164, 181, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayBackLiquidityIxArgs {
    pub strategy_type: DeploymentStrategyType,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PayBackLiquidityIxData(pub PayBackLiquidityIxArgs);
impl From<PayBackLiquidityIxArgs> for PayBackLiquidityIxData {
    fn from(args: PayBackLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl PayBackLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAY_BACK_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PayBackLiquidityIxArgs {
                strategy_type,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAY_BACK_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pay_back_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: PayBackLiquidityKeys,
    args: PayBackLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAY_BACK_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: PayBackLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pay_back_liquidity_ix(
    keys: PayBackLiquidityKeys,
    args: PayBackLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    pay_back_liquidity_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn pay_back_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PayBackLiquidityAccounts<'_, '_>,
    args: PayBackLiquidityIxArgs,
) -> ProgramResult {
    let keys: PayBackLiquidityKeys = accounts.into();
    let ix = pay_back_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pay_back_liquidity_invoke(
    accounts: PayBackLiquidityAccounts<'_, '_>,
    args: PayBackLiquidityIxArgs,
) -> ProgramResult {
    pay_back_liquidity_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn pay_back_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PayBackLiquidityAccounts<'_, '_>,
    args: PayBackLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PayBackLiquidityKeys = accounts.into();
    let ix = pay_back_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pay_back_liquidity_invoke_signed(
    accounts: PayBackLiquidityAccounts<'_, '_>,
    args: PayBackLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pay_back_liquidity_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pay_back_liquidity_verify_account_keys(
    accounts: PayBackLiquidityAccounts<'_, '_>,
    keys: PayBackLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.deployment_state.key, keys.deployment_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pay_back_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: PayBackLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.deployment_state,
        accounts.pool_authority,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pay_back_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: PayBackLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pay_back_liquidity_verify_account_privileges<'me, 'info>(
    accounts: PayBackLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pay_back_liquidity_verify_writable_privileges(accounts)?;
    pay_back_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAY_BACK_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct PayBackLiquidityManuallyAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub deployment_state: &'me AccountInfo<'info>,
    pub manual_strategy_manager: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PayBackLiquidityManuallyKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub deployment_config: Pubkey,
    pub deployment_state: Pubkey,
    pub manual_strategy_manager: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<PayBackLiquidityManuallyAccounts<'_, '_>> for PayBackLiquidityManuallyKeys {
    fn from(accounts: PayBackLiquidityManuallyAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            deployment_config: *accounts.deployment_config.key,
            deployment_state: *accounts.deployment_state.key,
            manual_strategy_manager: *accounts.manual_strategy_manager.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PayBackLiquidityManuallyKeys>
for [AccountMeta; PAY_BACK_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN] {
    fn from(keys: PayBackLiquidityManuallyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manual_strategy_manager,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; PAY_BACK_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN]>
for PayBackLiquidityManuallyKeys {
    fn from(pubkeys: [Pubkey; PAY_BACK_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            deployment_config: pubkeys[4],
            deployment_state: pubkeys[5],
            manual_strategy_manager: pubkeys[6],
            pool_authority: pubkeys[7],
            underlying_mint: pubkeys[8],
            pool_underlying_token: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<PayBackLiquidityManuallyAccounts<'_, 'info>>
for [AccountInfo<'info>; PAY_BACK_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: PayBackLiquidityManuallyAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.deployment_config.clone(),
            accounts.deployment_state.clone(),
            accounts.manual_strategy_manager.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAY_BACK_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN]>
for PayBackLiquidityManuallyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAY_BACK_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            deployment_config: &arr[4],
            deployment_state: &arr[5],
            manual_strategy_manager: &arr[6],
            pool_authority: &arr[7],
            underlying_mint: &arr[8],
            pool_underlying_token: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const PAY_BACK_LIQUIDITY_MANUALLY_IX_DISCM: [u8; 8usize] = [
    15, 105, 175, 222, 154, 87, 101, 132,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayBackLiquidityManuallyIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PayBackLiquidityManuallyIxData(pub PayBackLiquidityManuallyIxArgs);
impl From<PayBackLiquidityManuallyIxArgs> for PayBackLiquidityManuallyIxData {
    fn from(args: PayBackLiquidityManuallyIxArgs) -> Self {
        Self(args)
    }
}
impl PayBackLiquidityManuallyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAY_BACK_LIQUIDITY_MANUALLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PayBackLiquidityManuallyIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAY_BACK_LIQUIDITY_MANUALLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pay_back_liquidity_manually_ix_with_program_id(
    program_id: Pubkey,
    keys: PayBackLiquidityManuallyKeys,
    args: PayBackLiquidityManuallyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAY_BACK_LIQUIDITY_MANUALLY_IX_ACCOUNTS_LEN] = keys.into();
    let data: PayBackLiquidityManuallyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pay_back_liquidity_manually_ix(
    keys: PayBackLiquidityManuallyKeys,
    args: PayBackLiquidityManuallyIxArgs,
) -> std::io::Result<Instruction> {
    pay_back_liquidity_manually_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn pay_back_liquidity_manually_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PayBackLiquidityManuallyAccounts<'_, '_>,
    args: PayBackLiquidityManuallyIxArgs,
) -> ProgramResult {
    let keys: PayBackLiquidityManuallyKeys = accounts.into();
    let ix = pay_back_liquidity_manually_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pay_back_liquidity_manually_invoke(
    accounts: PayBackLiquidityManuallyAccounts<'_, '_>,
    args: PayBackLiquidityManuallyIxArgs,
) -> ProgramResult {
    pay_back_liquidity_manually_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn pay_back_liquidity_manually_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PayBackLiquidityManuallyAccounts<'_, '_>,
    args: PayBackLiquidityManuallyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PayBackLiquidityManuallyKeys = accounts.into();
    let ix = pay_back_liquidity_manually_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pay_back_liquidity_manually_invoke_signed(
    accounts: PayBackLiquidityManuallyAccounts<'_, '_>,
    args: PayBackLiquidityManuallyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pay_back_liquidity_manually_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pay_back_liquidity_manually_verify_account_keys(
    accounts: PayBackLiquidityManuallyAccounts<'_, '_>,
    keys: PayBackLiquidityManuallyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.deployment_state.key, keys.deployment_state),
        (*accounts.manual_strategy_manager.key, keys.manual_strategy_manager),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pay_back_liquidity_manually_verify_writable_privileges<'me, 'info>(
    accounts: PayBackLiquidityManuallyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.deployment_state,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pay_back_liquidity_manually_verify_signer_privileges<'me, 'info>(
    accounts: PayBackLiquidityManuallyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pay_back_liquidity_manually_verify_account_privileges<'me, 'info>(
    accounts: PayBackLiquidityManuallyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pay_back_liquidity_manually_verify_writable_privileges(accounts)?;
    pay_back_liquidity_manually_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PROCESS_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct ProcessRedemptionRequestAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub lender: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub redemption_request: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub lender_underlying_token: &'me AccountInfo<'info>,
    pub pool_mode_token: &'me AccountInfo<'info>,
    pub underlying_token_program: &'me AccountInfo<'info>,
    pub mode_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProcessRedemptionRequestKeys {
    pub signer: Pubkey,
    pub lender: Pubkey,
    pub payer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub redemption_request: Pubkey,
    pub lender_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub lender_underlying_token: Pubkey,
    pub pool_mode_token: Pubkey,
    pub underlying_token_program: Pubkey,
    pub mode_token_program: Pubkey,
}
impl From<ProcessRedemptionRequestAccounts<'_, '_>> for ProcessRedemptionRequestKeys {
    fn from(accounts: ProcessRedemptionRequestAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            lender: *accounts.lender.key,
            payer: *accounts.payer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            redemption_request: *accounts.redemption_request.key,
            lender_state: *accounts.lender_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_authority: *accounts.pool_authority.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            lender_underlying_token: *accounts.lender_underlying_token.key,
            pool_mode_token: *accounts.pool_mode_token.key,
            underlying_token_program: *accounts.underlying_token_program.key,
            mode_token_program: *accounts.mode_token_program.key,
        }
    }
}
impl From<ProcessRedemptionRequestKeys>
for [AccountMeta; PROCESS_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(keys: ProcessRedemptionRequestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lender,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PROCESS_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]>
for ProcessRedemptionRequestKeys {
    fn from(pubkeys: [Pubkey; PROCESS_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            lender: pubkeys[1],
            payer: pubkeys[2],
            huma_config: pubkeys[3],
            pool_config: pubkeys[4],
            pool_state: pubkeys[5],
            mode_config: pubkeys[6],
            mode_mint: pubkeys[7],
            redemption_request: pubkeys[8],
            lender_state: pubkeys[9],
            strategy_manager_wallet: pubkeys[10],
            underlying_mint: pubkeys[11],
            pool_authority: pubkeys[12],
            pool_underlying_token: pubkeys[13],
            lender_underlying_token: pubkeys[14],
            pool_mode_token: pubkeys[15],
            underlying_token_program: pubkeys[16],
            mode_token_program: pubkeys[17],
        }
    }
}
impl<'info> From<ProcessRedemptionRequestAccounts<'_, 'info>>
for [AccountInfo<'info>; PROCESS_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: ProcessRedemptionRequestAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.lender.clone(),
            accounts.payer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.redemption_request.clone(),
            accounts.lender_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.lender_underlying_token.clone(),
            accounts.pool_mode_token.clone(),
            accounts.underlying_token_program.clone(),
            accounts.mode_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PROCESS_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]>
for ProcessRedemptionRequestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PROCESS_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            lender: &arr[1],
            payer: &arr[2],
            huma_config: &arr[3],
            pool_config: &arr[4],
            pool_state: &arr[5],
            mode_config: &arr[6],
            mode_mint: &arr[7],
            redemption_request: &arr[8],
            lender_state: &arr[9],
            strategy_manager_wallet: &arr[10],
            underlying_mint: &arr[11],
            pool_authority: &arr[12],
            pool_underlying_token: &arr[13],
            lender_underlying_token: &arr[14],
            pool_mode_token: &arr[15],
            underlying_token_program: &arr[16],
            mode_token_program: &arr[17],
        }
    }
}
pub const PROCESS_REDEMPTION_REQUEST_IX_DISCM: [u8; 8usize] = [
    211, 90, 36, 251, 91, 185, 216, 35,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessRedemptionRequestIxData;
impl ProcessRedemptionRequestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROCESS_REDEMPTION_REQUEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROCESS_REDEMPTION_REQUEST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn process_redemption_request_ix_with_program_id(
    program_id: Pubkey,
    keys: ProcessRedemptionRequestKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROCESS_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ProcessRedemptionRequestIxData.try_to_vec()?,
    })
}
pub fn process_redemption_request_ix(
    keys: ProcessRedemptionRequestKeys,
) -> std::io::Result<Instruction> {
    process_redemption_request_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn process_redemption_request_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ProcessRedemptionRequestAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ProcessRedemptionRequestKeys = accounts.into();
    let ix = process_redemption_request_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn process_redemption_request_invoke(
    accounts: ProcessRedemptionRequestAccounts<'_, '_>,
) -> ProgramResult {
    process_redemption_request_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn process_redemption_request_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ProcessRedemptionRequestAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ProcessRedemptionRequestKeys = accounts.into();
    let ix = process_redemption_request_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn process_redemption_request_invoke_signed(
    accounts: ProcessRedemptionRequestAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    process_redemption_request_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn process_redemption_request_verify_account_keys(
    accounts: ProcessRedemptionRequestAccounts<'_, '_>,
    keys: ProcessRedemptionRequestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.lender.key, keys.lender),
        (*accounts.payer.key, keys.payer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.redemption_request.key, keys.redemption_request),
        (*accounts.lender_state.key, keys.lender_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.lender_underlying_token.key, keys.lender_underlying_token),
        (*accounts.pool_mode_token.key, keys.pool_mode_token),
        (*accounts.underlying_token_program.key, keys.underlying_token_program),
        (*accounts.mode_token_program.key, keys.mode_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn process_redemption_request_verify_writable_privileges<'me, 'info>(
    accounts: ProcessRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.mode_mint,
        accounts.redemption_request,
        accounts.lender_state,
        accounts.pool_underlying_token,
        accounts.lender_underlying_token,
        accounts.pool_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn process_redemption_request_verify_signer_privileges<'me, 'info>(
    accounts: ProcessRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn process_redemption_request_verify_account_privileges<'me, 'info>(
    accounts: ProcessRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    process_redemption_request_verify_writable_privileges(accounts)?;
    process_redemption_request_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PROPOSE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ProposeManualStrategyManagerAccounts<'me, 'info> {
    pub pool_owner: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProposeManualStrategyManagerKeys {
    pub pool_owner: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub proposal: Pubkey,
    pub system_program: Pubkey,
}
impl From<ProposeManualStrategyManagerAccounts<'_, '_>>
for ProposeManualStrategyManagerKeys {
    fn from(accounts: ProposeManualStrategyManagerAccounts) -> Self {
        Self {
            pool_owner: *accounts.pool_owner.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            proposal: *accounts.proposal.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ProposeManualStrategyManagerKeys>
for [AccountMeta; PROPOSE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: ProposeManualStrategyManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
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
impl From<[Pubkey; PROPOSE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]>
for ProposeManualStrategyManagerKeys {
    fn from(pubkeys: [Pubkey; PROPOSE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            proposal: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<ProposeManualStrategyManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; PROPOSE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ProposeManualStrategyManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_owner.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.proposal.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PROPOSE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]>
for ProposeManualStrategyManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PROPOSE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_owner: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            proposal: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const PROPOSE_MANUAL_STRATEGY_MANAGER_IX_DISCM: [u8; 8usize] = [
    124, 74, 236, 183, 200, 93, 107, 181,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProposeManualStrategyManagerIxArgs {
    pub wallet: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposeManualStrategyManagerIxData(pub ProposeManualStrategyManagerIxArgs);
impl From<ProposeManualStrategyManagerIxArgs> for ProposeManualStrategyManagerIxData {
    fn from(args: ProposeManualStrategyManagerIxArgs) -> Self {
        Self(args)
    }
}
impl ProposeManualStrategyManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPOSE_MANUAL_STRATEGY_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ProposeManualStrategyManagerIxArgs {
                wallet,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPOSE_MANUAL_STRATEGY_MANAGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.wallet, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn propose_manual_strategy_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: ProposeManualStrategyManagerKeys,
    args: ProposeManualStrategyManagerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROPOSE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ProposeManualStrategyManagerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn propose_manual_strategy_manager_ix(
    keys: ProposeManualStrategyManagerKeys,
    args: ProposeManualStrategyManagerIxArgs,
) -> std::io::Result<Instruction> {
    propose_manual_strategy_manager_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn propose_manual_strategy_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ProposeManualStrategyManagerAccounts<'_, '_>,
    args: ProposeManualStrategyManagerIxArgs,
) -> ProgramResult {
    let keys: ProposeManualStrategyManagerKeys = accounts.into();
    let ix = propose_manual_strategy_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn propose_manual_strategy_manager_invoke(
    accounts: ProposeManualStrategyManagerAccounts<'_, '_>,
    args: ProposeManualStrategyManagerIxArgs,
) -> ProgramResult {
    propose_manual_strategy_manager_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn propose_manual_strategy_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ProposeManualStrategyManagerAccounts<'_, '_>,
    args: ProposeManualStrategyManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ProposeManualStrategyManagerKeys = accounts.into();
    let ix = propose_manual_strategy_manager_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn propose_manual_strategy_manager_invoke_signed(
    accounts: ProposeManualStrategyManagerAccounts<'_, '_>,
    args: ProposeManualStrategyManagerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    propose_manual_strategy_manager_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn propose_manual_strategy_manager_verify_account_keys(
    accounts: ProposeManualStrategyManagerAccounts<'_, '_>,
    keys: ProposeManualStrategyManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_owner.key, keys.pool_owner),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn propose_manual_strategy_manager_verify_writable_privileges<'me, 'info>(
    accounts: ProposeManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_owner, accounts.proposal] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn propose_manual_strategy_manager_verify_signer_privileges<'me, 'info>(
    accounts: ProposeManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn propose_manual_strategy_manager_verify_account_privileges<'me, 'info>(
    accounts: ProposeManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    propose_manual_strategy_manager_verify_writable_privileges(accounts)?;
    propose_manual_strategy_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RECOVER_LOSS_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RecoverLossAccounts<'me, 'info> {
    pub loss_authority: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub loss_authority_underlying_token: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RecoverLossKeys {
    pub loss_authority: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub underlying_mint: Pubkey,
    pub loss_authority_underlying_token: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub pool_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<RecoverLossAccounts<'_, '_>> for RecoverLossKeys {
    fn from(accounts: RecoverLossAccounts) -> Self {
        Self {
            loss_authority: *accounts.loss_authority.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            underlying_mint: *accounts.underlying_mint.key,
            loss_authority_underlying_token: *accounts
                .loss_authority_underlying_token
                .key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            pool_authority: *accounts.pool_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RecoverLossKeys> for [AccountMeta; RECOVER_LOSS_IX_ACCOUNTS_LEN] {
    fn from(keys: RecoverLossKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.loss_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.loss_authority_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
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
impl From<[Pubkey; RECOVER_LOSS_IX_ACCOUNTS_LEN]> for RecoverLossKeys {
    fn from(pubkeys: [Pubkey; RECOVER_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            loss_authority: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            underlying_mint: pubkeys[4],
            loss_authority_underlying_token: pubkeys[5],
            pool_underlying_token: pubkeys[6],
            pool_authority: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<RecoverLossAccounts<'_, 'info>>
for [AccountInfo<'info>; RECOVER_LOSS_IX_ACCOUNTS_LEN] {
    fn from(accounts: RecoverLossAccounts<'_, 'info>) -> Self {
        [
            accounts.loss_authority.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.underlying_mint.clone(),
            accounts.loss_authority_underlying_token.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.pool_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RECOVER_LOSS_IX_ACCOUNTS_LEN]>
for RecoverLossAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RECOVER_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            loss_authority: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            underlying_mint: &arr[4],
            loss_authority_underlying_token: &arr[5],
            pool_underlying_token: &arr[6],
            pool_authority: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const RECOVER_LOSS_IX_DISCM: [u8; 8usize] = [32, 238, 42, 34, 64, 3, 58, 231];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecoverLossIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RecoverLossIxData(pub RecoverLossIxArgs);
impl From<RecoverLossIxArgs> for RecoverLossIxData {
    fn from(args: RecoverLossIxArgs) -> Self {
        Self(args)
    }
}
impl RecoverLossIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RECOVER_LOSS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RecoverLossIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RECOVER_LOSS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn recover_loss_ix_with_program_id(
    program_id: Pubkey,
    keys: RecoverLossKeys,
    args: RecoverLossIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RECOVER_LOSS_IX_ACCOUNTS_LEN] = keys.into();
    let data: RecoverLossIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn recover_loss_ix(
    keys: RecoverLossKeys,
    args: RecoverLossIxArgs,
) -> std::io::Result<Instruction> {
    recover_loss_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn recover_loss_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RecoverLossAccounts<'_, '_>,
    args: RecoverLossIxArgs,
) -> ProgramResult {
    let keys: RecoverLossKeys = accounts.into();
    let ix = recover_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn recover_loss_invoke(
    accounts: RecoverLossAccounts<'_, '_>,
    args: RecoverLossIxArgs,
) -> ProgramResult {
    recover_loss_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn recover_loss_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RecoverLossAccounts<'_, '_>,
    args: RecoverLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RecoverLossKeys = accounts.into();
    let ix = recover_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn recover_loss_invoke_signed(
    accounts: RecoverLossAccounts<'_, '_>,
    args: RecoverLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    recover_loss_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn recover_loss_verify_account_keys(
    accounts: RecoverLossAccounts<'_, '_>,
    keys: RecoverLossKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.loss_authority.key, keys.loss_authority),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (
            *accounts.loss_authority_underlying_token.key,
            keys.loss_authority_underlying_token,
        ),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn recover_loss_verify_writable_privileges<'me, 'info>(
    accounts: RecoverLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.loss_authority_underlying_token,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn recover_loss_verify_signer_privileges<'me, 'info>(
    accounts: RecoverLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.loss_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn recover_loss_verify_account_privileges<'me, 'info>(
    accounts: RecoverLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    recover_loss_verify_writable_privileges(accounts)?;
    recover_loss_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_MODE_ASSETS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RefreshModeAssetsAccounts<'me, 'info> {
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshModeAssetsKeys {
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
}
impl From<RefreshModeAssetsAccounts<'_, '_>> for RefreshModeAssetsKeys {
    fn from(accounts: RefreshModeAssetsAccounts) -> Self {
        Self {
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
        }
    }
}
impl From<RefreshModeAssetsKeys> for [AccountMeta; REFRESH_MODE_ASSETS_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshModeAssetsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REFRESH_MODE_ASSETS_IX_ACCOUNTS_LEN]> for RefreshModeAssetsKeys {
    fn from(pubkeys: [Pubkey; REFRESH_MODE_ASSETS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            huma_config: pubkeys[0],
            pool_config: pubkeys[1],
            pool_state: pubkeys[2],
            mode_config: pubkeys[3],
        }
    }
}
impl<'info> From<RefreshModeAssetsAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_MODE_ASSETS_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshModeAssetsAccounts<'_, 'info>) -> Self {
        [
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFRESH_MODE_ASSETS_IX_ACCOUNTS_LEN]>
for RefreshModeAssetsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REFRESH_MODE_ASSETS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            huma_config: &arr[0],
            pool_config: &arr[1],
            pool_state: &arr[2],
            mode_config: &arr[3],
        }
    }
}
pub const REFRESH_MODE_ASSETS_IX_DISCM: [u8; 8usize] = [
    190, 133, 209, 144, 221, 100, 126, 205,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshModeAssetsIxData;
impl RefreshModeAssetsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_MODE_ASSETS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_MODE_ASSETS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_mode_assets_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshModeAssetsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_MODE_ASSETS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefreshModeAssetsIxData.try_to_vec()?,
    })
}
pub fn refresh_mode_assets_ix(
    keys: RefreshModeAssetsKeys,
) -> std::io::Result<Instruction> {
    refresh_mode_assets_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn refresh_mode_assets_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshModeAssetsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshModeAssetsKeys = accounts.into();
    let ix = refresh_mode_assets_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_mode_assets_invoke(
    accounts: RefreshModeAssetsAccounts<'_, '_>,
) -> ProgramResult {
    refresh_mode_assets_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn refresh_mode_assets_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshModeAssetsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshModeAssetsKeys = accounts.into();
    let ix = refresh_mode_assets_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_mode_assets_invoke_signed(
    accounts: RefreshModeAssetsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_mode_assets_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn refresh_mode_assets_verify_account_keys(
    accounts: RefreshModeAssetsAccounts<'_, '_>,
    keys: RefreshModeAssetsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_mode_assets_verify_writable_privileges<'me, 'info>(
    accounts: RefreshModeAssetsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_mode_assets_verify_account_privileges<'me, 'info>(
    accounts: RefreshModeAssetsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_mode_assets_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_POOL_ASSETS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RefreshPoolAssetsAccounts<'me, 'info> {
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshPoolAssetsKeys {
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<RefreshPoolAssetsAccounts<'_, '_>> for RefreshPoolAssetsKeys {
    fn from(accounts: RefreshPoolAssetsAccounts) -> Self {
        Self {
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<RefreshPoolAssetsKeys> for [AccountMeta; REFRESH_POOL_ASSETS_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshPoolAssetsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REFRESH_POOL_ASSETS_IX_ACCOUNTS_LEN]> for RefreshPoolAssetsKeys {
    fn from(pubkeys: [Pubkey; REFRESH_POOL_ASSETS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            huma_config: pubkeys[0],
            pool_config: pubkeys[1],
            pool_state: pubkeys[2],
        }
    }
}
impl<'info> From<RefreshPoolAssetsAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_POOL_ASSETS_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshPoolAssetsAccounts<'_, 'info>) -> Self {
        [
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFRESH_POOL_ASSETS_IX_ACCOUNTS_LEN]>
for RefreshPoolAssetsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REFRESH_POOL_ASSETS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            huma_config: &arr[0],
            pool_config: &arr[1],
            pool_state: &arr[2],
        }
    }
}
pub const REFRESH_POOL_ASSETS_IX_DISCM: [u8; 8usize] = [
    179, 220, 112, 1, 214, 103, 3, 230,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshPoolAssetsIxData;
impl RefreshPoolAssetsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_POOL_ASSETS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_POOL_ASSETS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_pool_assets_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshPoolAssetsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_POOL_ASSETS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefreshPoolAssetsIxData.try_to_vec()?,
    })
}
pub fn refresh_pool_assets_ix(
    keys: RefreshPoolAssetsKeys,
) -> std::io::Result<Instruction> {
    refresh_pool_assets_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn refresh_pool_assets_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshPoolAssetsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshPoolAssetsKeys = accounts.into();
    let ix = refresh_pool_assets_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_pool_assets_invoke(
    accounts: RefreshPoolAssetsAccounts<'_, '_>,
) -> ProgramResult {
    refresh_pool_assets_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn refresh_pool_assets_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshPoolAssetsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshPoolAssetsKeys = accounts.into();
    let ix = refresh_pool_assets_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_pool_assets_invoke_signed(
    accounts: RefreshPoolAssetsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_pool_assets_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn refresh_pool_assets_verify_account_keys(
    accounts: RefreshPoolAssetsAccounts<'_, '_>,
    keys: RefreshPoolAssetsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_pool_assets_verify_writable_privileges<'me, 'info>(
    accounts: RefreshPoolAssetsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_pool_assets_verify_account_privileges<'me, 'info>(
    accounts: RefreshPoolAssetsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_pool_assets_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RemoveAsyncDeploymentTargetAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub async_deployment_config: &'me AccountInfo<'info>,
    pub async_deployment_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAsyncDeploymentTargetKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub async_deployment_config: Pubkey,
    pub async_deployment_state: Pubkey,
}
impl From<RemoveAsyncDeploymentTargetAccounts<'_, '_>>
for RemoveAsyncDeploymentTargetKeys {
    fn from(accounts: RemoveAsyncDeploymentTargetAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            async_deployment_config: *accounts.async_deployment_config.key,
            async_deployment_state: *accounts.async_deployment_state.key,
        }
    }
}
impl From<RemoveAsyncDeploymentTargetKeys>
for [AccountMeta; REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAsyncDeploymentTargetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.async_deployment_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.async_deployment_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for RemoveAsyncDeploymentTargetKeys {
    fn from(pubkeys: [Pubkey; REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            async_deployment_config: pubkeys[4],
            async_deployment_state: pubkeys[5],
        }
    }
}
impl<'info> From<RemoveAsyncDeploymentTargetAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAsyncDeploymentTargetAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.async_deployment_config.clone(),
            accounts.async_deployment_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for RemoveAsyncDeploymentTargetAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            async_deployment_config: &arr[4],
            async_deployment_state: &arr[5],
        }
    }
}
pub const REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_DISCM: [u8; 8usize] = [
    38, 131, 78, 127, 222, 252, 222, 246,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAsyncDeploymentTargetIxData;
impl RemoveAsyncDeploymentTargetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_async_deployment_target_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveAsyncDeploymentTargetKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ASYNC_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveAsyncDeploymentTargetIxData.try_to_vec()?,
    })
}
pub fn remove_async_deployment_target_ix(
    keys: RemoveAsyncDeploymentTargetKeys,
) -> std::io::Result<Instruction> {
    remove_async_deployment_target_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn remove_async_deployment_target_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAsyncDeploymentTargetAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveAsyncDeploymentTargetKeys = accounts.into();
    let ix = remove_async_deployment_target_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_async_deployment_target_invoke(
    accounts: RemoveAsyncDeploymentTargetAccounts<'_, '_>,
) -> ProgramResult {
    remove_async_deployment_target_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn remove_async_deployment_target_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAsyncDeploymentTargetAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAsyncDeploymentTargetKeys = accounts.into();
    let ix = remove_async_deployment_target_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_async_deployment_target_invoke_signed(
    accounts: RemoveAsyncDeploymentTargetAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_async_deployment_target_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_async_deployment_target_verify_account_keys(
    accounts: RemoveAsyncDeploymentTargetAccounts<'_, '_>,
    keys: RemoveAsyncDeploymentTargetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.async_deployment_config.key, keys.async_deployment_config),
        (*accounts.async_deployment_state.key, keys.async_deployment_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_async_deployment_target_verify_writable_privileges<'me, 'info>(
    accounts: RemoveAsyncDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.async_deployment_config,
        accounts.async_deployment_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_async_deployment_target_verify_signer_privileges<'me, 'info>(
    accounts: RemoveAsyncDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_async_deployment_target_verify_account_privileges<'me, 'info>(
    accounts: RemoveAsyncDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_async_deployment_target_verify_writable_privileges(accounts)?;
    remove_async_deployment_target_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RemoveDeploymentTargetAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub deployment_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveDeploymentTargetKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub deployment_config: Pubkey,
    pub deployment_state: Pubkey,
}
impl From<RemoveDeploymentTargetAccounts<'_, '_>> for RemoveDeploymentTargetKeys {
    fn from(accounts: RemoveDeploymentTargetAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            deployment_config: *accounts.deployment_config.key,
            deployment_state: *accounts.deployment_state.key,
        }
    }
}
impl From<RemoveDeploymentTargetKeys>
for [AccountMeta; REMOVE_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveDeploymentTargetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deployment_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for RemoveDeploymentTargetKeys {
    fn from(pubkeys: [Pubkey; REMOVE_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            deployment_config: pubkeys[4],
            deployment_state: pubkeys[5],
        }
    }
}
impl<'info> From<RemoveDeploymentTargetAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveDeploymentTargetAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.deployment_config.clone(),
            accounts.deployment_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for RemoveDeploymentTargetAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            deployment_config: &arr[4],
            deployment_state: &arr[5],
        }
    }
}
pub const REMOVE_DEPLOYMENT_TARGET_IX_DISCM: [u8; 8usize] = [
    82, 103, 198, 215, 110, 147, 220, 41,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveDeploymentTargetIxData;
impl RemoveDeploymentTargetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_DEPLOYMENT_TARGET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_DEPLOYMENT_TARGET_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_deployment_target_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveDeploymentTargetKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveDeploymentTargetIxData.try_to_vec()?,
    })
}
pub fn remove_deployment_target_ix(
    keys: RemoveDeploymentTargetKeys,
) -> std::io::Result<Instruction> {
    remove_deployment_target_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn remove_deployment_target_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveDeploymentTargetAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveDeploymentTargetKeys = accounts.into();
    let ix = remove_deployment_target_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_deployment_target_invoke(
    accounts: RemoveDeploymentTargetAccounts<'_, '_>,
) -> ProgramResult {
    remove_deployment_target_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn remove_deployment_target_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveDeploymentTargetAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveDeploymentTargetKeys = accounts.into();
    let ix = remove_deployment_target_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_deployment_target_invoke_signed(
    accounts: RemoveDeploymentTargetAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_deployment_target_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_deployment_target_verify_account_keys(
    accounts: RemoveDeploymentTargetAccounts<'_, '_>,
    keys: RemoveDeploymentTargetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.deployment_state.key, keys.deployment_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_deployment_target_verify_writable_privileges<'me, 'info>(
    accounts: RemoveDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.deployment_config,
        accounts.deployment_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_deployment_target_verify_signer_privileges<'me, 'info>(
    accounts: RemoveDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_deployment_target_verify_account_privileges<'me, 'info>(
    accounts: RemoveDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_deployment_target_verify_writable_privileges(accounts)?;
    remove_deployment_target_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemoveInstantWithdrawalLenderAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub instant_withdrawal_lender_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveInstantWithdrawalLenderKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub instant_withdrawal_lender_config: Pubkey,
}
impl From<RemoveInstantWithdrawalLenderAccounts<'_, '_>>
for RemoveInstantWithdrawalLenderKeys {
    fn from(accounts: RemoveInstantWithdrawalLenderAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            instant_withdrawal_lender_config: *accounts
                .instant_withdrawal_lender_config
                .key,
        }
    }
}
impl From<RemoveInstantWithdrawalLenderKeys>
for [AccountMeta; REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveInstantWithdrawalLenderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instant_withdrawal_lender_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN]>
for RemoveInstantWithdrawalLenderKeys {
    fn from(
        pubkeys: [Pubkey; REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            instant_withdrawal_lender_config: pubkeys[4],
        }
    }
}
impl<'info> From<RemoveInstantWithdrawalLenderAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveInstantWithdrawalLenderAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.instant_withdrawal_lender_config.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN]>
for RemoveInstantWithdrawalLenderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            instant_withdrawal_lender_config: &arr[4],
        }
    }
}
pub const REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_DISCM: [u8; 8usize] = [
    138, 101, 118, 74, 113, 190, 76, 214,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveInstantWithdrawalLenderIxArgs {
    pub lender: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveInstantWithdrawalLenderIxData(pub RemoveInstantWithdrawalLenderIxArgs);
impl From<RemoveInstantWithdrawalLenderIxArgs> for RemoveInstantWithdrawalLenderIxData {
    fn from(args: RemoveInstantWithdrawalLenderIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveInstantWithdrawalLenderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveInstantWithdrawalLenderIxArgs {
                lender,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lender, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_instant_withdrawal_lender_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveInstantWithdrawalLenderKeys,
    args: RemoveInstantWithdrawalLenderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_INSTANT_WITHDRAWAL_LENDER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: RemoveInstantWithdrawalLenderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_instant_withdrawal_lender_ix(
    keys: RemoveInstantWithdrawalLenderKeys,
    args: RemoveInstantWithdrawalLenderIxArgs,
) -> std::io::Result<Instruction> {
    remove_instant_withdrawal_lender_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn remove_instant_withdrawal_lender_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveInstantWithdrawalLenderAccounts<'_, '_>,
    args: RemoveInstantWithdrawalLenderIxArgs,
) -> ProgramResult {
    let keys: RemoveInstantWithdrawalLenderKeys = accounts.into();
    let ix = remove_instant_withdrawal_lender_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_instant_withdrawal_lender_invoke(
    accounts: RemoveInstantWithdrawalLenderAccounts<'_, '_>,
    args: RemoveInstantWithdrawalLenderIxArgs,
) -> ProgramResult {
    remove_instant_withdrawal_lender_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn remove_instant_withdrawal_lender_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveInstantWithdrawalLenderAccounts<'_, '_>,
    args: RemoveInstantWithdrawalLenderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveInstantWithdrawalLenderKeys = accounts.into();
    let ix = remove_instant_withdrawal_lender_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_instant_withdrawal_lender_invoke_signed(
    accounts: RemoveInstantWithdrawalLenderAccounts<'_, '_>,
    args: RemoveInstantWithdrawalLenderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_instant_withdrawal_lender_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_instant_withdrawal_lender_verify_account_keys(
    accounts: RemoveInstantWithdrawalLenderAccounts<'_, '_>,
    keys: RemoveInstantWithdrawalLenderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (
            *accounts.instant_withdrawal_lender_config.key,
            keys.instant_withdrawal_lender_config,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_instant_withdrawal_lender_verify_writable_privileges<'me, 'info>(
    accounts: RemoveInstantWithdrawalLenderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.instant_withdrawal_lender_config,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_instant_withdrawal_lender_verify_signer_privileges<'me, 'info>(
    accounts: RemoveInstantWithdrawalLenderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_instant_withdrawal_lender_verify_account_privileges<'me, 'info>(
    accounts: RemoveInstantWithdrawalLenderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_instant_withdrawal_lender_verify_writable_privileges(accounts)?;
    remove_instant_withdrawal_lender_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RemoveManualStrategyManagerAccounts<'me, 'info> {
    pub pool_owner: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub huma_owner: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub manual_strategy_manager: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveManualStrategyManagerKeys {
    pub pool_owner: Pubkey,
    pub huma_config: Pubkey,
    pub huma_owner: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub manual_strategy_manager: Pubkey,
}
impl From<RemoveManualStrategyManagerAccounts<'_, '_>>
for RemoveManualStrategyManagerKeys {
    fn from(accounts: RemoveManualStrategyManagerAccounts) -> Self {
        Self {
            pool_owner: *accounts.pool_owner.key,
            huma_config: *accounts.huma_config.key,
            huma_owner: *accounts.huma_owner.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            manual_strategy_manager: *accounts.manual_strategy_manager.key,
        }
    }
}
impl From<RemoveManualStrategyManagerKeys>
for [AccountMeta; REMOVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveManualStrategyManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.manual_strategy_manager,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]>
for RemoveManualStrategyManagerKeys {
    fn from(pubkeys: [Pubkey; REMOVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner: pubkeys[0],
            huma_config: pubkeys[1],
            huma_owner: pubkeys[2],
            pool_config: pubkeys[3],
            pool_state: pubkeys[4],
            manual_strategy_manager: pubkeys[5],
        }
    }
}
impl<'info> From<RemoveManualStrategyManagerAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveManualStrategyManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_owner.clone(),
            accounts.huma_config.clone(),
            accounts.huma_owner.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.manual_strategy_manager.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN]>
for RemoveManualStrategyManagerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_owner: &arr[0],
            huma_config: &arr[1],
            huma_owner: &arr[2],
            pool_config: &arr[3],
            pool_state: &arr[4],
            manual_strategy_manager: &arr[5],
        }
    }
}
pub const REMOVE_MANUAL_STRATEGY_MANAGER_IX_DISCM: [u8; 8usize] = [
    132, 184, 107, 166, 164, 53, 188, 0,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveManualStrategyManagerIxData;
impl RemoveManualStrategyManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_MANUAL_STRATEGY_MANAGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_MANUAL_STRATEGY_MANAGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_manual_strategy_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveManualStrategyManagerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_MANUAL_STRATEGY_MANAGER_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveManualStrategyManagerIxData.try_to_vec()?,
    })
}
pub fn remove_manual_strategy_manager_ix(
    keys: RemoveManualStrategyManagerKeys,
) -> std::io::Result<Instruction> {
    remove_manual_strategy_manager_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn remove_manual_strategy_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveManualStrategyManagerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveManualStrategyManagerKeys = accounts.into();
    let ix = remove_manual_strategy_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_manual_strategy_manager_invoke(
    accounts: RemoveManualStrategyManagerAccounts<'_, '_>,
) -> ProgramResult {
    remove_manual_strategy_manager_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn remove_manual_strategy_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveManualStrategyManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveManualStrategyManagerKeys = accounts.into();
    let ix = remove_manual_strategy_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_manual_strategy_manager_invoke_signed(
    accounts: RemoveManualStrategyManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_manual_strategy_manager_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_manual_strategy_manager_verify_account_keys(
    accounts: RemoveManualStrategyManagerAccounts<'_, '_>,
    keys: RemoveManualStrategyManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_owner.key, keys.pool_owner),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.huma_owner.key, keys.huma_owner),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.manual_strategy_manager.key, keys.manual_strategy_manager),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_manual_strategy_manager_verify_writable_privileges<'me, 'info>(
    accounts: RemoveManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.huma_owner, accounts.manual_strategy_manager] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_manual_strategy_manager_verify_signer_privileges<'me, 'info>(
    accounts: RemoveManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_manual_strategy_manager_verify_account_privileges<'me, 'info>(
    accounts: RemoveManualStrategyManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_manual_strategy_manager_verify_writable_privileges(accounts)?;
    remove_manual_strategy_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_MODE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RemoveModeAccounts<'me, 'info> {
    pub pool_owner: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub pool_mode_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveModeKeys {
    pub pool_owner: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_authority: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub pool_mode_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<RemoveModeAccounts<'_, '_>> for RemoveModeKeys {
    fn from(accounts: RemoveModeAccounts) -> Self {
        Self {
            pool_owner: *accounts.pool_owner.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_authority: *accounts.pool_authority.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            pool_mode_token: *accounts.pool_mode_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RemoveModeKeys> for [AccountMeta; REMOVE_MODE_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveModeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mode_token,
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
impl From<[Pubkey; REMOVE_MODE_IX_ACCOUNTS_LEN]> for RemoveModeKeys {
    fn from(pubkeys: [Pubkey; REMOVE_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner: pubkeys[0],
            pool_config: pubkeys[1],
            pool_state: pubkeys[2],
            pool_authority: pubkeys[3],
            mode_config: pubkeys[4],
            mode_mint: pubkeys[5],
            pool_mode_token: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<RemoveModeAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_MODE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveModeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_owner.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_authority.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.pool_mode_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_MODE_IX_ACCOUNTS_LEN]>
for RemoveModeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner: &arr[0],
            pool_config: &arr[1],
            pool_state: &arr[2],
            pool_authority: &arr[3],
            mode_config: &arr[4],
            mode_mint: &arr[5],
            pool_mode_token: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const REMOVE_MODE_IX_DISCM: [u8; 8usize] = [36, 197, 69, 56, 254, 146, 192, 62];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveModeIxArgs {
    pub mode_id: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveModeIxData(pub RemoveModeIxArgs);
impl From<RemoveModeIxArgs> for RemoveModeIxData {
    fn from(args: RemoveModeIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveModeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_MODE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mode_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RemoveModeIxArgs { mode_id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_MODE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mode_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_mode_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveModeKeys,
    args: RemoveModeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_MODE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveModeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_mode_ix(
    keys: RemoveModeKeys,
    args: RemoveModeIxArgs,
) -> std::io::Result<Instruction> {
    remove_mode_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn remove_mode_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveModeAccounts<'_, '_>,
    args: RemoveModeIxArgs,
) -> ProgramResult {
    let keys: RemoveModeKeys = accounts.into();
    let ix = remove_mode_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_mode_invoke(
    accounts: RemoveModeAccounts<'_, '_>,
    args: RemoveModeIxArgs,
) -> ProgramResult {
    remove_mode_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn remove_mode_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveModeAccounts<'_, '_>,
    args: RemoveModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveModeKeys = accounts.into();
    let ix = remove_mode_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_mode_invoke_signed(
    accounts: RemoveModeAccounts<'_, '_>,
    args: RemoveModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_mode_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn remove_mode_verify_account_keys(
    accounts: RemoveModeAccounts<'_, '_>,
    keys: RemoveModeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_owner.key, keys.pool_owner),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.pool_mode_token.key, keys.pool_mode_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_mode_verify_writable_privileges<'me, 'info>(
    accounts: RemoveModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_owner,
        accounts.pool_state,
        accounts.mode_config,
        accounts.mode_mint,
        accounts.pool_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_mode_verify_signer_privileges<'me, 'info>(
    accounts: RemoveModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_mode_verify_account_privileges<'me, 'info>(
    accounts: RemoveModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_mode_verify_writable_privileges(accounts)?;
    remove_mode_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_POOL_OPERATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemovePoolOperatorAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_operator_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePoolOperatorKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_operator_config: Pubkey,
}
impl From<RemovePoolOperatorAccounts<'_, '_>> for RemovePoolOperatorKeys {
    fn from(accounts: RemovePoolOperatorAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_operator_config: *accounts.pool_operator_config.key,
        }
    }
}
impl From<RemovePoolOperatorKeys>
for [AccountMeta; REMOVE_POOL_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: RemovePoolOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_operator_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_POOL_OPERATOR_IX_ACCOUNTS_LEN]> for RemovePoolOperatorKeys {
    fn from(pubkeys: [Pubkey; REMOVE_POOL_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            pool_operator_config: pubkeys[4],
        }
    }
}
impl<'info> From<RemovePoolOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_POOL_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemovePoolOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_operator_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_POOL_OPERATOR_IX_ACCOUNTS_LEN]>
for RemovePoolOperatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_POOL_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            pool_operator_config: &arr[4],
        }
    }
}
pub const REMOVE_POOL_OPERATOR_IX_DISCM: [u8; 8usize] = [
    70, 188, 152, 173, 117, 213, 144, 195,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemovePoolOperatorIxArgs {
    pub operator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePoolOperatorIxData(pub RemovePoolOperatorIxArgs);
impl From<RemovePoolOperatorIxArgs> for RemovePoolOperatorIxData {
    fn from(args: RemovePoolOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl RemovePoolOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_POOL_OPERATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemovePoolOperatorIxArgs {
                operator,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_POOL_OPERATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.operator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_pool_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: RemovePoolOperatorKeys,
    args: RemovePoolOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_POOL_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemovePoolOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_pool_operator_ix(
    keys: RemovePoolOperatorKeys,
    args: RemovePoolOperatorIxArgs,
) -> std::io::Result<Instruction> {
    remove_pool_operator_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn remove_pool_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemovePoolOperatorAccounts<'_, '_>,
    args: RemovePoolOperatorIxArgs,
) -> ProgramResult {
    let keys: RemovePoolOperatorKeys = accounts.into();
    let ix = remove_pool_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_pool_operator_invoke(
    accounts: RemovePoolOperatorAccounts<'_, '_>,
    args: RemovePoolOperatorIxArgs,
) -> ProgramResult {
    remove_pool_operator_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn remove_pool_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemovePoolOperatorAccounts<'_, '_>,
    args: RemovePoolOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemovePoolOperatorKeys = accounts.into();
    let ix = remove_pool_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_pool_operator_invoke_signed(
    accounts: RemovePoolOperatorAccounts<'_, '_>,
    args: RemovePoolOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_pool_operator_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_pool_operator_verify_account_keys(
    accounts: RemovePoolOperatorAccounts<'_, '_>,
    keys: RemovePoolOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_operator_config.key, keys.pool_operator_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_pool_operator_verify_writable_privileges<'me, 'info>(
    accounts: RemovePoolOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.pool_operator_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_pool_operator_verify_signer_privileges<'me, 'info>(
    accounts: RemovePoolOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_pool_operator_verify_account_privileges<'me, 'info>(
    accounts: RemovePoolOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_pool_operator_verify_writable_privileges(accounts)?;
    remove_pool_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemoveStrategyManagerWalletAccounts<'me, 'info> {
    pub pool_owner: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveStrategyManagerWalletKeys {
    pub pool_owner: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
}
impl From<RemoveStrategyManagerWalletAccounts<'_, '_>>
for RemoveStrategyManagerWalletKeys {
    fn from(accounts: RemoveStrategyManagerWalletAccounts) -> Self {
        Self {
            pool_owner: *accounts.pool_owner.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
        }
    }
}
impl From<RemoveStrategyManagerWalletKeys>
for [AccountMeta; REMOVE_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveStrategyManagerWalletKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN]>
for RemoveStrategyManagerWalletKeys {
    fn from(pubkeys: [Pubkey; REMOVE_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_owner: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            strategy_manager_wallet: pubkeys[4],
        }
    }
}
impl<'info> From<RemoveStrategyManagerWalletAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveStrategyManagerWalletAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_owner.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.strategy_manager_wallet.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN]>
for RemoveStrategyManagerWalletAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_owner: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            strategy_manager_wallet: &arr[4],
        }
    }
}
pub const REMOVE_STRATEGY_MANAGER_WALLET_IX_DISCM: [u8; 8usize] = [
    66, 249, 21, 34, 38, 197, 167, 96,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveStrategyManagerWalletIxData;
impl RemoveStrategyManagerWalletIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_STRATEGY_MANAGER_WALLET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_STRATEGY_MANAGER_WALLET_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_strategy_manager_wallet_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveStrategyManagerWalletKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_STRATEGY_MANAGER_WALLET_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveStrategyManagerWalletIxData.try_to_vec()?,
    })
}
pub fn remove_strategy_manager_wallet_ix(
    keys: RemoveStrategyManagerWalletKeys,
) -> std::io::Result<Instruction> {
    remove_strategy_manager_wallet_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn remove_strategy_manager_wallet_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveStrategyManagerWalletAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveStrategyManagerWalletKeys = accounts.into();
    let ix = remove_strategy_manager_wallet_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_strategy_manager_wallet_invoke(
    accounts: RemoveStrategyManagerWalletAccounts<'_, '_>,
) -> ProgramResult {
    remove_strategy_manager_wallet_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn remove_strategy_manager_wallet_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveStrategyManagerWalletAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveStrategyManagerWalletKeys = accounts.into();
    let ix = remove_strategy_manager_wallet_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_strategy_manager_wallet_invoke_signed(
    accounts: RemoveStrategyManagerWalletAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_strategy_manager_wallet_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_strategy_manager_wallet_verify_account_keys(
    accounts: RemoveStrategyManagerWalletAccounts<'_, '_>,
    keys: RemoveStrategyManagerWalletKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_owner.key, keys.pool_owner),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_strategy_manager_wallet_verify_writable_privileges<'me, 'info>(
    accounts: RemoveStrategyManagerWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_owner, accounts.strategy_manager_wallet] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_strategy_manager_wallet_verify_signer_privileges<'me, 'info>(
    accounts: RemoveStrategyManagerWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_strategy_manager_wallet_verify_account_privileges<'me, 'info>(
    accounts: RemoveStrategyManagerWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_strategy_manager_wallet_verify_writable_privileges(accounts)?;
    remove_strategy_manager_wallet_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_HUMA_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetHumaConfigAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub new_huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetHumaConfigKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub new_huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<SetHumaConfigAccounts<'_, '_>> for SetHumaConfigKeys {
    fn from(accounts: SetHumaConfigAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            new_huma_config: *accounts.new_huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<SetHumaConfigKeys> for [AccountMeta; SET_HUMA_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: SetHumaConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_HUMA_CONFIG_IX_ACCOUNTS_LEN]> for SetHumaConfigKeys {
    fn from(pubkeys: [Pubkey; SET_HUMA_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            new_huma_config: pubkeys[2],
            pool_config: pubkeys[3],
            pool_state: pubkeys[4],
        }
    }
}
impl<'info> From<SetHumaConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_HUMA_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetHumaConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.new_huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_HUMA_CONFIG_IX_ACCOUNTS_LEN]>
for SetHumaConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_HUMA_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            new_huma_config: &arr[2],
            pool_config: &arr[3],
            pool_state: &arr[4],
        }
    }
}
pub const SET_HUMA_CONFIG_IX_DISCM: [u8; 8usize] = [183, 30, 191, 113, 7, 254, 204, 0];
#[derive(Clone, Debug, PartialEq)]
pub struct SetHumaConfigIxData;
impl SetHumaConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_HUMA_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_HUMA_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_huma_config_ix_with_program_id(
    program_id: Pubkey,
    keys: SetHumaConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_HUMA_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetHumaConfigIxData.try_to_vec()?,
    })
}
pub fn set_huma_config_ix(keys: SetHumaConfigKeys) -> std::io::Result<Instruction> {
    set_huma_config_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn set_huma_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetHumaConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetHumaConfigKeys = accounts.into();
    let ix = set_huma_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_huma_config_invoke(accounts: SetHumaConfigAccounts<'_, '_>) -> ProgramResult {
    set_huma_config_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn set_huma_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetHumaConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetHumaConfigKeys = accounts.into();
    let ix = set_huma_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_huma_config_invoke_signed(
    accounts: SetHumaConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_huma_config_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, seeds)
}
pub fn set_huma_config_verify_account_keys(
    accounts: SetHumaConfigAccounts<'_, '_>,
    keys: SetHumaConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.new_huma_config.key, keys.new_huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_huma_config_verify_writable_privileges<'me, 'info>(
    accounts: SetHumaConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_huma_config_verify_signer_privileges<'me, 'info>(
    accounts: SetHumaConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_huma_config_verify_account_privileges<'me, 'info>(
    accounts: SetHumaConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_huma_config_verify_writable_privileges(accounts)?;
    set_huma_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetInstantWithdrawalFeeConfigsAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetInstantWithdrawalFeeConfigsKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<SetInstantWithdrawalFeeConfigsAccounts<'_, '_>>
for SetInstantWithdrawalFeeConfigsKeys {
    fn from(accounts: SetInstantWithdrawalFeeConfigsAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<SetInstantWithdrawalFeeConfigsKeys>
for [AccountMeta; SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetInstantWithdrawalFeeConfigsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_ACCOUNTS_LEN]>
for SetInstantWithdrawalFeeConfigsKeys {
    fn from(
        pubkeys: [Pubkey; SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
        }
    }
}
impl<'info> From<SetInstantWithdrawalFeeConfigsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetInstantWithdrawalFeeConfigsAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_ACCOUNTS_LEN]>
for SetInstantWithdrawalFeeConfigsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
        }
    }
}
pub const SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_DISCM: [u8; 8usize] = [
    86, 166, 57, 79, 173, 3, 153, 63,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetInstantWithdrawalFeeConfigsIxArgs {
    pub fee_configs: Vec<InstantWithdrawalFeeConfigInput>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetInstantWithdrawalFeeConfigsIxData(
    pub SetInstantWithdrawalFeeConfigsIxArgs,
);
impl From<SetInstantWithdrawalFeeConfigsIxArgs>
for SetInstantWithdrawalFeeConfigsIxData {
    fn from(args: SetInstantWithdrawalFeeConfigsIxArgs) -> Self {
        Self(args)
    }
}
impl SetInstantWithdrawalFeeConfigsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_configs: Vec<InstantWithdrawalFeeConfigInput> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SetInstantWithdrawalFeeConfigsIxArgs {
                fee_configs,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_configs, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_instant_withdrawal_fee_configs_ix_with_program_id(
    program_id: Pubkey,
    keys: SetInstantWithdrawalFeeConfigsKeys,
    args: SetInstantWithdrawalFeeConfigsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_INSTANT_WITHDRAWAL_FEE_CONFIGS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetInstantWithdrawalFeeConfigsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_instant_withdrawal_fee_configs_ix(
    keys: SetInstantWithdrawalFeeConfigsKeys,
    args: SetInstantWithdrawalFeeConfigsIxArgs,
) -> std::io::Result<Instruction> {
    set_instant_withdrawal_fee_configs_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn set_instant_withdrawal_fee_configs_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetInstantWithdrawalFeeConfigsAccounts<'_, '_>,
    args: SetInstantWithdrawalFeeConfigsIxArgs,
) -> ProgramResult {
    let keys: SetInstantWithdrawalFeeConfigsKeys = accounts.into();
    let ix = set_instant_withdrawal_fee_configs_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_instant_withdrawal_fee_configs_invoke(
    accounts: SetInstantWithdrawalFeeConfigsAccounts<'_, '_>,
    args: SetInstantWithdrawalFeeConfigsIxArgs,
) -> ProgramResult {
    set_instant_withdrawal_fee_configs_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_instant_withdrawal_fee_configs_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetInstantWithdrawalFeeConfigsAccounts<'_, '_>,
    args: SetInstantWithdrawalFeeConfigsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetInstantWithdrawalFeeConfigsKeys = accounts.into();
    let ix = set_instant_withdrawal_fee_configs_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_instant_withdrawal_fee_configs_invoke_signed(
    accounts: SetInstantWithdrawalFeeConfigsAccounts<'_, '_>,
    args: SetInstantWithdrawalFeeConfigsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_instant_withdrawal_fee_configs_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_instant_withdrawal_fee_configs_verify_account_keys(
    accounts: SetInstantWithdrawalFeeConfigsAccounts<'_, '_>,
    keys: SetInstantWithdrawalFeeConfigsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_fee_configs_verify_writable_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalFeeConfigsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_fee_configs_verify_signer_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalFeeConfigsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_fee_configs_verify_account_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalFeeConfigsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_instant_withdrawal_fee_configs_verify_writable_privileges(accounts)?;
    set_instant_withdrawal_fee_configs_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SetInstantWithdrawalLiquiditySourceAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetInstantWithdrawalLiquiditySourceKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub deployment_config: Pubkey,
}
impl From<SetInstantWithdrawalLiquiditySourceAccounts<'_, '_>>
for SetInstantWithdrawalLiquiditySourceKeys {
    fn from(accounts: SetInstantWithdrawalLiquiditySourceAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            deployment_config: *accounts.deployment_config.key,
        }
    }
}
impl From<SetInstantWithdrawalLiquiditySourceKeys>
for [AccountMeta; SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetInstantWithdrawalLiquiditySourceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN]>
for SetInstantWithdrawalLiquiditySourceKeys {
    fn from(
        pubkeys: [Pubkey; SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            strategy_manager_wallet: pubkeys[4],
            deployment_config: pubkeys[5],
        }
    }
}
impl<'info> From<SetInstantWithdrawalLiquiditySourceAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetInstantWithdrawalLiquiditySourceAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.deployment_config.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN],
> for SetInstantWithdrawalLiquiditySourceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            strategy_manager_wallet: &arr[4],
            deployment_config: &arr[5],
        }
    }
}
pub const SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM: [u8; 8usize] = [
    104, 139, 161, 239, 40, 211, 26, 252,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetInstantWithdrawalLiquiditySourceIxData;
impl SetInstantWithdrawalLiquiditySourceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_instant_withdrawal_liquidity_source_ix_with_program_id(
    program_id: Pubkey,
    keys: SetInstantWithdrawalLiquiditySourceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetInstantWithdrawalLiquiditySourceIxData.try_to_vec()?,
    })
}
pub fn set_instant_withdrawal_liquidity_source_ix(
    keys: SetInstantWithdrawalLiquiditySourceKeys,
) -> std::io::Result<Instruction> {
    set_instant_withdrawal_liquidity_source_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn set_instant_withdrawal_liquidity_source_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetInstantWithdrawalLiquiditySourceKeys = accounts.into();
    let ix = set_instant_withdrawal_liquidity_source_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_instant_withdrawal_liquidity_source_invoke(
    accounts: SetInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
) -> ProgramResult {
    set_instant_withdrawal_liquidity_source_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
    )
}
pub fn set_instant_withdrawal_liquidity_source_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetInstantWithdrawalLiquiditySourceKeys = accounts.into();
    let ix = set_instant_withdrawal_liquidity_source_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_instant_withdrawal_liquidity_source_invoke_signed(
    accounts: SetInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_instant_withdrawal_liquidity_source_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_instant_withdrawal_liquidity_source_verify_account_keys(
    accounts: SetInstantWithdrawalLiquiditySourceAccounts<'_, '_>,
    keys: SetInstantWithdrawalLiquiditySourceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.deployment_config.key, keys.deployment_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_liquidity_source_verify_writable_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalLiquiditySourceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_liquidity_source_verify_signer_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalLiquiditySourceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_liquidity_source_verify_account_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalLiquiditySourceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_instant_withdrawal_liquidity_source_verify_writable_privileges(accounts)?;
    set_instant_withdrawal_liquidity_source_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetInstantWithdrawalReserveLimitAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetInstantWithdrawalReserveLimitKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<SetInstantWithdrawalReserveLimitAccounts<'_, '_>>
for SetInstantWithdrawalReserveLimitKeys {
    fn from(accounts: SetInstantWithdrawalReserveLimitAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<SetInstantWithdrawalReserveLimitKeys>
for [AccountMeta; SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetInstantWithdrawalReserveLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_ACCOUNTS_LEN]>
for SetInstantWithdrawalReserveLimitKeys {
    fn from(
        pubkeys: [Pubkey; SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
        }
    }
}
impl<'info> From<SetInstantWithdrawalReserveLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetInstantWithdrawalReserveLimitAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_ACCOUNTS_LEN]>
for SetInstantWithdrawalReserveLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
        }
    }
}
pub const SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_DISCM: [u8; 8usize] = [
    119, 215, 114, 208, 241, 241, 178, 138,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetInstantWithdrawalReserveLimitIxArgs {
    pub new_reserve_limit: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetInstantWithdrawalReserveLimitIxData(
    pub SetInstantWithdrawalReserveLimitIxArgs,
);
impl From<SetInstantWithdrawalReserveLimitIxArgs>
for SetInstantWithdrawalReserveLimitIxData {
    fn from(args: SetInstantWithdrawalReserveLimitIxArgs) -> Self {
        Self(args)
    }
}
impl SetInstantWithdrawalReserveLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_reserve_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetInstantWithdrawalReserveLimitIxArgs {
                new_reserve_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_reserve_limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_instant_withdrawal_reserve_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: SetInstantWithdrawalReserveLimitKeys,
    args: SetInstantWithdrawalReserveLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_INSTANT_WITHDRAWAL_RESERVE_LIMIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetInstantWithdrawalReserveLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_instant_withdrawal_reserve_limit_ix(
    keys: SetInstantWithdrawalReserveLimitKeys,
    args: SetInstantWithdrawalReserveLimitIxArgs,
) -> std::io::Result<Instruction> {
    set_instant_withdrawal_reserve_limit_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn set_instant_withdrawal_reserve_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetInstantWithdrawalReserveLimitAccounts<'_, '_>,
    args: SetInstantWithdrawalReserveLimitIxArgs,
) -> ProgramResult {
    let keys: SetInstantWithdrawalReserveLimitKeys = accounts.into();
    let ix = set_instant_withdrawal_reserve_limit_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_instant_withdrawal_reserve_limit_invoke(
    accounts: SetInstantWithdrawalReserveLimitAccounts<'_, '_>,
    args: SetInstantWithdrawalReserveLimitIxArgs,
) -> ProgramResult {
    set_instant_withdrawal_reserve_limit_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_instant_withdrawal_reserve_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetInstantWithdrawalReserveLimitAccounts<'_, '_>,
    args: SetInstantWithdrawalReserveLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetInstantWithdrawalReserveLimitKeys = accounts.into();
    let ix = set_instant_withdrawal_reserve_limit_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_instant_withdrawal_reserve_limit_invoke_signed(
    accounts: SetInstantWithdrawalReserveLimitAccounts<'_, '_>,
    args: SetInstantWithdrawalReserveLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_instant_withdrawal_reserve_limit_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_instant_withdrawal_reserve_limit_verify_account_keys(
    accounts: SetInstantWithdrawalReserveLimitAccounts<'_, '_>,
    keys: SetInstantWithdrawalReserveLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_reserve_limit_verify_writable_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalReserveLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_reserve_limit_verify_signer_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalReserveLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_instant_withdrawal_reserve_limit_verify_account_privileges<'me, 'info>(
    accounts: SetInstantWithdrawalReserveLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_instant_withdrawal_reserve_limit_verify_writable_privileges(accounts)?;
    set_instant_withdrawal_reserve_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LIQUID_ASSETS_DEPLOYED_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetLiquidAssetsDeployedAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLiquidAssetsDeployedKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
}
impl From<SetLiquidAssetsDeployedAccounts<'_, '_>> for SetLiquidAssetsDeployedKeys {
    fn from(accounts: SetLiquidAssetsDeployedAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
        }
    }
}
impl From<SetLiquidAssetsDeployedKeys>
for [AccountMeta; SET_LIQUID_ASSETS_DEPLOYED_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLiquidAssetsDeployedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LIQUID_ASSETS_DEPLOYED_IX_ACCOUNTS_LEN]>
for SetLiquidAssetsDeployedKeys {
    fn from(pubkeys: [Pubkey; SET_LIQUID_ASSETS_DEPLOYED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            strategy_manager_wallet: pubkeys[4],
        }
    }
}
impl<'info> From<SetLiquidAssetsDeployedAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LIQUID_ASSETS_DEPLOYED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLiquidAssetsDeployedAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.strategy_manager_wallet.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_LIQUID_ASSETS_DEPLOYED_IX_ACCOUNTS_LEN]>
for SetLiquidAssetsDeployedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_LIQUID_ASSETS_DEPLOYED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            strategy_manager_wallet: &arr[4],
        }
    }
}
pub const SET_LIQUID_ASSETS_DEPLOYED_IX_DISCM: [u8; 8usize] = [
    187, 170, 45, 216, 14, 41, 151, 64,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLiquidAssetsDeployedIxArgs {
    pub liquid_assets_deployed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLiquidAssetsDeployedIxData(pub SetLiquidAssetsDeployedIxArgs);
impl From<SetLiquidAssetsDeployedIxArgs> for SetLiquidAssetsDeployedIxData {
    fn from(args: SetLiquidAssetsDeployedIxArgs) -> Self {
        Self(args)
    }
}
impl SetLiquidAssetsDeployedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LIQUID_ASSETS_DEPLOYED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquid_assets_deployed: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetLiquidAssetsDeployedIxArgs {
                liquid_assets_deployed,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LIQUID_ASSETS_DEPLOYED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquid_assets_deployed, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_liquid_assets_deployed_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLiquidAssetsDeployedKeys,
    args: SetLiquidAssetsDeployedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LIQUID_ASSETS_DEPLOYED_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLiquidAssetsDeployedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_liquid_assets_deployed_ix(
    keys: SetLiquidAssetsDeployedKeys,
    args: SetLiquidAssetsDeployedIxArgs,
) -> std::io::Result<Instruction> {
    set_liquid_assets_deployed_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn set_liquid_assets_deployed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLiquidAssetsDeployedAccounts<'_, '_>,
    args: SetLiquidAssetsDeployedIxArgs,
) -> ProgramResult {
    let keys: SetLiquidAssetsDeployedKeys = accounts.into();
    let ix = set_liquid_assets_deployed_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_liquid_assets_deployed_invoke(
    accounts: SetLiquidAssetsDeployedAccounts<'_, '_>,
    args: SetLiquidAssetsDeployedIxArgs,
) -> ProgramResult {
    set_liquid_assets_deployed_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn set_liquid_assets_deployed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLiquidAssetsDeployedAccounts<'_, '_>,
    args: SetLiquidAssetsDeployedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLiquidAssetsDeployedKeys = accounts.into();
    let ix = set_liquid_assets_deployed_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_liquid_assets_deployed_invoke_signed(
    accounts: SetLiquidAssetsDeployedAccounts<'_, '_>,
    args: SetLiquidAssetsDeployedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_liquid_assets_deployed_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_liquid_assets_deployed_verify_account_keys(
    accounts: SetLiquidAssetsDeployedAccounts<'_, '_>,
    keys: SetLiquidAssetsDeployedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_liquid_assets_deployed_verify_writable_privileges<'me, 'info>(
    accounts: SetLiquidAssetsDeployedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_liquid_assets_deployed_verify_signer_privileges<'me, 'info>(
    accounts: SetLiquidAssetsDeployedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_liquid_assets_deployed_verify_account_privileges<'me, 'info>(
    accounts: SetLiquidAssetsDeployedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_liquid_assets_deployed_verify_writable_privileges(accounts)?;
    set_liquid_assets_deployed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LOSS_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetLossAuthorityAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLossAuthorityKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<SetLossAuthorityAccounts<'_, '_>> for SetLossAuthorityKeys {
    fn from(accounts: SetLossAuthorityAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<SetLossAuthorityKeys> for [AccountMeta; SET_LOSS_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLossAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LOSS_AUTHORITY_IX_ACCOUNTS_LEN]> for SetLossAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_LOSS_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
        }
    }
}
impl<'info> From<SetLossAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LOSS_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLossAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LOSS_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetLossAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_LOSS_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
        }
    }
}
pub const SET_LOSS_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    73, 183, 94, 224, 117, 161, 23, 52,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLossAuthorityIxArgs {
    pub new_loss_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLossAuthorityIxData(pub SetLossAuthorityIxArgs);
impl From<SetLossAuthorityIxArgs> for SetLossAuthorityIxData {
    fn from(args: SetLossAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl SetLossAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LOSS_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetLossAuthorityIxArgs {
                new_loss_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LOSS_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_loss_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_loss_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLossAuthorityKeys,
    args: SetLossAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LOSS_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLossAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_loss_authority_ix(
    keys: SetLossAuthorityKeys,
    args: SetLossAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    set_loss_authority_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn set_loss_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLossAuthorityAccounts<'_, '_>,
    args: SetLossAuthorityIxArgs,
) -> ProgramResult {
    let keys: SetLossAuthorityKeys = accounts.into();
    let ix = set_loss_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_loss_authority_invoke(
    accounts: SetLossAuthorityAccounts<'_, '_>,
    args: SetLossAuthorityIxArgs,
) -> ProgramResult {
    set_loss_authority_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn set_loss_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLossAuthorityAccounts<'_, '_>,
    args: SetLossAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLossAuthorityKeys = accounts.into();
    let ix = set_loss_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_loss_authority_invoke_signed(
    accounts: SetLossAuthorityAccounts<'_, '_>,
    args: SetLossAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_loss_authority_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_loss_authority_verify_account_keys(
    accounts: SetLossAuthorityAccounts<'_, '_>,
    keys: SetLossAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_loss_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetLossAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_loss_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetLossAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_loss_authority_verify_account_privileges<'me, 'info>(
    accounts: SetLossAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_loss_authority_verify_writable_privileges(accounts)?;
    set_loss_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LP_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetLpConfigAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLpConfigKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub underlying_mint: Pubkey,
}
impl From<SetLpConfigAccounts<'_, '_>> for SetLpConfigKeys {
    fn from(accounts: SetLpConfigAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            underlying_mint: *accounts.underlying_mint.key,
        }
    }
}
impl From<SetLpConfigKeys> for [AccountMeta; SET_LP_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLpConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LP_CONFIG_IX_ACCOUNTS_LEN]> for SetLpConfigKeys {
    fn from(pubkeys: [Pubkey; SET_LP_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            underlying_mint: pubkeys[4],
        }
    }
}
impl<'info> From<SetLpConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LP_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLpConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.underlying_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LP_CONFIG_IX_ACCOUNTS_LEN]>
for SetLpConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_LP_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            underlying_mint: &arr[4],
        }
    }
}
pub const SET_LP_CONFIG_IX_DISCM: [u8; 8usize] = [243, 188, 179, 176, 217, 83, 174, 65];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLpConfigIxArgs {
    pub configs: LPConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLpConfigIxData(pub SetLpConfigIxArgs);
impl From<SetLpConfigIxArgs> for SetLpConfigIxData {
    fn from(args: SetLpConfigIxArgs) -> Self {
        Self(args)
    }
}
impl SetLpConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LP_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let configs = <LPConfig>::deserialize(&mut reader)?;
        Ok(Self(SetLpConfigIxArgs { configs }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LP_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.configs, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_lp_config_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLpConfigKeys,
    args: SetLpConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LP_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLpConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lp_config_ix(
    keys: SetLpConfigKeys,
    args: SetLpConfigIxArgs,
) -> std::io::Result<Instruction> {
    set_lp_config_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn set_lp_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLpConfigAccounts<'_, '_>,
    args: SetLpConfigIxArgs,
) -> ProgramResult {
    let keys: SetLpConfigKeys = accounts.into();
    let ix = set_lp_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_lp_config_invoke(
    accounts: SetLpConfigAccounts<'_, '_>,
    args: SetLpConfigIxArgs,
) -> ProgramResult {
    set_lp_config_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn set_lp_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLpConfigAccounts<'_, '_>,
    args: SetLpConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLpConfigKeys = accounts.into();
    let ix = set_lp_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_lp_config_invoke_signed(
    accounts: SetLpConfigAccounts<'_, '_>,
    args: SetLpConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_lp_config_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_lp_config_verify_account_keys(
    accounts: SetLpConfigAccounts<'_, '_>,
    keys: SetLpConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.underlying_mint.key, keys.underlying_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_lp_config_verify_writable_privileges<'me, 'info>(
    accounts: SetLpConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_lp_config_verify_signer_privileges<'me, 'info>(
    accounts: SetLpConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_lp_config_verify_account_privileges<'me, 'info>(
    accounts: SetLpConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_lp_config_verify_writable_privileges(accounts)?;
    set_lp_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MANUAL_DEPLOYMENT_LIMITS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetManualDeploymentLimitsAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetManualDeploymentLimitsKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<SetManualDeploymentLimitsAccounts<'_, '_>> for SetManualDeploymentLimitsKeys {
    fn from(accounts: SetManualDeploymentLimitsAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<SetManualDeploymentLimitsKeys>
for [AccountMeta; SET_MANUAL_DEPLOYMENT_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetManualDeploymentLimitsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_MANUAL_DEPLOYMENT_LIMITS_IX_ACCOUNTS_LEN]>
for SetManualDeploymentLimitsKeys {
    fn from(pubkeys: [Pubkey; SET_MANUAL_DEPLOYMENT_LIMITS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
        }
    }
}
impl<'info> From<SetManualDeploymentLimitsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MANUAL_DEPLOYMENT_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetManualDeploymentLimitsAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_MANUAL_DEPLOYMENT_LIMITS_IX_ACCOUNTS_LEN]>
for SetManualDeploymentLimitsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_MANUAL_DEPLOYMENT_LIMITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
        }
    }
}
pub const SET_MANUAL_DEPLOYMENT_LIMITS_IX_DISCM: [u8; 8usize] = [
    252, 59, 255, 197, 220, 82, 142, 20,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetManualDeploymentLimitsIxArgs {
    pub daily_limit: u64,
    pub per_wallet_limit: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetManualDeploymentLimitsIxData(pub SetManualDeploymentLimitsIxArgs);
impl From<SetManualDeploymentLimitsIxArgs> for SetManualDeploymentLimitsIxData {
    fn from(args: SetManualDeploymentLimitsIxArgs) -> Self {
        Self(args)
    }
}
impl SetManualDeploymentLimitsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MANUAL_DEPLOYMENT_LIMITS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let daily_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let per_wallet_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetManualDeploymentLimitsIxArgs {
                daily_limit,
                per_wallet_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MANUAL_DEPLOYMENT_LIMITS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.daily_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.per_wallet_limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_manual_deployment_limits_ix_with_program_id(
    program_id: Pubkey,
    keys: SetManualDeploymentLimitsKeys,
    args: SetManualDeploymentLimitsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MANUAL_DEPLOYMENT_LIMITS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetManualDeploymentLimitsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_manual_deployment_limits_ix(
    keys: SetManualDeploymentLimitsKeys,
    args: SetManualDeploymentLimitsIxArgs,
) -> std::io::Result<Instruction> {
    set_manual_deployment_limits_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn set_manual_deployment_limits_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetManualDeploymentLimitsAccounts<'_, '_>,
    args: SetManualDeploymentLimitsIxArgs,
) -> ProgramResult {
    let keys: SetManualDeploymentLimitsKeys = accounts.into();
    let ix = set_manual_deployment_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_manual_deployment_limits_invoke(
    accounts: SetManualDeploymentLimitsAccounts<'_, '_>,
    args: SetManualDeploymentLimitsIxArgs,
) -> ProgramResult {
    set_manual_deployment_limits_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn set_manual_deployment_limits_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetManualDeploymentLimitsAccounts<'_, '_>,
    args: SetManualDeploymentLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetManualDeploymentLimitsKeys = accounts.into();
    let ix = set_manual_deployment_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_manual_deployment_limits_invoke_signed(
    accounts: SetManualDeploymentLimitsAccounts<'_, '_>,
    args: SetManualDeploymentLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_manual_deployment_limits_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_manual_deployment_limits_verify_account_keys(
    accounts: SetManualDeploymentLimitsAccounts<'_, '_>,
    keys: SetManualDeploymentLimitsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_manual_deployment_limits_verify_writable_privileges<'me, 'info>(
    accounts: SetManualDeploymentLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_manual_deployment_limits_verify_signer_privileges<'me, 'info>(
    accounts: SetManualDeploymentLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_manual_deployment_limits_verify_account_privileges<'me, 'info>(
    accounts: SetManualDeploymentLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_manual_deployment_limits_verify_writable_privileges(accounts)?;
    set_manual_deployment_limits_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_OWNER_TREASURY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolOwnerTreasuryAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolOwnerTreasuryKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<SetPoolOwnerTreasuryAccounts<'_, '_>> for SetPoolOwnerTreasuryKeys {
    fn from(accounts: SetPoolOwnerTreasuryAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<SetPoolOwnerTreasuryKeys>
for [AccountMeta; SET_POOL_OWNER_TREASURY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolOwnerTreasuryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_OWNER_TREASURY_IX_ACCOUNTS_LEN]>
for SetPoolOwnerTreasuryKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_OWNER_TREASURY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
        }
    }
}
impl<'info> From<SetPoolOwnerTreasuryAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_OWNER_TREASURY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolOwnerTreasuryAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POOL_OWNER_TREASURY_IX_ACCOUNTS_LEN]>
for SetPoolOwnerTreasuryAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_POOL_OWNER_TREASURY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
        }
    }
}
pub const SET_POOL_OWNER_TREASURY_IX_DISCM: [u8; 8usize] = [
    95, 26, 200, 33, 36, 107, 65, 219,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPoolOwnerTreasuryIxArgs {
    pub new_treasury: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolOwnerTreasuryIxData(pub SetPoolOwnerTreasuryIxArgs);
impl From<SetPoolOwnerTreasuryIxArgs> for SetPoolOwnerTreasuryIxData {
    fn from(args: SetPoolOwnerTreasuryIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolOwnerTreasuryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_OWNER_TREASURY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPoolOwnerTreasuryIxArgs {
                new_treasury,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_OWNER_TREASURY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_treasury, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_owner_treasury_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolOwnerTreasuryKeys,
    args: SetPoolOwnerTreasuryIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_OWNER_TREASURY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolOwnerTreasuryIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_owner_treasury_ix(
    keys: SetPoolOwnerTreasuryKeys,
    args: SetPoolOwnerTreasuryIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_owner_treasury_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn set_pool_owner_treasury_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolOwnerTreasuryAccounts<'_, '_>,
    args: SetPoolOwnerTreasuryIxArgs,
) -> ProgramResult {
    let keys: SetPoolOwnerTreasuryKeys = accounts.into();
    let ix = set_pool_owner_treasury_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_owner_treasury_invoke(
    accounts: SetPoolOwnerTreasuryAccounts<'_, '_>,
    args: SetPoolOwnerTreasuryIxArgs,
) -> ProgramResult {
    set_pool_owner_treasury_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn set_pool_owner_treasury_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolOwnerTreasuryAccounts<'_, '_>,
    args: SetPoolOwnerTreasuryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolOwnerTreasuryKeys = accounts.into();
    let ix = set_pool_owner_treasury_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_owner_treasury_invoke_signed(
    accounts: SetPoolOwnerTreasuryAccounts<'_, '_>,
    args: SetPoolOwnerTreasuryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_owner_treasury_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pool_owner_treasury_verify_account_keys(
    accounts: SetPoolOwnerTreasuryAccounts<'_, '_>,
    keys: SetPoolOwnerTreasuryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_owner_treasury_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolOwnerTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_owner_treasury_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolOwnerTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_owner_treasury_verify_account_privileges<'me, 'info>(
    accounts: SetPoolOwnerTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_owner_treasury_verify_writable_privileges(accounts)?;
    set_pool_owner_treasury_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SETUP_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SetupDeploymentTargetAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub deployment_config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetupDeploymentTargetKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub deployment_config: Pubkey,
    pub pool_authority: Pubkey,
}
impl From<SetupDeploymentTargetAccounts<'_, '_>> for SetupDeploymentTargetKeys {
    fn from(accounts: SetupDeploymentTargetAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            deployment_config: *accounts.deployment_config.key,
            pool_authority: *accounts.pool_authority.key,
        }
    }
}
impl From<SetupDeploymentTargetKeys>
for [AccountMeta; SETUP_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(keys: SetupDeploymentTargetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SETUP_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for SetupDeploymentTargetKeys {
    fn from(pubkeys: [Pubkey; SETUP_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            deployment_config: pubkeys[4],
            pool_authority: pubkeys[5],
        }
    }
}
impl<'info> From<SetupDeploymentTargetAccounts<'_, 'info>>
for [AccountInfo<'info>; SETUP_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetupDeploymentTargetAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.deployment_config.clone(),
            accounts.pool_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETUP_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN]>
for SetupDeploymentTargetAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SETUP_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            deployment_config: &arr[4],
            pool_authority: &arr[5],
        }
    }
}
pub const SETUP_DEPLOYMENT_TARGET_IX_DISCM: [u8; 8usize] = [
    43, 248, 122, 37, 6, 251, 231, 224,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetupDeploymentTargetIxArgs {
    pub strategy_type: DeploymentStrategyType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetupDeploymentTargetIxData(pub SetupDeploymentTargetIxArgs);
impl From<SetupDeploymentTargetIxArgs> for SetupDeploymentTargetIxData {
    fn from(args: SetupDeploymentTargetIxArgs) -> Self {
        Self(args)
    }
}
impl SetupDeploymentTargetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETUP_DEPLOYMENT_TARGET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SetupDeploymentTargetIxArgs {
                strategy_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETUP_DEPLOYMENT_TARGET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn setup_deployment_target_ix_with_program_id(
    program_id: Pubkey,
    keys: SetupDeploymentTargetKeys,
    args: SetupDeploymentTargetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETUP_DEPLOYMENT_TARGET_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetupDeploymentTargetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn setup_deployment_target_ix(
    keys: SetupDeploymentTargetKeys,
    args: SetupDeploymentTargetIxArgs,
) -> std::io::Result<Instruction> {
    setup_deployment_target_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn setup_deployment_target_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetupDeploymentTargetAccounts<'_, '_>,
    args: SetupDeploymentTargetIxArgs,
) -> ProgramResult {
    let keys: SetupDeploymentTargetKeys = accounts.into();
    let ix = setup_deployment_target_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn setup_deployment_target_invoke(
    accounts: SetupDeploymentTargetAccounts<'_, '_>,
    args: SetupDeploymentTargetIxArgs,
) -> ProgramResult {
    setup_deployment_target_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn setup_deployment_target_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetupDeploymentTargetAccounts<'_, '_>,
    args: SetupDeploymentTargetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetupDeploymentTargetKeys = accounts.into();
    let ix = setup_deployment_target_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn setup_deployment_target_invoke_signed(
    accounts: SetupDeploymentTargetAccounts<'_, '_>,
    args: SetupDeploymentTargetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    setup_deployment_target_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn setup_deployment_target_verify_account_keys(
    accounts: SetupDeploymentTargetAccounts<'_, '_>,
    keys: SetupDeploymentTargetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.deployment_config.key, keys.deployment_config),
        (*accounts.pool_authority.key, keys.pool_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn setup_deployment_target_verify_writable_privileges<'me, 'info>(
    accounts: SetupDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.pool_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn setup_deployment_target_verify_signer_privileges<'me, 'info>(
    accounts: SetupDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn setup_deployment_target_verify_account_privileges<'me, 'info>(
    accounts: SetupDeploymentTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    setup_deployment_target_verify_writable_privileges(accounts)?;
    setup_deployment_target_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWITCH_MODE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SwitchModeAccounts<'me, 'info> {
    pub lender: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub source_mode_config: &'me AccountInfo<'info>,
    pub source_mode_mint: &'me AccountInfo<'info>,
    pub source_mode_token: &'me AccountInfo<'info>,
    pub destination_mode_config: &'me AccountInfo<'info>,
    pub destination_mode_mint: &'me AccountInfo<'info>,
    pub destination_mode_token: &'me AccountInfo<'info>,
    pub source_mode_token_program: &'me AccountInfo<'info>,
    pub destination_mode_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwitchModeKeys {
    pub lender: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub pool_authority: Pubkey,
    pub source_mode_config: Pubkey,
    pub source_mode_mint: Pubkey,
    pub source_mode_token: Pubkey,
    pub destination_mode_config: Pubkey,
    pub destination_mode_mint: Pubkey,
    pub destination_mode_token: Pubkey,
    pub source_mode_token_program: Pubkey,
    pub destination_mode_token_program: Pubkey,
}
impl From<SwitchModeAccounts<'_, '_>> for SwitchModeKeys {
    fn from(accounts: SwitchModeAccounts) -> Self {
        Self {
            lender: *accounts.lender.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            pool_authority: *accounts.pool_authority.key,
            source_mode_config: *accounts.source_mode_config.key,
            source_mode_mint: *accounts.source_mode_mint.key,
            source_mode_token: *accounts.source_mode_token.key,
            destination_mode_config: *accounts.destination_mode_config.key,
            destination_mode_mint: *accounts.destination_mode_mint.key,
            destination_mode_token: *accounts.destination_mode_token.key,
            source_mode_token_program: *accounts.source_mode_token_program.key,
            destination_mode_token_program: *accounts.destination_mode_token_program.key,
        }
    }
}
impl From<SwitchModeKeys> for [AccountMeta; SWITCH_MODE_IX_ACCOUNTS_LEN] {
    fn from(keys: SwitchModeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mode_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mode_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWITCH_MODE_IX_ACCOUNTS_LEN]> for SwitchModeKeys {
    fn from(pubkeys: [Pubkey; SWITCH_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lender: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            pool_authority: pubkeys[4],
            source_mode_config: pubkeys[5],
            source_mode_mint: pubkeys[6],
            source_mode_token: pubkeys[7],
            destination_mode_config: pubkeys[8],
            destination_mode_mint: pubkeys[9],
            destination_mode_token: pubkeys[10],
            source_mode_token_program: pubkeys[11],
            destination_mode_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<SwitchModeAccounts<'_, 'info>>
for [AccountInfo<'info>; SWITCH_MODE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwitchModeAccounts<'_, 'info>) -> Self {
        [
            accounts.lender.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.pool_authority.clone(),
            accounts.source_mode_config.clone(),
            accounts.source_mode_mint.clone(),
            accounts.source_mode_token.clone(),
            accounts.destination_mode_config.clone(),
            accounts.destination_mode_mint.clone(),
            accounts.destination_mode_token.clone(),
            accounts.source_mode_token_program.clone(),
            accounts.destination_mode_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWITCH_MODE_IX_ACCOUNTS_LEN]>
for SwitchModeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWITCH_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lender: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            pool_authority: &arr[4],
            source_mode_config: &arr[5],
            source_mode_mint: &arr[6],
            source_mode_token: &arr[7],
            destination_mode_config: &arr[8],
            destination_mode_mint: &arr[9],
            destination_mode_token: &arr[10],
            source_mode_token_program: &arr[11],
            destination_mode_token_program: &arr[12],
        }
    }
}
pub const SWITCH_MODE_IX_DISCM: [u8; 8usize] = [69, 115, 235, 74, 208, 155, 108, 62];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwitchModeIxArgs {
    pub shares: u64,
    pub investment_id: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwitchModeIxData(pub SwitchModeIxArgs);
impl From<SwitchModeIxArgs> for SwitchModeIxData {
    fn from(args: SwitchModeIxArgs) -> Self {
        Self(args)
    }
}
impl SwitchModeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWITCH_MODE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let investment_id: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwitchModeIxArgs {
                shares,
                investment_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWITCH_MODE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.investment_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn switch_mode_ix_with_program_id(
    program_id: Pubkey,
    keys: SwitchModeKeys,
    args: SwitchModeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWITCH_MODE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwitchModeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn switch_mode_ix(
    keys: SwitchModeKeys,
    args: SwitchModeIxArgs,
) -> std::io::Result<Instruction> {
    switch_mode_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn switch_mode_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwitchModeAccounts<'_, '_>,
    args: SwitchModeIxArgs,
) -> ProgramResult {
    let keys: SwitchModeKeys = accounts.into();
    let ix = switch_mode_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn switch_mode_invoke(
    accounts: SwitchModeAccounts<'_, '_>,
    args: SwitchModeIxArgs,
) -> ProgramResult {
    switch_mode_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn switch_mode_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwitchModeAccounts<'_, '_>,
    args: SwitchModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwitchModeKeys = accounts.into();
    let ix = switch_mode_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn switch_mode_invoke_signed(
    accounts: SwitchModeAccounts<'_, '_>,
    args: SwitchModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    switch_mode_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn switch_mode_verify_account_keys(
    accounts: SwitchModeAccounts<'_, '_>,
    keys: SwitchModeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lender.key, keys.lender),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.source_mode_config.key, keys.source_mode_config),
        (*accounts.source_mode_mint.key, keys.source_mode_mint),
        (*accounts.source_mode_token.key, keys.source_mode_token),
        (*accounts.destination_mode_config.key, keys.destination_mode_config),
        (*accounts.destination_mode_mint.key, keys.destination_mode_mint),
        (*accounts.destination_mode_token.key, keys.destination_mode_token),
        (*accounts.source_mode_token_program.key, keys.source_mode_token_program),
        (
            *accounts.destination_mode_token_program.key,
            keys.destination_mode_token_program,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn switch_mode_verify_writable_privileges<'me, 'info>(
    accounts: SwitchModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.source_mode_mint,
        accounts.source_mode_token,
        accounts.destination_mode_mint,
        accounts.destination_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn switch_mode_verify_signer_privileges<'me, 'info>(
    accounts: SwitchModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn switch_mode_verify_account_privileges<'me, 'info>(
    accounts: SwitchModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    switch_mode_verify_writable_privileges(accounts)?;
    switch_mode_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_MODE_APY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateModeApyAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateModeApyKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
}
impl From<UpdateModeApyAccounts<'_, '_>> for UpdateModeApyKeys {
    fn from(accounts: UpdateModeApyAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
        }
    }
}
impl From<UpdateModeApyKeys> for [AccountMeta; UPDATE_MODE_APY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateModeApyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_MODE_APY_IX_ACCOUNTS_LEN]> for UpdateModeApyKeys {
    fn from(pubkeys: [Pubkey; UPDATE_MODE_APY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateModeApyAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_MODE_APY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateModeApyAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_MODE_APY_IX_ACCOUNTS_LEN]>
for UpdateModeApyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_MODE_APY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
        }
    }
}
pub const UPDATE_MODE_APY_IX_DISCM: [u8; 8usize] = [2, 35, 3, 169, 206, 20, 85, 86];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateModeApyIxArgs {
    pub new_target_apy_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateModeApyIxData(pub UpdateModeApyIxArgs);
impl From<UpdateModeApyIxArgs> for UpdateModeApyIxData {
    fn from(args: UpdateModeApyIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateModeApyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_MODE_APY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_target_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateModeApyIxArgs {
                new_target_apy_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_MODE_APY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_target_apy_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_mode_apy_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateModeApyKeys,
    args: UpdateModeApyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_MODE_APY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateModeApyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_mode_apy_ix(
    keys: UpdateModeApyKeys,
    args: UpdateModeApyIxArgs,
) -> std::io::Result<Instruction> {
    update_mode_apy_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn update_mode_apy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateModeApyAccounts<'_, '_>,
    args: UpdateModeApyIxArgs,
) -> ProgramResult {
    let keys: UpdateModeApyKeys = accounts.into();
    let ix = update_mode_apy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_mode_apy_invoke(
    accounts: UpdateModeApyAccounts<'_, '_>,
    args: UpdateModeApyIxArgs,
) -> ProgramResult {
    update_mode_apy_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn update_mode_apy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateModeApyAccounts<'_, '_>,
    args: UpdateModeApyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateModeApyKeys = accounts.into();
    let ix = update_mode_apy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_mode_apy_invoke_signed(
    accounts: UpdateModeApyAccounts<'_, '_>,
    args: UpdateModeApyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_mode_apy_invoke_signed_with_program_id(HUMA_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_mode_apy_verify_account_keys(
    accounts: UpdateModeApyAccounts<'_, '_>,
    keys: UpdateModeApyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_mode_apy_verify_writable_privileges<'me, 'info>(
    accounts: UpdateModeApyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.mode_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_mode_apy_verify_signer_privileges<'me, 'info>(
    accounts: UpdateModeApyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_mode_apy_verify_account_privileges<'me, 'info>(
    accounts: UpdateModeApyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_mode_apy_verify_writable_privileges(accounts)?;
    update_mode_apy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_MODE_NAME_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateModeNameAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateModeNameKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
}
impl From<UpdateModeNameAccounts<'_, '_>> for UpdateModeNameKeys {
    fn from(accounts: UpdateModeNameAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
        }
    }
}
impl From<UpdateModeNameKeys> for [AccountMeta; UPDATE_MODE_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateModeNameKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_MODE_NAME_IX_ACCOUNTS_LEN]> for UpdateModeNameKeys {
    fn from(pubkeys: [Pubkey; UPDATE_MODE_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateModeNameAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_MODE_NAME_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateModeNameAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_MODE_NAME_IX_ACCOUNTS_LEN]>
for UpdateModeNameAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_MODE_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
        }
    }
}
pub const UPDATE_MODE_NAME_IX_DISCM: [u8; 8usize] = [
    161, 162, 16, 147, 165, 200, 140, 12,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateModeNameIxArgs {
    pub new_name: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateModeNameIxData(pub UpdateModeNameIxArgs);
impl From<UpdateModeNameIxArgs> for UpdateModeNameIxData {
    fn from(args: UpdateModeNameIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateModeNameIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_MODE_NAME_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_name: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdateModeNameIxArgs { new_name }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_MODE_NAME_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_name, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_mode_name_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateModeNameKeys,
    args: UpdateModeNameIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_MODE_NAME_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateModeNameIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_mode_name_ix(
    keys: UpdateModeNameKeys,
    args: UpdateModeNameIxArgs,
) -> std::io::Result<Instruction> {
    update_mode_name_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn update_mode_name_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateModeNameAccounts<'_, '_>,
    args: UpdateModeNameIxArgs,
) -> ProgramResult {
    let keys: UpdateModeNameKeys = accounts.into();
    let ix = update_mode_name_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_mode_name_invoke(
    accounts: UpdateModeNameAccounts<'_, '_>,
    args: UpdateModeNameIxArgs,
) -> ProgramResult {
    update_mode_name_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn update_mode_name_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateModeNameAccounts<'_, '_>,
    args: UpdateModeNameIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateModeNameKeys = accounts.into();
    let ix = update_mode_name_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_mode_name_invoke_signed(
    accounts: UpdateModeNameAccounts<'_, '_>,
    args: UpdateModeNameIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_mode_name_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_mode_name_verify_account_keys(
    accounts: UpdateModeNameAccounts<'_, '_>,
    keys: UpdateModeNameKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_mode_name_verify_writable_privileges<'me, 'info>(
    accounts: UpdateModeNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.mode_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_mode_name_verify_signer_privileges<'me, 'info>(
    accounts: UpdateModeNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_mode_name_verify_account_privileges<'me, 'info>(
    accounts: UpdateModeNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_mode_name_verify_writable_privileges(accounts)?;
    update_mode_name_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_MODE_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct UpdateModeTokenMetadataAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub token_metadata: &'me AccountInfo<'info>,
    pub mpl_token_metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateModeTokenMetadataKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub pool_authority: Pubkey,
    pub token_metadata: Pubkey,
    pub mpl_token_metadata_program: Pubkey,
}
impl From<UpdateModeTokenMetadataAccounts<'_, '_>> for UpdateModeTokenMetadataKeys {
    fn from(accounts: UpdateModeTokenMetadataAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            pool_authority: *accounts.pool_authority.key,
            token_metadata: *accounts.token_metadata.key,
            mpl_token_metadata_program: *accounts.mpl_token_metadata_program.key,
        }
    }
}
impl From<UpdateModeTokenMetadataKeys>
for [AccountMeta; UPDATE_MODE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateModeTokenMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_metadata,
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
impl From<[Pubkey; UPDATE_MODE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for UpdateModeTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; UPDATE_MODE_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
            pool_authority: pubkeys[5],
            token_metadata: pubkeys[6],
            mpl_token_metadata_program: pubkeys[7],
        }
    }
}
impl<'info> From<UpdateModeTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_MODE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateModeTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.pool_authority.clone(),
            accounts.token_metadata.clone(),
            accounts.mpl_token_metadata_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_MODE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for UpdateModeTokenMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_MODE_TOKEN_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
            pool_authority: &arr[5],
            token_metadata: &arr[6],
            mpl_token_metadata_program: &arr[7],
        }
    }
}
pub const UPDATE_MODE_TOKEN_METADATA_IX_DISCM: [u8; 8usize] = [
    146, 105, 133, 105, 200, 19, 3, 221,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateModeTokenMetadataIxArgs {
    pub args: ManageModeTokenMetadataArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateModeTokenMetadataIxData(pub UpdateModeTokenMetadataIxArgs);
impl From<UpdateModeTokenMetadataIxArgs> for UpdateModeTokenMetadataIxData {
    fn from(args: UpdateModeTokenMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateModeTokenMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_MODE_TOKEN_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ManageModeTokenMetadataArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateModeTokenMetadataIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_MODE_TOKEN_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_mode_token_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateModeTokenMetadataKeys,
    args: UpdateModeTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_MODE_TOKEN_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateModeTokenMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_mode_token_metadata_ix(
    keys: UpdateModeTokenMetadataKeys,
    args: UpdateModeTokenMetadataIxArgs,
) -> std::io::Result<Instruction> {
    update_mode_token_metadata_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn update_mode_token_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateModeTokenMetadataAccounts<'_, '_>,
    args: UpdateModeTokenMetadataIxArgs,
) -> ProgramResult {
    let keys: UpdateModeTokenMetadataKeys = accounts.into();
    let ix = update_mode_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_mode_token_metadata_invoke(
    accounts: UpdateModeTokenMetadataAccounts<'_, '_>,
    args: UpdateModeTokenMetadataIxArgs,
) -> ProgramResult {
    update_mode_token_metadata_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn update_mode_token_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateModeTokenMetadataAccounts<'_, '_>,
    args: UpdateModeTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateModeTokenMetadataKeys = accounts.into();
    let ix = update_mode_token_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_mode_token_metadata_invoke_signed(
    accounts: UpdateModeTokenMetadataAccounts<'_, '_>,
    args: UpdateModeTokenMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_mode_token_metadata_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_mode_token_metadata_verify_account_keys(
    accounts: UpdateModeTokenMetadataAccounts<'_, '_>,
    keys: UpdateModeTokenMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.token_metadata.key, keys.token_metadata),
        (*accounts.mpl_token_metadata_program.key, keys.mpl_token_metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_mode_token_metadata_verify_writable_privileges<'me, 'info>(
    accounts: UpdateModeTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_mode_token_metadata_verify_signer_privileges<'me, 'info>(
    accounts: UpdateModeTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_mode_token_metadata_verify_account_privileges<'me, 'info>(
    accounts: UpdateModeTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_mode_token_metadata_verify_writable_privileges(accounts)?;
    update_mode_token_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_POOL_NAME_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePoolNameAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePoolNameKeys {
    pub signer: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
}
impl From<UpdatePoolNameAccounts<'_, '_>> for UpdatePoolNameKeys {
    fn from(accounts: UpdatePoolNameAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<UpdatePoolNameKeys> for [AccountMeta; UPDATE_POOL_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePoolNameKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_POOL_NAME_IX_ACCOUNTS_LEN]> for UpdatePoolNameKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POOL_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
        }
    }
}
impl<'info> From<UpdatePoolNameAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POOL_NAME_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePoolNameAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_POOL_NAME_IX_ACCOUNTS_LEN]>
for UpdatePoolNameAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_POOL_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
        }
    }
}
pub const UPDATE_POOL_NAME_IX_DISCM: [u8; 8usize] = [134, 125, 13, 23, 145, 45, 49, 196];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePoolNameIxArgs {
    pub new_name: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePoolNameIxData(pub UpdatePoolNameIxArgs);
impl From<UpdatePoolNameIxArgs> for UpdatePoolNameIxData {
    fn from(args: UpdatePoolNameIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePoolNameIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POOL_NAME_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_name: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdatePoolNameIxArgs { new_name }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POOL_NAME_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_name, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_pool_name_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePoolNameKeys,
    args: UpdatePoolNameIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POOL_NAME_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePoolNameIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_pool_name_ix(
    keys: UpdatePoolNameKeys,
    args: UpdatePoolNameIxArgs,
) -> std::io::Result<Instruction> {
    update_pool_name_ix_with_program_id(HUMA_PROGRAM_ID, keys, args)
}
pub fn update_pool_name_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolNameAccounts<'_, '_>,
    args: UpdatePoolNameIxArgs,
) -> ProgramResult {
    let keys: UpdatePoolNameKeys = accounts.into();
    let ix = update_pool_name_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pool_name_invoke(
    accounts: UpdatePoolNameAccounts<'_, '_>,
    args: UpdatePoolNameIxArgs,
) -> ProgramResult {
    update_pool_name_invoke_with_program_id(HUMA_PROGRAM_ID, accounts, args)
}
pub fn update_pool_name_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolNameAccounts<'_, '_>,
    args: UpdatePoolNameIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePoolNameKeys = accounts.into();
    let ix = update_pool_name_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pool_name_invoke_signed(
    accounts: UpdatePoolNameAccounts<'_, '_>,
    args: UpdatePoolNameIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pool_name_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_pool_name_verify_account_keys(
    accounts: UpdatePoolNameAccounts<'_, '_>,
    keys: UpdatePoolNameKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_pool_name_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePoolNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pool_name_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePoolNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pool_name_verify_account_privileges<'me, 'info>(
    accounts: UpdatePoolNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pool_name_verify_writable_privileges(accounts)?;
    update_pool_name_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_AFTER_POOL_CLOSURE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAfterPoolClosureAccounts<'me, 'info> {
    pub lender: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub mode_config: &'me AccountInfo<'info>,
    pub mode_mint: &'me AccountInfo<'info>,
    pub lender_state: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub lender_underlying_token: &'me AccountInfo<'info>,
    pub lender_mode_token: &'me AccountInfo<'info>,
    pub underlying_token_program: &'me AccountInfo<'info>,
    pub mode_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawAfterPoolClosureKeys {
    pub lender: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub lender_state: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub lender_underlying_token: Pubkey,
    pub lender_mode_token: Pubkey,
    pub underlying_token_program: Pubkey,
    pub mode_token_program: Pubkey,
}
impl From<WithdrawAfterPoolClosureAccounts<'_, '_>> for WithdrawAfterPoolClosureKeys {
    fn from(accounts: WithdrawAfterPoolClosureAccounts) -> Self {
        Self {
            lender: *accounts.lender.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            mode_config: *accounts.mode_config.key,
            mode_mint: *accounts.mode_mint.key,
            lender_state: *accounts.lender_state.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_authority: *accounts.pool_authority.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            lender_underlying_token: *accounts.lender_underlying_token.key,
            lender_mode_token: *accounts.lender_mode_token.key,
            underlying_token_program: *accounts.underlying_token_program.key,
            mode_token_program: *accounts.mode_token_program.key,
        }
    }
}
impl From<WithdrawAfterPoolClosureKeys>
for [AccountMeta; WITHDRAW_AFTER_POOL_CLOSURE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawAfterPoolClosureKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mode_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_underlying_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lender_mode_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mode_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_AFTER_POOL_CLOSURE_IX_ACCOUNTS_LEN]>
for WithdrawAfterPoolClosureKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_AFTER_POOL_CLOSURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lender: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            mode_config: pubkeys[4],
            mode_mint: pubkeys[5],
            lender_state: pubkeys[6],
            underlying_mint: pubkeys[7],
            pool_authority: pubkeys[8],
            pool_underlying_token: pubkeys[9],
            lender_underlying_token: pubkeys[10],
            lender_mode_token: pubkeys[11],
            underlying_token_program: pubkeys[12],
            mode_token_program: pubkeys[13],
        }
    }
}
impl<'info> From<WithdrawAfterPoolClosureAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_AFTER_POOL_CLOSURE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAfterPoolClosureAccounts<'_, 'info>) -> Self {
        [
            accounts.lender.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.mode_config.clone(),
            accounts.mode_mint.clone(),
            accounts.lender_state.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.lender_underlying_token.clone(),
            accounts.lender_mode_token.clone(),
            accounts.underlying_token_program.clone(),
            accounts.mode_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_AFTER_POOL_CLOSURE_IX_ACCOUNTS_LEN]>
for WithdrawAfterPoolClosureAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_AFTER_POOL_CLOSURE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lender: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            mode_config: &arr[4],
            mode_mint: &arr[5],
            lender_state: &arr[6],
            underlying_mint: &arr[7],
            pool_authority: &arr[8],
            pool_underlying_token: &arr[9],
            lender_underlying_token: &arr[10],
            lender_mode_token: &arr[11],
            underlying_token_program: &arr[12],
            mode_token_program: &arr[13],
        }
    }
}
pub const WITHDRAW_AFTER_POOL_CLOSURE_IX_DISCM: [u8; 8usize] = [
    82, 21, 237, 73, 48, 153, 86, 168,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawAfterPoolClosureIxData;
impl WithdrawAfterPoolClosureIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_AFTER_POOL_CLOSURE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_AFTER_POOL_CLOSURE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_after_pool_closure_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawAfterPoolClosureKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_AFTER_POOL_CLOSURE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawAfterPoolClosureIxData.try_to_vec()?,
    })
}
pub fn withdraw_after_pool_closure_ix(
    keys: WithdrawAfterPoolClosureKeys,
) -> std::io::Result<Instruction> {
    withdraw_after_pool_closure_ix_with_program_id(HUMA_PROGRAM_ID, keys)
}
pub fn withdraw_after_pool_closure_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAfterPoolClosureAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawAfterPoolClosureKeys = accounts.into();
    let ix = withdraw_after_pool_closure_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_after_pool_closure_invoke(
    accounts: WithdrawAfterPoolClosureAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_after_pool_closure_invoke_with_program_id(HUMA_PROGRAM_ID, accounts)
}
pub fn withdraw_after_pool_closure_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAfterPoolClosureAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawAfterPoolClosureKeys = accounts.into();
    let ix = withdraw_after_pool_closure_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_after_pool_closure_invoke_signed(
    accounts: WithdrawAfterPoolClosureAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_after_pool_closure_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_after_pool_closure_verify_account_keys(
    accounts: WithdrawAfterPoolClosureAccounts<'_, '_>,
    keys: WithdrawAfterPoolClosureKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lender.key, keys.lender),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.mode_config.key, keys.mode_config),
        (*accounts.mode_mint.key, keys.mode_mint),
        (*accounts.lender_state.key, keys.lender_state),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.lender_underlying_token.key, keys.lender_underlying_token),
        (*accounts.lender_mode_token.key, keys.lender_mode_token),
        (*accounts.underlying_token_program.key, keys.underlying_token_program),
        (*accounts.mode_token_program.key, keys.mode_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_after_pool_closure_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAfterPoolClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.mode_mint,
        accounts.lender_state,
        accounts.pool_underlying_token,
        accounts.lender_underlying_token,
        accounts.lender_mode_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_after_pool_closure_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawAfterPoolClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_after_pool_closure_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAfterPoolClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_after_pool_closure_verify_writable_privileges(accounts)?;
    withdraw_after_pool_closure_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawLiquidityAfterTargetClosureAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub huma_config: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub async_deployment_config: &'me AccountInfo<'info>,
    pub async_deployment_state: &'me AccountInfo<'info>,
    pub strategy_manager_wallet: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub underlying_mint: &'me AccountInfo<'info>,
    pub pool_underlying_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawLiquidityAfterTargetClosureKeys {
    pub wallet: Pubkey,
    pub huma_config: Pubkey,
    pub pool_config: Pubkey,
    pub pool_state: Pubkey,
    pub async_deployment_config: Pubkey,
    pub async_deployment_state: Pubkey,
    pub strategy_manager_wallet: Pubkey,
    pub pool_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawLiquidityAfterTargetClosureAccounts<'_, '_>>
for WithdrawLiquidityAfterTargetClosureKeys {
    fn from(accounts: WithdrawLiquidityAfterTargetClosureAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            huma_config: *accounts.huma_config.key,
            pool_config: *accounts.pool_config.key,
            pool_state: *accounts.pool_state.key,
            async_deployment_config: *accounts.async_deployment_config.key,
            async_deployment_state: *accounts.async_deployment_state.key,
            strategy_manager_wallet: *accounts.strategy_manager_wallet.key,
            pool_authority: *accounts.pool_authority.key,
            underlying_mint: *accounts.underlying_mint.key,
            pool_underlying_token: *accounts.pool_underlying_token.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawLiquidityAfterTargetClosureKeys>
for [AccountMeta; WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawLiquidityAfterTargetClosureKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.huma_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.async_deployment_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.async_deployment_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_manager_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_underlying_token,
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
impl From<[Pubkey; WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_ACCOUNTS_LEN]>
for WithdrawLiquidityAfterTargetClosureKeys {
    fn from(
        pubkeys: [Pubkey; WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: pubkeys[0],
            huma_config: pubkeys[1],
            pool_config: pubkeys[2],
            pool_state: pubkeys[3],
            async_deployment_config: pubkeys[4],
            async_deployment_state: pubkeys[5],
            strategy_manager_wallet: pubkeys[6],
            pool_authority: pubkeys[7],
            underlying_mint: pubkeys[8],
            pool_underlying_token: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<WithdrawLiquidityAfterTargetClosureAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawLiquidityAfterTargetClosureAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.huma_config.clone(),
            accounts.pool_config.clone(),
            accounts.pool_state.clone(),
            accounts.async_deployment_config.clone(),
            accounts.async_deployment_state.clone(),
            accounts.strategy_manager_wallet.clone(),
            accounts.pool_authority.clone(),
            accounts.underlying_mint.clone(),
            accounts.pool_underlying_token.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_ACCOUNTS_LEN],
> for WithdrawLiquidityAfterTargetClosureAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            wallet: &arr[0],
            huma_config: &arr[1],
            pool_config: &arr[2],
            pool_state: &arr[3],
            async_deployment_config: &arr[4],
            async_deployment_state: &arr[5],
            strategy_manager_wallet: &arr[6],
            pool_authority: &arr[7],
            underlying_mint: &arr[8],
            pool_underlying_token: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_DISCM: [u8; 8usize] = [
    44, 42, 78, 82, 110, 8, 145, 197,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawLiquidityAfterTargetClosureIxArgs {
    pub strategy_type: AsyncDeploymentStrategyType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawLiquidityAfterTargetClosureIxData(
    pub WithdrawLiquidityAfterTargetClosureIxArgs,
);
impl From<WithdrawLiquidityAfterTargetClosureIxArgs>
for WithdrawLiquidityAfterTargetClosureIxData {
    fn from(args: WithdrawLiquidityAfterTargetClosureIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawLiquidityAfterTargetClosureIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(WithdrawLiquidityAfterTargetClosureIxArgs {
                strategy_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_liquidity_after_target_closure_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawLiquidityAfterTargetClosureKeys,
    args: WithdrawLiquidityAfterTargetClosureIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_LIQUIDITY_AFTER_TARGET_CLOSURE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawLiquidityAfterTargetClosureIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_liquidity_after_target_closure_ix(
    keys: WithdrawLiquidityAfterTargetClosureKeys,
    args: WithdrawLiquidityAfterTargetClosureIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_liquidity_after_target_closure_ix_with_program_id(
        HUMA_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn withdraw_liquidity_after_target_closure_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLiquidityAfterTargetClosureAccounts<'_, '_>,
    args: WithdrawLiquidityAfterTargetClosureIxArgs,
) -> ProgramResult {
    let keys: WithdrawLiquidityAfterTargetClosureKeys = accounts.into();
    let ix = withdraw_liquidity_after_target_closure_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_liquidity_after_target_closure_invoke(
    accounts: WithdrawLiquidityAfterTargetClosureAccounts<'_, '_>,
    args: WithdrawLiquidityAfterTargetClosureIxArgs,
) -> ProgramResult {
    withdraw_liquidity_after_target_closure_invoke_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_liquidity_after_target_closure_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLiquidityAfterTargetClosureAccounts<'_, '_>,
    args: WithdrawLiquidityAfterTargetClosureIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawLiquidityAfterTargetClosureKeys = accounts.into();
    let ix = withdraw_liquidity_after_target_closure_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_liquidity_after_target_closure_invoke_signed(
    accounts: WithdrawLiquidityAfterTargetClosureAccounts<'_, '_>,
    args: WithdrawLiquidityAfterTargetClosureIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_liquidity_after_target_closure_invoke_signed_with_program_id(
        HUMA_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_liquidity_after_target_closure_verify_account_keys(
    accounts: WithdrawLiquidityAfterTargetClosureAccounts<'_, '_>,
    keys: WithdrawLiquidityAfterTargetClosureKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.huma_config.key, keys.huma_config),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.async_deployment_config.key, keys.async_deployment_config),
        (*accounts.async_deployment_state.key, keys.async_deployment_state),
        (*accounts.strategy_manager_wallet.key, keys.strategy_manager_wallet),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.underlying_mint.key, keys.underlying_mint),
        (*accounts.pool_underlying_token.key, keys.pool_underlying_token),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_liquidity_after_target_closure_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawLiquidityAfterTargetClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.async_deployment_state,
        accounts.pool_underlying_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_liquidity_after_target_closure_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawLiquidityAfterTargetClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_liquidity_after_target_closure_verify_account_privileges<'me, 'info>(
    accounts: WithdrawLiquidityAfterTargetClosureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_liquidity_after_target_closure_verify_writable_privileges(accounts)?;
    withdraw_liquidity_after_target_closure_verify_signer_privileges(accounts)?;
    Ok(())
}
