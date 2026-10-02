use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum OnreappProgramIx {
    AcceptBoss,
    AddAdmin(AddAdminIxArgs),
    AddApprover(AddApproverIxArgs),
    AddOfferVector(AddOfferVectorIxArgs),
    BurnForNavIncrease(BurnForNavIncreaseIxArgs),
    CancelRedemptionRequest,
    ClearAdmins,
    CloseState,
    ConfigureMaxMintAmount(ConfigureMaxMintAmountIxArgs),
    ConfigureMaxSupply(ConfigureMaxSupplyIxArgs),
    ConfigurePropAmm(ConfigurePropAmmIxArgs),
    CreateRedemptionRequest(CreateRedemptionRequestIxArgs),
    DeleteAllOfferVectors,
    DeleteOfferVector(DeleteOfferVectorIxArgs),
    DepositReserveVault(DepositReserveVaultIxArgs),
    FulfillRedemptionRequest(FulfillRedemptionRequestIxArgs),
    GetApy,
    GetCirculatingSupply,
    GetCirculatingSupplyV2,
    GetNav,
    GetNavAdjustment,
    GetTvl,
    GetTvlV2,
    Initialize,
    InitializeBuffer,
    InitializePermissionlessAuthority(InitializePermissionlessAuthorityIxArgs),
    MakeOffer(MakeOfferIxArgs),
    MakeRedemptionOffer(MakeRedemptionOfferIxArgs),
    MintTo(MintToIxArgs),
    OfferVaultDeposit(OfferVaultDepositIxArgs),
    OfferVaultWithdraw(OfferVaultWithdrawIxArgs),
    OpenSwapBuy(OpenSwapBuyIxArgs),
    OpenSwapSell(OpenSwapSellIxArgs),
    ProposeBoss(ProposeBossIxArgs),
    QuoteSwapBuy(QuoteSwapBuyIxArgs),
    QuoteSwapSell(QuoteSwapSellIxArgs),
    RedemptionVaultDeposit(RedemptionVaultDepositIxArgs),
    RedemptionVaultWithdraw(RedemptionVaultWithdrawIxArgs),
    RefreshMarketStats,
    RemoveAdmin(RemoveAdminIxArgs),
    RemoveApprover(RemoveApproverIxArgs),
    SetBufferFeeConfig(SetBufferFeeConfigIxArgs),
    SetBufferGrossApr(SetBufferGrossAprIxArgs),
    SetCirculatingSupplyExcludedAccounts(SetCirculatingSupplyExcludedAccountsIxArgs),
    SetConfigurableVaultDestination(SetConfigurableVaultDestinationIxArgs),
    SetKillSwitch(SetKillSwitchIxArgs),
    SetMainOffer,
    SetOfferDisabled(SetOfferDisabledIxArgs),
    SetOnycMint,
    SetRedemptionOfferDisabled(SetRedemptionOfferDisabledIxArgs),
    SetWorker(SetWorkerIxArgs),
    SettleBuffer,
    TakeOffer(TakeOfferIxArgs),
    TakeOfferPermissionless(TakeOfferPermissionlessIxArgs),
    TakeOfferPermissionlessV2(TakeOfferPermissionlessV2IxArgs),
    TakeOfferV2(TakeOfferV2IxArgs),
    TransferMintAuthorityToBoss,
    TransferMintAuthorityToProgram,
    UpdateCirculatingSupplyExcludedBalance,
    UpdateOfferFee(UpdateOfferFeeIxArgs),
    UpdateOfferPermissionlessFee(UpdateOfferPermissionlessFeeIxArgs),
    UpdateRedemptionOfferFee(UpdateRedemptionOfferFeeIxArgs),
    UpdateRedemptionOfferPropAmmSellFee(UpdateRedemptionOfferPropAmmSellFeeIxArgs),
    UpdateRedemptionOfferVaultTarget(UpdateRedemptionOfferVaultTargetIxArgs),
    WithdrawConfigurableVault(WithdrawConfigurableVaultIxArgs),
    WithdrawReserveVault(WithdrawReserveVaultIxArgs),
}
impl OnreappProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ACCEPT_BOSS_IX_DISCM) {
            return Ok(Self::AcceptBoss);
        }
        if buf.starts_with(&ADD_ADMIN_IX_DISCM) {
            let mut reader = &buf[ADD_ADMIN_IX_DISCM.len()..];
            let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::AddAdmin(AddAdminIxArgs { new_admin }));
        }
        if buf.starts_with(&ADD_APPROVER_IX_DISCM) {
            let mut reader = &buf[ADD_APPROVER_IX_DISCM.len()..];
            let approver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::AddApprover(AddApproverIxArgs { approver }));
        }
        if buf.starts_with(&ADD_OFFER_VECTOR_IX_DISCM) {
            let mut reader = &buf[ADD_OFFER_VECTOR_IX_DISCM.len()..];
            let start_time: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let base_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            let base_price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let apr: u64 = crate::borsh_de_or_default(&mut reader)?;
            let price_fix_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddOfferVector(AddOfferVectorIxArgs {
                    start_time,
                    base_time,
                    base_price,
                    apr,
                    price_fix_duration,
                }),
            );
        }
        if buf.starts_with(&BURN_FOR_NAV_INCREASE_IX_DISCM) {
            let mut reader = &buf[BURN_FOR_NAV_INCREASE_IX_DISCM.len()..];
            let asset_adjustment_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BurnForNavIncrease(BurnForNavIncreaseIxArgs {
                    asset_adjustment_amount,
                }),
            );
        }
        if buf.starts_with(&CANCEL_REDEMPTION_REQUEST_IX_DISCM) {
            return Ok(Self::CancelRedemptionRequest);
        }
        if buf.starts_with(&CLEAR_ADMINS_IX_DISCM) {
            return Ok(Self::ClearAdmins);
        }
        if buf.starts_with(&CLOSE_STATE_IX_DISCM) {
            return Ok(Self::CloseState);
        }
        if buf.starts_with(&CONFIGURE_MAX_MINT_AMOUNT_IX_DISCM) {
            let mut reader = &buf[CONFIGURE_MAX_MINT_AMOUNT_IX_DISCM.len()..];
            let max_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ConfigureMaxMintAmount(ConfigureMaxMintAmountIxArgs {
                    max_mint_amount,
                }),
            );
        }
        if buf.starts_with(&CONFIGURE_MAX_SUPPLY_IX_DISCM) {
            let mut reader = &buf[CONFIGURE_MAX_SUPPLY_IX_DISCM.len()..];
            let max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ConfigureMaxSupply(ConfigureMaxSupplyIxArgs {
                    max_supply,
                }),
            );
        }
        if buf.starts_with(&CONFIGURE_PROP_AMM_IX_DISCM) {
            let mut reader = &buf[CONFIGURE_PROP_AMM_IX_DISCM.len()..];
            let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
            let curve_peg_haircut_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let curve_exponent_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
            let cadence_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
            let cadence_wave_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
            let epoch_duration_seconds: i64 = crate::borsh_de_or_default(&mut reader)?;
            let wall_sensitivity_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_sell_haircut_onyc: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ConfigurePropAmm(ConfigurePropAmmIxArgs {
                    enabled,
                    curve_peg_haircut_bps,
                    curve_exponent_scaled,
                    cadence_threshold,
                    cadence_wave_scaled,
                    epoch_duration_seconds,
                    wall_sensitivity_scaled,
                    minimum_sell_haircut_onyc,
                }),
            );
        }
        if buf.starts_with(&CREATE_REDEMPTION_REQUEST_IX_DISCM) {
            let mut reader = &buf[CREATE_REDEMPTION_REQUEST_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let request_id: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateRedemptionRequest(CreateRedemptionRequestIxArgs {
                    amount,
                    request_id,
                }),
            );
        }
        if buf.starts_with(&DELETE_ALL_OFFER_VECTORS_IX_DISCM) {
            return Ok(Self::DeleteAllOfferVectors);
        }
        if buf.starts_with(&DELETE_OFFER_VECTOR_IX_DISCM) {
            let mut reader = &buf[DELETE_OFFER_VECTOR_IX_DISCM.len()..];
            let vector_start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DeleteOfferVector(DeleteOfferVectorIxArgs {
                    vector_start_time,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_RESERVE_VAULT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_RESERVE_VAULT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositReserveVault(DepositReserveVaultIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&FULFILL_REDEMPTION_REQUEST_IX_DISCM) {
            let mut reader = &buf[FULFILL_REDEMPTION_REQUEST_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FulfillRedemptionRequest(FulfillRedemptionRequestIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&GET_APY_IX_DISCM) {
            return Ok(Self::GetApy);
        }
        if buf.starts_with(&GET_CIRCULATING_SUPPLY_IX_DISCM) {
            return Ok(Self::GetCirculatingSupply);
        }
        if buf.starts_with(&GET_CIRCULATING_SUPPLY_V2_IX_DISCM) {
            return Ok(Self::GetCirculatingSupplyV2);
        }
        if buf.starts_with(&GET_NAV_IX_DISCM) {
            return Ok(Self::GetNav);
        }
        if buf.starts_with(&GET_NAV_ADJUSTMENT_IX_DISCM) {
            return Ok(Self::GetNavAdjustment);
        }
        if buf.starts_with(&GET_TVL_IX_DISCM) {
            return Ok(Self::GetTvl);
        }
        if buf.starts_with(&GET_TVL_V2_IX_DISCM) {
            return Ok(Self::GetTvlV2);
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            return Ok(Self::Initialize);
        }
        if buf.starts_with(&INITIALIZE_BUFFER_IX_DISCM) {
            return Ok(Self::InitializeBuffer);
        }
        if buf.starts_with(&INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializePermissionlessAuthority(InitializePermissionlessAuthorityIxArgs {
                    name,
                }),
            );
        }
        if buf.starts_with(&MAKE_OFFER_IX_DISCM) {
            let mut reader = &buf[MAKE_OFFER_IX_DISCM.len()..];
            let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
            let needs_approval: bool = crate::borsh_de_or_default(&mut reader)?;
            let allow_permissionless: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::MakeOffer(MakeOfferIxArgs {
                    fee_basis_points,
                    needs_approval,
                    allow_permissionless,
                }),
            );
        }
        if buf.starts_with(&MAKE_REDEMPTION_OFFER_IX_DISCM) {
            let mut reader = &buf[MAKE_REDEMPTION_OFFER_IX_DISCM.len()..];
            let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
            let fee_basis_points_prop_amm_sell: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::MakeRedemptionOffer(MakeRedemptionOfferIxArgs {
                    fee_basis_points,
                    fee_basis_points_prop_amm_sell,
                }),
            );
        }
        if buf.starts_with(&MINT_TO_IX_DISCM) {
            let mut reader = &buf[MINT_TO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::MintTo(MintToIxArgs { amount }));
        }
        if buf.starts_with(&OFFER_VAULT_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[OFFER_VAULT_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::OfferVaultDeposit(OfferVaultDepositIxArgs { amount }));
        }
        if buf.starts_with(&OFFER_VAULT_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[OFFER_VAULT_WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::OfferVaultWithdraw(OfferVaultWithdrawIxArgs { amount }));
        }
        if buf.starts_with(&OPEN_SWAP_BUY_IX_DISCM) {
            let mut reader = &buf[OPEN_SWAP_BUY_IX_DISCM.len()..];
            let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OpenSwapBuy(OpenSwapBuyIxArgs {
                    token_in_amount,
                    minimum_out,
                }),
            );
        }
        if buf.starts_with(&OPEN_SWAP_SELL_IX_DISCM) {
            let mut reader = &buf[OPEN_SWAP_SELL_IX_DISCM.len()..];
            let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OpenSwapSell(OpenSwapSellIxArgs {
                    token_in_amount,
                    minimum_out,
                }),
            );
        }
        if buf.starts_with(&PROPOSE_BOSS_IX_DISCM) {
            let mut reader = &buf[PROPOSE_BOSS_IX_DISCM.len()..];
            let new_boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ProposeBoss(ProposeBossIxArgs { new_boss }));
        }
        if buf.starts_with(&QUOTE_SWAP_BUY_IX_DISCM) {
            let mut reader = &buf[QUOTE_SWAP_BUY_IX_DISCM.len()..];
            let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::QuoteSwapBuy(QuoteSwapBuyIxArgs {
                    token_in_amount,
                }),
            );
        }
        if buf.starts_with(&QUOTE_SWAP_SELL_IX_DISCM) {
            let mut reader = &buf[QUOTE_SWAP_SELL_IX_DISCM.len()..];
            let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::QuoteSwapSell(QuoteSwapSellIxArgs {
                    token_in_amount,
                }),
            );
        }
        if buf.starts_with(&REDEMPTION_VAULT_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[REDEMPTION_VAULT_DEPOSIT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RedemptionVaultDeposit(RedemptionVaultDepositIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&REDEMPTION_VAULT_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[REDEMPTION_VAULT_WITHDRAW_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RedemptionVaultWithdraw(RedemptionVaultWithdrawIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&REFRESH_MARKET_STATS_IX_DISCM) {
            return Ok(Self::RefreshMarketStats);
        }
        if buf.starts_with(&REMOVE_ADMIN_IX_DISCM) {
            let mut reader = &buf[REMOVE_ADMIN_IX_DISCM.len()..];
            let admin_to_remove: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveAdmin(RemoveAdminIxArgs {
                    admin_to_remove,
                }),
            );
        }
        if buf.starts_with(&REMOVE_APPROVER_IX_DISCM) {
            let mut reader = &buf[REMOVE_APPROVER_IX_DISCM.len()..];
            let approver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RemoveApprover(RemoveApproverIxArgs { approver }));
        }
        if buf.starts_with(&SET_BUFFER_FEE_CONFIG_IX_DISCM) {
            let mut reader = &buf[SET_BUFFER_FEE_CONFIG_IX_DISCM.len()..];
            let management_fee_basis_points: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let performance_fee_basis_points: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let performance_fee_high_watermark_enabled: bool = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetBufferFeeConfig(SetBufferFeeConfigIxArgs {
                    management_fee_basis_points,
                    performance_fee_basis_points,
                    performance_fee_high_watermark_enabled,
                }),
            );
        }
        if buf.starts_with(&SET_BUFFER_GROSS_APR_IX_DISCM) {
            let mut reader = &buf[SET_BUFFER_GROSS_APR_IX_DISCM.len()..];
            let gross_yield: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetBufferGrossApr(SetBufferGrossAprIxArgs {
                    gross_yield,
                }),
            );
        }
        if buf.starts_with(&SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_DISCM) {
            let mut reader = &buf[SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_DISCM
                .len()..];
            let owners: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetCirculatingSupplyExcludedAccounts(SetCirculatingSupplyExcludedAccountsIxArgs {
                    owners,
                }),
            );
        }
        if buf.starts_with(&SET_CONFIGURABLE_VAULT_DESTINATION_IX_DISCM) {
            let mut reader = &buf[SET_CONFIGURABLE_VAULT_DESTINATION_IX_DISCM.len()..];
            let kind: ConfigurableVaultKind = crate::borsh_de_or_default(&mut reader)?;
            let withdrawal_destination: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetConfigurableVaultDestination(SetConfigurableVaultDestinationIxArgs {
                    kind,
                    withdrawal_destination,
                }),
            );
        }
        if buf.starts_with(&SET_KILL_SWITCH_IX_DISCM) {
            let mut reader = &buf[SET_KILL_SWITCH_IX_DISCM.len()..];
            let enable: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetKillSwitch(SetKillSwitchIxArgs { enable }));
        }
        if buf.starts_with(&SET_MAIN_OFFER_IX_DISCM) {
            return Ok(Self::SetMainOffer);
        }
        if buf.starts_with(&SET_OFFER_DISABLED_IX_DISCM) {
            let mut reader = &buf[SET_OFFER_DISABLED_IX_DISCM.len()..];
            let disabled: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetOfferDisabled(SetOfferDisabledIxArgs { disabled }));
        }
        if buf.starts_with(&SET_ONYC_MINT_IX_DISCM) {
            return Ok(Self::SetOnycMint);
        }
        if buf.starts_with(&SET_REDEMPTION_OFFER_DISABLED_IX_DISCM) {
            let mut reader = &buf[SET_REDEMPTION_OFFER_DISABLED_IX_DISCM.len()..];
            let disabled: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetRedemptionOfferDisabled(SetRedemptionOfferDisabledIxArgs {
                    disabled,
                }),
            );
        }
        if buf.starts_with(&SET_WORKER_IX_DISCM) {
            let mut reader = &buf[SET_WORKER_IX_DISCM.len()..];
            let new_worker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetWorker(SetWorkerIxArgs { new_worker }));
        }
        if buf.starts_with(&SETTLE_BUFFER_IX_DISCM) {
            return Ok(Self::SettleBuffer);
        }
        if buf.starts_with(&TAKE_OFFER_IX_DISCM) {
            let mut reader = &buf[TAKE_OFFER_IX_DISCM.len()..];
            let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let approval_message: Option<ApprovalMessage> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::TakeOffer(TakeOfferIxArgs {
                    token_in_amount,
                    approval_message,
                }),
            );
        }
        if buf.starts_with(&TAKE_OFFER_PERMISSIONLESS_IX_DISCM) {
            let mut reader = &buf[TAKE_OFFER_PERMISSIONLESS_IX_DISCM.len()..];
            let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let approval_message: Option<ApprovalMessage> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::TakeOfferPermissionless(TakeOfferPermissionlessIxArgs {
                    token_in_amount,
                    approval_message,
                }),
            );
        }
        if buf.starts_with(&TAKE_OFFER_PERMISSIONLESS_V2_IX_DISCM) {
            let mut reader = &buf[TAKE_OFFER_PERMISSIONLESS_V2_IX_DISCM.len()..];
            let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TakeOfferPermissionlessV2(TakeOfferPermissionlessV2IxArgs {
                    token_in_amount,
                }),
            );
        }
        if buf.starts_with(&TAKE_OFFER_V2_IX_DISCM) {
            let mut reader = &buf[TAKE_OFFER_V2_IX_DISCM.len()..];
            let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let approval_message: Option<ApprovalMessage> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::TakeOfferV2(TakeOfferV2IxArgs {
                    token_in_amount,
                    approval_message,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_DISCM) {
            return Ok(Self::TransferMintAuthorityToBoss);
        }
        if buf.starts_with(&TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_DISCM) {
            return Ok(Self::TransferMintAuthorityToProgram);
        }
        if buf.starts_with(&UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_DISCM) {
            return Ok(Self::UpdateCirculatingSupplyExcludedBalance);
        }
        if buf.starts_with(&UPDATE_OFFER_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_OFFER_FEE_IX_DISCM.len()..];
            let new_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateOfferFee(UpdateOfferFeeIxArgs {
                    new_fee_basis_points,
                }),
            );
        }
        if buf.starts_with(&UPDATE_OFFER_PERMISSIONLESS_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_OFFER_PERMISSIONLESS_FEE_IX_DISCM.len()..];
            let new_fee_basis_points_permissionless: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateOfferPermissionlessFee(UpdateOfferPermissionlessFeeIxArgs {
                    new_fee_basis_points_permissionless,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REDEMPTION_OFFER_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_REDEMPTION_OFFER_FEE_IX_DISCM.len()..];
            let new_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRedemptionOfferFee(UpdateRedemptionOfferFeeIxArgs {
                    new_fee_basis_points,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_DISCM
                .len()..];
            let new_fee_basis_points_prop_amm_sell: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateRedemptionOfferPropAmmSellFee(UpdateRedemptionOfferPropAmmSellFeeIxArgs {
                    new_fee_basis_points_prop_amm_sell,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_DISCM) {
            let mut reader = &buf[UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_DISCM.len()..];
            let new_vault_target_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRedemptionOfferVaultTarget(UpdateRedemptionOfferVaultTargetIxArgs {
                    new_vault_target_bps,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_CONFIGURABLE_VAULT_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_CONFIGURABLE_VAULT_IX_DISCM.len()..];
            let kind: ConfigurableVaultKind = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawConfigurableVault(WithdrawConfigurableVaultIxArgs {
                    kind,
                    amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_RESERVE_VAULT_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_RESERVE_VAULT_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawReserveVault(WithdrawReserveVaultIxArgs {
                    amount,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AcceptBoss => writer.write_all(&ACCEPT_BOSS_IX_DISCM),
            Self::AddAdmin(args) => {
                writer.write_all(&ADD_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_admin, &mut writer)?;
                Ok(())
            }
            Self::AddApprover(args) => {
                writer.write_all(&ADD_APPROVER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.approver, &mut writer)?;
                Ok(())
            }
            Self::AddOfferVector(args) => {
                writer.write_all(&ADD_OFFER_VECTOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.start_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.base_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.base_price, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.apr, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price_fix_duration, &mut writer)?;
                Ok(())
            }
            Self::BurnForNavIncrease(args) => {
                writer.write_all(&BURN_FOR_NAV_INCREASE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.asset_adjustment_amount,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CancelRedemptionRequest => {
                writer.write_all(&CANCEL_REDEMPTION_REQUEST_IX_DISCM)
            }
            Self::ClearAdmins => writer.write_all(&CLEAR_ADMINS_IX_DISCM),
            Self::CloseState => writer.write_all(&CLOSE_STATE_IX_DISCM),
            Self::ConfigureMaxMintAmount(args) => {
                writer.write_all(&CONFIGURE_MAX_MINT_AMOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_mint_amount, &mut writer)?;
                Ok(())
            }
            Self::ConfigureMaxSupply(args) => {
                writer.write_all(&CONFIGURE_MAX_SUPPLY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_supply, &mut writer)?;
                Ok(())
            }
            Self::ConfigurePropAmm(args) => {
                writer.write_all(&CONFIGURE_PROP_AMM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.enabled, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.curve_peg_haircut_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.curve_exponent_scaled,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.cadence_threshold, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.cadence_wave_scaled,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.epoch_duration_seconds,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.wall_sensitivity_scaled,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_sell_haircut_onyc,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CreateRedemptionRequest(args) => {
                writer.write_all(&CREATE_REDEMPTION_REQUEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.request_id, &mut writer)?;
                Ok(())
            }
            Self::DeleteAllOfferVectors => {
                writer.write_all(&DELETE_ALL_OFFER_VECTORS_IX_DISCM)
            }
            Self::DeleteOfferVector(args) => {
                writer.write_all(&DELETE_OFFER_VECTOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.vector_start_time, &mut writer)?;
                Ok(())
            }
            Self::DepositReserveVault(args) => {
                writer.write_all(&DEPOSIT_RESERVE_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::FulfillRedemptionRequest(args) => {
                writer.write_all(&FULFILL_REDEMPTION_REQUEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::GetApy => writer.write_all(&GET_APY_IX_DISCM),
            Self::GetCirculatingSupply => {
                writer.write_all(&GET_CIRCULATING_SUPPLY_IX_DISCM)
            }
            Self::GetCirculatingSupplyV2 => {
                writer.write_all(&GET_CIRCULATING_SUPPLY_V2_IX_DISCM)
            }
            Self::GetNav => writer.write_all(&GET_NAV_IX_DISCM),
            Self::GetNavAdjustment => writer.write_all(&GET_NAV_ADJUSTMENT_IX_DISCM),
            Self::GetTvl => writer.write_all(&GET_TVL_IX_DISCM),
            Self::GetTvlV2 => writer.write_all(&GET_TVL_V2_IX_DISCM),
            Self::Initialize => writer.write_all(&INITIALIZE_IX_DISCM),
            Self::InitializeBuffer => writer.write_all(&INITIALIZE_BUFFER_IX_DISCM),
            Self::InitializePermissionlessAuthority(args) => {
                writer.write_all(&INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                Ok(())
            }
            Self::MakeOffer(args) => {
                writer.write_all(&MAKE_OFFER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_basis_points, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.needs_approval, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.allow_permissionless,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::MakeRedemptionOffer(args) => {
                writer.write_all(&MAKE_REDEMPTION_OFFER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_basis_points, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.fee_basis_points_prop_amm_sell,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::MintTo(args) => {
                writer.write_all(&MINT_TO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::OfferVaultDeposit(args) => {
                writer.write_all(&OFFER_VAULT_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::OfferVaultWithdraw(args) => {
                writer.write_all(&OFFER_VAULT_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::OpenSwapBuy(args) => {
                writer.write_all(&OPEN_SWAP_BUY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_out, &mut writer)?;
                Ok(())
            }
            Self::OpenSwapSell(args) => {
                writer.write_all(&OPEN_SWAP_SELL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_out, &mut writer)?;
                Ok(())
            }
            Self::ProposeBoss(args) => {
                writer.write_all(&PROPOSE_BOSS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_boss, &mut writer)?;
                Ok(())
            }
            Self::QuoteSwapBuy(args) => {
                writer.write_all(&QUOTE_SWAP_BUY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in_amount, &mut writer)?;
                Ok(())
            }
            Self::QuoteSwapSell(args) => {
                writer.write_all(&QUOTE_SWAP_SELL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in_amount, &mut writer)?;
                Ok(())
            }
            Self::RedemptionVaultDeposit(args) => {
                writer.write_all(&REDEMPTION_VAULT_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::RedemptionVaultWithdraw(args) => {
                writer.write_all(&REDEMPTION_VAULT_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::RefreshMarketStats => writer.write_all(&REFRESH_MARKET_STATS_IX_DISCM),
            Self::RemoveAdmin(args) => {
                writer.write_all(&REMOVE_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.admin_to_remove, &mut writer)?;
                Ok(())
            }
            Self::RemoveApprover(args) => {
                writer.write_all(&REMOVE_APPROVER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.approver, &mut writer)?;
                Ok(())
            }
            Self::SetBufferFeeConfig(args) => {
                writer.write_all(&SET_BUFFER_FEE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.management_fee_basis_points,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.performance_fee_basis_points,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.performance_fee_high_watermark_enabled,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetBufferGrossApr(args) => {
                writer.write_all(&SET_BUFFER_GROSS_APR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.gross_yield, &mut writer)?;
                Ok(())
            }
            Self::SetCirculatingSupplyExcludedAccounts(args) => {
                writer.write_all(&SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.owners, &mut writer)?;
                Ok(())
            }
            Self::SetConfigurableVaultDestination(args) => {
                writer.write_all(&SET_CONFIGURABLE_VAULT_DESTINATION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.kind, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.withdrawal_destination,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetKillSwitch(args) => {
                writer.write_all(&SET_KILL_SWITCH_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.enable, &mut writer)?;
                Ok(())
            }
            Self::SetMainOffer => writer.write_all(&SET_MAIN_OFFER_IX_DISCM),
            Self::SetOfferDisabled(args) => {
                writer.write_all(&SET_OFFER_DISABLED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.disabled, &mut writer)?;
                Ok(())
            }
            Self::SetOnycMint => writer.write_all(&SET_ONYC_MINT_IX_DISCM),
            Self::SetRedemptionOfferDisabled(args) => {
                writer.write_all(&SET_REDEMPTION_OFFER_DISABLED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.disabled, &mut writer)?;
                Ok(())
            }
            Self::SetWorker(args) => {
                writer.write_all(&SET_WORKER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_worker, &mut writer)?;
                Ok(())
            }
            Self::SettleBuffer => writer.write_all(&SETTLE_BUFFER_IX_DISCM),
            Self::TakeOffer(args) => {
                writer.write_all(&TAKE_OFFER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.approval_message, &mut writer)?;
                Ok(())
            }
            Self::TakeOfferPermissionless(args) => {
                writer.write_all(&TAKE_OFFER_PERMISSIONLESS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.approval_message, &mut writer)?;
                Ok(())
            }
            Self::TakeOfferPermissionlessV2(args) => {
                writer.write_all(&TAKE_OFFER_PERMISSIONLESS_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in_amount, &mut writer)?;
                Ok(())
            }
            Self::TakeOfferV2(args) => {
                writer.write_all(&TAKE_OFFER_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.approval_message, &mut writer)?;
                Ok(())
            }
            Self::TransferMintAuthorityToBoss => {
                writer.write_all(&TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_DISCM)
            }
            Self::TransferMintAuthorityToProgram => {
                writer.write_all(&TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_DISCM)
            }
            Self::UpdateCirculatingSupplyExcludedBalance => {
                writer.write_all(&UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_DISCM)
            }
            Self::UpdateOfferFee(args) => {
                writer.write_all(&UPDATE_OFFER_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_fee_basis_points,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateOfferPermissionlessFee(args) => {
                writer.write_all(&UPDATE_OFFER_PERMISSIONLESS_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_fee_basis_points_permissionless,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateRedemptionOfferFee(args) => {
                writer.write_all(&UPDATE_REDEMPTION_OFFER_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_fee_basis_points,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateRedemptionOfferPropAmmSellFee(args) => {
                writer.write_all(&UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_fee_basis_points_prop_amm_sell,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateRedemptionOfferVaultTarget(args) => {
                writer.write_all(&UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_vault_target_bps,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::WithdrawConfigurableVault(args) => {
                writer.write_all(&WITHDRAW_CONFIGURABLE_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.kind, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawReserveVault(args) => {
                writer.write_all(&WITHDRAW_RESERVE_VAULT_IX_DISCM)?;
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
pub const ACCEPT_BOSS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AcceptBossAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub new_boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptBossKeys {
    pub state: Pubkey,
    pub new_boss: Pubkey,
}
impl From<AcceptBossAccounts<'_, '_>> for AcceptBossKeys {
    fn from(accounts: AcceptBossAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            new_boss: *accounts.new_boss.key,
        }
    }
}
impl From<AcceptBossKeys> for [AccountMeta; ACCEPT_BOSS_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptBossKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ACCEPT_BOSS_IX_ACCOUNTS_LEN]> for AcceptBossKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_BOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            new_boss: pubkeys[1],
        }
    }
}
impl<'info> From<AcceptBossAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_BOSS_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptBossAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.new_boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ACCEPT_BOSS_IX_ACCOUNTS_LEN]>
for AcceptBossAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ACCEPT_BOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            new_boss: &arr[1],
        }
    }
}
pub const ACCEPT_BOSS_IX_DISCM: [u8; 8usize] = [152, 63, 117, 209, 67, 11, 250, 242];
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptBossIxData;
impl AcceptBossIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_BOSS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_BOSS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_boss_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptBossKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_BOSS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AcceptBossIxData.try_to_vec()?,
    })
}
pub fn accept_boss_ix(keys: AcceptBossKeys) -> std::io::Result<Instruction> {
    accept_boss_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn accept_boss_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptBossAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AcceptBossKeys = accounts.into();
    let ix = accept_boss_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_boss_invoke(accounts: AcceptBossAccounts<'_, '_>) -> ProgramResult {
    accept_boss_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn accept_boss_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptBossAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptBossKeys = accounts.into();
    let ix = accept_boss_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_boss_invoke_signed(
    accounts: AcceptBossAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_boss_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn accept_boss_verify_account_keys(
    accounts: AcceptBossAccounts<'_, '_>,
    keys: AcceptBossKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.new_boss.key, keys.new_boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_boss_verify_writable_privileges<'me, 'info>(
    accounts: AcceptBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_boss_verify_signer_privileges<'me, 'info>(
    accounts: AcceptBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_boss_verify_account_privileges<'me, 'info>(
    accounts: AcceptBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_boss_verify_writable_privileges(accounts)?;
    accept_boss_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AddAdminAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddAdminKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<AddAdminAccounts<'_, '_>> for AddAdminKeys {
    fn from(accounts: AddAdminAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<AddAdminKeys> for [AccountMeta; ADD_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: AddAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_ADMIN_IX_ACCOUNTS_LEN]> for AddAdminKeys {
    fn from(pubkeys: [Pubkey; ADD_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<AddAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddAdminAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_ADMIN_IX_ACCOUNTS_LEN]>
for AddAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const ADD_ADMIN_IX_DISCM: [u8; 8usize] = [177, 236, 33, 205, 124, 152, 55, 186];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddAdminIxArgs {
    pub new_admin: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddAdminIxData(pub AddAdminIxArgs);
impl From<AddAdminIxArgs> for AddAdminIxData {
    fn from(args: AddAdminIxArgs) -> Self {
        Self(args)
    }
}
impl AddAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(AddAdminIxArgs { new_admin }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_admin, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: AddAdminKeys,
    args: AddAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_admin_ix(
    keys: AddAdminKeys,
    args: AddAdminIxArgs,
) -> std::io::Result<Instruction> {
    add_admin_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn add_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddAdminAccounts<'_, '_>,
    args: AddAdminIxArgs,
) -> ProgramResult {
    let keys: AddAdminKeys = accounts.into();
    let ix = add_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_admin_invoke(
    accounts: AddAdminAccounts<'_, '_>,
    args: AddAdminIxArgs,
) -> ProgramResult {
    add_admin_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn add_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddAdminAccounts<'_, '_>,
    args: AddAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddAdminKeys = accounts.into();
    let ix = add_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_admin_invoke_signed(
    accounts: AddAdminAccounts<'_, '_>,
    args: AddAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_admin_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_admin_verify_account_keys(
    accounts: AddAdminAccounts<'_, '_>,
    keys: AddAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_admin_verify_writable_privileges<'me, 'info>(
    accounts: AddAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_admin_verify_signer_privileges<'me, 'info>(
    accounts: AddAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_admin_verify_account_privileges<'me, 'info>(
    accounts: AddAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_admin_verify_writable_privileges(accounts)?;
    add_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_APPROVER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AddApproverAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddApproverKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<AddApproverAccounts<'_, '_>> for AddApproverKeys {
    fn from(accounts: AddApproverAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<AddApproverKeys> for [AccountMeta; ADD_APPROVER_IX_ACCOUNTS_LEN] {
    fn from(keys: AddApproverKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_APPROVER_IX_ACCOUNTS_LEN]> for AddApproverKeys {
    fn from(pubkeys: [Pubkey; ADD_APPROVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<AddApproverAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_APPROVER_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddApproverAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_APPROVER_IX_ACCOUNTS_LEN]>
for AddApproverAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_APPROVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const ADD_APPROVER_IX_DISCM: [u8; 8usize] = [213, 245, 135, 79, 129, 129, 22, 80];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddApproverIxArgs {
    pub approver: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddApproverIxData(pub AddApproverIxArgs);
impl From<AddApproverIxArgs> for AddApproverIxData {
    fn from(args: AddApproverIxArgs) -> Self {
        Self(args)
    }
}
impl AddApproverIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_APPROVER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let approver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(AddApproverIxArgs { approver }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_APPROVER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.approver, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_approver_ix_with_program_id(
    program_id: Pubkey,
    keys: AddApproverKeys,
    args: AddApproverIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_APPROVER_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddApproverIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_approver_ix(
    keys: AddApproverKeys,
    args: AddApproverIxArgs,
) -> std::io::Result<Instruction> {
    add_approver_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn add_approver_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddApproverAccounts<'_, '_>,
    args: AddApproverIxArgs,
) -> ProgramResult {
    let keys: AddApproverKeys = accounts.into();
    let ix = add_approver_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_approver_invoke(
    accounts: AddApproverAccounts<'_, '_>,
    args: AddApproverIxArgs,
) -> ProgramResult {
    add_approver_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn add_approver_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddApproverAccounts<'_, '_>,
    args: AddApproverIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddApproverKeys = accounts.into();
    let ix = add_approver_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_approver_invoke_signed(
    accounts: AddApproverAccounts<'_, '_>,
    args: AddApproverIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_approver_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_approver_verify_account_keys(
    accounts: AddApproverAccounts<'_, '_>,
    keys: AddApproverKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_approver_verify_writable_privileges<'me, 'info>(
    accounts: AddApproverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_approver_verify_signer_privileges<'me, 'info>(
    accounts: AddApproverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_approver_verify_account_privileges<'me, 'info>(
    accounts: AddApproverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_approver_verify_writable_privileges(accounts)?;
    add_approver_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_OFFER_VECTOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AddOfferVectorAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddOfferVectorKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<AddOfferVectorAccounts<'_, '_>> for AddOfferVectorKeys {
    fn from(accounts: AddOfferVectorAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<AddOfferVectorKeys> for [AccountMeta; ADD_OFFER_VECTOR_IX_ACCOUNTS_LEN] {
    fn from(keys: AddOfferVectorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_OFFER_VECTOR_IX_ACCOUNTS_LEN]> for AddOfferVectorKeys {
    fn from(pubkeys: [Pubkey; ADD_OFFER_VECTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
            state: pubkeys[3],
            boss: pubkeys[4],
        }
    }
}
impl<'info> From<AddOfferVectorAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_OFFER_VECTOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddOfferVectorAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_OFFER_VECTOR_IX_ACCOUNTS_LEN]>
for AddOfferVectorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_OFFER_VECTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
            state: &arr[3],
            boss: &arr[4],
        }
    }
}
pub const ADD_OFFER_VECTOR_IX_DISCM: [u8; 8usize] = [
    198, 139, 180, 6, 156, 171, 188, 61,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddOfferVectorIxArgs {
    pub start_time: Option<u64>,
    pub base_time: u64,
    pub base_price: u64,
    pub apr: u64,
    pub price_fix_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddOfferVectorIxData(pub AddOfferVectorIxArgs);
impl From<AddOfferVectorIxArgs> for AddOfferVectorIxData {
    fn from(args: AddOfferVectorIxArgs) -> Self {
        Self(args)
    }
}
impl AddOfferVectorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_OFFER_VECTOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let start_time: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let base_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let apr: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_fix_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddOfferVectorIxArgs {
                start_time,
                base_time,
                base_price,
                apr,
                price_fix_duration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_OFFER_VECTOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.base_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.base_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.apr, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price_fix_duration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_offer_vector_ix_with_program_id(
    program_id: Pubkey,
    keys: AddOfferVectorKeys,
    args: AddOfferVectorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_OFFER_VECTOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddOfferVectorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_offer_vector_ix(
    keys: AddOfferVectorKeys,
    args: AddOfferVectorIxArgs,
) -> std::io::Result<Instruction> {
    add_offer_vector_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn add_offer_vector_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddOfferVectorAccounts<'_, '_>,
    args: AddOfferVectorIxArgs,
) -> ProgramResult {
    let keys: AddOfferVectorKeys = accounts.into();
    let ix = add_offer_vector_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_offer_vector_invoke(
    accounts: AddOfferVectorAccounts<'_, '_>,
    args: AddOfferVectorIxArgs,
) -> ProgramResult {
    add_offer_vector_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn add_offer_vector_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddOfferVectorAccounts<'_, '_>,
    args: AddOfferVectorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddOfferVectorKeys = accounts.into();
    let ix = add_offer_vector_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_offer_vector_invoke_signed(
    accounts: AddOfferVectorAccounts<'_, '_>,
    args: AddOfferVectorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_offer_vector_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_offer_vector_verify_account_keys(
    accounts: AddOfferVectorAccounts<'_, '_>,
    keys: AddOfferVectorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_offer_vector_verify_writable_privileges<'me, 'info>(
    accounts: AddOfferVectorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_offer_vector_verify_signer_privileges<'me, 'info>(
    accounts: AddOfferVectorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_offer_vector_verify_account_privileges<'me, 'info>(
    accounts: AddOfferVectorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_offer_vector_verify_writable_privileges(accounts)?;
    add_offer_vector_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_FOR_NAV_INCREASE_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct BurnForNavIncreaseAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub buffer_state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub offer_vault_authority: &'me AccountInfo<'info>,
    pub reserve_vault_authority: &'me AccountInfo<'info>,
    pub reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub management_fee_vault: &'me AccountInfo<'info>,
    pub management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub performance_fee_vault: &'me AccountInfo<'info>,
    pub performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BurnForNavIncreaseKeys {
    pub state: Pubkey,
    pub buffer_state: Pubkey,
    pub boss: Pubkey,
    pub main_offer: Pubkey,
    pub onyc_mint: Pubkey,
    pub offer_vault_authority: Pubkey,
    pub reserve_vault_authority: Pubkey,
    pub reserve_vault_onyc_account: Pubkey,
    pub management_fee_vault: Pubkey,
    pub management_fee_vault_onyc_account: Pubkey,
    pub performance_fee_vault: Pubkey,
    pub performance_fee_vault_onyc_account: Pubkey,
    pub mint_authority: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
}
impl From<BurnForNavIncreaseAccounts<'_, '_>> for BurnForNavIncreaseKeys {
    fn from(accounts: BurnForNavIncreaseAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            buffer_state: *accounts.buffer_state.key,
            boss: *accounts.boss.key,
            main_offer: *accounts.main_offer.key,
            onyc_mint: *accounts.onyc_mint.key,
            offer_vault_authority: *accounts.offer_vault_authority.key,
            reserve_vault_authority: *accounts.reserve_vault_authority.key,
            reserve_vault_onyc_account: *accounts.reserve_vault_onyc_account.key,
            management_fee_vault: *accounts.management_fee_vault.key,
            management_fee_vault_onyc_account: *accounts
                .management_fee_vault_onyc_account
                .key,
            performance_fee_vault: *accounts.performance_fee_vault.key,
            performance_fee_vault_onyc_account: *accounts
                .performance_fee_vault_onyc_account
                .key,
            mint_authority: *accounts.mint_authority.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
        }
    }
}
impl From<BurnForNavIncreaseKeys>
for [AccountMeta; BURN_FOR_NAV_INCREASE_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnForNavIncreaseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.management_fee_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.performance_fee_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.performance_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BURN_FOR_NAV_INCREASE_IX_ACCOUNTS_LEN]> for BurnForNavIncreaseKeys {
    fn from(pubkeys: [Pubkey; BURN_FOR_NAV_INCREASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            buffer_state: pubkeys[1],
            boss: pubkeys[2],
            main_offer: pubkeys[3],
            onyc_mint: pubkeys[4],
            offer_vault_authority: pubkeys[5],
            reserve_vault_authority: pubkeys[6],
            reserve_vault_onyc_account: pubkeys[7],
            management_fee_vault: pubkeys[8],
            management_fee_vault_onyc_account: pubkeys[9],
            performance_fee_vault: pubkeys[10],
            performance_fee_vault_onyc_account: pubkeys[11],
            mint_authority: pubkeys[12],
            token_program: pubkeys[13],
            system_program: pubkeys[14],
            market_stats: pubkeys[15],
            circulating_supply_excluded_balance: pubkeys[16],
        }
    }
}
impl<'info> From<BurnForNavIncreaseAccounts<'_, 'info>>
for [AccountInfo<'info>; BURN_FOR_NAV_INCREASE_IX_ACCOUNTS_LEN] {
    fn from(accounts: BurnForNavIncreaseAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.buffer_state.clone(),
            accounts.boss.clone(),
            accounts.main_offer.clone(),
            accounts.onyc_mint.clone(),
            accounts.offer_vault_authority.clone(),
            accounts.reserve_vault_authority.clone(),
            accounts.reserve_vault_onyc_account.clone(),
            accounts.management_fee_vault.clone(),
            accounts.management_fee_vault_onyc_account.clone(),
            accounts.performance_fee_vault.clone(),
            accounts.performance_fee_vault_onyc_account.clone(),
            accounts.mint_authority.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_FOR_NAV_INCREASE_IX_ACCOUNTS_LEN]>
for BurnForNavIncreaseAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BURN_FOR_NAV_INCREASE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            buffer_state: &arr[1],
            boss: &arr[2],
            main_offer: &arr[3],
            onyc_mint: &arr[4],
            offer_vault_authority: &arr[5],
            reserve_vault_authority: &arr[6],
            reserve_vault_onyc_account: &arr[7],
            management_fee_vault: &arr[8],
            management_fee_vault_onyc_account: &arr[9],
            performance_fee_vault: &arr[10],
            performance_fee_vault_onyc_account: &arr[11],
            mint_authority: &arr[12],
            token_program: &arr[13],
            system_program: &arr[14],
            market_stats: &arr[15],
            circulating_supply_excluded_balance: &arr[16],
        }
    }
}
pub const BURN_FOR_NAV_INCREASE_IX_DISCM: [u8; 8usize] = [
    8, 13, 69, 178, 183, 45, 102, 205,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BurnForNavIncreaseIxArgs {
    pub asset_adjustment_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnForNavIncreaseIxData(pub BurnForNavIncreaseIxArgs);
impl From<BurnForNavIncreaseIxArgs> for BurnForNavIncreaseIxData {
    fn from(args: BurnForNavIncreaseIxArgs) -> Self {
        Self(args)
    }
}
impl BurnForNavIncreaseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BURN_FOR_NAV_INCREASE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let asset_adjustment_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BurnForNavIncreaseIxArgs {
                asset_adjustment_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BURN_FOR_NAV_INCREASE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.asset_adjustment_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn burn_for_nav_increase_ix_with_program_id(
    program_id: Pubkey,
    keys: BurnForNavIncreaseKeys,
    args: BurnForNavIncreaseIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_FOR_NAV_INCREASE_IX_ACCOUNTS_LEN] = keys.into();
    let data: BurnForNavIncreaseIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn burn_for_nav_increase_ix(
    keys: BurnForNavIncreaseKeys,
    args: BurnForNavIncreaseIxArgs,
) -> std::io::Result<Instruction> {
    burn_for_nav_increase_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn burn_for_nav_increase_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BurnForNavIncreaseAccounts<'_, '_>,
    args: BurnForNavIncreaseIxArgs,
) -> ProgramResult {
    let keys: BurnForNavIncreaseKeys = accounts.into();
    let ix = burn_for_nav_increase_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn burn_for_nav_increase_invoke(
    accounts: BurnForNavIncreaseAccounts<'_, '_>,
    args: BurnForNavIncreaseIxArgs,
) -> ProgramResult {
    burn_for_nav_increase_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn burn_for_nav_increase_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BurnForNavIncreaseAccounts<'_, '_>,
    args: BurnForNavIncreaseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnForNavIncreaseKeys = accounts.into();
    let ix = burn_for_nav_increase_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn burn_for_nav_increase_invoke_signed(
    accounts: BurnForNavIncreaseAccounts<'_, '_>,
    args: BurnForNavIncreaseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    burn_for_nav_increase_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn burn_for_nav_increase_verify_account_keys(
    accounts: BurnForNavIncreaseAccounts<'_, '_>,
    keys: BurnForNavIncreaseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.buffer_state.key, keys.buffer_state),
        (*accounts.boss.key, keys.boss),
        (*accounts.main_offer.key, keys.main_offer),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.offer_vault_authority.key, keys.offer_vault_authority),
        (*accounts.reserve_vault_authority.key, keys.reserve_vault_authority),
        (*accounts.reserve_vault_onyc_account.key, keys.reserve_vault_onyc_account),
        (*accounts.management_fee_vault.key, keys.management_fee_vault),
        (
            *accounts.management_fee_vault_onyc_account.key,
            keys.management_fee_vault_onyc_account,
        ),
        (*accounts.performance_fee_vault.key, keys.performance_fee_vault),
        (
            *accounts.performance_fee_vault_onyc_account.key,
            keys.performance_fee_vault_onyc_account,
        ),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn burn_for_nav_increase_verify_writable_privileges<'me, 'info>(
    accounts: BurnForNavIncreaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.buffer_state,
        accounts.boss,
        accounts.onyc_mint,
        accounts.reserve_vault_onyc_account,
        accounts.management_fee_vault_onyc_account,
        accounts.performance_fee_vault_onyc_account,
        accounts.market_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_for_nav_increase_verify_signer_privileges<'me, 'info>(
    accounts: BurnForNavIncreaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_for_nav_increase_verify_account_privileges<'me, 'info>(
    accounts: BurnForNavIncreaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_for_nav_increase_verify_writable_privileges(accounts)?;
    burn_for_nav_increase_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CancelRedemptionRequestAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub redemption_request: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub redeemer: &'me AccountInfo<'info>,
    pub worker: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub redeemer_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelRedemptionRequestKeys {
    pub state: Pubkey,
    pub redemption_offer: Pubkey,
    pub redemption_request: Pubkey,
    pub signer: Pubkey,
    pub redeemer: Pubkey,
    pub worker: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub token_in_mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub redeemer_token_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<CancelRedemptionRequestAccounts<'_, '_>> for CancelRedemptionRequestKeys {
    fn from(accounts: CancelRedemptionRequestAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            redemption_offer: *accounts.redemption_offer.key,
            redemption_request: *accounts.redemption_request.key,
            signer: *accounts.signer.key,
            redeemer: *accounts.redeemer.key,
            worker: *accounts.worker.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            token_in_mint: *accounts.token_in_mint.key,
            vault_token_account: *accounts.vault_token_account.key,
            redeemer_token_account: *accounts.redeemer_token_account.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<CancelRedemptionRequestKeys>
for [AccountMeta; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelRedemptionRequestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redeemer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.worker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redeemer_token_account,
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
                pubkey: keys.associated_token_program,
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
            state: pubkeys[0],
            redemption_offer: pubkeys[1],
            redemption_request: pubkeys[2],
            signer: pubkeys[3],
            redeemer: pubkeys[4],
            worker: pubkeys[5],
            redemption_vault_authority: pubkeys[6],
            token_in_mint: pubkeys[7],
            vault_token_account: pubkeys[8],
            redeemer_token_account: pubkeys[9],
            token_program: pubkeys[10],
            system_program: pubkeys[11],
            associated_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<CancelRedemptionRequestAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelRedemptionRequestAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.redemption_offer.clone(),
            accounts.redemption_request.clone(),
            accounts.signer.clone(),
            accounts.redeemer.clone(),
            accounts.worker.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.token_in_mint.clone(),
            accounts.vault_token_account.clone(),
            accounts.redeemer_token_account.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
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
            state: &arr[0],
            redemption_offer: &arr[1],
            redemption_request: &arr[2],
            signer: &arr[3],
            redeemer: &arr[4],
            worker: &arr[5],
            redemption_vault_authority: &arr[6],
            token_in_mint: &arr[7],
            vault_token_account: &arr[8],
            redeemer_token_account: &arr[9],
            token_program: &arr[10],
            system_program: &arr[11],
            associated_token_program: &arr[12],
        }
    }
}
pub const CANCEL_REDEMPTION_REQUEST_IX_DISCM: [u8; 8usize] = [
    77, 155, 4, 179, 114, 233, 162, 45,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelRedemptionRequestIxData;
impl CancelRedemptionRequestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_REDEMPTION_REQUEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_REDEMPTION_REQUEST_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelRedemptionRequestIxData.try_to_vec()?,
    })
}
pub fn cancel_redemption_request_ix(
    keys: CancelRedemptionRequestKeys,
) -> std::io::Result<Instruction> {
    cancel_redemption_request_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn cancel_redemption_request_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelRedemptionRequestKeys = accounts.into();
    let ix = cancel_redemption_request_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_redemption_request_invoke(
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
) -> ProgramResult {
    cancel_redemption_request_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn cancel_redemption_request_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelRedemptionRequestKeys = accounts.into();
    let ix = cancel_redemption_request_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_redemption_request_invoke_signed(
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_redemption_request_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn cancel_redemption_request_verify_account_keys(
    accounts: CancelRedemptionRequestAccounts<'_, '_>,
    keys: CancelRedemptionRequestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.redemption_request.key, keys.redemption_request),
        (*accounts.signer.key, keys.signer),
        (*accounts.redeemer.key, keys.redeemer),
        (*accounts.worker.key, keys.worker),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.vault_token_account.key, keys.vault_token_account),
        (*accounts.redeemer_token_account.key, keys.redeemer_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
        accounts.redemption_offer,
        accounts.redemption_request,
        accounts.signer,
        accounts.worker,
        accounts.vault_token_account,
        accounts.redeemer_token_account,
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
pub const CLEAR_ADMINS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ClearAdminsAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClearAdminsKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<ClearAdminsAccounts<'_, '_>> for ClearAdminsKeys {
    fn from(accounts: ClearAdminsAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<ClearAdminsKeys> for [AccountMeta; CLEAR_ADMINS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClearAdminsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLEAR_ADMINS_IX_ACCOUNTS_LEN]> for ClearAdminsKeys {
    fn from(pubkeys: [Pubkey; CLEAR_ADMINS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<ClearAdminsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLEAR_ADMINS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClearAdminsAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLEAR_ADMINS_IX_ACCOUNTS_LEN]>
for ClearAdminsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLEAR_ADMINS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const CLEAR_ADMINS_IX_DISCM: [u8; 8usize] = [39, 200, 132, 30, 196, 160, 73, 55];
#[derive(Clone, Debug, PartialEq)]
pub struct ClearAdminsIxData;
impl ClearAdminsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLEAR_ADMINS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLEAR_ADMINS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn clear_admins_ix_with_program_id(
    program_id: Pubkey,
    keys: ClearAdminsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLEAR_ADMINS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClearAdminsIxData.try_to_vec()?,
    })
}
pub fn clear_admins_ix(keys: ClearAdminsKeys) -> std::io::Result<Instruction> {
    clear_admins_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn clear_admins_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClearAdminsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClearAdminsKeys = accounts.into();
    let ix = clear_admins_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn clear_admins_invoke(accounts: ClearAdminsAccounts<'_, '_>) -> ProgramResult {
    clear_admins_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn clear_admins_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClearAdminsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClearAdminsKeys = accounts.into();
    let ix = clear_admins_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn clear_admins_invoke_signed(
    accounts: ClearAdminsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    clear_admins_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn clear_admins_verify_account_keys(
    accounts: ClearAdminsAccounts<'_, '_>,
    keys: ClearAdminsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn clear_admins_verify_writable_privileges<'me, 'info>(
    accounts: ClearAdminsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn clear_admins_verify_signer_privileges<'me, 'info>(
    accounts: ClearAdminsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn clear_admins_verify_account_privileges<'me, 'info>(
    accounts: ClearAdminsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    clear_admins_verify_writable_privileges(accounts)?;
    clear_admins_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_STATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseStateAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseStateKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseStateAccounts<'_, '_>> for CloseStateKeys {
    fn from(accounts: CloseStateAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseStateKeys> for [AccountMeta; CLOSE_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
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
impl From<[Pubkey; CLOSE_STATE_IX_ACCOUNTS_LEN]> for CloseStateKeys {
    fn from(pubkeys: [Pubkey; CLOSE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CloseStateAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseStateAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone(), accounts.system_program.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_STATE_IX_ACCOUNTS_LEN]>
for CloseStateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CLOSE_STATE_IX_DISCM: [u8; 8usize] = [25, 1, 184, 101, 200, 245, 210, 246];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseStateIxData;
impl CloseStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_STATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_state_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseStateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_STATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseStateIxData.try_to_vec()?,
    })
}
pub fn close_state_ix(keys: CloseStateKeys) -> std::io::Result<Instruction> {
    close_state_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn close_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseStateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseStateKeys = accounts.into();
    let ix = close_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_state_invoke(accounts: CloseStateAccounts<'_, '_>) -> ProgramResult {
    close_state_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn close_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseStateKeys = accounts.into();
    let ix = close_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_state_invoke_signed(
    accounts: CloseStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_state_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn close_state_verify_account_keys(
    accounts: CloseStateAccounts<'_, '_>,
    keys: CloseStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_state_verify_writable_privileges<'me, 'info>(
    accounts: CloseStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.boss] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_state_verify_signer_privileges<'me, 'info>(
    accounts: CloseStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_state_verify_account_privileges<'me, 'info>(
    accounts: CloseStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_state_verify_writable_privileges(accounts)?;
    close_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONFIGURE_MAX_MINT_AMOUNT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ConfigureMaxMintAmountAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigureMaxMintAmountKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<ConfigureMaxMintAmountAccounts<'_, '_>> for ConfigureMaxMintAmountKeys {
    fn from(accounts: ConfigureMaxMintAmountAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<ConfigureMaxMintAmountKeys>
for [AccountMeta; CONFIGURE_MAX_MINT_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigureMaxMintAmountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONFIGURE_MAX_MINT_AMOUNT_IX_ACCOUNTS_LEN]>
for ConfigureMaxMintAmountKeys {
    fn from(pubkeys: [Pubkey; CONFIGURE_MAX_MINT_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<ConfigureMaxMintAmountAccounts<'_, 'info>>
for [AccountInfo<'info>; CONFIGURE_MAX_MINT_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConfigureMaxMintAmountAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONFIGURE_MAX_MINT_AMOUNT_IX_ACCOUNTS_LEN]>
for ConfigureMaxMintAmountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONFIGURE_MAX_MINT_AMOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const CONFIGURE_MAX_MINT_AMOUNT_IX_DISCM: [u8; 8usize] = [
    7, 197, 225, 52, 209, 53, 89, 172,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigureMaxMintAmountIxArgs {
    pub max_mint_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigureMaxMintAmountIxData(pub ConfigureMaxMintAmountIxArgs);
impl From<ConfigureMaxMintAmountIxArgs> for ConfigureMaxMintAmountIxData {
    fn from(args: ConfigureMaxMintAmountIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigureMaxMintAmountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURE_MAX_MINT_AMOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ConfigureMaxMintAmountIxArgs {
                max_mint_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURE_MAX_MINT_AMOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_mint_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn configure_max_mint_amount_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigureMaxMintAmountKeys,
    args: ConfigureMaxMintAmountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIGURE_MAX_MINT_AMOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConfigureMaxMintAmountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn configure_max_mint_amount_ix(
    keys: ConfigureMaxMintAmountKeys,
    args: ConfigureMaxMintAmountIxArgs,
) -> std::io::Result<Instruction> {
    configure_max_mint_amount_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn configure_max_mint_amount_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureMaxMintAmountAccounts<'_, '_>,
    args: ConfigureMaxMintAmountIxArgs,
) -> ProgramResult {
    let keys: ConfigureMaxMintAmountKeys = accounts.into();
    let ix = configure_max_mint_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn configure_max_mint_amount_invoke(
    accounts: ConfigureMaxMintAmountAccounts<'_, '_>,
    args: ConfigureMaxMintAmountIxArgs,
) -> ProgramResult {
    configure_max_mint_amount_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn configure_max_mint_amount_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureMaxMintAmountAccounts<'_, '_>,
    args: ConfigureMaxMintAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigureMaxMintAmountKeys = accounts.into();
    let ix = configure_max_mint_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn configure_max_mint_amount_invoke_signed(
    accounts: ConfigureMaxMintAmountAccounts<'_, '_>,
    args: ConfigureMaxMintAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    configure_max_mint_amount_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn configure_max_mint_amount_verify_account_keys(
    accounts: ConfigureMaxMintAmountAccounts<'_, '_>,
    keys: ConfigureMaxMintAmountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn configure_max_mint_amount_verify_writable_privileges<'me, 'info>(
    accounts: ConfigureMaxMintAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn configure_max_mint_amount_verify_signer_privileges<'me, 'info>(
    accounts: ConfigureMaxMintAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn configure_max_mint_amount_verify_account_privileges<'me, 'info>(
    accounts: ConfigureMaxMintAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    configure_max_mint_amount_verify_writable_privileges(accounts)?;
    configure_max_mint_amount_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONFIGURE_MAX_SUPPLY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ConfigureMaxSupplyAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigureMaxSupplyKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<ConfigureMaxSupplyAccounts<'_, '_>> for ConfigureMaxSupplyKeys {
    fn from(accounts: ConfigureMaxSupplyAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<ConfigureMaxSupplyKeys>
for [AccountMeta; CONFIGURE_MAX_SUPPLY_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigureMaxSupplyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONFIGURE_MAX_SUPPLY_IX_ACCOUNTS_LEN]> for ConfigureMaxSupplyKeys {
    fn from(pubkeys: [Pubkey; CONFIGURE_MAX_SUPPLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<ConfigureMaxSupplyAccounts<'_, 'info>>
for [AccountInfo<'info>; CONFIGURE_MAX_SUPPLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConfigureMaxSupplyAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONFIGURE_MAX_SUPPLY_IX_ACCOUNTS_LEN]>
for ConfigureMaxSupplyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONFIGURE_MAX_SUPPLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const CONFIGURE_MAX_SUPPLY_IX_DISCM: [u8; 8usize] = [
    145, 100, 133, 229, 142, 59, 96, 62,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigureMaxSupplyIxArgs {
    pub max_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigureMaxSupplyIxData(pub ConfigureMaxSupplyIxArgs);
impl From<ConfigureMaxSupplyIxArgs> for ConfigureMaxSupplyIxData {
    fn from(args: ConfigureMaxSupplyIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigureMaxSupplyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURE_MAX_SUPPLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ConfigureMaxSupplyIxArgs {
                max_supply,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURE_MAX_SUPPLY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_supply, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn configure_max_supply_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigureMaxSupplyKeys,
    args: ConfigureMaxSupplyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIGURE_MAX_SUPPLY_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConfigureMaxSupplyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn configure_max_supply_ix(
    keys: ConfigureMaxSupplyKeys,
    args: ConfigureMaxSupplyIxArgs,
) -> std::io::Result<Instruction> {
    configure_max_supply_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn configure_max_supply_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureMaxSupplyAccounts<'_, '_>,
    args: ConfigureMaxSupplyIxArgs,
) -> ProgramResult {
    let keys: ConfigureMaxSupplyKeys = accounts.into();
    let ix = configure_max_supply_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn configure_max_supply_invoke(
    accounts: ConfigureMaxSupplyAccounts<'_, '_>,
    args: ConfigureMaxSupplyIxArgs,
) -> ProgramResult {
    configure_max_supply_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn configure_max_supply_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigureMaxSupplyAccounts<'_, '_>,
    args: ConfigureMaxSupplyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigureMaxSupplyKeys = accounts.into();
    let ix = configure_max_supply_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn configure_max_supply_invoke_signed(
    accounts: ConfigureMaxSupplyAccounts<'_, '_>,
    args: ConfigureMaxSupplyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    configure_max_supply_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn configure_max_supply_verify_account_keys(
    accounts: ConfigureMaxSupplyAccounts<'_, '_>,
    keys: ConfigureMaxSupplyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn configure_max_supply_verify_writable_privileges<'me, 'info>(
    accounts: ConfigureMaxSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn configure_max_supply_verify_signer_privileges<'me, 'info>(
    accounts: ConfigureMaxSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn configure_max_supply_verify_account_privileges<'me, 'info>(
    accounts: ConfigureMaxSupplyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    configure_max_supply_verify_writable_privileges(accounts)?;
    configure_max_supply_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONFIGURE_PROP_AMM_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ConfigurePropAmmAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub offer: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub prop_amm_pair_state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigurePropAmmKeys {
    pub state: Pubkey,
    pub offer: Pubkey,
    pub asset_mint: Pubkey,
    pub prop_amm_pair_state: Pubkey,
    pub boss: Pubkey,
    pub system_program: Pubkey,
}
impl From<ConfigurePropAmmAccounts<'_, '_>> for ConfigurePropAmmKeys {
    fn from(accounts: ConfigurePropAmmAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            offer: *accounts.offer.key,
            asset_mint: *accounts.asset_mint.key,
            prop_amm_pair_state: *accounts.prop_amm_pair_state.key,
            boss: *accounts.boss.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ConfigurePropAmmKeys> for [AccountMeta; CONFIGURE_PROP_AMM_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigurePropAmmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.prop_amm_pair_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
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
impl From<[Pubkey; CONFIGURE_PROP_AMM_IX_ACCOUNTS_LEN]> for ConfigurePropAmmKeys {
    fn from(pubkeys: [Pubkey; CONFIGURE_PROP_AMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            offer: pubkeys[1],
            asset_mint: pubkeys[2],
            prop_amm_pair_state: pubkeys[3],
            boss: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<ConfigurePropAmmAccounts<'_, 'info>>
for [AccountInfo<'info>; CONFIGURE_PROP_AMM_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConfigurePropAmmAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.offer.clone(),
            accounts.asset_mint.clone(),
            accounts.prop_amm_pair_state.clone(),
            accounts.boss.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONFIGURE_PROP_AMM_IX_ACCOUNTS_LEN]>
for ConfigurePropAmmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CONFIGURE_PROP_AMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            offer: &arr[1],
            asset_mint: &arr[2],
            prop_amm_pair_state: &arr[3],
            boss: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const CONFIGURE_PROP_AMM_IX_DISCM: [u8; 8usize] = [
    235, 104, 216, 250, 252, 160, 107, 181,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigurePropAmmIxArgs {
    pub enabled: bool,
    pub curve_peg_haircut_bps: u16,
    pub curve_exponent_scaled: u32,
    pub cadence_threshold: u32,
    pub cadence_wave_scaled: u32,
    pub epoch_duration_seconds: i64,
    pub wall_sensitivity_scaled: u32,
    pub minimum_sell_haircut_onyc: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigurePropAmmIxData(pub ConfigurePropAmmIxArgs);
impl From<ConfigurePropAmmIxArgs> for ConfigurePropAmmIxData {
    fn from(args: ConfigurePropAmmIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigurePropAmmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURE_PROP_AMM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let curve_peg_haircut_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let curve_exponent_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let cadence_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
        let cadence_wave_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let epoch_duration_seconds: i64 = crate::borsh_de_or_default(&mut reader)?;
        let wall_sensitivity_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_sell_haircut_onyc: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ConfigurePropAmmIxArgs {
                enabled,
                curve_peg_haircut_bps,
                curve_exponent_scaled,
                cadence_threshold,
                cadence_wave_scaled,
                epoch_duration_seconds,
                wall_sensitivity_scaled,
                minimum_sell_haircut_onyc,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURE_PROP_AMM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.curve_peg_haircut_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.curve_exponent_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.cadence_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.cadence_wave_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.epoch_duration_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.wall_sensitivity_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.minimum_sell_haircut_onyc,
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
pub fn configure_prop_amm_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigurePropAmmKeys,
    args: ConfigurePropAmmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIGURE_PROP_AMM_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConfigurePropAmmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn configure_prop_amm_ix(
    keys: ConfigurePropAmmKeys,
    args: ConfigurePropAmmIxArgs,
) -> std::io::Result<Instruction> {
    configure_prop_amm_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn configure_prop_amm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigurePropAmmAccounts<'_, '_>,
    args: ConfigurePropAmmIxArgs,
) -> ProgramResult {
    let keys: ConfigurePropAmmKeys = accounts.into();
    let ix = configure_prop_amm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn configure_prop_amm_invoke(
    accounts: ConfigurePropAmmAccounts<'_, '_>,
    args: ConfigurePropAmmIxArgs,
) -> ProgramResult {
    configure_prop_amm_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn configure_prop_amm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigurePropAmmAccounts<'_, '_>,
    args: ConfigurePropAmmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigurePropAmmKeys = accounts.into();
    let ix = configure_prop_amm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn configure_prop_amm_invoke_signed(
    accounts: ConfigurePropAmmAccounts<'_, '_>,
    args: ConfigurePropAmmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    configure_prop_amm_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn configure_prop_amm_verify_account_keys(
    accounts: ConfigurePropAmmAccounts<'_, '_>,
    keys: ConfigurePropAmmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.offer.key, keys.offer),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.prop_amm_pair_state.key, keys.prop_amm_pair_state),
        (*accounts.boss.key, keys.boss),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn configure_prop_amm_verify_writable_privileges<'me, 'info>(
    accounts: ConfigurePropAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.prop_amm_pair_state, accounts.boss] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn configure_prop_amm_verify_signer_privileges<'me, 'info>(
    accounts: ConfigurePropAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn configure_prop_amm_verify_account_privileges<'me, 'info>(
    accounts: ConfigurePropAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    configure_prop_amm_verify_writable_privileges(accounts)?;
    configure_prop_amm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CreateRedemptionRequestAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub offer: &'me AccountInfo<'info>,
    pub redemption_request: &'me AccountInfo<'info>,
    pub redeemer: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub redeemer_token_account: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateRedemptionRequestKeys {
    pub state: Pubkey,
    pub redemption_offer: Pubkey,
    pub offer: Pubkey,
    pub redemption_request: Pubkey,
    pub redeemer: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub token_in_mint: Pubkey,
    pub redeemer_token_account: Pubkey,
    pub vault_token_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateRedemptionRequestAccounts<'_, '_>> for CreateRedemptionRequestKeys {
    fn from(accounts: CreateRedemptionRequestAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            redemption_offer: *accounts.redemption_offer.key,
            offer: *accounts.offer.key,
            redemption_request: *accounts.redemption_request.key,
            redeemer: *accounts.redeemer.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            token_in_mint: *accounts.token_in_mint.key,
            redeemer_token_account: *accounts.redeemer_token_account.key,
            vault_token_account: *accounts.vault_token_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateRedemptionRequestKeys>
for [AccountMeta; CREATE_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateRedemptionRequestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redeemer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redeemer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account,
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
impl From<[Pubkey; CREATE_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]>
for CreateRedemptionRequestKeys {
    fn from(pubkeys: [Pubkey; CREATE_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            redemption_offer: pubkeys[1],
            offer: pubkeys[2],
            redemption_request: pubkeys[3],
            redeemer: pubkeys[4],
            redemption_vault_authority: pubkeys[5],
            token_in_mint: pubkeys[6],
            redeemer_token_account: pubkeys[7],
            vault_token_account: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<CreateRedemptionRequestAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateRedemptionRequestAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.redemption_offer.clone(),
            accounts.offer.clone(),
            accounts.redemption_request.clone(),
            accounts.redeemer.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.token_in_mint.clone(),
            accounts.redeemer_token_account.clone(),
            accounts.vault_token_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]>
for CreateRedemptionRequestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            redemption_offer: &arr[1],
            offer: &arr[2],
            redemption_request: &arr[3],
            redeemer: &arr[4],
            redemption_vault_authority: &arr[5],
            token_in_mint: &arr[6],
            redeemer_token_account: &arr[7],
            vault_token_account: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const CREATE_REDEMPTION_REQUEST_IX_DISCM: [u8; 8usize] = [
    201, 53, 181, 254, 115, 137, 70, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateRedemptionRequestIxArgs {
    pub amount: u64,
    pub request_id: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateRedemptionRequestIxData(pub CreateRedemptionRequestIxArgs);
impl From<CreateRedemptionRequestIxArgs> for CreateRedemptionRequestIxData {
    fn from(args: CreateRedemptionRequestIxArgs) -> Self {
        Self(args)
    }
}
impl CreateRedemptionRequestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_REDEMPTION_REQUEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_id: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateRedemptionRequestIxArgs {
                amount,
                request_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_REDEMPTION_REQUEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.request_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_redemption_request_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateRedemptionRequestKeys,
    args: CreateRedemptionRequestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateRedemptionRequestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_redemption_request_ix(
    keys: CreateRedemptionRequestKeys,
    args: CreateRedemptionRequestIxArgs,
) -> std::io::Result<Instruction> {
    create_redemption_request_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn create_redemption_request_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateRedemptionRequestAccounts<'_, '_>,
    args: CreateRedemptionRequestIxArgs,
) -> ProgramResult {
    let keys: CreateRedemptionRequestKeys = accounts.into();
    let ix = create_redemption_request_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_redemption_request_invoke(
    accounts: CreateRedemptionRequestAccounts<'_, '_>,
    args: CreateRedemptionRequestIxArgs,
) -> ProgramResult {
    create_redemption_request_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn create_redemption_request_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateRedemptionRequestAccounts<'_, '_>,
    args: CreateRedemptionRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateRedemptionRequestKeys = accounts.into();
    let ix = create_redemption_request_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_redemption_request_invoke_signed(
    accounts: CreateRedemptionRequestAccounts<'_, '_>,
    args: CreateRedemptionRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_redemption_request_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_redemption_request_verify_account_keys(
    accounts: CreateRedemptionRequestAccounts<'_, '_>,
    keys: CreateRedemptionRequestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.offer.key, keys.offer),
        (*accounts.redemption_request.key, keys.redemption_request),
        (*accounts.redeemer.key, keys.redeemer),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.redeemer_token_account.key, keys.redeemer_token_account),
        (*accounts.vault_token_account.key, keys.vault_token_account),
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
pub fn create_redemption_request_verify_writable_privileges<'me, 'info>(
    accounts: CreateRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.redemption_offer,
        accounts.redemption_request,
        accounts.redeemer,
        accounts.redeemer_token_account,
        accounts.vault_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_redemption_request_verify_signer_privileges<'me, 'info>(
    accounts: CreateRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.redeemer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_redemption_request_verify_account_privileges<'me, 'info>(
    accounts: CreateRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_redemption_request_verify_writable_privileges(accounts)?;
    create_redemption_request_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELETE_ALL_OFFER_VECTORS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct DeleteAllOfferVectorsAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeleteAllOfferVectorsKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<DeleteAllOfferVectorsAccounts<'_, '_>> for DeleteAllOfferVectorsKeys {
    fn from(accounts: DeleteAllOfferVectorsAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<DeleteAllOfferVectorsKeys>
for [AccountMeta; DELETE_ALL_OFFER_VECTORS_IX_ACCOUNTS_LEN] {
    fn from(keys: DeleteAllOfferVectorsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DELETE_ALL_OFFER_VECTORS_IX_ACCOUNTS_LEN]>
for DeleteAllOfferVectorsKeys {
    fn from(pubkeys: [Pubkey; DELETE_ALL_OFFER_VECTORS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
            state: pubkeys[3],
            boss: pubkeys[4],
        }
    }
}
impl<'info> From<DeleteAllOfferVectorsAccounts<'_, 'info>>
for [AccountInfo<'info>; DELETE_ALL_OFFER_VECTORS_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeleteAllOfferVectorsAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DELETE_ALL_OFFER_VECTORS_IX_ACCOUNTS_LEN]>
for DeleteAllOfferVectorsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DELETE_ALL_OFFER_VECTORS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
            state: &arr[3],
            boss: &arr[4],
        }
    }
}
pub const DELETE_ALL_OFFER_VECTORS_IX_DISCM: [u8; 8usize] = [
    26, 201, 38, 207, 76, 51, 79, 15,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteAllOfferVectorsIxData;
impl DeleteAllOfferVectorsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_ALL_OFFER_VECTORS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_ALL_OFFER_VECTORS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_all_offer_vectors_ix_with_program_id(
    program_id: Pubkey,
    keys: DeleteAllOfferVectorsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELETE_ALL_OFFER_VECTORS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DeleteAllOfferVectorsIxData.try_to_vec()?,
    })
}
pub fn delete_all_offer_vectors_ix(
    keys: DeleteAllOfferVectorsKeys,
) -> std::io::Result<Instruction> {
    delete_all_offer_vectors_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn delete_all_offer_vectors_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeleteAllOfferVectorsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeleteAllOfferVectorsKeys = accounts.into();
    let ix = delete_all_offer_vectors_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn delete_all_offer_vectors_invoke(
    accounts: DeleteAllOfferVectorsAccounts<'_, '_>,
) -> ProgramResult {
    delete_all_offer_vectors_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn delete_all_offer_vectors_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeleteAllOfferVectorsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeleteAllOfferVectorsKeys = accounts.into();
    let ix = delete_all_offer_vectors_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delete_all_offer_vectors_invoke_signed(
    accounts: DeleteAllOfferVectorsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delete_all_offer_vectors_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn delete_all_offer_vectors_verify_account_keys(
    accounts: DeleteAllOfferVectorsAccounts<'_, '_>,
    keys: DeleteAllOfferVectorsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn delete_all_offer_vectors_verify_writable_privileges<'me, 'info>(
    accounts: DeleteAllOfferVectorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delete_all_offer_vectors_verify_signer_privileges<'me, 'info>(
    accounts: DeleteAllOfferVectorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delete_all_offer_vectors_verify_account_privileges<'me, 'info>(
    accounts: DeleteAllOfferVectorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delete_all_offer_vectors_verify_writable_privileges(accounts)?;
    delete_all_offer_vectors_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELETE_OFFER_VECTOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct DeleteOfferVectorAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeleteOfferVectorKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<DeleteOfferVectorAccounts<'_, '_>> for DeleteOfferVectorKeys {
    fn from(accounts: DeleteOfferVectorAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<DeleteOfferVectorKeys> for [AccountMeta; DELETE_OFFER_VECTOR_IX_ACCOUNTS_LEN] {
    fn from(keys: DeleteOfferVectorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DELETE_OFFER_VECTOR_IX_ACCOUNTS_LEN]> for DeleteOfferVectorKeys {
    fn from(pubkeys: [Pubkey; DELETE_OFFER_VECTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
            state: pubkeys[3],
            boss: pubkeys[4],
        }
    }
}
impl<'info> From<DeleteOfferVectorAccounts<'_, 'info>>
for [AccountInfo<'info>; DELETE_OFFER_VECTOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeleteOfferVectorAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELETE_OFFER_VECTOR_IX_ACCOUNTS_LEN]>
for DeleteOfferVectorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DELETE_OFFER_VECTOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
            state: &arr[3],
            boss: &arr[4],
        }
    }
}
pub const DELETE_OFFER_VECTOR_IX_DISCM: [u8; 8usize] = [
    87, 40, 79, 151, 78, 121, 46, 159,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteOfferVectorIxArgs {
    pub vector_start_time: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteOfferVectorIxData(pub DeleteOfferVectorIxArgs);
impl From<DeleteOfferVectorIxArgs> for DeleteOfferVectorIxData {
    fn from(args: DeleteOfferVectorIxArgs) -> Self {
        Self(args)
    }
}
impl DeleteOfferVectorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_OFFER_VECTOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let vector_start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DeleteOfferVectorIxArgs {
                vector_start_time,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_OFFER_VECTOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.vector_start_time, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_offer_vector_ix_with_program_id(
    program_id: Pubkey,
    keys: DeleteOfferVectorKeys,
    args: DeleteOfferVectorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELETE_OFFER_VECTOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: DeleteOfferVectorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn delete_offer_vector_ix(
    keys: DeleteOfferVectorKeys,
    args: DeleteOfferVectorIxArgs,
) -> std::io::Result<Instruction> {
    delete_offer_vector_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn delete_offer_vector_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeleteOfferVectorAccounts<'_, '_>,
    args: DeleteOfferVectorIxArgs,
) -> ProgramResult {
    let keys: DeleteOfferVectorKeys = accounts.into();
    let ix = delete_offer_vector_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn delete_offer_vector_invoke(
    accounts: DeleteOfferVectorAccounts<'_, '_>,
    args: DeleteOfferVectorIxArgs,
) -> ProgramResult {
    delete_offer_vector_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn delete_offer_vector_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeleteOfferVectorAccounts<'_, '_>,
    args: DeleteOfferVectorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeleteOfferVectorKeys = accounts.into();
    let ix = delete_offer_vector_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delete_offer_vector_invoke_signed(
    accounts: DeleteOfferVectorAccounts<'_, '_>,
    args: DeleteOfferVectorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delete_offer_vector_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn delete_offer_vector_verify_account_keys(
    accounts: DeleteOfferVectorAccounts<'_, '_>,
    keys: DeleteOfferVectorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn delete_offer_vector_verify_writable_privileges<'me, 'info>(
    accounts: DeleteOfferVectorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delete_offer_vector_verify_signer_privileges<'me, 'info>(
    accounts: DeleteOfferVectorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delete_offer_vector_verify_account_privileges<'me, 'info>(
    accounts: DeleteOfferVectorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delete_offer_vector_verify_writable_privileges(accounts)?;
    delete_offer_vector_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_RESERVE_VAULT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositReserveVaultAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub buffer_state: &'me AccountInfo<'info>,
    pub reserve_vault_authority: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub depositor_onyc_account: &'me AccountInfo<'info>,
    pub reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub depositor: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositReserveVaultKeys {
    pub state: Pubkey,
    pub buffer_state: Pubkey,
    pub reserve_vault_authority: Pubkey,
    pub onyc_mint: Pubkey,
    pub depositor_onyc_account: Pubkey,
    pub reserve_vault_onyc_account: Pubkey,
    pub depositor: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<DepositReserveVaultAccounts<'_, '_>> for DepositReserveVaultKeys {
    fn from(accounts: DepositReserveVaultAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            buffer_state: *accounts.buffer_state.key,
            reserve_vault_authority: *accounts.reserve_vault_authority.key,
            onyc_mint: *accounts.onyc_mint.key,
            depositor_onyc_account: *accounts.depositor_onyc_account.key,
            reserve_vault_onyc_account: *accounts.reserve_vault_onyc_account.key,
            depositor: *accounts.depositor.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DepositReserveVaultKeys>
for [AccountMeta; DEPOSIT_RESERVE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositReserveVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.depositor_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: true,
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
impl From<[Pubkey; DEPOSIT_RESERVE_VAULT_IX_ACCOUNTS_LEN]> for DepositReserveVaultKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_RESERVE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            buffer_state: pubkeys[1],
            reserve_vault_authority: pubkeys[2],
            onyc_mint: pubkeys[3],
            depositor_onyc_account: pubkeys[4],
            reserve_vault_onyc_account: pubkeys[5],
            depositor: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<DepositReserveVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_RESERVE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositReserveVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.buffer_state.clone(),
            accounts.reserve_vault_authority.clone(),
            accounts.onyc_mint.clone(),
            accounts.depositor_onyc_account.clone(),
            accounts.reserve_vault_onyc_account.clone(),
            accounts.depositor.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_RESERVE_VAULT_IX_ACCOUNTS_LEN]>
for DepositReserveVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_RESERVE_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            buffer_state: &arr[1],
            reserve_vault_authority: &arr[2],
            onyc_mint: &arr[3],
            depositor_onyc_account: &arr[4],
            reserve_vault_onyc_account: &arr[5],
            depositor: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const DEPOSIT_RESERVE_VAULT_IX_DISCM: [u8; 8usize] = [
    159, 91, 174, 234, 207, 12, 167, 9,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositReserveVaultIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositReserveVaultIxData(pub DepositReserveVaultIxArgs);
impl From<DepositReserveVaultIxArgs> for DepositReserveVaultIxData {
    fn from(args: DepositReserveVaultIxArgs) -> Self {
        Self(args)
    }
}
impl DepositReserveVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_RESERVE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositReserveVaultIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_RESERVE_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_reserve_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositReserveVaultKeys,
    args: DepositReserveVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_RESERVE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositReserveVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_reserve_vault_ix(
    keys: DepositReserveVaultKeys,
    args: DepositReserveVaultIxArgs,
) -> std::io::Result<Instruction> {
    deposit_reserve_vault_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn deposit_reserve_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositReserveVaultAccounts<'_, '_>,
    args: DepositReserveVaultIxArgs,
) -> ProgramResult {
    let keys: DepositReserveVaultKeys = accounts.into();
    let ix = deposit_reserve_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_reserve_vault_invoke(
    accounts: DepositReserveVaultAccounts<'_, '_>,
    args: DepositReserveVaultIxArgs,
) -> ProgramResult {
    deposit_reserve_vault_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn deposit_reserve_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositReserveVaultAccounts<'_, '_>,
    args: DepositReserveVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositReserveVaultKeys = accounts.into();
    let ix = deposit_reserve_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_reserve_vault_invoke_signed(
    accounts: DepositReserveVaultAccounts<'_, '_>,
    args: DepositReserveVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_reserve_vault_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_reserve_vault_verify_account_keys(
    accounts: DepositReserveVaultAccounts<'_, '_>,
    keys: DepositReserveVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.buffer_state.key, keys.buffer_state),
        (*accounts.reserve_vault_authority.key, keys.reserve_vault_authority),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.depositor_onyc_account.key, keys.depositor_onyc_account),
        (*accounts.reserve_vault_onyc_account.key, keys.reserve_vault_onyc_account),
        (*accounts.depositor.key, keys.depositor),
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
pub fn deposit_reserve_vault_verify_writable_privileges<'me, 'info>(
    accounts: DepositReserveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.depositor_onyc_account,
        accounts.reserve_vault_onyc_account,
        accounts.depositor,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_reserve_vault_verify_signer_privileges<'me, 'info>(
    accounts: DepositReserveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.depositor] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_reserve_vault_verify_account_privileges<'me, 'info>(
    accounts: DepositReserveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_reserve_vault_verify_writable_privileges(accounts)?;
    deposit_reserve_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FULFILL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN: usize = 30;
#[derive(Copy, Clone, Debug)]
pub struct FulfillRedemptionRequestAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub offer: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub redemption_request: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub vault_token_in_account: &'me AccountInfo<'info>,
    pub vault_token_out_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub offer_proceeds_vault: &'me AccountInfo<'info>,
    pub offer_proceeds_token_in_account: &'me AccountInfo<'info>,
    pub redemption_fee_vault: &'me AccountInfo<'info>,
    pub redemption_fee_token_in_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub redeemer: &'me AccountInfo<'info>,
    pub worker: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub offer_vault_authority: &'me AccountInfo<'info>,
    pub offer_vault_onyc_account: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FulfillRedemptionRequestKeys {
    pub state: Pubkey,
    pub offer: Pubkey,
    pub redemption_offer: Pubkey,
    pub redemption_request: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub vault_token_in_account: Pubkey,
    pub vault_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub user_token_out_account: Pubkey,
    pub offer_proceeds_vault: Pubkey,
    pub offer_proceeds_token_in_account: Pubkey,
    pub redemption_fee_vault: Pubkey,
    pub redemption_fee_token_in_account: Pubkey,
    pub mint_authority: Pubkey,
    pub redeemer: Pubkey,
    pub worker: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub offer_vault_authority: Pubkey,
    pub offer_vault_onyc_account: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
    pub main_offer: Pubkey,
}
impl From<FulfillRedemptionRequestAccounts<'_, '_>> for FulfillRedemptionRequestKeys {
    fn from(accounts: FulfillRedemptionRequestAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            offer: *accounts.offer.key,
            redemption_offer: *accounts.redemption_offer.key,
            redemption_request: *accounts.redemption_request.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            vault_token_in_account: *accounts.vault_token_in_account.key,
            vault_token_out_account: *accounts.vault_token_out_account.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            offer_proceeds_vault: *accounts.offer_proceeds_vault.key,
            offer_proceeds_token_in_account: *accounts
                .offer_proceeds_token_in_account
                .key,
            redemption_fee_vault: *accounts.redemption_fee_vault.key,
            redemption_fee_token_in_account: *accounts
                .redemption_fee_token_in_account
                .key,
            mint_authority: *accounts.mint_authority.key,
            redeemer: *accounts.redeemer.key,
            worker: *accounts.worker.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            offer_vault_authority: *accounts.offer_vault_authority.key,
            offer_vault_onyc_account: *accounts.offer_vault_onyc_account.key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
            main_offer: *accounts.main_offer.key,
        }
    }
}
impl From<FulfillRedemptionRequestKeys>
for [AccountMeta; FULFILL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(keys: FulfillRedemptionRequestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_proceeds_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_proceeds_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_fee_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redeemer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.worker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.offer_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer_vault_onyc_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FULFILL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]>
for FulfillRedemptionRequestKeys {
    fn from(pubkeys: [Pubkey; FULFILL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            offer: pubkeys[1],
            redemption_offer: pubkeys[2],
            redemption_request: pubkeys[3],
            redemption_vault_authority: pubkeys[4],
            vault_token_in_account: pubkeys[5],
            vault_token_out_account: pubkeys[6],
            token_in_mint: pubkeys[7],
            token_in_program: pubkeys[8],
            token_out_mint: pubkeys[9],
            token_out_program: pubkeys[10],
            user_token_out_account: pubkeys[11],
            offer_proceeds_vault: pubkeys[12],
            offer_proceeds_token_in_account: pubkeys[13],
            redemption_fee_vault: pubkeys[14],
            redemption_fee_token_in_account: pubkeys[15],
            mint_authority: pubkeys[16],
            redeemer: pubkeys[17],
            worker: pubkeys[18],
            buffer_accounts_buffer_state: pubkeys[19],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[20],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[21],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[22],
            associated_token_program: pubkeys[23],
            system_program: pubkeys[24],
            offer_vault_authority: pubkeys[25],
            offer_vault_onyc_account: pubkeys[26],
            market_stats: pubkeys[27],
            circulating_supply_excluded_balance: pubkeys[28],
            main_offer: pubkeys[29],
        }
    }
}
impl<'info> From<FulfillRedemptionRequestAccounts<'_, 'info>>
for [AccountInfo<'info>; FULFILL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: FulfillRedemptionRequestAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.offer.clone(),
            accounts.redemption_offer.clone(),
            accounts.redemption_request.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.vault_token_in_account.clone(),
            accounts.vault_token_out_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.user_token_out_account.clone(),
            accounts.offer_proceeds_vault.clone(),
            accounts.offer_proceeds_token_in_account.clone(),
            accounts.redemption_fee_vault.clone(),
            accounts.redemption_fee_token_in_account.clone(),
            accounts.mint_authority.clone(),
            accounts.redeemer.clone(),
            accounts.worker.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.offer_vault_authority.clone(),
            accounts.offer_vault_onyc_account.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
            accounts.main_offer.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FULFILL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN]>
for FulfillRedemptionRequestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FULFILL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            offer: &arr[1],
            redemption_offer: &arr[2],
            redemption_request: &arr[3],
            redemption_vault_authority: &arr[4],
            vault_token_in_account: &arr[5],
            vault_token_out_account: &arr[6],
            token_in_mint: &arr[7],
            token_in_program: &arr[8],
            token_out_mint: &arr[9],
            token_out_program: &arr[10],
            user_token_out_account: &arr[11],
            offer_proceeds_vault: &arr[12],
            offer_proceeds_token_in_account: &arr[13],
            redemption_fee_vault: &arr[14],
            redemption_fee_token_in_account: &arr[15],
            mint_authority: &arr[16],
            redeemer: &arr[17],
            worker: &arr[18],
            buffer_accounts_buffer_state: &arr[19],
            buffer_accounts_reserve_vault_onyc_account: &arr[20],
            buffer_accounts_management_fee_vault_onyc_account: &arr[21],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[22],
            associated_token_program: &arr[23],
            system_program: &arr[24],
            offer_vault_authority: &arr[25],
            offer_vault_onyc_account: &arr[26],
            market_stats: &arr[27],
            circulating_supply_excluded_balance: &arr[28],
            main_offer: &arr[29],
        }
    }
}
pub const FULFILL_REDEMPTION_REQUEST_IX_DISCM: [u8; 8usize] = [
    140, 124, 139, 242, 179, 153, 208, 66,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FulfillRedemptionRequestIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FulfillRedemptionRequestIxData(pub FulfillRedemptionRequestIxArgs);
impl From<FulfillRedemptionRequestIxArgs> for FulfillRedemptionRequestIxData {
    fn from(args: FulfillRedemptionRequestIxArgs) -> Self {
        Self(args)
    }
}
impl FulfillRedemptionRequestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FULFILL_REDEMPTION_REQUEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FulfillRedemptionRequestIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FULFILL_REDEMPTION_REQUEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fulfill_redemption_request_ix_with_program_id(
    program_id: Pubkey,
    keys: FulfillRedemptionRequestKeys,
    args: FulfillRedemptionRequestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FULFILL_REDEMPTION_REQUEST_IX_ACCOUNTS_LEN] = keys.into();
    let data: FulfillRedemptionRequestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fulfill_redemption_request_ix(
    keys: FulfillRedemptionRequestKeys,
    args: FulfillRedemptionRequestIxArgs,
) -> std::io::Result<Instruction> {
    fulfill_redemption_request_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn fulfill_redemption_request_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FulfillRedemptionRequestAccounts<'_, '_>,
    args: FulfillRedemptionRequestIxArgs,
) -> ProgramResult {
    let keys: FulfillRedemptionRequestKeys = accounts.into();
    let ix = fulfill_redemption_request_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fulfill_redemption_request_invoke(
    accounts: FulfillRedemptionRequestAccounts<'_, '_>,
    args: FulfillRedemptionRequestIxArgs,
) -> ProgramResult {
    fulfill_redemption_request_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn fulfill_redemption_request_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FulfillRedemptionRequestAccounts<'_, '_>,
    args: FulfillRedemptionRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FulfillRedemptionRequestKeys = accounts.into();
    let ix = fulfill_redemption_request_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fulfill_redemption_request_invoke_signed(
    accounts: FulfillRedemptionRequestAccounts<'_, '_>,
    args: FulfillRedemptionRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fulfill_redemption_request_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn fulfill_redemption_request_verify_account_keys(
    accounts: FulfillRedemptionRequestAccounts<'_, '_>,
    keys: FulfillRedemptionRequestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.offer.key, keys.offer),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.redemption_request.key, keys.redemption_request),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (*accounts.vault_token_in_account.key, keys.vault_token_in_account),
        (*accounts.vault_token_out_account.key, keys.vault_token_out_account),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.user_token_out_account.key, keys.user_token_out_account),
        (*accounts.offer_proceeds_vault.key, keys.offer_proceeds_vault),
        (
            *accounts.offer_proceeds_token_in_account.key,
            keys.offer_proceeds_token_in_account,
        ),
        (*accounts.redemption_fee_vault.key, keys.redemption_fee_vault),
        (
            *accounts.redemption_fee_token_in_account.key,
            keys.redemption_fee_token_in_account,
        ),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.redeemer.key, keys.redeemer),
        (*accounts.worker.key, keys.worker),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.offer_vault_authority.key, keys.offer_vault_authority),
        (*accounts.offer_vault_onyc_account.key, keys.offer_vault_onyc_account),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
        (*accounts.main_offer.key, keys.main_offer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fulfill_redemption_request_verify_writable_privileges<'me, 'info>(
    accounts: FulfillRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.redemption_offer,
        accounts.redemption_request,
        accounts.vault_token_in_account,
        accounts.vault_token_out_account,
        accounts.token_in_mint,
        accounts.token_out_mint,
        accounts.user_token_out_account,
        accounts.offer_proceeds_vault,
        accounts.offer_proceeds_token_in_account,
        accounts.redemption_fee_vault,
        accounts.redemption_fee_token_in_account,
        accounts.worker,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fulfill_redemption_request_verify_signer_privileges<'me, 'info>(
    accounts: FulfillRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.worker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fulfill_redemption_request_verify_account_privileges<'me, 'info>(
    accounts: FulfillRedemptionRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fulfill_redemption_request_verify_writable_privileges(accounts)?;
    fulfill_redemption_request_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GET_APY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct GetApyAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetApyKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
}
impl From<GetApyAccounts<'_, '_>> for GetApyKeys {
    fn from(accounts: GetApyAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
        }
    }
}
impl From<GetApyKeys> for [AccountMeta; GET_APY_IX_ACCOUNTS_LEN] {
    fn from(keys: GetApyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_APY_IX_ACCOUNTS_LEN]> for GetApyKeys {
    fn from(pubkeys: [Pubkey; GET_APY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
        }
    }
}
impl<'info> From<GetApyAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_APY_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetApyAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_APY_IX_ACCOUNTS_LEN]>
for GetApyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_APY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
        }
    }
}
pub const GET_APY_IX_DISCM: [u8; 8usize] = [194, 123, 183, 54, 181, 74, 194, 97];
#[derive(Clone, Debug, PartialEq)]
pub struct GetApyIxData;
impl GetApyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_APY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_APY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_apy_ix_with_program_id(
    program_id: Pubkey,
    keys: GetApyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_APY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetApyIxData.try_to_vec()?,
    })
}
pub fn get_apy_ix(keys: GetApyKeys) -> std::io::Result<Instruction> {
    get_apy_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn get_apy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetApyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetApyKeys = accounts.into();
    let ix = get_apy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_apy_invoke(accounts: GetApyAccounts<'_, '_>) -> ProgramResult {
    get_apy_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn get_apy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetApyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetApyKeys = accounts.into();
    let ix = get_apy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_apy_invoke_signed(
    accounts: GetApyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_apy_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn get_apy_verify_account_keys(
    accounts: GetApyAccounts<'_, '_>,
    keys: GetApyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_CIRCULATING_SUPPLY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct GetCirculatingSupplyAccounts<'me, 'info> {
    pub onyc_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub onyc_vault_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetCirculatingSupplyKeys {
    pub onyc_mint: Pubkey,
    pub state: Pubkey,
    pub vault_authority: Pubkey,
    pub onyc_vault_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<GetCirculatingSupplyAccounts<'_, '_>> for GetCirculatingSupplyKeys {
    fn from(accounts: GetCirculatingSupplyAccounts) -> Self {
        Self {
            onyc_mint: *accounts.onyc_mint.key,
            state: *accounts.state.key,
            vault_authority: *accounts.vault_authority.key,
            onyc_vault_account: *accounts.onyc_vault_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<GetCirculatingSupplyKeys>
for [AccountMeta; GET_CIRCULATING_SUPPLY_IX_ACCOUNTS_LEN] {
    fn from(keys: GetCirculatingSupplyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_vault_account,
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
impl From<[Pubkey; GET_CIRCULATING_SUPPLY_IX_ACCOUNTS_LEN]>
for GetCirculatingSupplyKeys {
    fn from(pubkeys: [Pubkey; GET_CIRCULATING_SUPPLY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            onyc_mint: pubkeys[0],
            state: pubkeys[1],
            vault_authority: pubkeys[2],
            onyc_vault_account: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<GetCirculatingSupplyAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_CIRCULATING_SUPPLY_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetCirculatingSupplyAccounts<'_, 'info>) -> Self {
        [
            accounts.onyc_mint.clone(),
            accounts.state.clone(),
            accounts.vault_authority.clone(),
            accounts.onyc_vault_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_CIRCULATING_SUPPLY_IX_ACCOUNTS_LEN]>
for GetCirculatingSupplyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GET_CIRCULATING_SUPPLY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            onyc_mint: &arr[0],
            state: &arr[1],
            vault_authority: &arr[2],
            onyc_vault_account: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const GET_CIRCULATING_SUPPLY_IX_DISCM: [u8; 8usize] = [
    132, 168, 96, 104, 217, 255, 111, 152,
];
#[derive(Clone, Debug, PartialEq)]
pub struct GetCirculatingSupplyIxData;
impl GetCirculatingSupplyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_CIRCULATING_SUPPLY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_CIRCULATING_SUPPLY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_circulating_supply_ix_with_program_id(
    program_id: Pubkey,
    keys: GetCirculatingSupplyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_CIRCULATING_SUPPLY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetCirculatingSupplyIxData.try_to_vec()?,
    })
}
pub fn get_circulating_supply_ix(
    keys: GetCirculatingSupplyKeys,
) -> std::io::Result<Instruction> {
    get_circulating_supply_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn get_circulating_supply_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetCirculatingSupplyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetCirculatingSupplyKeys = accounts.into();
    let ix = get_circulating_supply_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_circulating_supply_invoke(
    accounts: GetCirculatingSupplyAccounts<'_, '_>,
) -> ProgramResult {
    get_circulating_supply_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn get_circulating_supply_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetCirculatingSupplyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetCirculatingSupplyKeys = accounts.into();
    let ix = get_circulating_supply_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_circulating_supply_invoke_signed(
    accounts: GetCirculatingSupplyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_circulating_supply_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn get_circulating_supply_verify_account_keys(
    accounts: GetCirculatingSupplyAccounts<'_, '_>,
    keys: GetCirculatingSupplyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.state.key, keys.state),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.onyc_vault_account.key, keys.onyc_vault_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_CIRCULATING_SUPPLY_V2_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct GetCirculatingSupplyV2Accounts<'me, 'info> {
    pub onyc_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetCirculatingSupplyV2Keys {
    pub onyc_mint: Pubkey,
    pub state: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
}
impl From<GetCirculatingSupplyV2Accounts<'_, '_>> for GetCirculatingSupplyV2Keys {
    fn from(accounts: GetCirculatingSupplyV2Accounts) -> Self {
        Self {
            onyc_mint: *accounts.onyc_mint.key,
            state: *accounts.state.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
        }
    }
}
impl From<GetCirculatingSupplyV2Keys>
for [AccountMeta; GET_CIRCULATING_SUPPLY_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: GetCirculatingSupplyV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_CIRCULATING_SUPPLY_V2_IX_ACCOUNTS_LEN]>
for GetCirculatingSupplyV2Keys {
    fn from(pubkeys: [Pubkey; GET_CIRCULATING_SUPPLY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            onyc_mint: pubkeys[0],
            state: pubkeys[1],
            circulating_supply_excluded_balance: pubkeys[2],
        }
    }
}
impl<'info> From<GetCirculatingSupplyV2Accounts<'_, 'info>>
for [AccountInfo<'info>; GET_CIRCULATING_SUPPLY_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetCirculatingSupplyV2Accounts<'_, 'info>) -> Self {
        [
            accounts.onyc_mint.clone(),
            accounts.state.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GET_CIRCULATING_SUPPLY_V2_IX_ACCOUNTS_LEN]>
for GetCirculatingSupplyV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GET_CIRCULATING_SUPPLY_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            onyc_mint: &arr[0],
            state: &arr[1],
            circulating_supply_excluded_balance: &arr[2],
        }
    }
}
pub const GET_CIRCULATING_SUPPLY_V2_IX_DISCM: [u8; 8usize] = [
    57, 115, 7, 115, 9, 100, 135, 111,
];
#[derive(Clone, Debug, PartialEq)]
pub struct GetCirculatingSupplyV2IxData;
impl GetCirculatingSupplyV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_CIRCULATING_SUPPLY_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_CIRCULATING_SUPPLY_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_circulating_supply_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: GetCirculatingSupplyV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_CIRCULATING_SUPPLY_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetCirculatingSupplyV2IxData.try_to_vec()?,
    })
}
pub fn get_circulating_supply_v2_ix(
    keys: GetCirculatingSupplyV2Keys,
) -> std::io::Result<Instruction> {
    get_circulating_supply_v2_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn get_circulating_supply_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetCirculatingSupplyV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: GetCirculatingSupplyV2Keys = accounts.into();
    let ix = get_circulating_supply_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_circulating_supply_v2_invoke(
    accounts: GetCirculatingSupplyV2Accounts<'_, '_>,
) -> ProgramResult {
    get_circulating_supply_v2_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn get_circulating_supply_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetCirculatingSupplyV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetCirculatingSupplyV2Keys = accounts.into();
    let ix = get_circulating_supply_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_circulating_supply_v2_invoke_signed(
    accounts: GetCirculatingSupplyV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_circulating_supply_v2_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn get_circulating_supply_v2_verify_account_keys(
    accounts: GetCirculatingSupplyV2Accounts<'_, '_>,
    keys: GetCirculatingSupplyV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.state.key, keys.state),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_NAV_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct GetNavAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetNavKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
}
impl From<GetNavAccounts<'_, '_>> for GetNavKeys {
    fn from(accounts: GetNavAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
        }
    }
}
impl From<GetNavKeys> for [AccountMeta; GET_NAV_IX_ACCOUNTS_LEN] {
    fn from(keys: GetNavKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_NAV_IX_ACCOUNTS_LEN]> for GetNavKeys {
    fn from(pubkeys: [Pubkey; GET_NAV_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
        }
    }
}
impl<'info> From<GetNavAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_NAV_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetNavAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_NAV_IX_ACCOUNTS_LEN]>
for GetNavAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_NAV_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
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
    get_nav_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
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
    get_nav_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
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
    get_nav_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn get_nav_verify_account_keys(
    accounts: GetNavAccounts<'_, '_>,
    keys: GetNavKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_NAV_ADJUSTMENT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct GetNavAdjustmentAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetNavAdjustmentKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
}
impl From<GetNavAdjustmentAccounts<'_, '_>> for GetNavAdjustmentKeys {
    fn from(accounts: GetNavAdjustmentAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
        }
    }
}
impl From<GetNavAdjustmentKeys> for [AccountMeta; GET_NAV_ADJUSTMENT_IX_ACCOUNTS_LEN] {
    fn from(keys: GetNavAdjustmentKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_NAV_ADJUSTMENT_IX_ACCOUNTS_LEN]> for GetNavAdjustmentKeys {
    fn from(pubkeys: [Pubkey; GET_NAV_ADJUSTMENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
        }
    }
}
impl<'info> From<GetNavAdjustmentAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_NAV_ADJUSTMENT_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetNavAdjustmentAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_NAV_ADJUSTMENT_IX_ACCOUNTS_LEN]>
for GetNavAdjustmentAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_NAV_ADJUSTMENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
        }
    }
}
pub const GET_NAV_ADJUSTMENT_IX_DISCM: [u8; 8usize] = [
    70, 198, 229, 129, 238, 233, 143, 94,
];
#[derive(Clone, Debug, PartialEq)]
pub struct GetNavAdjustmentIxData;
impl GetNavAdjustmentIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_NAV_ADJUSTMENT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_NAV_ADJUSTMENT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_nav_adjustment_ix_with_program_id(
    program_id: Pubkey,
    keys: GetNavAdjustmentKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_NAV_ADJUSTMENT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetNavAdjustmentIxData.try_to_vec()?,
    })
}
pub fn get_nav_adjustment_ix(
    keys: GetNavAdjustmentKeys,
) -> std::io::Result<Instruction> {
    get_nav_adjustment_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn get_nav_adjustment_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetNavAdjustmentAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetNavAdjustmentKeys = accounts.into();
    let ix = get_nav_adjustment_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_nav_adjustment_invoke(
    accounts: GetNavAdjustmentAccounts<'_, '_>,
) -> ProgramResult {
    get_nav_adjustment_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn get_nav_adjustment_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetNavAdjustmentAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetNavAdjustmentKeys = accounts.into();
    let ix = get_nav_adjustment_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_nav_adjustment_invoke_signed(
    accounts: GetNavAdjustmentAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_nav_adjustment_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn get_nav_adjustment_verify_account_keys(
    accounts: GetNavAdjustmentAccounts<'_, '_>,
    keys: GetNavAdjustmentKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_TVL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GetTvlAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_token_out_account: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetTvlKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_token_out_account: Pubkey,
    pub token_out_program: Pubkey,
}
impl From<GetTvlAccounts<'_, '_>> for GetTvlKeys {
    fn from(accounts: GetTvlAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            vault_authority: *accounts.vault_authority.key,
            vault_token_out_account: *accounts.vault_token_out_account.key,
            token_out_program: *accounts.token_out_program.key,
        }
    }
}
impl From<GetTvlKeys> for [AccountMeta; GET_TVL_IX_ACCOUNTS_LEN] {
    fn from(keys: GetTvlKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_out_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_TVL_IX_ACCOUNTS_LEN]> for GetTvlKeys {
    fn from(pubkeys: [Pubkey; GET_TVL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
            vault_authority: pubkeys[3],
            vault_token_out_account: pubkeys[4],
            token_out_program: pubkeys[5],
        }
    }
}
impl<'info> From<GetTvlAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_TVL_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetTvlAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_token_out_account.clone(),
            accounts.token_out_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_TVL_IX_ACCOUNTS_LEN]>
for GetTvlAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_TVL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
            vault_authority: &arr[3],
            vault_token_out_account: &arr[4],
            token_out_program: &arr[5],
        }
    }
}
pub const GET_TVL_IX_DISCM: [u8; 8usize] = [88, 225, 219, 204, 86, 91, 184, 51];
#[derive(Clone, Debug, PartialEq)]
pub struct GetTvlIxData;
impl GetTvlIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_TVL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_TVL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_tvl_ix_with_program_id(
    program_id: Pubkey,
    keys: GetTvlKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_TVL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetTvlIxData.try_to_vec()?,
    })
}
pub fn get_tvl_ix(keys: GetTvlKeys) -> std::io::Result<Instruction> {
    get_tvl_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn get_tvl_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetTvlAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetTvlKeys = accounts.into();
    let ix = get_tvl_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_tvl_invoke(accounts: GetTvlAccounts<'_, '_>) -> ProgramResult {
    get_tvl_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn get_tvl_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetTvlAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetTvlKeys = accounts.into();
    let ix = get_tvl_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_tvl_invoke_signed(
    accounts: GetTvlAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_tvl_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn get_tvl_verify_account_keys(
    accounts: GetTvlAccounts<'_, '_>,
    keys: GetTvlKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_token_out_account.key, keys.vault_token_out_account),
        (*accounts.token_out_program.key, keys.token_out_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_TVL_V2_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct GetTvlV2Accounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetTvlV2Keys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub state: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
}
impl From<GetTvlV2Accounts<'_, '_>> for GetTvlV2Keys {
    fn from(accounts: GetTvlV2Accounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            state: *accounts.state.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
        }
    }
}
impl From<GetTvlV2Keys> for [AccountMeta; GET_TVL_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: GetTvlV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_TVL_V2_IX_ACCOUNTS_LEN]> for GetTvlV2Keys {
    fn from(pubkeys: [Pubkey; GET_TVL_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
            state: pubkeys[3],
            circulating_supply_excluded_balance: pubkeys[4],
        }
    }
}
impl<'info> From<GetTvlV2Accounts<'_, 'info>>
for [AccountInfo<'info>; GET_TVL_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetTvlV2Accounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.state.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_TVL_V2_IX_ACCOUNTS_LEN]>
for GetTvlV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_TVL_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
            state: &arr[3],
            circulating_supply_excluded_balance: &arr[4],
        }
    }
}
pub const GET_TVL_V2_IX_DISCM: [u8; 8usize] = [64, 43, 63, 112, 186, 8, 1, 165];
#[derive(Clone, Debug, PartialEq)]
pub struct GetTvlV2IxData;
impl GetTvlV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_TVL_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_TVL_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_tvl_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: GetTvlV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_TVL_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetTvlV2IxData.try_to_vec()?,
    })
}
pub fn get_tvl_v2_ix(keys: GetTvlV2Keys) -> std::io::Result<Instruction> {
    get_tvl_v2_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn get_tvl_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetTvlV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: GetTvlV2Keys = accounts.into();
    let ix = get_tvl_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_tvl_v2_invoke(accounts: GetTvlV2Accounts<'_, '_>) -> ProgramResult {
    get_tvl_v2_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn get_tvl_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetTvlV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetTvlV2Keys = accounts.into();
    let ix = get_tvl_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_tvl_v2_invoke_signed(
    accounts: GetTvlV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_tvl_v2_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn get_tvl_v2_verify_account_keys(
    accounts: GetTvlV2Accounts<'_, '_>,
    keys: GetTvlV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.state.key, keys.state),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub offer_vault_authority: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub state: Pubkey,
    pub mint_authority: Pubkey,
    pub offer_vault_authority: Pubkey,
    pub boss: Pubkey,
    pub program: Pubkey,
    pub program_data: Pubkey,
    pub onyc_mint: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            mint_authority: *accounts.mint_authority.key,
            offer_vault_authority: *accounts.offer_vault_authority.key,
            boss: *accounts.boss.key,
            program: *accounts.program.key,
            program_data: *accounts.program_data.key,
            onyc_mint: *accounts.onyc_mint.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_data,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            mint_authority: pubkeys[1],
            offer_vault_authority: pubkeys[2],
            boss: pubkeys[3],
            program: pubkeys[4],
            program_data: pubkeys[5],
            onyc_mint: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.mint_authority.clone(),
            accounts.offer_vault_authority.clone(),
            accounts.boss.clone(),
            accounts.program.clone(),
            accounts.program_data.clone(),
            accounts.onyc_mint.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            mint_authority: &arr[1],
            offer_vault_authority: &arr[2],
            boss: &arr[3],
            program: &arr[4],
            program_data: &arr[5],
            onyc_mint: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData;
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeIxData.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(accounts: InitializeAccounts<'_, '_>) -> ProgramResult {
    initialize_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.offer_vault_authority.key, keys.offer_vault_authority),
        (*accounts.boss.key, keys.boss),
        (*accounts.program.key, keys.program),
        (*accounts.program_data.key, keys.program_data),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.system_program.key, keys.system_program),
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
    for should_be_writable in [
        accounts.state,
        accounts.mint_authority,
        accounts.offer_vault_authority,
        accounts.boss,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
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
pub const INITIALIZE_BUFFER_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InitializeBufferAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub buffer_state: &'me AccountInfo<'info>,
    pub reserve_vault_authority: &'me AccountInfo<'info>,
    pub management_fee_vault: &'me AccountInfo<'info>,
    pub performance_fee_vault: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub offer: &'me AccountInfo<'info>,
    pub reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeBufferKeys {
    pub state: Pubkey,
    pub buffer_state: Pubkey,
    pub reserve_vault_authority: Pubkey,
    pub management_fee_vault: Pubkey,
    pub performance_fee_vault: Pubkey,
    pub boss: Pubkey,
    pub onyc_mint: Pubkey,
    pub offer: Pubkey,
    pub reserve_vault_onyc_account: Pubkey,
    pub management_fee_vault_onyc_account: Pubkey,
    pub performance_fee_vault_onyc_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeBufferAccounts<'_, '_>> for InitializeBufferKeys {
    fn from(accounts: InitializeBufferAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            buffer_state: *accounts.buffer_state.key,
            reserve_vault_authority: *accounts.reserve_vault_authority.key,
            management_fee_vault: *accounts.management_fee_vault.key,
            performance_fee_vault: *accounts.performance_fee_vault.key,
            boss: *accounts.boss.key,
            onyc_mint: *accounts.onyc_mint.key,
            offer: *accounts.offer.key,
            reserve_vault_onyc_account: *accounts.reserve_vault_onyc_account.key,
            management_fee_vault_onyc_account: *accounts
                .management_fee_vault_onyc_account
                .key,
            performance_fee_vault_onyc_account: *accounts
                .performance_fee_vault_onyc_account
                .key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeBufferKeys> for [AccountMeta; INITIALIZE_BUFFER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeBufferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.management_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.performance_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.performance_fee_vault_onyc_account,
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
impl From<[Pubkey; INITIALIZE_BUFFER_IX_ACCOUNTS_LEN]> for InitializeBufferKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_BUFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            buffer_state: pubkeys[1],
            reserve_vault_authority: pubkeys[2],
            management_fee_vault: pubkeys[3],
            performance_fee_vault: pubkeys[4],
            boss: pubkeys[5],
            onyc_mint: pubkeys[6],
            offer: pubkeys[7],
            reserve_vault_onyc_account: pubkeys[8],
            management_fee_vault_onyc_account: pubkeys[9],
            performance_fee_vault_onyc_account: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<InitializeBufferAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_BUFFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeBufferAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.buffer_state.clone(),
            accounts.reserve_vault_authority.clone(),
            accounts.management_fee_vault.clone(),
            accounts.performance_fee_vault.clone(),
            accounts.boss.clone(),
            accounts.onyc_mint.clone(),
            accounts.offer.clone(),
            accounts.reserve_vault_onyc_account.clone(),
            accounts.management_fee_vault_onyc_account.clone(),
            accounts.performance_fee_vault_onyc_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_BUFFER_IX_ACCOUNTS_LEN]>
for InitializeBufferAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_BUFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            buffer_state: &arr[1],
            reserve_vault_authority: &arr[2],
            management_fee_vault: &arr[3],
            performance_fee_vault: &arr[4],
            boss: &arr[5],
            onyc_mint: &arr[6],
            offer: &arr[7],
            reserve_vault_onyc_account: &arr[8],
            management_fee_vault_onyc_account: &arr[9],
            performance_fee_vault_onyc_account: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            system_program: &arr[13],
        }
    }
}
pub const INITIALIZE_BUFFER_IX_DISCM: [u8; 8usize] = [
    43, 127, 69, 196, 129, 6, 159, 210,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeBufferIxData;
impl InitializeBufferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_BUFFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_BUFFER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_buffer_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeBufferKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_BUFFER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeBufferIxData.try_to_vec()?,
    })
}
pub fn initialize_buffer_ix(keys: InitializeBufferKeys) -> std::io::Result<Instruction> {
    initialize_buffer_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn initialize_buffer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBufferAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeBufferKeys = accounts.into();
    let ix = initialize_buffer_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_buffer_invoke(
    accounts: InitializeBufferAccounts<'_, '_>,
) -> ProgramResult {
    initialize_buffer_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn initialize_buffer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBufferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeBufferKeys = accounts.into();
    let ix = initialize_buffer_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_buffer_invoke_signed(
    accounts: InitializeBufferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_buffer_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_buffer_verify_account_keys(
    accounts: InitializeBufferAccounts<'_, '_>,
    keys: InitializeBufferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.buffer_state.key, keys.buffer_state),
        (*accounts.reserve_vault_authority.key, keys.reserve_vault_authority),
        (*accounts.management_fee_vault.key, keys.management_fee_vault),
        (*accounts.performance_fee_vault.key, keys.performance_fee_vault),
        (*accounts.boss.key, keys.boss),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.offer.key, keys.offer),
        (*accounts.reserve_vault_onyc_account.key, keys.reserve_vault_onyc_account),
        (
            *accounts.management_fee_vault_onyc_account.key,
            keys.management_fee_vault_onyc_account,
        ),
        (
            *accounts.performance_fee_vault_onyc_account.key,
            keys.performance_fee_vault_onyc_account,
        ),
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
pub fn initialize_buffer_verify_writable_privileges<'me, 'info>(
    accounts: InitializeBufferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.buffer_state,
        accounts.reserve_vault_authority,
        accounts.management_fee_vault,
        accounts.performance_fee_vault,
        accounts.boss,
        accounts.reserve_vault_onyc_account,
        accounts.management_fee_vault_onyc_account,
        accounts.performance_fee_vault_onyc_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_buffer_verify_signer_privileges<'me, 'info>(
    accounts: InitializeBufferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_buffer_verify_account_privileges<'me, 'info>(
    accounts: InitializeBufferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_buffer_verify_writable_privileges(accounts)?;
    initialize_buffer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializePermissionlessAuthorityAccounts<'me, 'info> {
    pub permissionless_authority: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePermissionlessAuthorityKeys {
    pub permissionless_authority: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePermissionlessAuthorityAccounts<'_, '_>>
for InitializePermissionlessAuthorityKeys {
    fn from(accounts: InitializePermissionlessAuthorityAccounts) -> Self {
        Self {
            permissionless_authority: *accounts.permissionless_authority.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePermissionlessAuthorityKeys>
for [AccountMeta; INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePermissionlessAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.permissionless_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
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
impl From<[Pubkey; INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_ACCOUNTS_LEN]>
for InitializePermissionlessAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            permissionless_authority: pubkeys[0],
            state: pubkeys[1],
            boss: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializePermissionlessAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePermissionlessAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.permissionless_authority.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_ACCOUNTS_LEN]>
for InitializePermissionlessAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            permissionless_authority: &arr[0],
            state: &arr[1],
            boss: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    89, 93, 43, 180, 148, 16, 238, 24,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePermissionlessAuthorityIxArgs {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionlessAuthorityIxData(
    pub InitializePermissionlessAuthorityIxArgs,
);
impl From<InitializePermissionlessAuthorityIxArgs>
for InitializePermissionlessAuthorityIxData {
    fn from(args: InitializePermissionlessAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePermissionlessAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializePermissionlessAuthorityIxArgs {
                name,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_permissionless_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePermissionlessAuthorityKeys,
    args: InitializePermissionlessAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PERMISSIONLESS_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePermissionlessAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_permissionless_authority_ix(
    keys: InitializePermissionlessAuthorityKeys,
    args: InitializePermissionlessAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    initialize_permissionless_authority_ix_with_program_id(
        ONREAPP_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_permissionless_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessAuthorityAccounts<'_, '_>,
    args: InitializePermissionlessAuthorityIxArgs,
) -> ProgramResult {
    let keys: InitializePermissionlessAuthorityKeys = accounts.into();
    let ix = initialize_permissionless_authority_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_permissionless_authority_invoke(
    accounts: InitializePermissionlessAuthorityAccounts<'_, '_>,
    args: InitializePermissionlessAuthorityIxArgs,
) -> ProgramResult {
    initialize_permissionless_authority_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_permissionless_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessAuthorityAccounts<'_, '_>,
    args: InitializePermissionlessAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePermissionlessAuthorityKeys = accounts.into();
    let ix = initialize_permissionless_authority_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_permissionless_authority_invoke_signed(
    accounts: InitializePermissionlessAuthorityAccounts<'_, '_>,
    args: InitializePermissionlessAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_permissionless_authority_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_permissionless_authority_verify_account_keys(
    accounts: InitializePermissionlessAuthorityAccounts<'_, '_>,
    keys: InitializePermissionlessAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.permissionless_authority.key, keys.permissionless_authority),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_authority_verify_writable_privileges<'me, 'info>(
    accounts: InitializePermissionlessAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.permissionless_authority, accounts.boss] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_authority_verify_signer_privileges<'me, 'info>(
    accounts: InitializePermissionlessAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_authority_verify_account_privileges<'me, 'info>(
    accounts: InitializePermissionlessAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_permissionless_authority_verify_writable_privileges(accounts)?;
    initialize_permissionless_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MAKE_OFFER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct MakeOfferAccounts<'me, 'info> {
    pub vault_authority: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub vault_token_in_account: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MakeOfferKeys {
    pub vault_authority: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub vault_token_in_account: Pubkey,
    pub token_out_mint: Pubkey,
    pub offer: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<MakeOfferAccounts<'_, '_>> for MakeOfferKeys {
    fn from(accounts: MakeOfferAccounts) -> Self {
        Self {
            vault_authority: *accounts.vault_authority.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            vault_token_in_account: *accounts.vault_token_in_account.key,
            token_out_mint: *accounts.token_out_mint.key,
            offer: *accounts.offer.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MakeOfferKeys> for [AccountMeta; MAKE_OFFER_IX_ACCOUNTS_LEN] {
    fn from(keys: MakeOfferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; MAKE_OFFER_IX_ACCOUNTS_LEN]> for MakeOfferKeys {
    fn from(pubkeys: [Pubkey; MAKE_OFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_authority: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_in_program: pubkeys[2],
            vault_token_in_account: pubkeys[3],
            token_out_mint: pubkeys[4],
            offer: pubkeys[5],
            state: pubkeys[6],
            boss: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<MakeOfferAccounts<'_, 'info>>
for [AccountInfo<'info>; MAKE_OFFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: MakeOfferAccounts<'_, 'info>) -> Self {
        [
            accounts.vault_authority.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.vault_token_in_account.clone(),
            accounts.token_out_mint.clone(),
            accounts.offer.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MAKE_OFFER_IX_ACCOUNTS_LEN]>
for MakeOfferAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MAKE_OFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_authority: &arr[0],
            token_in_mint: &arr[1],
            token_in_program: &arr[2],
            vault_token_in_account: &arr[3],
            token_out_mint: &arr[4],
            offer: &arr[5],
            state: &arr[6],
            boss: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const MAKE_OFFER_IX_DISCM: [u8; 8usize] = [214, 98, 97, 35, 59, 12, 44, 178];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MakeOfferIxArgs {
    pub fee_basis_points: u16,
    pub needs_approval: bool,
    pub allow_permissionless: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MakeOfferIxData(pub MakeOfferIxArgs);
impl From<MakeOfferIxArgs> for MakeOfferIxData {
    fn from(args: MakeOfferIxArgs) -> Self {
        Self(args)
    }
}
impl MakeOfferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MAKE_OFFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let needs_approval: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_permissionless: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(MakeOfferIxArgs {
                fee_basis_points,
                needs_approval,
                allow_permissionless,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MAKE_OFFER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.needs_approval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.allow_permissionless, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn make_offer_ix_with_program_id(
    program_id: Pubkey,
    keys: MakeOfferKeys,
    args: MakeOfferIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MAKE_OFFER_IX_ACCOUNTS_LEN] = keys.into();
    let data: MakeOfferIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn make_offer_ix(
    keys: MakeOfferKeys,
    args: MakeOfferIxArgs,
) -> std::io::Result<Instruction> {
    make_offer_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn make_offer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MakeOfferAccounts<'_, '_>,
    args: MakeOfferIxArgs,
) -> ProgramResult {
    let keys: MakeOfferKeys = accounts.into();
    let ix = make_offer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn make_offer_invoke(
    accounts: MakeOfferAccounts<'_, '_>,
    args: MakeOfferIxArgs,
) -> ProgramResult {
    make_offer_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn make_offer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MakeOfferAccounts<'_, '_>,
    args: MakeOfferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MakeOfferKeys = accounts.into();
    let ix = make_offer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn make_offer_invoke_signed(
    accounts: MakeOfferAccounts<'_, '_>,
    args: MakeOfferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    make_offer_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, args, seeds)
}
pub fn make_offer_verify_account_keys(
    accounts: MakeOfferAccounts<'_, '_>,
    keys: MakeOfferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.vault_token_in_account.key, keys.vault_token_in_account),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.offer.key, keys.offer),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn make_offer_verify_writable_privileges<'me, 'info>(
    accounts: MakeOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_token_in_account,
        accounts.offer,
        accounts.boss,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn make_offer_verify_signer_privileges<'me, 'info>(
    accounts: MakeOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn make_offer_verify_account_privileges<'me, 'info>(
    accounts: MakeOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    make_offer_verify_writable_privileges(accounts)?;
    make_offer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MAKE_REDEMPTION_OFFER_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct MakeRedemptionOfferAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub offer: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub vault_token_in_account: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub vault_token_out_account: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MakeRedemptionOfferKeys {
    pub state: Pubkey,
    pub offer: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub vault_token_in_account: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub vault_token_out_account: Pubkey,
    pub redemption_offer: Pubkey,
    pub boss: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<MakeRedemptionOfferAccounts<'_, '_>> for MakeRedemptionOfferKeys {
    fn from(accounts: MakeRedemptionOfferAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            offer: *accounts.offer.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            vault_token_in_account: *accounts.vault_token_in_account.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            vault_token_out_account: *accounts.vault_token_out_account.key,
            redemption_offer: *accounts.redemption_offer.key,
            boss: *accounts.boss.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MakeRedemptionOfferKeys>
for [AccountMeta; MAKE_REDEMPTION_OFFER_IX_ACCOUNTS_LEN] {
    fn from(keys: MakeRedemptionOfferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; MAKE_REDEMPTION_OFFER_IX_ACCOUNTS_LEN]> for MakeRedemptionOfferKeys {
    fn from(pubkeys: [Pubkey; MAKE_REDEMPTION_OFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            offer: pubkeys[1],
            redemption_vault_authority: pubkeys[2],
            token_in_mint: pubkeys[3],
            token_in_program: pubkeys[4],
            vault_token_in_account: pubkeys[5],
            token_out_mint: pubkeys[6],
            token_out_program: pubkeys[7],
            vault_token_out_account: pubkeys[8],
            redemption_offer: pubkeys[9],
            boss: pubkeys[10],
            associated_token_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<MakeRedemptionOfferAccounts<'_, 'info>>
for [AccountInfo<'info>; MAKE_REDEMPTION_OFFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: MakeRedemptionOfferAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.offer.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.vault_token_in_account.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.vault_token_out_account.clone(),
            accounts.redemption_offer.clone(),
            accounts.boss.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MAKE_REDEMPTION_OFFER_IX_ACCOUNTS_LEN]>
for MakeRedemptionOfferAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MAKE_REDEMPTION_OFFER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            offer: &arr[1],
            redemption_vault_authority: &arr[2],
            token_in_mint: &arr[3],
            token_in_program: &arr[4],
            vault_token_in_account: &arr[5],
            token_out_mint: &arr[6],
            token_out_program: &arr[7],
            vault_token_out_account: &arr[8],
            redemption_offer: &arr[9],
            boss: &arr[10],
            associated_token_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const MAKE_REDEMPTION_OFFER_IX_DISCM: [u8; 8usize] = [
    6, 130, 180, 160, 163, 166, 51, 41,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MakeRedemptionOfferIxArgs {
    pub fee_basis_points: u16,
    pub fee_basis_points_prop_amm_sell: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MakeRedemptionOfferIxData(pub MakeRedemptionOfferIxArgs);
impl From<MakeRedemptionOfferIxArgs> for MakeRedemptionOfferIxData {
    fn from(args: MakeRedemptionOfferIxArgs) -> Self {
        Self(args)
    }
}
impl MakeRedemptionOfferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MAKE_REDEMPTION_OFFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points_prop_amm_sell: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(MakeRedemptionOfferIxArgs {
                fee_basis_points,
                fee_basis_points_prop_amm_sell,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MAKE_REDEMPTION_OFFER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.fee_basis_points_prop_amm_sell,
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
pub fn make_redemption_offer_ix_with_program_id(
    program_id: Pubkey,
    keys: MakeRedemptionOfferKeys,
    args: MakeRedemptionOfferIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MAKE_REDEMPTION_OFFER_IX_ACCOUNTS_LEN] = keys.into();
    let data: MakeRedemptionOfferIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn make_redemption_offer_ix(
    keys: MakeRedemptionOfferKeys,
    args: MakeRedemptionOfferIxArgs,
) -> std::io::Result<Instruction> {
    make_redemption_offer_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn make_redemption_offer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MakeRedemptionOfferAccounts<'_, '_>,
    args: MakeRedemptionOfferIxArgs,
) -> ProgramResult {
    let keys: MakeRedemptionOfferKeys = accounts.into();
    let ix = make_redemption_offer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn make_redemption_offer_invoke(
    accounts: MakeRedemptionOfferAccounts<'_, '_>,
    args: MakeRedemptionOfferIxArgs,
) -> ProgramResult {
    make_redemption_offer_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn make_redemption_offer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MakeRedemptionOfferAccounts<'_, '_>,
    args: MakeRedemptionOfferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MakeRedemptionOfferKeys = accounts.into();
    let ix = make_redemption_offer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn make_redemption_offer_invoke_signed(
    accounts: MakeRedemptionOfferAccounts<'_, '_>,
    args: MakeRedemptionOfferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    make_redemption_offer_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn make_redemption_offer_verify_account_keys(
    accounts: MakeRedemptionOfferAccounts<'_, '_>,
    keys: MakeRedemptionOfferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.offer.key, keys.offer),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.vault_token_in_account.key, keys.vault_token_in_account),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.vault_token_out_account.key, keys.vault_token_out_account),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.boss.key, keys.boss),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn make_redemption_offer_verify_writable_privileges<'me, 'info>(
    accounts: MakeRedemptionOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_token_in_account,
        accounts.vault_token_out_account,
        accounts.redemption_offer,
        accounts.boss,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn make_redemption_offer_verify_signer_privileges<'me, 'info>(
    accounts: MakeRedemptionOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn make_redemption_offer_verify_account_privileges<'me, 'info>(
    accounts: MakeRedemptionOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    make_redemption_offer_verify_writable_privileges(accounts)?;
    make_redemption_offer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_TO_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct MintToAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub boss_onyc_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintToKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
    pub onyc_mint: Pubkey,
    pub boss_onyc_account: Pubkey,
    pub mint_authority: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub main_offer: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
}
impl From<MintToAccounts<'_, '_>> for MintToKeys {
    fn from(accounts: MintToAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            onyc_mint: *accounts.onyc_mint.key,
            boss_onyc_account: *accounts.boss_onyc_account.key,
            mint_authority: *accounts.mint_authority.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            main_offer: *accounts.main_offer.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
        }
    }
}
impl From<MintToKeys> for [AccountMeta; MINT_TO_IX_ACCOUNTS_LEN] {
    fn from(keys: MintToKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
            AccountMeta {
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_TO_IX_ACCOUNTS_LEN]> for MintToKeys {
    fn from(pubkeys: [Pubkey; MINT_TO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
            onyc_mint: pubkeys[2],
            boss_onyc_account: pubkeys[3],
            mint_authority: pubkeys[4],
            token_program: pubkeys[5],
            associated_token_program: pubkeys[6],
            system_program: pubkeys[7],
            main_offer: pubkeys[8],
            buffer_accounts_buffer_state: pubkeys[9],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[10],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[11],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[12],
            market_stats: pubkeys[13],
            circulating_supply_excluded_balance: pubkeys[14],
        }
    }
}
impl<'info> From<MintToAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintToAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.onyc_mint.clone(),
            accounts.boss_onyc_account.clone(),
            accounts.mint_authority.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.main_offer.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN]>
for MintToAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
            onyc_mint: &arr[2],
            boss_onyc_account: &arr[3],
            mint_authority: &arr[4],
            token_program: &arr[5],
            associated_token_program: &arr[6],
            system_program: &arr[7],
            main_offer: &arr[8],
            buffer_accounts_buffer_state: &arr[9],
            buffer_accounts_reserve_vault_onyc_account: &arr[10],
            buffer_accounts_management_fee_vault_onyc_account: &arr[11],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[12],
            market_stats: &arr[13],
            circulating_supply_excluded_balance: &arr[14],
        }
    }
}
pub const MINT_TO_IX_DISCM: [u8; 8usize] = [241, 34, 48, 186, 37, 179, 123, 192];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintToIxArgs {
    pub amount: u64,
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
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_TO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(MintToIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_TO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
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
    mint_to_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
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
    mint_to_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
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
    mint_to_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, args, seeds)
}
pub fn mint_to_verify_account_keys(
    accounts: MintToAccounts<'_, '_>,
    keys: MintToKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.boss_onyc_account.key, keys.boss_onyc_account),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.main_offer.key, keys.main_offer),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_to_verify_writable_privileges<'me, 'info>(
    accounts: MintToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.boss,
        accounts.onyc_mint,
        accounts.boss_onyc_account,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
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
    for should_be_signer in [accounts.boss] {
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
pub const OFFER_VAULT_DEPOSIT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct OfferVaultDepositAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub depositor: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OfferVaultDepositKeys {
    pub state: Pubkey,
    pub vault_authority: Pubkey,
    pub token_mint: Pubkey,
    pub depositor_token_account: Pubkey,
    pub vault_token_account: Pubkey,
    pub depositor: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<OfferVaultDepositAccounts<'_, '_>> for OfferVaultDepositKeys {
    fn from(accounts: OfferVaultDepositAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            vault_authority: *accounts.vault_authority.key,
            token_mint: *accounts.token_mint.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            vault_token_account: *accounts.vault_token_account.key,
            depositor: *accounts.depositor.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<OfferVaultDepositKeys> for [AccountMeta; OFFER_VAULT_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: OfferVaultDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.depositor,
                is_signer: true,
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
impl From<[Pubkey; OFFER_VAULT_DEPOSIT_IX_ACCOUNTS_LEN]> for OfferVaultDepositKeys {
    fn from(pubkeys: [Pubkey; OFFER_VAULT_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            vault_authority: pubkeys[1],
            token_mint: pubkeys[2],
            depositor_token_account: pubkeys[3],
            vault_token_account: pubkeys[4],
            depositor: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<OfferVaultDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; OFFER_VAULT_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: OfferVaultDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.vault_authority.clone(),
            accounts.token_mint.clone(),
            accounts.depositor_token_account.clone(),
            accounts.vault_token_account.clone(),
            accounts.depositor.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OFFER_VAULT_DEPOSIT_IX_ACCOUNTS_LEN]>
for OfferVaultDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OFFER_VAULT_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            vault_authority: &arr[1],
            token_mint: &arr[2],
            depositor_token_account: &arr[3],
            vault_token_account: &arr[4],
            depositor: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const OFFER_VAULT_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    69, 131, 100, 85, 82, 151, 72, 74,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferVaultDepositIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferVaultDepositIxData(pub OfferVaultDepositIxArgs);
impl From<OfferVaultDepositIxArgs> for OfferVaultDepositIxData {
    fn from(args: OfferVaultDepositIxArgs) -> Self {
        Self(args)
    }
}
impl OfferVaultDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_VAULT_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(OfferVaultDepositIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_VAULT_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn offer_vault_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: OfferVaultDepositKeys,
    args: OfferVaultDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OFFER_VAULT_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: OfferVaultDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn offer_vault_deposit_ix(
    keys: OfferVaultDepositKeys,
    args: OfferVaultDepositIxArgs,
) -> std::io::Result<Instruction> {
    offer_vault_deposit_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn offer_vault_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OfferVaultDepositAccounts<'_, '_>,
    args: OfferVaultDepositIxArgs,
) -> ProgramResult {
    let keys: OfferVaultDepositKeys = accounts.into();
    let ix = offer_vault_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn offer_vault_deposit_invoke(
    accounts: OfferVaultDepositAccounts<'_, '_>,
    args: OfferVaultDepositIxArgs,
) -> ProgramResult {
    offer_vault_deposit_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn offer_vault_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OfferVaultDepositAccounts<'_, '_>,
    args: OfferVaultDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OfferVaultDepositKeys = accounts.into();
    let ix = offer_vault_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn offer_vault_deposit_invoke_signed(
    accounts: OfferVaultDepositAccounts<'_, '_>,
    args: OfferVaultDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    offer_vault_deposit_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn offer_vault_deposit_verify_account_keys(
    accounts: OfferVaultDepositAccounts<'_, '_>,
    keys: OfferVaultDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.depositor_token_account.key, keys.depositor_token_account),
        (*accounts.vault_token_account.key, keys.vault_token_account),
        (*accounts.depositor.key, keys.depositor),
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
pub fn offer_vault_deposit_verify_writable_privileges<'me, 'info>(
    accounts: OfferVaultDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.depositor_token_account,
        accounts.vault_token_account,
        accounts.depositor,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn offer_vault_deposit_verify_signer_privileges<'me, 'info>(
    accounts: OfferVaultDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.depositor] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn offer_vault_deposit_verify_account_privileges<'me, 'info>(
    accounts: OfferVaultDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    offer_vault_deposit_verify_writable_privileges(accounts)?;
    offer_vault_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OFFER_VAULT_WITHDRAW_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct OfferVaultWithdrawAccounts<'me, 'info> {
    pub vault_authority: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub boss_token_account: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OfferVaultWithdrawKeys {
    pub vault_authority: Pubkey,
    pub token_mint: Pubkey,
    pub boss_token_account: Pubkey,
    pub vault_token_account: Pubkey,
    pub boss: Pubkey,
    pub state: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<OfferVaultWithdrawAccounts<'_, '_>> for OfferVaultWithdrawKeys {
    fn from(accounts: OfferVaultWithdrawAccounts) -> Self {
        Self {
            vault_authority: *accounts.vault_authority.key,
            token_mint: *accounts.token_mint.key,
            boss_token_account: *accounts.boss_token_account.key,
            vault_token_account: *accounts.vault_token_account.key,
            boss: *accounts.boss.key,
            state: *accounts.state.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<OfferVaultWithdrawKeys>
for [AccountMeta; OFFER_VAULT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: OfferVaultWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
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
impl From<[Pubkey; OFFER_VAULT_WITHDRAW_IX_ACCOUNTS_LEN]> for OfferVaultWithdrawKeys {
    fn from(pubkeys: [Pubkey; OFFER_VAULT_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault_authority: pubkeys[0],
            token_mint: pubkeys[1],
            boss_token_account: pubkeys[2],
            vault_token_account: pubkeys[3],
            boss: pubkeys[4],
            state: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<OfferVaultWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; OFFER_VAULT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: OfferVaultWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault_authority.clone(),
            accounts.token_mint.clone(),
            accounts.boss_token_account.clone(),
            accounts.vault_token_account.clone(),
            accounts.boss.clone(),
            accounts.state.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OFFER_VAULT_WITHDRAW_IX_ACCOUNTS_LEN]>
for OfferVaultWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OFFER_VAULT_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault_authority: &arr[0],
            token_mint: &arr[1],
            boss_token_account: &arr[2],
            vault_token_account: &arr[3],
            boss: &arr[4],
            state: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const OFFER_VAULT_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    230, 135, 129, 48, 138, 202, 20, 72,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfferVaultWithdrawIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferVaultWithdrawIxData(pub OfferVaultWithdrawIxArgs);
impl From<OfferVaultWithdrawIxArgs> for OfferVaultWithdrawIxData {
    fn from(args: OfferVaultWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl OfferVaultWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_VAULT_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(OfferVaultWithdrawIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_VAULT_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn offer_vault_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: OfferVaultWithdrawKeys,
    args: OfferVaultWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OFFER_VAULT_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: OfferVaultWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn offer_vault_withdraw_ix(
    keys: OfferVaultWithdrawKeys,
    args: OfferVaultWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    offer_vault_withdraw_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn offer_vault_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OfferVaultWithdrawAccounts<'_, '_>,
    args: OfferVaultWithdrawIxArgs,
) -> ProgramResult {
    let keys: OfferVaultWithdrawKeys = accounts.into();
    let ix = offer_vault_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn offer_vault_withdraw_invoke(
    accounts: OfferVaultWithdrawAccounts<'_, '_>,
    args: OfferVaultWithdrawIxArgs,
) -> ProgramResult {
    offer_vault_withdraw_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn offer_vault_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OfferVaultWithdrawAccounts<'_, '_>,
    args: OfferVaultWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OfferVaultWithdrawKeys = accounts.into();
    let ix = offer_vault_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn offer_vault_withdraw_invoke_signed(
    accounts: OfferVaultWithdrawAccounts<'_, '_>,
    args: OfferVaultWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    offer_vault_withdraw_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn offer_vault_withdraw_verify_account_keys(
    accounts: OfferVaultWithdrawAccounts<'_, '_>,
    keys: OfferVaultWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.boss_token_account.key, keys.boss_token_account),
        (*accounts.vault_token_account.key, keys.vault_token_account),
        (*accounts.boss.key, keys.boss),
        (*accounts.state.key, keys.state),
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
pub fn offer_vault_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: OfferVaultWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.boss_token_account,
        accounts.vault_token_account,
        accounts.boss,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn offer_vault_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: OfferVaultWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn offer_vault_withdraw_verify_account_privileges<'me, 'info>(
    accounts: OfferVaultWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    offer_vault_withdraw_verify_writable_privileges(accounts)?;
    offer_vault_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_SWAP_BUY_IX_ACCOUNTS_LEN: usize = 34;
#[derive(Copy, Clone, Debug)]
pub struct OpenSwapBuyAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub prop_amm_pair_state: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub offer_vault_authority: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub offer_vault_token_in_account: &'me AccountInfo<'info>,
    pub offer_vault_token_out_account: &'me AccountInfo<'info>,
    pub redemption_vault_token_in_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub prop_amm_proceeds_vault: &'me AccountInfo<'info>,
    pub prop_amm_proceeds_token_in_account: &'me AccountInfo<'info>,
    pub prop_amm_buy_fee_vault: &'me AccountInfo<'info>,
    pub prop_amm_buy_fee_token_in_account: &'me AccountInfo<'info>,
    pub permissionless_authority: &'me AccountInfo<'info>,
    pub permissionless_token_in_account: &'me AccountInfo<'info>,
    pub permissionless_token_out_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenSwapBuyKeys {
    pub offer: Pubkey,
    pub prop_amm_pair_state: Pubkey,
    pub redemption_offer: Pubkey,
    pub state: Pubkey,
    pub offer_vault_authority: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub offer_vault_token_in_account: Pubkey,
    pub offer_vault_token_out_account: Pubkey,
    pub redemption_vault_token_in_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub prop_amm_proceeds_vault: Pubkey,
    pub prop_amm_proceeds_token_in_account: Pubkey,
    pub prop_amm_buy_fee_vault: Pubkey,
    pub prop_amm_buy_fee_token_in_account: Pubkey,
    pub permissionless_authority: Pubkey,
    pub permissionless_token_in_account: Pubkey,
    pub permissionless_token_out_account: Pubkey,
    pub mint_authority: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub user: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub main_offer: Pubkey,
}
impl From<OpenSwapBuyAccounts<'_, '_>> for OpenSwapBuyKeys {
    fn from(accounts: OpenSwapBuyAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            prop_amm_pair_state: *accounts.prop_amm_pair_state.key,
            redemption_offer: *accounts.redemption_offer.key,
            state: *accounts.state.key,
            offer_vault_authority: *accounts.offer_vault_authority.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            offer_vault_token_in_account: *accounts.offer_vault_token_in_account.key,
            offer_vault_token_out_account: *accounts.offer_vault_token_out_account.key,
            redemption_vault_token_in_account: *accounts
                .redemption_vault_token_in_account
                .key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            user_token_in_account: *accounts.user_token_in_account.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            prop_amm_proceeds_vault: *accounts.prop_amm_proceeds_vault.key,
            prop_amm_proceeds_token_in_account: *accounts
                .prop_amm_proceeds_token_in_account
                .key,
            prop_amm_buy_fee_vault: *accounts.prop_amm_buy_fee_vault.key,
            prop_amm_buy_fee_token_in_account: *accounts
                .prop_amm_buy_fee_token_in_account
                .key,
            permissionless_authority: *accounts.permissionless_authority.key,
            permissionless_token_in_account: *accounts
                .permissionless_token_in_account
                .key,
            permissionless_token_out_account: *accounts
                .permissionless_token_out_account
                .key,
            mint_authority: *accounts.mint_authority.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            user: *accounts.user.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            main_offer: *accounts.main_offer.key,
        }
    }
}
impl From<OpenSwapBuyKeys> for [AccountMeta; OPEN_SWAP_BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenSwapBuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.prop_amm_pair_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer_vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_vault_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prop_amm_proceeds_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prop_amm_proceeds_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prop_amm_buy_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prop_amm_buy_fee_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permissionless_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permissionless_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permissionless_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_SWAP_BUY_IX_ACCOUNTS_LEN]> for OpenSwapBuyKeys {
    fn from(pubkeys: [Pubkey; OPEN_SWAP_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            prop_amm_pair_state: pubkeys[1],
            redemption_offer: pubkeys[2],
            state: pubkeys[3],
            offer_vault_authority: pubkeys[4],
            redemption_vault_authority: pubkeys[5],
            offer_vault_token_in_account: pubkeys[6],
            offer_vault_token_out_account: pubkeys[7],
            redemption_vault_token_in_account: pubkeys[8],
            token_in_mint: pubkeys[9],
            token_in_program: pubkeys[10],
            token_out_mint: pubkeys[11],
            token_out_program: pubkeys[12],
            user_token_in_account: pubkeys[13],
            user_token_out_account: pubkeys[14],
            prop_amm_proceeds_vault: pubkeys[15],
            prop_amm_proceeds_token_in_account: pubkeys[16],
            prop_amm_buy_fee_vault: pubkeys[17],
            prop_amm_buy_fee_token_in_account: pubkeys[18],
            permissionless_authority: pubkeys[19],
            permissionless_token_in_account: pubkeys[20],
            permissionless_token_out_account: pubkeys[21],
            mint_authority: pubkeys[22],
            buffer_accounts_buffer_state: pubkeys[23],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[24],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[25],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[26],
            market_stats: pubkeys[27],
            circulating_supply_excluded_balance: pubkeys[28],
            instructions_sysvar: pubkeys[29],
            user: pubkeys[30],
            associated_token_program: pubkeys[31],
            system_program: pubkeys[32],
            main_offer: pubkeys[33],
        }
    }
}
impl<'info> From<OpenSwapBuyAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_SWAP_BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenSwapBuyAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.prop_amm_pair_state.clone(),
            accounts.redemption_offer.clone(),
            accounts.state.clone(),
            accounts.offer_vault_authority.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.offer_vault_token_in_account.clone(),
            accounts.offer_vault_token_out_account.clone(),
            accounts.redemption_vault_token_in_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.user_token_in_account.clone(),
            accounts.user_token_out_account.clone(),
            accounts.prop_amm_proceeds_vault.clone(),
            accounts.prop_amm_proceeds_token_in_account.clone(),
            accounts.prop_amm_buy_fee_vault.clone(),
            accounts.prop_amm_buy_fee_token_in_account.clone(),
            accounts.permissionless_authority.clone(),
            accounts.permissionless_token_in_account.clone(),
            accounts.permissionless_token_out_account.clone(),
            accounts.mint_authority.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.user.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.main_offer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_SWAP_BUY_IX_ACCOUNTS_LEN]>
for OpenSwapBuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_SWAP_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            prop_amm_pair_state: &arr[1],
            redemption_offer: &arr[2],
            state: &arr[3],
            offer_vault_authority: &arr[4],
            redemption_vault_authority: &arr[5],
            offer_vault_token_in_account: &arr[6],
            offer_vault_token_out_account: &arr[7],
            redemption_vault_token_in_account: &arr[8],
            token_in_mint: &arr[9],
            token_in_program: &arr[10],
            token_out_mint: &arr[11],
            token_out_program: &arr[12],
            user_token_in_account: &arr[13],
            user_token_out_account: &arr[14],
            prop_amm_proceeds_vault: &arr[15],
            prop_amm_proceeds_token_in_account: &arr[16],
            prop_amm_buy_fee_vault: &arr[17],
            prop_amm_buy_fee_token_in_account: &arr[18],
            permissionless_authority: &arr[19],
            permissionless_token_in_account: &arr[20],
            permissionless_token_out_account: &arr[21],
            mint_authority: &arr[22],
            buffer_accounts_buffer_state: &arr[23],
            buffer_accounts_reserve_vault_onyc_account: &arr[24],
            buffer_accounts_management_fee_vault_onyc_account: &arr[25],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[26],
            market_stats: &arr[27],
            circulating_supply_excluded_balance: &arr[28],
            instructions_sysvar: &arr[29],
            user: &arr[30],
            associated_token_program: &arr[31],
            system_program: &arr[32],
            main_offer: &arr[33],
        }
    }
}
pub const OPEN_SWAP_BUY_IX_DISCM: [u8; 8usize] = [
    143, 202, 194, 184, 129, 189, 219, 139,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenSwapBuyIxArgs {
    pub token_in_amount: u64,
    pub minimum_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenSwapBuyIxData(pub OpenSwapBuyIxArgs);
impl From<OpenSwapBuyIxArgs> for OpenSwapBuyIxData {
    fn from(args: OpenSwapBuyIxArgs) -> Self {
        Self(args)
    }
}
impl OpenSwapBuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_SWAP_BUY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OpenSwapBuyIxArgs {
                token_in_amount,
                minimum_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_SWAP_BUY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_swap_buy_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenSwapBuyKeys,
    args: OpenSwapBuyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_SWAP_BUY_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenSwapBuyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_swap_buy_ix(
    keys: OpenSwapBuyKeys,
    args: OpenSwapBuyIxArgs,
) -> std::io::Result<Instruction> {
    open_swap_buy_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn open_swap_buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenSwapBuyAccounts<'_, '_>,
    args: OpenSwapBuyIxArgs,
) -> ProgramResult {
    let keys: OpenSwapBuyKeys = accounts.into();
    let ix = open_swap_buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_swap_buy_invoke(
    accounts: OpenSwapBuyAccounts<'_, '_>,
    args: OpenSwapBuyIxArgs,
) -> ProgramResult {
    open_swap_buy_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn open_swap_buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenSwapBuyAccounts<'_, '_>,
    args: OpenSwapBuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenSwapBuyKeys = accounts.into();
    let ix = open_swap_buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_swap_buy_invoke_signed(
    accounts: OpenSwapBuyAccounts<'_, '_>,
    args: OpenSwapBuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_swap_buy_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_swap_buy_verify_account_keys(
    accounts: OpenSwapBuyAccounts<'_, '_>,
    keys: OpenSwapBuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.prop_amm_pair_state.key, keys.prop_amm_pair_state),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.state.key, keys.state),
        (*accounts.offer_vault_authority.key, keys.offer_vault_authority),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (*accounts.offer_vault_token_in_account.key, keys.offer_vault_token_in_account),
        (
            *accounts.offer_vault_token_out_account.key,
            keys.offer_vault_token_out_account,
        ),
        (
            *accounts.redemption_vault_token_in_account.key,
            keys.redemption_vault_token_in_account,
        ),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.user_token_in_account.key, keys.user_token_in_account),
        (*accounts.user_token_out_account.key, keys.user_token_out_account),
        (*accounts.prop_amm_proceeds_vault.key, keys.prop_amm_proceeds_vault),
        (
            *accounts.prop_amm_proceeds_token_in_account.key,
            keys.prop_amm_proceeds_token_in_account,
        ),
        (*accounts.prop_amm_buy_fee_vault.key, keys.prop_amm_buy_fee_vault),
        (
            *accounts.prop_amm_buy_fee_token_in_account.key,
            keys.prop_amm_buy_fee_token_in_account,
        ),
        (*accounts.permissionless_authority.key, keys.permissionless_authority),
        (
            *accounts.permissionless_token_in_account.key,
            keys.permissionless_token_in_account,
        ),
        (
            *accounts.permissionless_token_out_account.key,
            keys.permissionless_token_out_account,
        ),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.user.key, keys.user),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.main_offer.key, keys.main_offer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_swap_buy_verify_writable_privileges<'me, 'info>(
    accounts: OpenSwapBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.prop_amm_pair_state,
        accounts.offer_vault_token_in_account,
        accounts.offer_vault_token_out_account,
        accounts.redemption_vault_token_in_account,
        accounts.token_in_mint,
        accounts.token_out_mint,
        accounts.user_token_in_account,
        accounts.user_token_out_account,
        accounts.prop_amm_proceeds_vault,
        accounts.prop_amm_proceeds_token_in_account,
        accounts.prop_amm_buy_fee_vault,
        accounts.prop_amm_buy_fee_token_in_account,
        accounts.permissionless_token_in_account,
        accounts.permissionless_token_out_account,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_swap_buy_verify_signer_privileges<'me, 'info>(
    accounts: OpenSwapBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_swap_buy_verify_account_privileges<'me, 'info>(
    accounts: OpenSwapBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_swap_buy_verify_writable_privileges(accounts)?;
    open_swap_buy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_SWAP_SELL_IX_ACCOUNTS_LEN: usize = 31;
#[derive(Copy, Clone, Debug)]
pub struct OpenSwapSellAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub prop_amm_pair_state: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub offer_vault_authority: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub redemption_vault_token_in_account: &'me AccountInfo<'info>,
    pub redemption_vault_token_out_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub prop_amm_proceeds_vault: &'me AccountInfo<'info>,
    pub prop_amm_proceeds_token_in_account: &'me AccountInfo<'info>,
    pub prop_amm_sell_fee_vault: &'me AccountInfo<'info>,
    pub prop_amm_sell_fee_token_in_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
    pub offer_vault_onyc_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenSwapSellKeys {
    pub offer: Pubkey,
    pub prop_amm_pair_state: Pubkey,
    pub redemption_offer: Pubkey,
    pub state: Pubkey,
    pub offer_vault_authority: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub redemption_vault_token_in_account: Pubkey,
    pub redemption_vault_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub prop_amm_proceeds_vault: Pubkey,
    pub prop_amm_proceeds_token_in_account: Pubkey,
    pub prop_amm_sell_fee_vault: Pubkey,
    pub prop_amm_sell_fee_token_in_account: Pubkey,
    pub mint_authority: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub user: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub main_offer: Pubkey,
    pub offer_vault_onyc_account: Pubkey,
}
impl From<OpenSwapSellAccounts<'_, '_>> for OpenSwapSellKeys {
    fn from(accounts: OpenSwapSellAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            prop_amm_pair_state: *accounts.prop_amm_pair_state.key,
            redemption_offer: *accounts.redemption_offer.key,
            state: *accounts.state.key,
            offer_vault_authority: *accounts.offer_vault_authority.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            redemption_vault_token_in_account: *accounts
                .redemption_vault_token_in_account
                .key,
            redemption_vault_token_out_account: *accounts
                .redemption_vault_token_out_account
                .key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            user_token_in_account: *accounts.user_token_in_account.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            prop_amm_proceeds_vault: *accounts.prop_amm_proceeds_vault.key,
            prop_amm_proceeds_token_in_account: *accounts
                .prop_amm_proceeds_token_in_account
                .key,
            prop_amm_sell_fee_vault: *accounts.prop_amm_sell_fee_vault.key,
            prop_amm_sell_fee_token_in_account: *accounts
                .prop_amm_sell_fee_token_in_account
                .key,
            mint_authority: *accounts.mint_authority.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            user: *accounts.user.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            main_offer: *accounts.main_offer.key,
            offer_vault_onyc_account: *accounts.offer_vault_onyc_account.key,
        }
    }
}
impl From<OpenSwapSellKeys> for [AccountMeta; OPEN_SWAP_SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenSwapSellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.prop_amm_pair_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prop_amm_proceeds_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prop_amm_proceeds_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prop_amm_sell_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prop_amm_sell_fee_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer_vault_onyc_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_SWAP_SELL_IX_ACCOUNTS_LEN]> for OpenSwapSellKeys {
    fn from(pubkeys: [Pubkey; OPEN_SWAP_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            prop_amm_pair_state: pubkeys[1],
            redemption_offer: pubkeys[2],
            state: pubkeys[3],
            offer_vault_authority: pubkeys[4],
            redemption_vault_authority: pubkeys[5],
            redemption_vault_token_in_account: pubkeys[6],
            redemption_vault_token_out_account: pubkeys[7],
            token_in_mint: pubkeys[8],
            token_in_program: pubkeys[9],
            token_out_mint: pubkeys[10],
            token_out_program: pubkeys[11],
            user_token_in_account: pubkeys[12],
            user_token_out_account: pubkeys[13],
            prop_amm_proceeds_vault: pubkeys[14],
            prop_amm_proceeds_token_in_account: pubkeys[15],
            prop_amm_sell_fee_vault: pubkeys[16],
            prop_amm_sell_fee_token_in_account: pubkeys[17],
            mint_authority: pubkeys[18],
            buffer_accounts_buffer_state: pubkeys[19],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[20],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[21],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[22],
            market_stats: pubkeys[23],
            circulating_supply_excluded_balance: pubkeys[24],
            instructions_sysvar: pubkeys[25],
            user: pubkeys[26],
            associated_token_program: pubkeys[27],
            system_program: pubkeys[28],
            main_offer: pubkeys[29],
            offer_vault_onyc_account: pubkeys[30],
        }
    }
}
impl<'info> From<OpenSwapSellAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_SWAP_SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenSwapSellAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.prop_amm_pair_state.clone(),
            accounts.redemption_offer.clone(),
            accounts.state.clone(),
            accounts.offer_vault_authority.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.redemption_vault_token_in_account.clone(),
            accounts.redemption_vault_token_out_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.user_token_in_account.clone(),
            accounts.user_token_out_account.clone(),
            accounts.prop_amm_proceeds_vault.clone(),
            accounts.prop_amm_proceeds_token_in_account.clone(),
            accounts.prop_amm_sell_fee_vault.clone(),
            accounts.prop_amm_sell_fee_token_in_account.clone(),
            accounts.mint_authority.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.user.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.main_offer.clone(),
            accounts.offer_vault_onyc_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_SWAP_SELL_IX_ACCOUNTS_LEN]>
for OpenSwapSellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_SWAP_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            prop_amm_pair_state: &arr[1],
            redemption_offer: &arr[2],
            state: &arr[3],
            offer_vault_authority: &arr[4],
            redemption_vault_authority: &arr[5],
            redemption_vault_token_in_account: &arr[6],
            redemption_vault_token_out_account: &arr[7],
            token_in_mint: &arr[8],
            token_in_program: &arr[9],
            token_out_mint: &arr[10],
            token_out_program: &arr[11],
            user_token_in_account: &arr[12],
            user_token_out_account: &arr[13],
            prop_amm_proceeds_vault: &arr[14],
            prop_amm_proceeds_token_in_account: &arr[15],
            prop_amm_sell_fee_vault: &arr[16],
            prop_amm_sell_fee_token_in_account: &arr[17],
            mint_authority: &arr[18],
            buffer_accounts_buffer_state: &arr[19],
            buffer_accounts_reserve_vault_onyc_account: &arr[20],
            buffer_accounts_management_fee_vault_onyc_account: &arr[21],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[22],
            market_stats: &arr[23],
            circulating_supply_excluded_balance: &arr[24],
            instructions_sysvar: &arr[25],
            user: &arr[26],
            associated_token_program: &arr[27],
            system_program: &arr[28],
            main_offer: &arr[29],
            offer_vault_onyc_account: &arr[30],
        }
    }
}
pub const OPEN_SWAP_SELL_IX_DISCM: [u8; 8usize] = [93, 206, 188, 72, 45, 138, 181, 71];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenSwapSellIxArgs {
    pub token_in_amount: u64,
    pub minimum_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenSwapSellIxData(pub OpenSwapSellIxArgs);
impl From<OpenSwapSellIxArgs> for OpenSwapSellIxData {
    fn from(args: OpenSwapSellIxArgs) -> Self {
        Self(args)
    }
}
impl OpenSwapSellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_SWAP_SELL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OpenSwapSellIxArgs {
                token_in_amount,
                minimum_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_SWAP_SELL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_swap_sell_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenSwapSellKeys,
    args: OpenSwapSellIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_SWAP_SELL_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenSwapSellIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_swap_sell_ix(
    keys: OpenSwapSellKeys,
    args: OpenSwapSellIxArgs,
) -> std::io::Result<Instruction> {
    open_swap_sell_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn open_swap_sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenSwapSellAccounts<'_, '_>,
    args: OpenSwapSellIxArgs,
) -> ProgramResult {
    let keys: OpenSwapSellKeys = accounts.into();
    let ix = open_swap_sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_swap_sell_invoke(
    accounts: OpenSwapSellAccounts<'_, '_>,
    args: OpenSwapSellIxArgs,
) -> ProgramResult {
    open_swap_sell_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn open_swap_sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenSwapSellAccounts<'_, '_>,
    args: OpenSwapSellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenSwapSellKeys = accounts.into();
    let ix = open_swap_sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_swap_sell_invoke_signed(
    accounts: OpenSwapSellAccounts<'_, '_>,
    args: OpenSwapSellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_swap_sell_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_swap_sell_verify_account_keys(
    accounts: OpenSwapSellAccounts<'_, '_>,
    keys: OpenSwapSellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.prop_amm_pair_state.key, keys.prop_amm_pair_state),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.state.key, keys.state),
        (*accounts.offer_vault_authority.key, keys.offer_vault_authority),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (
            *accounts.redemption_vault_token_in_account.key,
            keys.redemption_vault_token_in_account,
        ),
        (
            *accounts.redemption_vault_token_out_account.key,
            keys.redemption_vault_token_out_account,
        ),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.user_token_in_account.key, keys.user_token_in_account),
        (*accounts.user_token_out_account.key, keys.user_token_out_account),
        (*accounts.prop_amm_proceeds_vault.key, keys.prop_amm_proceeds_vault),
        (
            *accounts.prop_amm_proceeds_token_in_account.key,
            keys.prop_amm_proceeds_token_in_account,
        ),
        (*accounts.prop_amm_sell_fee_vault.key, keys.prop_amm_sell_fee_vault),
        (
            *accounts.prop_amm_sell_fee_token_in_account.key,
            keys.prop_amm_sell_fee_token_in_account,
        ),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.user.key, keys.user),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.main_offer.key, keys.main_offer),
        (*accounts.offer_vault_onyc_account.key, keys.offer_vault_onyc_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_swap_sell_verify_writable_privileges<'me, 'info>(
    accounts: OpenSwapSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.prop_amm_pair_state,
        accounts.redemption_vault_token_in_account,
        accounts.redemption_vault_token_out_account,
        accounts.token_in_mint,
        accounts.token_out_mint,
        accounts.user_token_in_account,
        accounts.user_token_out_account,
        accounts.prop_amm_proceeds_vault,
        accounts.prop_amm_proceeds_token_in_account,
        accounts.prop_amm_sell_fee_vault,
        accounts.prop_amm_sell_fee_token_in_account,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_swap_sell_verify_signer_privileges<'me, 'info>(
    accounts: OpenSwapSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_swap_sell_verify_account_privileges<'me, 'info>(
    accounts: OpenSwapSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_swap_sell_verify_writable_privileges(accounts)?;
    open_swap_sell_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PROPOSE_BOSS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ProposeBossAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProposeBossKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<ProposeBossAccounts<'_, '_>> for ProposeBossKeys {
    fn from(accounts: ProposeBossAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<ProposeBossKeys> for [AccountMeta; PROPOSE_BOSS_IX_ACCOUNTS_LEN] {
    fn from(keys: ProposeBossKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PROPOSE_BOSS_IX_ACCOUNTS_LEN]> for ProposeBossKeys {
    fn from(pubkeys: [Pubkey; PROPOSE_BOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<ProposeBossAccounts<'_, 'info>>
for [AccountInfo<'info>; PROPOSE_BOSS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ProposeBossAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PROPOSE_BOSS_IX_ACCOUNTS_LEN]>
for ProposeBossAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PROPOSE_BOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const PROPOSE_BOSS_IX_DISCM: [u8; 8usize] = [163, 199, 158, 47, 155, 78, 174, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProposeBossIxArgs {
    pub new_boss: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposeBossIxData(pub ProposeBossIxArgs);
impl From<ProposeBossIxArgs> for ProposeBossIxData {
    fn from(args: ProposeBossIxArgs) -> Self {
        Self(args)
    }
}
impl ProposeBossIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPOSE_BOSS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ProposeBossIxArgs { new_boss }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPOSE_BOSS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_boss, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn propose_boss_ix_with_program_id(
    program_id: Pubkey,
    keys: ProposeBossKeys,
    args: ProposeBossIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROPOSE_BOSS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ProposeBossIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn propose_boss_ix(
    keys: ProposeBossKeys,
    args: ProposeBossIxArgs,
) -> std::io::Result<Instruction> {
    propose_boss_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn propose_boss_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ProposeBossAccounts<'_, '_>,
    args: ProposeBossIxArgs,
) -> ProgramResult {
    let keys: ProposeBossKeys = accounts.into();
    let ix = propose_boss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn propose_boss_invoke(
    accounts: ProposeBossAccounts<'_, '_>,
    args: ProposeBossIxArgs,
) -> ProgramResult {
    propose_boss_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn propose_boss_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ProposeBossAccounts<'_, '_>,
    args: ProposeBossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ProposeBossKeys = accounts.into();
    let ix = propose_boss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn propose_boss_invoke_signed(
    accounts: ProposeBossAccounts<'_, '_>,
    args: ProposeBossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    propose_boss_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, args, seeds)
}
pub fn propose_boss_verify_account_keys(
    accounts: ProposeBossAccounts<'_, '_>,
    keys: ProposeBossKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn propose_boss_verify_writable_privileges<'me, 'info>(
    accounts: ProposeBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn propose_boss_verify_signer_privileges<'me, 'info>(
    accounts: ProposeBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn propose_boss_verify_account_privileges<'me, 'info>(
    accounts: ProposeBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    propose_boss_verify_writable_privileges(accounts)?;
    propose_boss_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const QUOTE_SWAP_BUY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct QuoteSwapBuyAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub prop_amm_pair_state: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuoteSwapBuyKeys {
    pub offer: Pubkey,
    pub prop_amm_pair_state: Pubkey,
    pub state: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
}
impl From<QuoteSwapBuyAccounts<'_, '_>> for QuoteSwapBuyKeys {
    fn from(accounts: QuoteSwapBuyAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            prop_amm_pair_state: *accounts.prop_amm_pair_state.key,
            state: *accounts.state.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
        }
    }
}
impl From<QuoteSwapBuyKeys> for [AccountMeta; QUOTE_SWAP_BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: QuoteSwapBuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.prop_amm_pair_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; QUOTE_SWAP_BUY_IX_ACCOUNTS_LEN]> for QuoteSwapBuyKeys {
    fn from(pubkeys: [Pubkey; QUOTE_SWAP_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            prop_amm_pair_state: pubkeys[1],
            state: pubkeys[2],
            token_in_mint: pubkeys[3],
            token_out_mint: pubkeys[4],
        }
    }
}
impl<'info> From<QuoteSwapBuyAccounts<'_, 'info>>
for [AccountInfo<'info>; QUOTE_SWAP_BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: QuoteSwapBuyAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.prop_amm_pair_state.clone(),
            accounts.state.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; QUOTE_SWAP_BUY_IX_ACCOUNTS_LEN]>
for QuoteSwapBuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; QUOTE_SWAP_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            prop_amm_pair_state: &arr[1],
            state: &arr[2],
            token_in_mint: &arr[3],
            token_out_mint: &arr[4],
        }
    }
}
pub const QUOTE_SWAP_BUY_IX_DISCM: [u8; 8usize] = [229, 148, 9, 48, 34, 165, 115, 166];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuoteSwapBuyIxArgs {
    pub token_in_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteSwapBuyIxData(pub QuoteSwapBuyIxArgs);
impl From<QuoteSwapBuyIxArgs> for QuoteSwapBuyIxData {
    fn from(args: QuoteSwapBuyIxArgs) -> Self {
        Self(args)
    }
}
impl QuoteSwapBuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_SWAP_BUY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(QuoteSwapBuyIxArgs {
                token_in_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_SWAP_BUY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn quote_swap_buy_ix_with_program_id(
    program_id: Pubkey,
    keys: QuoteSwapBuyKeys,
    args: QuoteSwapBuyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; QUOTE_SWAP_BUY_IX_ACCOUNTS_LEN] = keys.into();
    let data: QuoteSwapBuyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn quote_swap_buy_ix(
    keys: QuoteSwapBuyKeys,
    args: QuoteSwapBuyIxArgs,
) -> std::io::Result<Instruction> {
    quote_swap_buy_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn quote_swap_buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: QuoteSwapBuyAccounts<'_, '_>,
    args: QuoteSwapBuyIxArgs,
) -> ProgramResult {
    let keys: QuoteSwapBuyKeys = accounts.into();
    let ix = quote_swap_buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn quote_swap_buy_invoke(
    accounts: QuoteSwapBuyAccounts<'_, '_>,
    args: QuoteSwapBuyIxArgs,
) -> ProgramResult {
    quote_swap_buy_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn quote_swap_buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: QuoteSwapBuyAccounts<'_, '_>,
    args: QuoteSwapBuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: QuoteSwapBuyKeys = accounts.into();
    let ix = quote_swap_buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn quote_swap_buy_invoke_signed(
    accounts: QuoteSwapBuyAccounts<'_, '_>,
    args: QuoteSwapBuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    quote_swap_buy_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn quote_swap_buy_verify_account_keys(
    accounts: QuoteSwapBuyAccounts<'_, '_>,
    keys: QuoteSwapBuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.prop_amm_pair_state.key, keys.prop_amm_pair_state),
        (*accounts.state.key, keys.state),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const QUOTE_SWAP_SELL_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct QuoteSwapSellAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub prop_amm_pair_state: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub redemption_vault_token_out_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuoteSwapSellKeys {
    pub offer: Pubkey,
    pub prop_amm_pair_state: Pubkey,
    pub redemption_offer: Pubkey,
    pub state: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub redemption_vault_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub market_stats: Pubkey,
}
impl From<QuoteSwapSellAccounts<'_, '_>> for QuoteSwapSellKeys {
    fn from(accounts: QuoteSwapSellAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            prop_amm_pair_state: *accounts.prop_amm_pair_state.key,
            redemption_offer: *accounts.redemption_offer.key,
            state: *accounts.state.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            redemption_vault_token_out_account: *accounts
                .redemption_vault_token_out_account
                .key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            market_stats: *accounts.market_stats.key,
        }
    }
}
impl From<QuoteSwapSellKeys> for [AccountMeta; QUOTE_SWAP_SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: QuoteSwapSellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.prop_amm_pair_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_token_out_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; QUOTE_SWAP_SELL_IX_ACCOUNTS_LEN]> for QuoteSwapSellKeys {
    fn from(pubkeys: [Pubkey; QUOTE_SWAP_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            prop_amm_pair_state: pubkeys[1],
            redemption_offer: pubkeys[2],
            state: pubkeys[3],
            redemption_vault_authority: pubkeys[4],
            redemption_vault_token_out_account: pubkeys[5],
            token_in_mint: pubkeys[6],
            token_out_mint: pubkeys[7],
            token_out_program: pubkeys[8],
            market_stats: pubkeys[9],
        }
    }
}
impl<'info> From<QuoteSwapSellAccounts<'_, 'info>>
for [AccountInfo<'info>; QUOTE_SWAP_SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: QuoteSwapSellAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.prop_amm_pair_state.clone(),
            accounts.redemption_offer.clone(),
            accounts.state.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.redemption_vault_token_out_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.market_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; QUOTE_SWAP_SELL_IX_ACCOUNTS_LEN]>
for QuoteSwapSellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; QUOTE_SWAP_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            prop_amm_pair_state: &arr[1],
            redemption_offer: &arr[2],
            state: &arr[3],
            redemption_vault_authority: &arr[4],
            redemption_vault_token_out_account: &arr[5],
            token_in_mint: &arr[6],
            token_out_mint: &arr[7],
            token_out_program: &arr[8],
            market_stats: &arr[9],
        }
    }
}
pub const QUOTE_SWAP_SELL_IX_DISCM: [u8; 8usize] = [198, 1, 48, 226, 172, 136, 51, 251];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuoteSwapSellIxArgs {
    pub token_in_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteSwapSellIxData(pub QuoteSwapSellIxArgs);
impl From<QuoteSwapSellIxArgs> for QuoteSwapSellIxData {
    fn from(args: QuoteSwapSellIxArgs) -> Self {
        Self(args)
    }
}
impl QuoteSwapSellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_SWAP_SELL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(QuoteSwapSellIxArgs {
                token_in_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_SWAP_SELL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn quote_swap_sell_ix_with_program_id(
    program_id: Pubkey,
    keys: QuoteSwapSellKeys,
    args: QuoteSwapSellIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; QUOTE_SWAP_SELL_IX_ACCOUNTS_LEN] = keys.into();
    let data: QuoteSwapSellIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn quote_swap_sell_ix(
    keys: QuoteSwapSellKeys,
    args: QuoteSwapSellIxArgs,
) -> std::io::Result<Instruction> {
    quote_swap_sell_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn quote_swap_sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: QuoteSwapSellAccounts<'_, '_>,
    args: QuoteSwapSellIxArgs,
) -> ProgramResult {
    let keys: QuoteSwapSellKeys = accounts.into();
    let ix = quote_swap_sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn quote_swap_sell_invoke(
    accounts: QuoteSwapSellAccounts<'_, '_>,
    args: QuoteSwapSellIxArgs,
) -> ProgramResult {
    quote_swap_sell_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn quote_swap_sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: QuoteSwapSellAccounts<'_, '_>,
    args: QuoteSwapSellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: QuoteSwapSellKeys = accounts.into();
    let ix = quote_swap_sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn quote_swap_sell_invoke_signed(
    accounts: QuoteSwapSellAccounts<'_, '_>,
    args: QuoteSwapSellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    quote_swap_sell_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn quote_swap_sell_verify_account_keys(
    accounts: QuoteSwapSellAccounts<'_, '_>,
    keys: QuoteSwapSellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.prop_amm_pair_state.key, keys.prop_amm_pair_state),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.state.key, keys.state),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (
            *accounts.redemption_vault_token_out_account.key,
            keys.redemption_vault_token_out_account,
        ),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.market_stats.key, keys.market_stats),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const REDEMPTION_VAULT_DEPOSIT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RedemptionVaultDepositAccounts<'me, 'info> {
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub depositor_token_account: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub depositor: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedemptionVaultDepositKeys {
    pub redemption_vault_authority: Pubkey,
    pub token_mint: Pubkey,
    pub depositor_token_account: Pubkey,
    pub vault_token_account: Pubkey,
    pub depositor: Pubkey,
    pub state: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RedemptionVaultDepositAccounts<'_, '_>> for RedemptionVaultDepositKeys {
    fn from(accounts: RedemptionVaultDepositAccounts) -> Self {
        Self {
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            token_mint: *accounts.token_mint.key,
            depositor_token_account: *accounts.depositor_token_account.key,
            vault_token_account: *accounts.vault_token_account.key,
            depositor: *accounts.depositor.key,
            state: *accounts.state.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RedemptionVaultDepositKeys>
for [AccountMeta; REDEMPTION_VAULT_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: RedemptionVaultDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.depositor,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
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
impl From<[Pubkey; REDEMPTION_VAULT_DEPOSIT_IX_ACCOUNTS_LEN]>
for RedemptionVaultDepositKeys {
    fn from(pubkeys: [Pubkey; REDEMPTION_VAULT_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            redemption_vault_authority: pubkeys[0],
            token_mint: pubkeys[1],
            depositor_token_account: pubkeys[2],
            vault_token_account: pubkeys[3],
            depositor: pubkeys[4],
            state: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<RedemptionVaultDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEMPTION_VAULT_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedemptionVaultDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.redemption_vault_authority.clone(),
            accounts.token_mint.clone(),
            accounts.depositor_token_account.clone(),
            accounts.vault_token_account.clone(),
            accounts.depositor.clone(),
            accounts.state.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REDEMPTION_VAULT_DEPOSIT_IX_ACCOUNTS_LEN]>
for RedemptionVaultDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEMPTION_VAULT_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            redemption_vault_authority: &arr[0],
            token_mint: &arr[1],
            depositor_token_account: &arr[2],
            vault_token_account: &arr[3],
            depositor: &arr[4],
            state: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const REDEMPTION_VAULT_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    67, 104, 107, 131, 141, 2, 140, 122,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionVaultDepositIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionVaultDepositIxData(pub RedemptionVaultDepositIxArgs);
impl From<RedemptionVaultDepositIxArgs> for RedemptionVaultDepositIxData {
    fn from(args: RedemptionVaultDepositIxArgs) -> Self {
        Self(args)
    }
}
impl RedemptionVaultDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_VAULT_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RedemptionVaultDepositIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_VAULT_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redemption_vault_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: RedemptionVaultDepositKeys,
    args: RedemptionVaultDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEMPTION_VAULT_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedemptionVaultDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redemption_vault_deposit_ix(
    keys: RedemptionVaultDepositKeys,
    args: RedemptionVaultDepositIxArgs,
) -> std::io::Result<Instruction> {
    redemption_vault_deposit_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn redemption_vault_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedemptionVaultDepositAccounts<'_, '_>,
    args: RedemptionVaultDepositIxArgs,
) -> ProgramResult {
    let keys: RedemptionVaultDepositKeys = accounts.into();
    let ix = redemption_vault_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redemption_vault_deposit_invoke(
    accounts: RedemptionVaultDepositAccounts<'_, '_>,
    args: RedemptionVaultDepositIxArgs,
) -> ProgramResult {
    redemption_vault_deposit_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn redemption_vault_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedemptionVaultDepositAccounts<'_, '_>,
    args: RedemptionVaultDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedemptionVaultDepositKeys = accounts.into();
    let ix = redemption_vault_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redemption_vault_deposit_invoke_signed(
    accounts: RedemptionVaultDepositAccounts<'_, '_>,
    args: RedemptionVaultDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redemption_vault_deposit_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redemption_vault_deposit_verify_account_keys(
    accounts: RedemptionVaultDepositAccounts<'_, '_>,
    keys: RedemptionVaultDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.depositor_token_account.key, keys.depositor_token_account),
        (*accounts.vault_token_account.key, keys.vault_token_account),
        (*accounts.depositor.key, keys.depositor),
        (*accounts.state.key, keys.state),
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
pub fn redemption_vault_deposit_verify_writable_privileges<'me, 'info>(
    accounts: RedemptionVaultDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.depositor_token_account,
        accounts.vault_token_account,
        accounts.depositor,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redemption_vault_deposit_verify_signer_privileges<'me, 'info>(
    accounts: RedemptionVaultDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.depositor] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redemption_vault_deposit_verify_account_privileges<'me, 'info>(
    accounts: RedemptionVaultDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redemption_vault_deposit_verify_writable_privileges(accounts)?;
    redemption_vault_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEMPTION_VAULT_WITHDRAW_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RedemptionVaultWithdrawAccounts<'me, 'info> {
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub boss_token_account: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedemptionVaultWithdrawKeys {
    pub redemption_vault_authority: Pubkey,
    pub token_mint: Pubkey,
    pub boss_token_account: Pubkey,
    pub vault_token_account: Pubkey,
    pub boss: Pubkey,
    pub state: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<RedemptionVaultWithdrawAccounts<'_, '_>> for RedemptionVaultWithdrawKeys {
    fn from(accounts: RedemptionVaultWithdrawAccounts) -> Self {
        Self {
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            token_mint: *accounts.token_mint.key,
            boss_token_account: *accounts.boss_token_account.key,
            vault_token_account: *accounts.vault_token_account.key,
            boss: *accounts.boss.key,
            state: *accounts.state.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RedemptionVaultWithdrawKeys>
for [AccountMeta; REDEMPTION_VAULT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: RedemptionVaultWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
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
impl From<[Pubkey; REDEMPTION_VAULT_WITHDRAW_IX_ACCOUNTS_LEN]>
for RedemptionVaultWithdrawKeys {
    fn from(pubkeys: [Pubkey; REDEMPTION_VAULT_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            redemption_vault_authority: pubkeys[0],
            token_mint: pubkeys[1],
            boss_token_account: pubkeys[2],
            vault_token_account: pubkeys[3],
            boss: pubkeys[4],
            state: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<RedemptionVaultWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEMPTION_VAULT_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedemptionVaultWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.redemption_vault_authority.clone(),
            accounts.token_mint.clone(),
            accounts.boss_token_account.clone(),
            accounts.vault_token_account.clone(),
            accounts.boss.clone(),
            accounts.state.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REDEMPTION_VAULT_WITHDRAW_IX_ACCOUNTS_LEN]>
for RedemptionVaultWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEMPTION_VAULT_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            redemption_vault_authority: &arr[0],
            token_mint: &arr[1],
            boss_token_account: &arr[2],
            vault_token_account: &arr[3],
            boss: &arr[4],
            state: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const REDEMPTION_VAULT_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    48, 214, 145, 15, 168, 122, 39, 48,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionVaultWithdrawIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionVaultWithdrawIxData(pub RedemptionVaultWithdrawIxArgs);
impl From<RedemptionVaultWithdrawIxArgs> for RedemptionVaultWithdrawIxData {
    fn from(args: RedemptionVaultWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl RedemptionVaultWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_VAULT_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RedemptionVaultWithdrawIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_VAULT_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redemption_vault_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: RedemptionVaultWithdrawKeys,
    args: RedemptionVaultWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEMPTION_VAULT_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedemptionVaultWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redemption_vault_withdraw_ix(
    keys: RedemptionVaultWithdrawKeys,
    args: RedemptionVaultWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    redemption_vault_withdraw_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn redemption_vault_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedemptionVaultWithdrawAccounts<'_, '_>,
    args: RedemptionVaultWithdrawIxArgs,
) -> ProgramResult {
    let keys: RedemptionVaultWithdrawKeys = accounts.into();
    let ix = redemption_vault_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redemption_vault_withdraw_invoke(
    accounts: RedemptionVaultWithdrawAccounts<'_, '_>,
    args: RedemptionVaultWithdrawIxArgs,
) -> ProgramResult {
    redemption_vault_withdraw_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn redemption_vault_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedemptionVaultWithdrawAccounts<'_, '_>,
    args: RedemptionVaultWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedemptionVaultWithdrawKeys = accounts.into();
    let ix = redemption_vault_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redemption_vault_withdraw_invoke_signed(
    accounts: RedemptionVaultWithdrawAccounts<'_, '_>,
    args: RedemptionVaultWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redemption_vault_withdraw_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redemption_vault_withdraw_verify_account_keys(
    accounts: RedemptionVaultWithdrawAccounts<'_, '_>,
    keys: RedemptionVaultWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.boss_token_account.key, keys.boss_token_account),
        (*accounts.vault_token_account.key, keys.vault_token_account),
        (*accounts.boss.key, keys.boss),
        (*accounts.state.key, keys.state),
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
pub fn redemption_vault_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: RedemptionVaultWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.boss_token_account,
        accounts.vault_token_account,
        accounts.boss,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redemption_vault_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: RedemptionVaultWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redemption_vault_withdraw_verify_account_privileges<'me, 'info>(
    accounts: RedemptionVaultWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redemption_vault_withdraw_verify_writable_privileges(accounts)?;
    redemption_vault_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_MARKET_STATS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RefreshMarketStatsAccounts<'me, 'info> {
    pub main_offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshMarketStatsKeys {
    pub main_offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub state: Pubkey,
    pub onyc_mint: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
    pub market_stats: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
}
impl From<RefreshMarketStatsAccounts<'_, '_>> for RefreshMarketStatsKeys {
    fn from(accounts: RefreshMarketStatsAccounts) -> Self {
        Self {
            main_offer: *accounts.main_offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            state: *accounts.state.key,
            onyc_mint: *accounts.onyc_mint.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
            market_stats: *accounts.market_stats.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RefreshMarketStatsKeys>
for [AccountMeta; REFRESH_MARKET_STATS_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshMarketStatsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
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
impl From<[Pubkey; REFRESH_MARKET_STATS_IX_ACCOUNTS_LEN]> for RefreshMarketStatsKeys {
    fn from(pubkeys: [Pubkey; REFRESH_MARKET_STATS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            main_offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            state: pubkeys[2],
            onyc_mint: pubkeys[3],
            circulating_supply_excluded_balance: pubkeys[4],
            market_stats: pubkeys[5],
            signer: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<RefreshMarketStatsAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_MARKET_STATS_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshMarketStatsAccounts<'_, 'info>) -> Self {
        [
            accounts.main_offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.state.clone(),
            accounts.onyc_mint.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
            accounts.market_stats.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFRESH_MARKET_STATS_IX_ACCOUNTS_LEN]>
for RefreshMarketStatsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REFRESH_MARKET_STATS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            main_offer: &arr[0],
            token_in_mint: &arr[1],
            state: &arr[2],
            onyc_mint: &arr[3],
            circulating_supply_excluded_balance: &arr[4],
            market_stats: &arr[5],
            signer: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const REFRESH_MARKET_STATS_IX_DISCM: [u8; 8usize] = [
    51, 221, 140, 112, 205, 53, 22, 233,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshMarketStatsIxData;
impl RefreshMarketStatsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_MARKET_STATS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_MARKET_STATS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_market_stats_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshMarketStatsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_MARKET_STATS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefreshMarketStatsIxData.try_to_vec()?,
    })
}
pub fn refresh_market_stats_ix(
    keys: RefreshMarketStatsKeys,
) -> std::io::Result<Instruction> {
    refresh_market_stats_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn refresh_market_stats_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshMarketStatsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshMarketStatsKeys = accounts.into();
    let ix = refresh_market_stats_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_market_stats_invoke(
    accounts: RefreshMarketStatsAccounts<'_, '_>,
) -> ProgramResult {
    refresh_market_stats_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn refresh_market_stats_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshMarketStatsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshMarketStatsKeys = accounts.into();
    let ix = refresh_market_stats_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_market_stats_invoke_signed(
    accounts: RefreshMarketStatsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_market_stats_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn refresh_market_stats_verify_account_keys(
    accounts: RefreshMarketStatsAccounts<'_, '_>,
    keys: RefreshMarketStatsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.main_offer.key, keys.main_offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.state.key, keys.state),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
        (*accounts.market_stats.key, keys.market_stats),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_market_stats_verify_writable_privileges<'me, 'info>(
    accounts: RefreshMarketStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market_stats, accounts.signer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_market_stats_verify_signer_privileges<'me, 'info>(
    accounts: RefreshMarketStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn refresh_market_stats_verify_account_privileges<'me, 'info>(
    accounts: RefreshMarketStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_market_stats_verify_writable_privileges(accounts)?;
    refresh_market_stats_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RemoveAdminAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAdminKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<RemoveAdminAccounts<'_, '_>> for RemoveAdminKeys {
    fn from(accounts: RemoveAdminAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<RemoveAdminKeys> for [AccountMeta; REMOVE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_ADMIN_IX_ACCOUNTS_LEN]> for RemoveAdminKeys {
    fn from(pubkeys: [Pubkey; REMOVE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<RemoveAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAdminAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_ADMIN_IX_ACCOUNTS_LEN]>
for RemoveAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const REMOVE_ADMIN_IX_DISCM: [u8; 8usize] = [74, 202, 71, 106, 252, 31, 72, 183];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveAdminIxArgs {
    pub admin_to_remove: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAdminIxData(pub RemoveAdminIxArgs);
impl From<RemoveAdminIxArgs> for RemoveAdminIxData {
    fn from(args: RemoveAdminIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let admin_to_remove: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveAdminIxArgs {
                admin_to_remove,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.admin_to_remove, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveAdminKeys,
    args: RemoveAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_admin_ix(
    keys: RemoveAdminKeys,
    args: RemoveAdminIxArgs,
) -> std::io::Result<Instruction> {
    remove_admin_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn remove_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAdminAccounts<'_, '_>,
    args: RemoveAdminIxArgs,
) -> ProgramResult {
    let keys: RemoveAdminKeys = accounts.into();
    let ix = remove_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_admin_invoke(
    accounts: RemoveAdminAccounts<'_, '_>,
    args: RemoveAdminIxArgs,
) -> ProgramResult {
    remove_admin_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn remove_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAdminAccounts<'_, '_>,
    args: RemoveAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAdminKeys = accounts.into();
    let ix = remove_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_admin_invoke_signed(
    accounts: RemoveAdminAccounts<'_, '_>,
    args: RemoveAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_admin_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, args, seeds)
}
pub fn remove_admin_verify_account_keys(
    accounts: RemoveAdminAccounts<'_, '_>,
    keys: RemoveAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_admin_verify_writable_privileges<'me, 'info>(
    accounts: RemoveAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_admin_verify_signer_privileges<'me, 'info>(
    accounts: RemoveAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_admin_verify_account_privileges<'me, 'info>(
    accounts: RemoveAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_admin_verify_writable_privileges(accounts)?;
    remove_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_APPROVER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RemoveApproverAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveApproverKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<RemoveApproverAccounts<'_, '_>> for RemoveApproverKeys {
    fn from(accounts: RemoveApproverAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<RemoveApproverKeys> for [AccountMeta; REMOVE_APPROVER_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveApproverKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_APPROVER_IX_ACCOUNTS_LEN]> for RemoveApproverKeys {
    fn from(pubkeys: [Pubkey; REMOVE_APPROVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<RemoveApproverAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_APPROVER_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveApproverAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_APPROVER_IX_ACCOUNTS_LEN]>
for RemoveApproverAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_APPROVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const REMOVE_APPROVER_IX_DISCM: [u8; 8usize] = [214, 72, 133, 48, 50, 58, 227, 224];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveApproverIxArgs {
    pub approver: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveApproverIxData(pub RemoveApproverIxArgs);
impl From<RemoveApproverIxArgs> for RemoveApproverIxData {
    fn from(args: RemoveApproverIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveApproverIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_APPROVER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let approver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RemoveApproverIxArgs { approver }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_APPROVER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.approver, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_approver_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveApproverKeys,
    args: RemoveApproverIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_APPROVER_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveApproverIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_approver_ix(
    keys: RemoveApproverKeys,
    args: RemoveApproverIxArgs,
) -> std::io::Result<Instruction> {
    remove_approver_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn remove_approver_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveApproverAccounts<'_, '_>,
    args: RemoveApproverIxArgs,
) -> ProgramResult {
    let keys: RemoveApproverKeys = accounts.into();
    let ix = remove_approver_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_approver_invoke(
    accounts: RemoveApproverAccounts<'_, '_>,
    args: RemoveApproverIxArgs,
) -> ProgramResult {
    remove_approver_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn remove_approver_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveApproverAccounts<'_, '_>,
    args: RemoveApproverIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveApproverKeys = accounts.into();
    let ix = remove_approver_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_approver_invoke_signed(
    accounts: RemoveApproverAccounts<'_, '_>,
    args: RemoveApproverIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_approver_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_approver_verify_account_keys(
    accounts: RemoveApproverAccounts<'_, '_>,
    keys: RemoveApproverKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_approver_verify_writable_privileges<'me, 'info>(
    accounts: RemoveApproverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_approver_verify_signer_privileges<'me, 'info>(
    accounts: RemoveApproverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_approver_verify_account_privileges<'me, 'info>(
    accounts: RemoveApproverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_approver_verify_writable_privileges(accounts)?;
    remove_approver_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_BUFFER_FEE_CONFIG_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SetBufferFeeConfigAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub offer_vault_authority: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetBufferFeeConfigKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
    pub main_offer: Pubkey,
    pub onyc_mint: Pubkey,
    pub offer_vault_authority: Pubkey,
    pub mint_authority: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
}
impl From<SetBufferFeeConfigAccounts<'_, '_>> for SetBufferFeeConfigKeys {
    fn from(accounts: SetBufferFeeConfigAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            main_offer: *accounts.main_offer.key,
            onyc_mint: *accounts.onyc_mint.key,
            offer_vault_authority: *accounts.offer_vault_authority.key,
            mint_authority: *accounts.mint_authority.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
        }
    }
}
impl From<SetBufferFeeConfigKeys>
for [AccountMeta; SET_BUFFER_FEE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: SetBufferFeeConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
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
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_BUFFER_FEE_CONFIG_IX_ACCOUNTS_LEN]> for SetBufferFeeConfigKeys {
    fn from(pubkeys: [Pubkey; SET_BUFFER_FEE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
            main_offer: pubkeys[2],
            onyc_mint: pubkeys[3],
            offer_vault_authority: pubkeys[4],
            mint_authority: pubkeys[5],
            buffer_accounts_buffer_state: pubkeys[6],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[7],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[8],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[9],
            token_program: pubkeys[10],
            system_program: pubkeys[11],
            market_stats: pubkeys[12],
            circulating_supply_excluded_balance: pubkeys[13],
        }
    }
}
impl<'info> From<SetBufferFeeConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_BUFFER_FEE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetBufferFeeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.main_offer.clone(),
            accounts.onyc_mint.clone(),
            accounts.offer_vault_authority.clone(),
            accounts.mint_authority.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_BUFFER_FEE_CONFIG_IX_ACCOUNTS_LEN]>
for SetBufferFeeConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_BUFFER_FEE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
            main_offer: &arr[2],
            onyc_mint: &arr[3],
            offer_vault_authority: &arr[4],
            mint_authority: &arr[5],
            buffer_accounts_buffer_state: &arr[6],
            buffer_accounts_reserve_vault_onyc_account: &arr[7],
            buffer_accounts_management_fee_vault_onyc_account: &arr[8],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[9],
            token_program: &arr[10],
            system_program: &arr[11],
            market_stats: &arr[12],
            circulating_supply_excluded_balance: &arr[13],
        }
    }
}
pub const SET_BUFFER_FEE_CONFIG_IX_DISCM: [u8; 8usize] = [
    68, 233, 51, 70, 228, 167, 74, 147,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetBufferFeeConfigIxArgs {
    pub management_fee_basis_points: u16,
    pub performance_fee_basis_points: u16,
    pub performance_fee_high_watermark_enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetBufferFeeConfigIxData(pub SetBufferFeeConfigIxArgs);
impl From<SetBufferFeeConfigIxArgs> for SetBufferFeeConfigIxData {
    fn from(args: SetBufferFeeConfigIxArgs) -> Self {
        Self(args)
    }
}
impl SetBufferFeeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_BUFFER_FEE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let management_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_high_watermark_enabled: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SetBufferFeeConfigIxArgs {
                management_fee_basis_points,
                performance_fee_basis_points,
                performance_fee_high_watermark_enabled,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_BUFFER_FEE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.management_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.performance_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.performance_fee_high_watermark_enabled,
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
pub fn set_buffer_fee_config_ix_with_program_id(
    program_id: Pubkey,
    keys: SetBufferFeeConfigKeys,
    args: SetBufferFeeConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_BUFFER_FEE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetBufferFeeConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_buffer_fee_config_ix(
    keys: SetBufferFeeConfigKeys,
    args: SetBufferFeeConfigIxArgs,
) -> std::io::Result<Instruction> {
    set_buffer_fee_config_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn set_buffer_fee_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetBufferFeeConfigAccounts<'_, '_>,
    args: SetBufferFeeConfigIxArgs,
) -> ProgramResult {
    let keys: SetBufferFeeConfigKeys = accounts.into();
    let ix = set_buffer_fee_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_buffer_fee_config_invoke(
    accounts: SetBufferFeeConfigAccounts<'_, '_>,
    args: SetBufferFeeConfigIxArgs,
) -> ProgramResult {
    set_buffer_fee_config_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn set_buffer_fee_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetBufferFeeConfigAccounts<'_, '_>,
    args: SetBufferFeeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetBufferFeeConfigKeys = accounts.into();
    let ix = set_buffer_fee_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_buffer_fee_config_invoke_signed(
    accounts: SetBufferFeeConfigAccounts<'_, '_>,
    args: SetBufferFeeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_buffer_fee_config_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_buffer_fee_config_verify_account_keys(
    accounts: SetBufferFeeConfigAccounts<'_, '_>,
    keys: SetBufferFeeConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.main_offer.key, keys.main_offer),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.offer_vault_authority.key, keys.offer_vault_authority),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_buffer_fee_config_verify_writable_privileges<'me, 'info>(
    accounts: SetBufferFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.boss,
        accounts.onyc_mint,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_buffer_fee_config_verify_signer_privileges<'me, 'info>(
    accounts: SetBufferFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_buffer_fee_config_verify_account_privileges<'me, 'info>(
    accounts: SetBufferFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_buffer_fee_config_verify_writable_privileges(accounts)?;
    set_buffer_fee_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_BUFFER_GROSS_APR_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SetBufferGrossAprAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub offer_vault_authority: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetBufferGrossAprKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
    pub main_offer: Pubkey,
    pub onyc_mint: Pubkey,
    pub offer_vault_authority: Pubkey,
    pub mint_authority: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
}
impl From<SetBufferGrossAprAccounts<'_, '_>> for SetBufferGrossAprKeys {
    fn from(accounts: SetBufferGrossAprAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            main_offer: *accounts.main_offer.key,
            onyc_mint: *accounts.onyc_mint.key,
            offer_vault_authority: *accounts.offer_vault_authority.key,
            mint_authority: *accounts.mint_authority.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
        }
    }
}
impl From<SetBufferGrossAprKeys>
for [AccountMeta; SET_BUFFER_GROSS_APR_IX_ACCOUNTS_LEN] {
    fn from(keys: SetBufferGrossAprKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
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
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_BUFFER_GROSS_APR_IX_ACCOUNTS_LEN]> for SetBufferGrossAprKeys {
    fn from(pubkeys: [Pubkey; SET_BUFFER_GROSS_APR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
            main_offer: pubkeys[2],
            onyc_mint: pubkeys[3],
            offer_vault_authority: pubkeys[4],
            mint_authority: pubkeys[5],
            buffer_accounts_buffer_state: pubkeys[6],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[7],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[8],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[9],
            token_program: pubkeys[10],
            system_program: pubkeys[11],
            market_stats: pubkeys[12],
            circulating_supply_excluded_balance: pubkeys[13],
        }
    }
}
impl<'info> From<SetBufferGrossAprAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_BUFFER_GROSS_APR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetBufferGrossAprAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.main_offer.clone(),
            accounts.onyc_mint.clone(),
            accounts.offer_vault_authority.clone(),
            accounts.mint_authority.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_BUFFER_GROSS_APR_IX_ACCOUNTS_LEN]>
for SetBufferGrossAprAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_BUFFER_GROSS_APR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
            main_offer: &arr[2],
            onyc_mint: &arr[3],
            offer_vault_authority: &arr[4],
            mint_authority: &arr[5],
            buffer_accounts_buffer_state: &arr[6],
            buffer_accounts_reserve_vault_onyc_account: &arr[7],
            buffer_accounts_management_fee_vault_onyc_account: &arr[8],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[9],
            token_program: &arr[10],
            system_program: &arr[11],
            market_stats: &arr[12],
            circulating_supply_excluded_balance: &arr[13],
        }
    }
}
pub const SET_BUFFER_GROSS_APR_IX_DISCM: [u8; 8usize] = [
    245, 49, 142, 190, 30, 123, 184, 11,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetBufferGrossAprIxArgs {
    pub gross_yield: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetBufferGrossAprIxData(pub SetBufferGrossAprIxArgs);
impl From<SetBufferGrossAprIxArgs> for SetBufferGrossAprIxData {
    fn from(args: SetBufferGrossAprIxArgs) -> Self {
        Self(args)
    }
}
impl SetBufferGrossAprIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_BUFFER_GROSS_APR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let gross_yield: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetBufferGrossAprIxArgs {
                gross_yield,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_BUFFER_GROSS_APR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.gross_yield, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_buffer_gross_apr_ix_with_program_id(
    program_id: Pubkey,
    keys: SetBufferGrossAprKeys,
    args: SetBufferGrossAprIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_BUFFER_GROSS_APR_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetBufferGrossAprIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_buffer_gross_apr_ix(
    keys: SetBufferGrossAprKeys,
    args: SetBufferGrossAprIxArgs,
) -> std::io::Result<Instruction> {
    set_buffer_gross_apr_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn set_buffer_gross_apr_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetBufferGrossAprAccounts<'_, '_>,
    args: SetBufferGrossAprIxArgs,
) -> ProgramResult {
    let keys: SetBufferGrossAprKeys = accounts.into();
    let ix = set_buffer_gross_apr_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_buffer_gross_apr_invoke(
    accounts: SetBufferGrossAprAccounts<'_, '_>,
    args: SetBufferGrossAprIxArgs,
) -> ProgramResult {
    set_buffer_gross_apr_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn set_buffer_gross_apr_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetBufferGrossAprAccounts<'_, '_>,
    args: SetBufferGrossAprIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetBufferGrossAprKeys = accounts.into();
    let ix = set_buffer_gross_apr_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_buffer_gross_apr_invoke_signed(
    accounts: SetBufferGrossAprAccounts<'_, '_>,
    args: SetBufferGrossAprIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_buffer_gross_apr_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_buffer_gross_apr_verify_account_keys(
    accounts: SetBufferGrossAprAccounts<'_, '_>,
    keys: SetBufferGrossAprKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.main_offer.key, keys.main_offer),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.offer_vault_authority.key, keys.offer_vault_authority),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_buffer_gross_apr_verify_writable_privileges<'me, 'info>(
    accounts: SetBufferGrossAprAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.boss,
        accounts.onyc_mint,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_buffer_gross_apr_verify_signer_privileges<'me, 'info>(
    accounts: SetBufferGrossAprAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_buffer_gross_apr_verify_account_privileges<'me, 'info>(
    accounts: SetBufferGrossAprAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_buffer_gross_apr_verify_writable_privileges(accounts)?;
    set_buffer_gross_apr_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetCirculatingSupplyExcludedAccountsAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub excluded_accounts: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetCirculatingSupplyExcludedAccountsKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
    pub excluded_accounts: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetCirculatingSupplyExcludedAccountsAccounts<'_, '_>>
for SetCirculatingSupplyExcludedAccountsKeys {
    fn from(accounts: SetCirculatingSupplyExcludedAccountsAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            excluded_accounts: *accounts.excluded_accounts.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetCirculatingSupplyExcludedAccountsKeys>
for [AccountMeta; SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetCirculatingSupplyExcludedAccountsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.excluded_accounts,
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
impl From<[Pubkey; SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_ACCOUNTS_LEN]>
for SetCirculatingSupplyExcludedAccountsKeys {
    fn from(
        pubkeys: [Pubkey; SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
            excluded_accounts: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<SetCirculatingSupplyExcludedAccountsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetCirculatingSupplyExcludedAccountsAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.excluded_accounts.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_ACCOUNTS_LEN],
> for SetCirculatingSupplyExcludedAccountsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
            excluded_accounts: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_DISCM: [u8; 8usize] = [
    109, 247, 233, 248, 196, 107, 17, 66,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetCirculatingSupplyExcludedAccountsIxArgs {
    pub owners: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetCirculatingSupplyExcludedAccountsIxData(
    pub SetCirculatingSupplyExcludedAccountsIxArgs,
);
impl From<SetCirculatingSupplyExcludedAccountsIxArgs>
for SetCirculatingSupplyExcludedAccountsIxData {
    fn from(args: SetCirculatingSupplyExcludedAccountsIxArgs) -> Self {
        Self(args)
    }
}
impl SetCirculatingSupplyExcludedAccountsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let owners: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetCirculatingSupplyExcludedAccountsIxArgs {
                owners,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.owners, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_circulating_supply_excluded_accounts_ix_with_program_id(
    program_id: Pubkey,
    keys: SetCirculatingSupplyExcludedAccountsKeys,
    args: SetCirculatingSupplyExcludedAccountsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetCirculatingSupplyExcludedAccountsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_circulating_supply_excluded_accounts_ix(
    keys: SetCirculatingSupplyExcludedAccountsKeys,
    args: SetCirculatingSupplyExcludedAccountsIxArgs,
) -> std::io::Result<Instruction> {
    set_circulating_supply_excluded_accounts_ix_with_program_id(
        ONREAPP_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn set_circulating_supply_excluded_accounts_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetCirculatingSupplyExcludedAccountsAccounts<'_, '_>,
    args: SetCirculatingSupplyExcludedAccountsIxArgs,
) -> ProgramResult {
    let keys: SetCirculatingSupplyExcludedAccountsKeys = accounts.into();
    let ix = set_circulating_supply_excluded_accounts_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_circulating_supply_excluded_accounts_invoke(
    accounts: SetCirculatingSupplyExcludedAccountsAccounts<'_, '_>,
    args: SetCirculatingSupplyExcludedAccountsIxArgs,
) -> ProgramResult {
    set_circulating_supply_excluded_accounts_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_circulating_supply_excluded_accounts_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetCirculatingSupplyExcludedAccountsAccounts<'_, '_>,
    args: SetCirculatingSupplyExcludedAccountsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetCirculatingSupplyExcludedAccountsKeys = accounts.into();
    let ix = set_circulating_supply_excluded_accounts_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_circulating_supply_excluded_accounts_invoke_signed(
    accounts: SetCirculatingSupplyExcludedAccountsAccounts<'_, '_>,
    args: SetCirculatingSupplyExcludedAccountsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_circulating_supply_excluded_accounts_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_circulating_supply_excluded_accounts_verify_account_keys(
    accounts: SetCirculatingSupplyExcludedAccountsAccounts<'_, '_>,
    keys: SetCirculatingSupplyExcludedAccountsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.excluded_accounts.key, keys.excluded_accounts),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_circulating_supply_excluded_accounts_verify_writable_privileges<'me, 'info>(
    accounts: SetCirculatingSupplyExcludedAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.boss, accounts.excluded_accounts] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_circulating_supply_excluded_accounts_verify_signer_privileges<'me, 'info>(
    accounts: SetCirculatingSupplyExcludedAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_circulating_supply_excluded_accounts_verify_account_privileges<'me, 'info>(
    accounts: SetCirculatingSupplyExcludedAccountsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_circulating_supply_excluded_accounts_verify_writable_privileges(accounts)?;
    set_circulating_supply_excluded_accounts_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_CONFIGURABLE_VAULT_DESTINATION_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetConfigurableVaultDestinationAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub configurable_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetConfigurableVaultDestinationKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
    pub configurable_vault: Pubkey,
    pub system_program: Pubkey,
}
impl From<SetConfigurableVaultDestinationAccounts<'_, '_>>
for SetConfigurableVaultDestinationKeys {
    fn from(accounts: SetConfigurableVaultDestinationAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            configurable_vault: *accounts.configurable_vault.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SetConfigurableVaultDestinationKeys>
for [AccountMeta; SET_CONFIGURABLE_VAULT_DESTINATION_IX_ACCOUNTS_LEN] {
    fn from(keys: SetConfigurableVaultDestinationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.configurable_vault,
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
impl From<[Pubkey; SET_CONFIGURABLE_VAULT_DESTINATION_IX_ACCOUNTS_LEN]>
for SetConfigurableVaultDestinationKeys {
    fn from(
        pubkeys: [Pubkey; SET_CONFIGURABLE_VAULT_DESTINATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
            configurable_vault: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<SetConfigurableVaultDestinationAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CONFIGURABLE_VAULT_DESTINATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetConfigurableVaultDestinationAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.configurable_vault.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_CONFIGURABLE_VAULT_DESTINATION_IX_ACCOUNTS_LEN]>
for SetConfigurableVaultDestinationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_CONFIGURABLE_VAULT_DESTINATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
            configurable_vault: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const SET_CONFIGURABLE_VAULT_DESTINATION_IX_DISCM: [u8; 8usize] = [
    133, 244, 254, 4, 120, 3, 31, 123,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetConfigurableVaultDestinationIxArgs {
    pub kind: ConfigurableVaultKind,
    pub withdrawal_destination: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetConfigurableVaultDestinationIxData(
    pub SetConfigurableVaultDestinationIxArgs,
);
impl From<SetConfigurableVaultDestinationIxArgs>
for SetConfigurableVaultDestinationIxData {
    fn from(args: SetConfigurableVaultDestinationIxArgs) -> Self {
        Self(args)
    }
}
impl SetConfigurableVaultDestinationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_CONFIGURABLE_VAULT_DESTINATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kind: ConfigurableVaultKind = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_destination: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetConfigurableVaultDestinationIxArgs {
                kind,
                withdrawal_destination,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_CONFIGURABLE_VAULT_DESTINATION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.withdrawal_destination, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_configurable_vault_destination_ix_with_program_id(
    program_id: Pubkey,
    keys: SetConfigurableVaultDestinationKeys,
    args: SetConfigurableVaultDestinationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CONFIGURABLE_VAULT_DESTINATION_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetConfigurableVaultDestinationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_configurable_vault_destination_ix(
    keys: SetConfigurableVaultDestinationKeys,
    args: SetConfigurableVaultDestinationIxArgs,
) -> std::io::Result<Instruction> {
    set_configurable_vault_destination_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn set_configurable_vault_destination_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetConfigurableVaultDestinationAccounts<'_, '_>,
    args: SetConfigurableVaultDestinationIxArgs,
) -> ProgramResult {
    let keys: SetConfigurableVaultDestinationKeys = accounts.into();
    let ix = set_configurable_vault_destination_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_configurable_vault_destination_invoke(
    accounts: SetConfigurableVaultDestinationAccounts<'_, '_>,
    args: SetConfigurableVaultDestinationIxArgs,
) -> ProgramResult {
    set_configurable_vault_destination_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_configurable_vault_destination_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetConfigurableVaultDestinationAccounts<'_, '_>,
    args: SetConfigurableVaultDestinationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetConfigurableVaultDestinationKeys = accounts.into();
    let ix = set_configurable_vault_destination_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_configurable_vault_destination_invoke_signed(
    accounts: SetConfigurableVaultDestinationAccounts<'_, '_>,
    args: SetConfigurableVaultDestinationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_configurable_vault_destination_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_configurable_vault_destination_verify_account_keys(
    accounts: SetConfigurableVaultDestinationAccounts<'_, '_>,
    keys: SetConfigurableVaultDestinationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.configurable_vault.key, keys.configurable_vault),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_configurable_vault_destination_verify_writable_privileges<'me, 'info>(
    accounts: SetConfigurableVaultDestinationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.boss, accounts.configurable_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_configurable_vault_destination_verify_signer_privileges<'me, 'info>(
    accounts: SetConfigurableVaultDestinationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_configurable_vault_destination_verify_account_privileges<'me, 'info>(
    accounts: SetConfigurableVaultDestinationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_configurable_vault_destination_verify_writable_privileges(accounts)?;
    set_configurable_vault_destination_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_KILL_SWITCH_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetKillSwitchAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetKillSwitchKeys {
    pub state: Pubkey,
    pub signer: Pubkey,
}
impl From<SetKillSwitchAccounts<'_, '_>> for SetKillSwitchKeys {
    fn from(accounts: SetKillSwitchAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<SetKillSwitchKeys> for [AccountMeta; SET_KILL_SWITCH_IX_ACCOUNTS_LEN] {
    fn from(keys: SetKillSwitchKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_KILL_SWITCH_IX_ACCOUNTS_LEN]> for SetKillSwitchKeys {
    fn from(pubkeys: [Pubkey; SET_KILL_SWITCH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            signer: pubkeys[1],
        }
    }
}
impl<'info> From<SetKillSwitchAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_KILL_SWITCH_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetKillSwitchAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.signer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_KILL_SWITCH_IX_ACCOUNTS_LEN]>
for SetKillSwitchAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_KILL_SWITCH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            signer: &arr[1],
        }
    }
}
pub const SET_KILL_SWITCH_IX_DISCM: [u8; 8usize] = [
    228, 119, 172, 135, 209, 250, 172, 216,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetKillSwitchIxArgs {
    pub enable: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetKillSwitchIxData(pub SetKillSwitchIxArgs);
impl From<SetKillSwitchIxArgs> for SetKillSwitchIxData {
    fn from(args: SetKillSwitchIxArgs) -> Self {
        Self(args)
    }
}
impl SetKillSwitchIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_KILL_SWITCH_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let enable: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetKillSwitchIxArgs { enable }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_KILL_SWITCH_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.enable, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_kill_switch_ix_with_program_id(
    program_id: Pubkey,
    keys: SetKillSwitchKeys,
    args: SetKillSwitchIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_KILL_SWITCH_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetKillSwitchIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_kill_switch_ix(
    keys: SetKillSwitchKeys,
    args: SetKillSwitchIxArgs,
) -> std::io::Result<Instruction> {
    set_kill_switch_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn set_kill_switch_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetKillSwitchAccounts<'_, '_>,
    args: SetKillSwitchIxArgs,
) -> ProgramResult {
    let keys: SetKillSwitchKeys = accounts.into();
    let ix = set_kill_switch_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_kill_switch_invoke(
    accounts: SetKillSwitchAccounts<'_, '_>,
    args: SetKillSwitchIxArgs,
) -> ProgramResult {
    set_kill_switch_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn set_kill_switch_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetKillSwitchAccounts<'_, '_>,
    args: SetKillSwitchIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetKillSwitchKeys = accounts.into();
    let ix = set_kill_switch_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_kill_switch_invoke_signed(
    accounts: SetKillSwitchAccounts<'_, '_>,
    args: SetKillSwitchIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_kill_switch_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_kill_switch_verify_account_keys(
    accounts: SetKillSwitchAccounts<'_, '_>,
    keys: SetKillSwitchKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_kill_switch_verify_writable_privileges<'me, 'info>(
    accounts: SetKillSwitchAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_kill_switch_verify_signer_privileges<'me, 'info>(
    accounts: SetKillSwitchAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_kill_switch_verify_account_privileges<'me, 'info>(
    accounts: SetKillSwitchAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_kill_switch_verify_writable_privileges(accounts)?;
    set_kill_switch_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MAIN_OFFER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetMainOfferAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub offer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMainOfferKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
    pub offer: Pubkey,
}
impl From<SetMainOfferAccounts<'_, '_>> for SetMainOfferKeys {
    fn from(accounts: SetMainOfferAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            offer: *accounts.offer.key,
        }
    }
}
impl From<SetMainOfferKeys> for [AccountMeta; SET_MAIN_OFFER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMainOfferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_MAIN_OFFER_IX_ACCOUNTS_LEN]> for SetMainOfferKeys {
    fn from(pubkeys: [Pubkey; SET_MAIN_OFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
            offer: pubkeys[2],
        }
    }
}
impl<'info> From<SetMainOfferAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MAIN_OFFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMainOfferAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone(), accounts.offer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MAIN_OFFER_IX_ACCOUNTS_LEN]>
for SetMainOfferAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_MAIN_OFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
            offer: &arr[2],
        }
    }
}
pub const SET_MAIN_OFFER_IX_DISCM: [u8; 8usize] = [129, 95, 103, 81, 225, 142, 102, 227];
#[derive(Clone, Debug, PartialEq)]
pub struct SetMainOfferIxData;
impl SetMainOfferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MAIN_OFFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MAIN_OFFER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_main_offer_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMainOfferKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MAIN_OFFER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetMainOfferIxData.try_to_vec()?,
    })
}
pub fn set_main_offer_ix(keys: SetMainOfferKeys) -> std::io::Result<Instruction> {
    set_main_offer_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn set_main_offer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMainOfferAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetMainOfferKeys = accounts.into();
    let ix = set_main_offer_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_main_offer_invoke(accounts: SetMainOfferAccounts<'_, '_>) -> ProgramResult {
    set_main_offer_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn set_main_offer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMainOfferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMainOfferKeys = accounts.into();
    let ix = set_main_offer_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_main_offer_invoke_signed(
    accounts: SetMainOfferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_main_offer_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn set_main_offer_verify_account_keys(
    accounts: SetMainOfferAccounts<'_, '_>,
    keys: SetMainOfferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.offer.key, keys.offer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_main_offer_verify_writable_privileges<'me, 'info>(
    accounts: SetMainOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_main_offer_verify_signer_privileges<'me, 'info>(
    accounts: SetMainOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_main_offer_verify_account_privileges<'me, 'info>(
    accounts: SetMainOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_main_offer_verify_writable_privileges(accounts)?;
    set_main_offer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_OFFER_DISABLED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetOfferDisabledAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOfferDisabledKeys {
    pub offer: Pubkey,
    pub state: Pubkey,
    pub signer: Pubkey,
}
impl From<SetOfferDisabledAccounts<'_, '_>> for SetOfferDisabledKeys {
    fn from(accounts: SetOfferDisabledAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            state: *accounts.state.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<SetOfferDisabledKeys> for [AccountMeta; SET_OFFER_DISABLED_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOfferDisabledKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_OFFER_DISABLED_IX_ACCOUNTS_LEN]> for SetOfferDisabledKeys {
    fn from(pubkeys: [Pubkey; SET_OFFER_DISABLED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            state: pubkeys[1],
            signer: pubkeys[2],
        }
    }
}
impl<'info> From<SetOfferDisabledAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_OFFER_DISABLED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOfferDisabledAccounts<'_, 'info>) -> Self {
        [accounts.offer.clone(), accounts.state.clone(), accounts.signer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_OFFER_DISABLED_IX_ACCOUNTS_LEN]>
for SetOfferDisabledAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_OFFER_DISABLED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            state: &arr[1],
            signer: &arr[2],
        }
    }
}
pub const SET_OFFER_DISABLED_IX_DISCM: [u8; 8usize] = [
    250, 160, 113, 13, 46, 252, 4, 86,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetOfferDisabledIxArgs {
    pub disabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetOfferDisabledIxData(pub SetOfferDisabledIxArgs);
impl From<SetOfferDisabledIxArgs> for SetOfferDisabledIxData {
    fn from(args: SetOfferDisabledIxArgs) -> Self {
        Self(args)
    }
}
impl SetOfferDisabledIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_OFFER_DISABLED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let disabled: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetOfferDisabledIxArgs { disabled }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_OFFER_DISABLED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.disabled, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_offer_disabled_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOfferDisabledKeys,
    args: SetOfferDisabledIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_OFFER_DISABLED_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetOfferDisabledIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_offer_disabled_ix(
    keys: SetOfferDisabledKeys,
    args: SetOfferDisabledIxArgs,
) -> std::io::Result<Instruction> {
    set_offer_disabled_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn set_offer_disabled_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOfferDisabledAccounts<'_, '_>,
    args: SetOfferDisabledIxArgs,
) -> ProgramResult {
    let keys: SetOfferDisabledKeys = accounts.into();
    let ix = set_offer_disabled_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_offer_disabled_invoke(
    accounts: SetOfferDisabledAccounts<'_, '_>,
    args: SetOfferDisabledIxArgs,
) -> ProgramResult {
    set_offer_disabled_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn set_offer_disabled_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOfferDisabledAccounts<'_, '_>,
    args: SetOfferDisabledIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOfferDisabledKeys = accounts.into();
    let ix = set_offer_disabled_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_offer_disabled_invoke_signed(
    accounts: SetOfferDisabledAccounts<'_, '_>,
    args: SetOfferDisabledIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_offer_disabled_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_offer_disabled_verify_account_keys(
    accounts: SetOfferDisabledAccounts<'_, '_>,
    keys: SetOfferDisabledKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.state.key, keys.state),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_offer_disabled_verify_writable_privileges<'me, 'info>(
    accounts: SetOfferDisabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_offer_disabled_verify_signer_privileges<'me, 'info>(
    accounts: SetOfferDisabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_offer_disabled_verify_account_privileges<'me, 'info>(
    accounts: SetOfferDisabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_offer_disabled_verify_writable_privileges(accounts)?;
    set_offer_disabled_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ONYC_MINT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetOnycMintAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOnycMintKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
    pub onyc_mint: Pubkey,
}
impl From<SetOnycMintAccounts<'_, '_>> for SetOnycMintKeys {
    fn from(accounts: SetOnycMintAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            onyc_mint: *accounts.onyc_mint.key,
        }
    }
}
impl From<SetOnycMintKeys> for [AccountMeta; SET_ONYC_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOnycMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_ONYC_MINT_IX_ACCOUNTS_LEN]> for SetOnycMintKeys {
    fn from(pubkeys: [Pubkey; SET_ONYC_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
            onyc_mint: pubkeys[2],
        }
    }
}
impl<'info> From<SetOnycMintAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ONYC_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOnycMintAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone(), accounts.onyc_mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ONYC_MINT_IX_ACCOUNTS_LEN]>
for SetOnycMintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_ONYC_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
            onyc_mint: &arr[2],
        }
    }
}
pub const SET_ONYC_MINT_IX_DISCM: [u8; 8usize] = [177, 83, 119, 179, 44, 141, 201, 24];
#[derive(Clone, Debug, PartialEq)]
pub struct SetOnycMintIxData;
impl SetOnycMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ONYC_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ONYC_MINT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_onyc_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOnycMintKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ONYC_MINT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetOnycMintIxData.try_to_vec()?,
    })
}
pub fn set_onyc_mint_ix(keys: SetOnycMintKeys) -> std::io::Result<Instruction> {
    set_onyc_mint_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn set_onyc_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOnycMintAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetOnycMintKeys = accounts.into();
    let ix = set_onyc_mint_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_onyc_mint_invoke(accounts: SetOnycMintAccounts<'_, '_>) -> ProgramResult {
    set_onyc_mint_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn set_onyc_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOnycMintAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOnycMintKeys = accounts.into();
    let ix = set_onyc_mint_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_onyc_mint_invoke_signed(
    accounts: SetOnycMintAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_onyc_mint_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn set_onyc_mint_verify_account_keys(
    accounts: SetOnycMintAccounts<'_, '_>,
    keys: SetOnycMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.onyc_mint.key, keys.onyc_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_onyc_mint_verify_writable_privileges<'me, 'info>(
    accounts: SetOnycMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_onyc_mint_verify_signer_privileges<'me, 'info>(
    accounts: SetOnycMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_onyc_mint_verify_account_privileges<'me, 'info>(
    accounts: SetOnycMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_onyc_mint_verify_writable_privileges(accounts)?;
    set_onyc_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_REDEMPTION_OFFER_DISABLED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetRedemptionOfferDisabledAccounts<'me, 'info> {
    pub redemption_offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRedemptionOfferDisabledKeys {
    pub redemption_offer: Pubkey,
    pub state: Pubkey,
    pub signer: Pubkey,
}
impl From<SetRedemptionOfferDisabledAccounts<'_, '_>>
for SetRedemptionOfferDisabledKeys {
    fn from(accounts: SetRedemptionOfferDisabledAccounts) -> Self {
        Self {
            redemption_offer: *accounts.redemption_offer.key,
            state: *accounts.state.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<SetRedemptionOfferDisabledKeys>
for [AccountMeta; SET_REDEMPTION_OFFER_DISABLED_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRedemptionOfferDisabledKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_REDEMPTION_OFFER_DISABLED_IX_ACCOUNTS_LEN]>
for SetRedemptionOfferDisabledKeys {
    fn from(pubkeys: [Pubkey; SET_REDEMPTION_OFFER_DISABLED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            redemption_offer: pubkeys[0],
            state: pubkeys[1],
            signer: pubkeys[2],
        }
    }
}
impl<'info> From<SetRedemptionOfferDisabledAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_REDEMPTION_OFFER_DISABLED_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRedemptionOfferDisabledAccounts<'_, 'info>) -> Self {
        [
            accounts.redemption_offer.clone(),
            accounts.state.clone(),
            accounts.signer.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_REDEMPTION_OFFER_DISABLED_IX_ACCOUNTS_LEN]>
for SetRedemptionOfferDisabledAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_REDEMPTION_OFFER_DISABLED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            redemption_offer: &arr[0],
            state: &arr[1],
            signer: &arr[2],
        }
    }
}
pub const SET_REDEMPTION_OFFER_DISABLED_IX_DISCM: [u8; 8usize] = [
    44, 130, 57, 167, 162, 117, 37, 107,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetRedemptionOfferDisabledIxArgs {
    pub disabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRedemptionOfferDisabledIxData(pub SetRedemptionOfferDisabledIxArgs);
impl From<SetRedemptionOfferDisabledIxArgs> for SetRedemptionOfferDisabledIxData {
    fn from(args: SetRedemptionOfferDisabledIxArgs) -> Self {
        Self(args)
    }
}
impl SetRedemptionOfferDisabledIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_REDEMPTION_OFFER_DISABLED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let disabled: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetRedemptionOfferDisabledIxArgs {
                disabled,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_REDEMPTION_OFFER_DISABLED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.disabled, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_redemption_offer_disabled_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRedemptionOfferDisabledKeys,
    args: SetRedemptionOfferDisabledIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REDEMPTION_OFFER_DISABLED_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetRedemptionOfferDisabledIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_redemption_offer_disabled_ix(
    keys: SetRedemptionOfferDisabledKeys,
    args: SetRedemptionOfferDisabledIxArgs,
) -> std::io::Result<Instruction> {
    set_redemption_offer_disabled_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn set_redemption_offer_disabled_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRedemptionOfferDisabledAccounts<'_, '_>,
    args: SetRedemptionOfferDisabledIxArgs,
) -> ProgramResult {
    let keys: SetRedemptionOfferDisabledKeys = accounts.into();
    let ix = set_redemption_offer_disabled_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_redemption_offer_disabled_invoke(
    accounts: SetRedemptionOfferDisabledAccounts<'_, '_>,
    args: SetRedemptionOfferDisabledIxArgs,
) -> ProgramResult {
    set_redemption_offer_disabled_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_redemption_offer_disabled_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRedemptionOfferDisabledAccounts<'_, '_>,
    args: SetRedemptionOfferDisabledIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRedemptionOfferDisabledKeys = accounts.into();
    let ix = set_redemption_offer_disabled_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_redemption_offer_disabled_invoke_signed(
    accounts: SetRedemptionOfferDisabledAccounts<'_, '_>,
    args: SetRedemptionOfferDisabledIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_redemption_offer_disabled_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_redemption_offer_disabled_verify_account_keys(
    accounts: SetRedemptionOfferDisabledAccounts<'_, '_>,
    keys: SetRedemptionOfferDisabledKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.state.key, keys.state),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_redemption_offer_disabled_verify_writable_privileges<'me, 'info>(
    accounts: SetRedemptionOfferDisabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.redemption_offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_redemption_offer_disabled_verify_signer_privileges<'me, 'info>(
    accounts: SetRedemptionOfferDisabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_redemption_offer_disabled_verify_account_privileges<'me, 'info>(
    accounts: SetRedemptionOfferDisabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_redemption_offer_disabled_verify_writable_privileges(accounts)?;
    set_redemption_offer_disabled_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_WORKER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetWorkerAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWorkerKeys {
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<SetWorkerAccounts<'_, '_>> for SetWorkerKeys {
    fn from(accounts: SetWorkerAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<SetWorkerKeys> for [AccountMeta; SET_WORKER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWorkerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_WORKER_IX_ACCOUNTS_LEN]> for SetWorkerKeys {
    fn from(pubkeys: [Pubkey; SET_WORKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            boss: pubkeys[1],
        }
    }
}
impl<'info> From<SetWorkerAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WORKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWorkerAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.boss.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WORKER_IX_ACCOUNTS_LEN]>
for SetWorkerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_WORKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            boss: &arr[1],
        }
    }
}
pub const SET_WORKER_IX_DISCM: [u8; 8usize] = [36, 210, 154, 242, 201, 126, 112, 33];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetWorkerIxArgs {
    pub new_worker: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWorkerIxData(pub SetWorkerIxArgs);
impl From<SetWorkerIxArgs> for SetWorkerIxData {
    fn from(args: SetWorkerIxArgs) -> Self {
        Self(args)
    }
}
impl SetWorkerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WORKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_worker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetWorkerIxArgs { new_worker }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WORKER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_worker, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_worker_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWorkerKeys,
    args: SetWorkerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WORKER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWorkerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_worker_ix(
    keys: SetWorkerKeys,
    args: SetWorkerIxArgs,
) -> std::io::Result<Instruction> {
    set_worker_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn set_worker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWorkerAccounts<'_, '_>,
    args: SetWorkerIxArgs,
) -> ProgramResult {
    let keys: SetWorkerKeys = accounts.into();
    let ix = set_worker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_worker_invoke(
    accounts: SetWorkerAccounts<'_, '_>,
    args: SetWorkerIxArgs,
) -> ProgramResult {
    set_worker_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn set_worker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWorkerAccounts<'_, '_>,
    args: SetWorkerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWorkerKeys = accounts.into();
    let ix = set_worker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_worker_invoke_signed(
    accounts: SetWorkerAccounts<'_, '_>,
    args: SetWorkerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_worker_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_worker_verify_account_keys(
    accounts: SetWorkerAccounts<'_, '_>,
    keys: SetWorkerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_worker_verify_writable_privileges<'me, 'info>(
    accounts: SetWorkerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_worker_verify_signer_privileges<'me, 'info>(
    accounts: SetWorkerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_worker_verify_account_privileges<'me, 'info>(
    accounts: SetWorkerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_worker_verify_writable_privileges(accounts)?;
    set_worker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SETTLE_BUFFER_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SettleBufferAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub worker: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleBufferKeys {
    pub state: Pubkey,
    pub worker: Pubkey,
    pub onyc_mint: Pubkey,
    pub mint_authority: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub main_offer: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
}
impl From<SettleBufferAccounts<'_, '_>> for SettleBufferKeys {
    fn from(accounts: SettleBufferAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            worker: *accounts.worker.key,
            onyc_mint: *accounts.onyc_mint.key,
            mint_authority: *accounts.mint_authority.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            main_offer: *accounts.main_offer.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
        }
    }
}
impl From<SettleBufferKeys> for [AccountMeta; SETTLE_BUFFER_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleBufferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.worker,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SETTLE_BUFFER_IX_ACCOUNTS_LEN]> for SettleBufferKeys {
    fn from(pubkeys: [Pubkey; SETTLE_BUFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            worker: pubkeys[1],
            onyc_mint: pubkeys[2],
            mint_authority: pubkeys[3],
            token_program: pubkeys[4],
            system_program: pubkeys[5],
            main_offer: pubkeys[6],
            buffer_accounts_buffer_state: pubkeys[7],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[8],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[9],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[10],
            market_stats: pubkeys[11],
            circulating_supply_excluded_balance: pubkeys[12],
        }
    }
}
impl<'info> From<SettleBufferAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_BUFFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleBufferAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.worker.clone(),
            accounts.onyc_mint.clone(),
            accounts.mint_authority.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.main_offer.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_BUFFER_IX_ACCOUNTS_LEN]>
for SettleBufferAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SETTLE_BUFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            worker: &arr[1],
            onyc_mint: &arr[2],
            mint_authority: &arr[3],
            token_program: &arr[4],
            system_program: &arr[5],
            main_offer: &arr[6],
            buffer_accounts_buffer_state: &arr[7],
            buffer_accounts_reserve_vault_onyc_account: &arr[8],
            buffer_accounts_management_fee_vault_onyc_account: &arr[9],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[10],
            market_stats: &arr[11],
            circulating_supply_excluded_balance: &arr[12],
        }
    }
}
pub const SETTLE_BUFFER_IX_DISCM: [u8; 8usize] = [196, 144, 145, 221, 54, 210, 224, 9];
#[derive(Clone, Debug, PartialEq)]
pub struct SettleBufferIxData;
impl SettleBufferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_BUFFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_BUFFER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_buffer_ix_with_program_id(
    program_id: Pubkey,
    keys: SettleBufferKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETTLE_BUFFER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SettleBufferIxData.try_to_vec()?,
    })
}
pub fn settle_buffer_ix(keys: SettleBufferKeys) -> std::io::Result<Instruction> {
    settle_buffer_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn settle_buffer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SettleBufferAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SettleBufferKeys = accounts.into();
    let ix = settle_buffer_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn settle_buffer_invoke(accounts: SettleBufferAccounts<'_, '_>) -> ProgramResult {
    settle_buffer_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn settle_buffer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SettleBufferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SettleBufferKeys = accounts.into();
    let ix = settle_buffer_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn settle_buffer_invoke_signed(
    accounts: SettleBufferAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    settle_buffer_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, seeds)
}
pub fn settle_buffer_verify_account_keys(
    accounts: SettleBufferAccounts<'_, '_>,
    keys: SettleBufferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.worker.key, keys.worker),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.main_offer.key, keys.main_offer),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn settle_buffer_verify_writable_privileges<'me, 'info>(
    accounts: SettleBufferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.worker,
        accounts.onyc_mint,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn settle_buffer_verify_signer_privileges<'me, 'info>(
    accounts: SettleBufferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.worker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn settle_buffer_verify_account_privileges<'me, 'info>(
    accounts: SettleBufferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    settle_buffer_verify_writable_privileges(accounts)?;
    settle_buffer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TAKE_OFFER_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct TakeOfferAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_token_in_account: &'me AccountInfo<'info>,
    pub vault_token_out_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub boss_token_in_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeOfferKeys {
    pub offer: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_token_in_account: Pubkey,
    pub vault_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub boss_token_in_account: Pubkey,
    pub mint_authority: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub user: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<TakeOfferAccounts<'_, '_>> for TakeOfferKeys {
    fn from(accounts: TakeOfferAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            vault_authority: *accounts.vault_authority.key,
            vault_token_in_account: *accounts.vault_token_in_account.key,
            vault_token_out_account: *accounts.vault_token_out_account.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            user_token_in_account: *accounts.user_token_in_account.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            boss_token_in_account: *accounts.boss_token_in_account.key,
            mint_authority: *accounts.mint_authority.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            user: *accounts.user.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TakeOfferKeys> for [AccountMeta; TAKE_OFFER_IX_ACCOUNTS_LEN] {
    fn from(keys: TakeOfferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; TAKE_OFFER_IX_ACCOUNTS_LEN]> for TakeOfferKeys {
    fn from(pubkeys: [Pubkey; TAKE_OFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            state: pubkeys[1],
            boss: pubkeys[2],
            vault_authority: pubkeys[3],
            vault_token_in_account: pubkeys[4],
            vault_token_out_account: pubkeys[5],
            token_in_mint: pubkeys[6],
            token_in_program: pubkeys[7],
            token_out_mint: pubkeys[8],
            token_out_program: pubkeys[9],
            user_token_in_account: pubkeys[10],
            user_token_out_account: pubkeys[11],
            boss_token_in_account: pubkeys[12],
            mint_authority: pubkeys[13],
            instructions_sysvar: pubkeys[14],
            user: pubkeys[15],
            associated_token_program: pubkeys[16],
            system_program: pubkeys[17],
        }
    }
}
impl<'info> From<TakeOfferAccounts<'_, 'info>>
for [AccountInfo<'info>; TAKE_OFFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TakeOfferAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_token_in_account.clone(),
            accounts.vault_token_out_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.user_token_in_account.clone(),
            accounts.user_token_out_account.clone(),
            accounts.boss_token_in_account.clone(),
            accounts.mint_authority.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.user.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TAKE_OFFER_IX_ACCOUNTS_LEN]>
for TakeOfferAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TAKE_OFFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            state: &arr[1],
            boss: &arr[2],
            vault_authority: &arr[3],
            vault_token_in_account: &arr[4],
            vault_token_out_account: &arr[5],
            token_in_mint: &arr[6],
            token_in_program: &arr[7],
            token_out_mint: &arr[8],
            token_out_program: &arr[9],
            user_token_in_account: &arr[10],
            user_token_out_account: &arr[11],
            boss_token_in_account: &arr[12],
            mint_authority: &arr[13],
            instructions_sysvar: &arr[14],
            user: &arr[15],
            associated_token_program: &arr[16],
            system_program: &arr[17],
        }
    }
}
pub const TAKE_OFFER_IX_DISCM: [u8; 8usize] = [128, 156, 242, 207, 237, 192, 103, 240];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TakeOfferIxArgs {
    pub token_in_amount: u64,
    pub approval_message: Option<ApprovalMessage>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TakeOfferIxData(pub TakeOfferIxArgs);
impl From<TakeOfferIxArgs> for TakeOfferIxData {
    fn from(args: TakeOfferIxArgs) -> Self {
        Self(args)
    }
}
impl TakeOfferIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TAKE_OFFER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let approval_message: Option<ApprovalMessage> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(TakeOfferIxArgs {
                token_in_amount,
                approval_message,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TAKE_OFFER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.approval_message, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn take_offer_ix_with_program_id(
    program_id: Pubkey,
    keys: TakeOfferKeys,
    args: TakeOfferIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TAKE_OFFER_IX_ACCOUNTS_LEN] = keys.into();
    let data: TakeOfferIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn take_offer_ix(
    keys: TakeOfferKeys,
    args: TakeOfferIxArgs,
) -> std::io::Result<Instruction> {
    take_offer_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn take_offer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TakeOfferAccounts<'_, '_>,
    args: TakeOfferIxArgs,
) -> ProgramResult {
    let keys: TakeOfferKeys = accounts.into();
    let ix = take_offer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn take_offer_invoke(
    accounts: TakeOfferAccounts<'_, '_>,
    args: TakeOfferIxArgs,
) -> ProgramResult {
    take_offer_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn take_offer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TakeOfferAccounts<'_, '_>,
    args: TakeOfferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TakeOfferKeys = accounts.into();
    let ix = take_offer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn take_offer_invoke_signed(
    accounts: TakeOfferAccounts<'_, '_>,
    args: TakeOfferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    take_offer_invoke_signed_with_program_id(ONREAPP_PROGRAM_ID, accounts, args, seeds)
}
pub fn take_offer_verify_account_keys(
    accounts: TakeOfferAccounts<'_, '_>,
    keys: TakeOfferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_token_in_account.key, keys.vault_token_in_account),
        (*accounts.vault_token_out_account.key, keys.vault_token_out_account),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.user_token_in_account.key, keys.user_token_in_account),
        (*accounts.user_token_out_account.key, keys.user_token_out_account),
        (*accounts.boss_token_in_account.key, keys.boss_token_in_account),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.user.key, keys.user),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn take_offer_verify_writable_privileges<'me, 'info>(
    accounts: TakeOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.offer,
        accounts.vault_token_in_account,
        accounts.vault_token_out_account,
        accounts.token_in_mint,
        accounts.token_out_mint,
        accounts.user_token_in_account,
        accounts.user_token_out_account,
        accounts.boss_token_in_account,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn take_offer_verify_signer_privileges<'me, 'info>(
    accounts: TakeOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn take_offer_verify_account_privileges<'me, 'info>(
    accounts: TakeOfferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    take_offer_verify_writable_privileges(accounts)?;
    take_offer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TAKE_OFFER_PERMISSIONLESS_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct TakeOfferPermissionlessAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_token_in_account: &'me AccountInfo<'info>,
    pub vault_token_out_account: &'me AccountInfo<'info>,
    pub permissionless_authority: &'me AccountInfo<'info>,
    pub permissionless_token_in_account: &'me AccountInfo<'info>,
    pub permissionless_token_out_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub boss_token_in_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeOfferPermissionlessKeys {
    pub offer: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_token_in_account: Pubkey,
    pub vault_token_out_account: Pubkey,
    pub permissionless_authority: Pubkey,
    pub permissionless_token_in_account: Pubkey,
    pub permissionless_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub boss_token_in_account: Pubkey,
    pub mint_authority: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub user: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<TakeOfferPermissionlessAccounts<'_, '_>> for TakeOfferPermissionlessKeys {
    fn from(accounts: TakeOfferPermissionlessAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
            vault_authority: *accounts.vault_authority.key,
            vault_token_in_account: *accounts.vault_token_in_account.key,
            vault_token_out_account: *accounts.vault_token_out_account.key,
            permissionless_authority: *accounts.permissionless_authority.key,
            permissionless_token_in_account: *accounts
                .permissionless_token_in_account
                .key,
            permissionless_token_out_account: *accounts
                .permissionless_token_out_account
                .key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            user_token_in_account: *accounts.user_token_in_account.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            boss_token_in_account: *accounts.boss_token_in_account.key,
            mint_authority: *accounts.mint_authority.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            user: *accounts.user.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TakeOfferPermissionlessKeys>
for [AccountMeta; TAKE_OFFER_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(keys: TakeOfferPermissionlessKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permissionless_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permissionless_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permissionless_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; TAKE_OFFER_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for TakeOfferPermissionlessKeys {
    fn from(pubkeys: [Pubkey; TAKE_OFFER_PERMISSIONLESS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            state: pubkeys[1],
            boss: pubkeys[2],
            vault_authority: pubkeys[3],
            vault_token_in_account: pubkeys[4],
            vault_token_out_account: pubkeys[5],
            permissionless_authority: pubkeys[6],
            permissionless_token_in_account: pubkeys[7],
            permissionless_token_out_account: pubkeys[8],
            token_in_mint: pubkeys[9],
            token_in_program: pubkeys[10],
            token_out_mint: pubkeys[11],
            token_out_program: pubkeys[12],
            user_token_in_account: pubkeys[13],
            user_token_out_account: pubkeys[14],
            boss_token_in_account: pubkeys[15],
            mint_authority: pubkeys[16],
            instructions_sysvar: pubkeys[17],
            user: pubkeys[18],
            associated_token_program: pubkeys[19],
            system_program: pubkeys[20],
        }
    }
}
impl<'info> From<TakeOfferPermissionlessAccounts<'_, 'info>>
for [AccountInfo<'info>; TAKE_OFFER_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: TakeOfferPermissionlessAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_token_in_account.clone(),
            accounts.vault_token_out_account.clone(),
            accounts.permissionless_authority.clone(),
            accounts.permissionless_token_in_account.clone(),
            accounts.permissionless_token_out_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.user_token_in_account.clone(),
            accounts.user_token_out_account.clone(),
            accounts.boss_token_in_account.clone(),
            accounts.mint_authority.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.user.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TAKE_OFFER_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for TakeOfferPermissionlessAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TAKE_OFFER_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            offer: &arr[0],
            state: &arr[1],
            boss: &arr[2],
            vault_authority: &arr[3],
            vault_token_in_account: &arr[4],
            vault_token_out_account: &arr[5],
            permissionless_authority: &arr[6],
            permissionless_token_in_account: &arr[7],
            permissionless_token_out_account: &arr[8],
            token_in_mint: &arr[9],
            token_in_program: &arr[10],
            token_out_mint: &arr[11],
            token_out_program: &arr[12],
            user_token_in_account: &arr[13],
            user_token_out_account: &arr[14],
            boss_token_in_account: &arr[15],
            mint_authority: &arr[16],
            instructions_sysvar: &arr[17],
            user: &arr[18],
            associated_token_program: &arr[19],
            system_program: &arr[20],
        }
    }
}
pub const TAKE_OFFER_PERMISSIONLESS_IX_DISCM: [u8; 8usize] = [
    37, 190, 224, 77, 197, 39, 203, 230,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TakeOfferPermissionlessIxArgs {
    pub token_in_amount: u64,
    pub approval_message: Option<ApprovalMessage>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TakeOfferPermissionlessIxData(pub TakeOfferPermissionlessIxArgs);
impl From<TakeOfferPermissionlessIxArgs> for TakeOfferPermissionlessIxData {
    fn from(args: TakeOfferPermissionlessIxArgs) -> Self {
        Self(args)
    }
}
impl TakeOfferPermissionlessIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TAKE_OFFER_PERMISSIONLESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let approval_message: Option<ApprovalMessage> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(TakeOfferPermissionlessIxArgs {
                token_in_amount,
                approval_message,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TAKE_OFFER_PERMISSIONLESS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.approval_message, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn take_offer_permissionless_ix_with_program_id(
    program_id: Pubkey,
    keys: TakeOfferPermissionlessKeys,
    args: TakeOfferPermissionlessIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TAKE_OFFER_PERMISSIONLESS_IX_ACCOUNTS_LEN] = keys.into();
    let data: TakeOfferPermissionlessIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn take_offer_permissionless_ix(
    keys: TakeOfferPermissionlessKeys,
    args: TakeOfferPermissionlessIxArgs,
) -> std::io::Result<Instruction> {
    take_offer_permissionless_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn take_offer_permissionless_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TakeOfferPermissionlessAccounts<'_, '_>,
    args: TakeOfferPermissionlessIxArgs,
) -> ProgramResult {
    let keys: TakeOfferPermissionlessKeys = accounts.into();
    let ix = take_offer_permissionless_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn take_offer_permissionless_invoke(
    accounts: TakeOfferPermissionlessAccounts<'_, '_>,
    args: TakeOfferPermissionlessIxArgs,
) -> ProgramResult {
    take_offer_permissionless_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn take_offer_permissionless_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TakeOfferPermissionlessAccounts<'_, '_>,
    args: TakeOfferPermissionlessIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TakeOfferPermissionlessKeys = accounts.into();
    let ix = take_offer_permissionless_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn take_offer_permissionless_invoke_signed(
    accounts: TakeOfferPermissionlessAccounts<'_, '_>,
    args: TakeOfferPermissionlessIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    take_offer_permissionless_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn take_offer_permissionless_verify_account_keys(
    accounts: TakeOfferPermissionlessAccounts<'_, '_>,
    keys: TakeOfferPermissionlessKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_token_in_account.key, keys.vault_token_in_account),
        (*accounts.vault_token_out_account.key, keys.vault_token_out_account),
        (*accounts.permissionless_authority.key, keys.permissionless_authority),
        (
            *accounts.permissionless_token_in_account.key,
            keys.permissionless_token_in_account,
        ),
        (
            *accounts.permissionless_token_out_account.key,
            keys.permissionless_token_out_account,
        ),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.user_token_in_account.key, keys.user_token_in_account),
        (*accounts.user_token_out_account.key, keys.user_token_out_account),
        (*accounts.boss_token_in_account.key, keys.boss_token_in_account),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.user.key, keys.user),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn take_offer_permissionless_verify_writable_privileges<'me, 'info>(
    accounts: TakeOfferPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.offer,
        accounts.vault_token_in_account,
        accounts.vault_token_out_account,
        accounts.permissionless_token_in_account,
        accounts.permissionless_token_out_account,
        accounts.token_in_mint,
        accounts.token_out_mint,
        accounts.user_token_in_account,
        accounts.user_token_out_account,
        accounts.boss_token_in_account,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn take_offer_permissionless_verify_signer_privileges<'me, 'info>(
    accounts: TakeOfferPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn take_offer_permissionless_verify_account_privileges<'me, 'info>(
    accounts: TakeOfferPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    take_offer_permissionless_verify_writable_privileges(accounts)?;
    take_offer_permissionless_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TAKE_OFFER_PERMISSIONLESS_V2_IX_ACCOUNTS_LEN: usize = 32;
#[derive(Copy, Clone, Debug)]
pub struct TakeOfferPermissionlessV2Accounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_token_in_account: &'me AccountInfo<'info>,
    pub vault_token_out_account: &'me AccountInfo<'info>,
    pub permissionless_authority: &'me AccountInfo<'info>,
    pub permissionless_token_in_account: &'me AccountInfo<'info>,
    pub permissionless_token_out_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub redemption_vault_token_in_account: &'me AccountInfo<'info>,
    pub offer_proceeds_vault: &'me AccountInfo<'info>,
    pub offer_proceeds_token_in_account: &'me AccountInfo<'info>,
    pub permissionless_offer_fee_vault: &'me AccountInfo<'info>,
    pub permissionless_offer_fee_token_in_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeOfferPermissionlessV2Keys {
    pub offer: Pubkey,
    pub state: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_token_in_account: Pubkey,
    pub vault_token_out_account: Pubkey,
    pub permissionless_authority: Pubkey,
    pub permissionless_token_in_account: Pubkey,
    pub permissionless_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub redemption_offer: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub redemption_vault_token_in_account: Pubkey,
    pub offer_proceeds_vault: Pubkey,
    pub offer_proceeds_token_in_account: Pubkey,
    pub permissionless_offer_fee_vault: Pubkey,
    pub permissionless_offer_fee_token_in_account: Pubkey,
    pub mint_authority: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
    pub user: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub main_offer: Pubkey,
}
impl From<TakeOfferPermissionlessV2Accounts<'_, '_>> for TakeOfferPermissionlessV2Keys {
    fn from(accounts: TakeOfferPermissionlessV2Accounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            state: *accounts.state.key,
            vault_authority: *accounts.vault_authority.key,
            vault_token_in_account: *accounts.vault_token_in_account.key,
            vault_token_out_account: *accounts.vault_token_out_account.key,
            permissionless_authority: *accounts.permissionless_authority.key,
            permissionless_token_in_account: *accounts
                .permissionless_token_in_account
                .key,
            permissionless_token_out_account: *accounts
                .permissionless_token_out_account
                .key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            user_token_in_account: *accounts.user_token_in_account.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            redemption_offer: *accounts.redemption_offer.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            redemption_vault_token_in_account: *accounts
                .redemption_vault_token_in_account
                .key,
            offer_proceeds_vault: *accounts.offer_proceeds_vault.key,
            offer_proceeds_token_in_account: *accounts
                .offer_proceeds_token_in_account
                .key,
            permissionless_offer_fee_vault: *accounts.permissionless_offer_fee_vault.key,
            permissionless_offer_fee_token_in_account: *accounts
                .permissionless_offer_fee_token_in_account
                .key,
            mint_authority: *accounts.mint_authority.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
            user: *accounts.user.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            main_offer: *accounts.main_offer.key,
        }
    }
}
impl From<TakeOfferPermissionlessV2Keys>
for [AccountMeta; TAKE_OFFER_PERMISSIONLESS_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: TakeOfferPermissionlessV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permissionless_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permissionless_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permissionless_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_proceeds_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_proceeds_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permissionless_offer_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permissionless_offer_fee_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TAKE_OFFER_PERMISSIONLESS_V2_IX_ACCOUNTS_LEN]>
for TakeOfferPermissionlessV2Keys {
    fn from(pubkeys: [Pubkey; TAKE_OFFER_PERMISSIONLESS_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            state: pubkeys[1],
            vault_authority: pubkeys[2],
            vault_token_in_account: pubkeys[3],
            vault_token_out_account: pubkeys[4],
            permissionless_authority: pubkeys[5],
            permissionless_token_in_account: pubkeys[6],
            permissionless_token_out_account: pubkeys[7],
            token_in_mint: pubkeys[8],
            token_in_program: pubkeys[9],
            token_out_mint: pubkeys[10],
            token_out_program: pubkeys[11],
            user_token_in_account: pubkeys[12],
            user_token_out_account: pubkeys[13],
            redemption_offer: pubkeys[14],
            redemption_vault_authority: pubkeys[15],
            redemption_vault_token_in_account: pubkeys[16],
            offer_proceeds_vault: pubkeys[17],
            offer_proceeds_token_in_account: pubkeys[18],
            permissionless_offer_fee_vault: pubkeys[19],
            permissionless_offer_fee_token_in_account: pubkeys[20],
            mint_authority: pubkeys[21],
            buffer_accounts_buffer_state: pubkeys[22],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[23],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[24],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[25],
            market_stats: pubkeys[26],
            circulating_supply_excluded_balance: pubkeys[27],
            user: pubkeys[28],
            associated_token_program: pubkeys[29],
            system_program: pubkeys[30],
            main_offer: pubkeys[31],
        }
    }
}
impl<'info> From<TakeOfferPermissionlessV2Accounts<'_, 'info>>
for [AccountInfo<'info>; TAKE_OFFER_PERMISSIONLESS_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: TakeOfferPermissionlessV2Accounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.state.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_token_in_account.clone(),
            accounts.vault_token_out_account.clone(),
            accounts.permissionless_authority.clone(),
            accounts.permissionless_token_in_account.clone(),
            accounts.permissionless_token_out_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.user_token_in_account.clone(),
            accounts.user_token_out_account.clone(),
            accounts.redemption_offer.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.redemption_vault_token_in_account.clone(),
            accounts.offer_proceeds_vault.clone(),
            accounts.offer_proceeds_token_in_account.clone(),
            accounts.permissionless_offer_fee_vault.clone(),
            accounts.permissionless_offer_fee_token_in_account.clone(),
            accounts.mint_authority.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
            accounts.user.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.main_offer.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TAKE_OFFER_PERMISSIONLESS_V2_IX_ACCOUNTS_LEN]>
for TakeOfferPermissionlessV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TAKE_OFFER_PERMISSIONLESS_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            offer: &arr[0],
            state: &arr[1],
            vault_authority: &arr[2],
            vault_token_in_account: &arr[3],
            vault_token_out_account: &arr[4],
            permissionless_authority: &arr[5],
            permissionless_token_in_account: &arr[6],
            permissionless_token_out_account: &arr[7],
            token_in_mint: &arr[8],
            token_in_program: &arr[9],
            token_out_mint: &arr[10],
            token_out_program: &arr[11],
            user_token_in_account: &arr[12],
            user_token_out_account: &arr[13],
            redemption_offer: &arr[14],
            redemption_vault_authority: &arr[15],
            redemption_vault_token_in_account: &arr[16],
            offer_proceeds_vault: &arr[17],
            offer_proceeds_token_in_account: &arr[18],
            permissionless_offer_fee_vault: &arr[19],
            permissionless_offer_fee_token_in_account: &arr[20],
            mint_authority: &arr[21],
            buffer_accounts_buffer_state: &arr[22],
            buffer_accounts_reserve_vault_onyc_account: &arr[23],
            buffer_accounts_management_fee_vault_onyc_account: &arr[24],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[25],
            market_stats: &arr[26],
            circulating_supply_excluded_balance: &arr[27],
            user: &arr[28],
            associated_token_program: &arr[29],
            system_program: &arr[30],
            main_offer: &arr[31],
        }
    }
}
pub const TAKE_OFFER_PERMISSIONLESS_V2_IX_DISCM: [u8; 8usize] = [
    250, 180, 68, 89, 124, 124, 31, 250,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TakeOfferPermissionlessV2IxArgs {
    pub token_in_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TakeOfferPermissionlessV2IxData(pub TakeOfferPermissionlessV2IxArgs);
impl From<TakeOfferPermissionlessV2IxArgs> for TakeOfferPermissionlessV2IxData {
    fn from(args: TakeOfferPermissionlessV2IxArgs) -> Self {
        Self(args)
    }
}
impl TakeOfferPermissionlessV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TAKE_OFFER_PERMISSIONLESS_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TakeOfferPermissionlessV2IxArgs {
                token_in_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TAKE_OFFER_PERMISSIONLESS_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn take_offer_permissionless_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: TakeOfferPermissionlessV2Keys,
    args: TakeOfferPermissionlessV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TAKE_OFFER_PERMISSIONLESS_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: TakeOfferPermissionlessV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn take_offer_permissionless_v2_ix(
    keys: TakeOfferPermissionlessV2Keys,
    args: TakeOfferPermissionlessV2IxArgs,
) -> std::io::Result<Instruction> {
    take_offer_permissionless_v2_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn take_offer_permissionless_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TakeOfferPermissionlessV2Accounts<'_, '_>,
    args: TakeOfferPermissionlessV2IxArgs,
) -> ProgramResult {
    let keys: TakeOfferPermissionlessV2Keys = accounts.into();
    let ix = take_offer_permissionless_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn take_offer_permissionless_v2_invoke(
    accounts: TakeOfferPermissionlessV2Accounts<'_, '_>,
    args: TakeOfferPermissionlessV2IxArgs,
) -> ProgramResult {
    take_offer_permissionless_v2_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn take_offer_permissionless_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TakeOfferPermissionlessV2Accounts<'_, '_>,
    args: TakeOfferPermissionlessV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TakeOfferPermissionlessV2Keys = accounts.into();
    let ix = take_offer_permissionless_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn take_offer_permissionless_v2_invoke_signed(
    accounts: TakeOfferPermissionlessV2Accounts<'_, '_>,
    args: TakeOfferPermissionlessV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    take_offer_permissionless_v2_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn take_offer_permissionless_v2_verify_account_keys(
    accounts: TakeOfferPermissionlessV2Accounts<'_, '_>,
    keys: TakeOfferPermissionlessV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.state.key, keys.state),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_token_in_account.key, keys.vault_token_in_account),
        (*accounts.vault_token_out_account.key, keys.vault_token_out_account),
        (*accounts.permissionless_authority.key, keys.permissionless_authority),
        (
            *accounts.permissionless_token_in_account.key,
            keys.permissionless_token_in_account,
        ),
        (
            *accounts.permissionless_token_out_account.key,
            keys.permissionless_token_out_account,
        ),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.user_token_in_account.key, keys.user_token_in_account),
        (*accounts.user_token_out_account.key, keys.user_token_out_account),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (
            *accounts.redemption_vault_token_in_account.key,
            keys.redemption_vault_token_in_account,
        ),
        (*accounts.offer_proceeds_vault.key, keys.offer_proceeds_vault),
        (
            *accounts.offer_proceeds_token_in_account.key,
            keys.offer_proceeds_token_in_account,
        ),
        (
            *accounts.permissionless_offer_fee_vault.key,
            keys.permissionless_offer_fee_vault,
        ),
        (
            *accounts.permissionless_offer_fee_token_in_account.key,
            keys.permissionless_offer_fee_token_in_account,
        ),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
        (*accounts.user.key, keys.user),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.main_offer.key, keys.main_offer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn take_offer_permissionless_v2_verify_writable_privileges<'me, 'info>(
    accounts: TakeOfferPermissionlessV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.offer,
        accounts.vault_token_in_account,
        accounts.vault_token_out_account,
        accounts.permissionless_token_in_account,
        accounts.permissionless_token_out_account,
        accounts.token_in_mint,
        accounts.token_out_mint,
        accounts.user_token_in_account,
        accounts.user_token_out_account,
        accounts.redemption_vault_token_in_account,
        accounts.offer_proceeds_vault,
        accounts.offer_proceeds_token_in_account,
        accounts.permissionless_offer_fee_vault,
        accounts.permissionless_offer_fee_token_in_account,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn take_offer_permissionless_v2_verify_signer_privileges<'me, 'info>(
    accounts: TakeOfferPermissionlessV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn take_offer_permissionless_v2_verify_account_privileges<'me, 'info>(
    accounts: TakeOfferPermissionlessV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    take_offer_permissionless_v2_verify_writable_privileges(accounts)?;
    take_offer_permissionless_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TAKE_OFFER_V2_IX_ACCOUNTS_LEN: usize = 30;
#[derive(Copy, Clone, Debug)]
pub struct TakeOfferV2Accounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_token_in_account: &'me AccountInfo<'info>,
    pub vault_token_out_account: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_in_program: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub token_out_program: &'me AccountInfo<'info>,
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub redemption_offer: &'me AccountInfo<'info>,
    pub redemption_vault_authority: &'me AccountInfo<'info>,
    pub redemption_vault_token_in_account: &'me AccountInfo<'info>,
    pub offer_proceeds_vault: &'me AccountInfo<'info>,
    pub offer_proceeds_token_in_account: &'me AccountInfo<'info>,
    pub offer_fee_vault: &'me AccountInfo<'info>,
    pub offer_fee_token_in_account: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub buffer_accounts_buffer_state: &'me AccountInfo<'info>,
    pub buffer_accounts_reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_management_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub buffer_accounts_performance_fee_vault_onyc_account: &'me AccountInfo<'info>,
    pub market_stats: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub main_offer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TakeOfferV2Keys {
    pub offer: Pubkey,
    pub state: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_token_in_account: Pubkey,
    pub vault_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_in_program: Pubkey,
    pub token_out_mint: Pubkey,
    pub token_out_program: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub redemption_offer: Pubkey,
    pub redemption_vault_authority: Pubkey,
    pub redemption_vault_token_in_account: Pubkey,
    pub offer_proceeds_vault: Pubkey,
    pub offer_proceeds_token_in_account: Pubkey,
    pub offer_fee_vault: Pubkey,
    pub offer_fee_token_in_account: Pubkey,
    pub mint_authority: Pubkey,
    pub buffer_accounts_buffer_state: Pubkey,
    pub buffer_accounts_reserve_vault_onyc_account: Pubkey,
    pub buffer_accounts_management_fee_vault_onyc_account: Pubkey,
    pub buffer_accounts_performance_fee_vault_onyc_account: Pubkey,
    pub market_stats: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub user: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub main_offer: Pubkey,
}
impl From<TakeOfferV2Accounts<'_, '_>> for TakeOfferV2Keys {
    fn from(accounts: TakeOfferV2Accounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            state: *accounts.state.key,
            vault_authority: *accounts.vault_authority.key,
            vault_token_in_account: *accounts.vault_token_in_account.key,
            vault_token_out_account: *accounts.vault_token_out_account.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_in_program: *accounts.token_in_program.key,
            token_out_mint: *accounts.token_out_mint.key,
            token_out_program: *accounts.token_out_program.key,
            user_token_in_account: *accounts.user_token_in_account.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            redemption_offer: *accounts.redemption_offer.key,
            redemption_vault_authority: *accounts.redemption_vault_authority.key,
            redemption_vault_token_in_account: *accounts
                .redemption_vault_token_in_account
                .key,
            offer_proceeds_vault: *accounts.offer_proceeds_vault.key,
            offer_proceeds_token_in_account: *accounts
                .offer_proceeds_token_in_account
                .key,
            offer_fee_vault: *accounts.offer_fee_vault.key,
            offer_fee_token_in_account: *accounts.offer_fee_token_in_account.key,
            mint_authority: *accounts.mint_authority.key,
            buffer_accounts_buffer_state: *accounts.buffer_accounts_buffer_state.key,
            buffer_accounts_reserve_vault_onyc_account: *accounts
                .buffer_accounts_reserve_vault_onyc_account
                .key,
            buffer_accounts_management_fee_vault_onyc_account: *accounts
                .buffer_accounts_management_fee_vault_onyc_account
                .key,
            buffer_accounts_performance_fee_vault_onyc_account: *accounts
                .buffer_accounts_performance_fee_vault_onyc_account
                .key,
            market_stats: *accounts.market_stats.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            user: *accounts.user.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            main_offer: *accounts.main_offer.key,
        }
    }
}
impl From<TakeOfferV2Keys> for [AccountMeta; TAKE_OFFER_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: TakeOfferV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_out_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redemption_vault_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_proceeds_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_proceeds_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.offer_fee_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_buffer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_management_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buffer_accounts_performance_fee_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.main_offer,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TAKE_OFFER_V2_IX_ACCOUNTS_LEN]> for TakeOfferV2Keys {
    fn from(pubkeys: [Pubkey; TAKE_OFFER_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            state: pubkeys[1],
            vault_authority: pubkeys[2],
            vault_token_in_account: pubkeys[3],
            vault_token_out_account: pubkeys[4],
            token_in_mint: pubkeys[5],
            token_in_program: pubkeys[6],
            token_out_mint: pubkeys[7],
            token_out_program: pubkeys[8],
            user_token_in_account: pubkeys[9],
            user_token_out_account: pubkeys[10],
            redemption_offer: pubkeys[11],
            redemption_vault_authority: pubkeys[12],
            redemption_vault_token_in_account: pubkeys[13],
            offer_proceeds_vault: pubkeys[14],
            offer_proceeds_token_in_account: pubkeys[15],
            offer_fee_vault: pubkeys[16],
            offer_fee_token_in_account: pubkeys[17],
            mint_authority: pubkeys[18],
            buffer_accounts_buffer_state: pubkeys[19],
            buffer_accounts_reserve_vault_onyc_account: pubkeys[20],
            buffer_accounts_management_fee_vault_onyc_account: pubkeys[21],
            buffer_accounts_performance_fee_vault_onyc_account: pubkeys[22],
            market_stats: pubkeys[23],
            circulating_supply_excluded_balance: pubkeys[24],
            instructions_sysvar: pubkeys[25],
            user: pubkeys[26],
            associated_token_program: pubkeys[27],
            system_program: pubkeys[28],
            main_offer: pubkeys[29],
        }
    }
}
impl<'info> From<TakeOfferV2Accounts<'_, 'info>>
for [AccountInfo<'info>; TAKE_OFFER_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: TakeOfferV2Accounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.state.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_token_in_account.clone(),
            accounts.vault_token_out_account.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_in_program.clone(),
            accounts.token_out_mint.clone(),
            accounts.token_out_program.clone(),
            accounts.user_token_in_account.clone(),
            accounts.user_token_out_account.clone(),
            accounts.redemption_offer.clone(),
            accounts.redemption_vault_authority.clone(),
            accounts.redemption_vault_token_in_account.clone(),
            accounts.offer_proceeds_vault.clone(),
            accounts.offer_proceeds_token_in_account.clone(),
            accounts.offer_fee_vault.clone(),
            accounts.offer_fee_token_in_account.clone(),
            accounts.mint_authority.clone(),
            accounts.buffer_accounts_buffer_state.clone(),
            accounts.buffer_accounts_reserve_vault_onyc_account.clone(),
            accounts.buffer_accounts_management_fee_vault_onyc_account.clone(),
            accounts.buffer_accounts_performance_fee_vault_onyc_account.clone(),
            accounts.market_stats.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.user.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.main_offer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TAKE_OFFER_V2_IX_ACCOUNTS_LEN]>
for TakeOfferV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TAKE_OFFER_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            state: &arr[1],
            vault_authority: &arr[2],
            vault_token_in_account: &arr[3],
            vault_token_out_account: &arr[4],
            token_in_mint: &arr[5],
            token_in_program: &arr[6],
            token_out_mint: &arr[7],
            token_out_program: &arr[8],
            user_token_in_account: &arr[9],
            user_token_out_account: &arr[10],
            redemption_offer: &arr[11],
            redemption_vault_authority: &arr[12],
            redemption_vault_token_in_account: &arr[13],
            offer_proceeds_vault: &arr[14],
            offer_proceeds_token_in_account: &arr[15],
            offer_fee_vault: &arr[16],
            offer_fee_token_in_account: &arr[17],
            mint_authority: &arr[18],
            buffer_accounts_buffer_state: &arr[19],
            buffer_accounts_reserve_vault_onyc_account: &arr[20],
            buffer_accounts_management_fee_vault_onyc_account: &arr[21],
            buffer_accounts_performance_fee_vault_onyc_account: &arr[22],
            market_stats: &arr[23],
            circulating_supply_excluded_balance: &arr[24],
            instructions_sysvar: &arr[25],
            user: &arr[26],
            associated_token_program: &arr[27],
            system_program: &arr[28],
            main_offer: &arr[29],
        }
    }
}
pub const TAKE_OFFER_V2_IX_DISCM: [u8; 8usize] = [203, 29, 22, 81, 189, 205, 210, 60];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TakeOfferV2IxArgs {
    pub token_in_amount: u64,
    pub approval_message: Option<ApprovalMessage>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TakeOfferV2IxData(pub TakeOfferV2IxArgs);
impl From<TakeOfferV2IxArgs> for TakeOfferV2IxData {
    fn from(args: TakeOfferV2IxArgs) -> Self {
        Self(args)
    }
}
impl TakeOfferV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TAKE_OFFER_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let approval_message: Option<ApprovalMessage> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(TakeOfferV2IxArgs {
                token_in_amount,
                approval_message,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TAKE_OFFER_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.approval_message, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn take_offer_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: TakeOfferV2Keys,
    args: TakeOfferV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TAKE_OFFER_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: TakeOfferV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn take_offer_v2_ix(
    keys: TakeOfferV2Keys,
    args: TakeOfferV2IxArgs,
) -> std::io::Result<Instruction> {
    take_offer_v2_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn take_offer_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TakeOfferV2Accounts<'_, '_>,
    args: TakeOfferV2IxArgs,
) -> ProgramResult {
    let keys: TakeOfferV2Keys = accounts.into();
    let ix = take_offer_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn take_offer_v2_invoke(
    accounts: TakeOfferV2Accounts<'_, '_>,
    args: TakeOfferV2IxArgs,
) -> ProgramResult {
    take_offer_v2_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn take_offer_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TakeOfferV2Accounts<'_, '_>,
    args: TakeOfferV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TakeOfferV2Keys = accounts.into();
    let ix = take_offer_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn take_offer_v2_invoke_signed(
    accounts: TakeOfferV2Accounts<'_, '_>,
    args: TakeOfferV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    take_offer_v2_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn take_offer_v2_verify_account_keys(
    accounts: TakeOfferV2Accounts<'_, '_>,
    keys: TakeOfferV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.state.key, keys.state),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_token_in_account.key, keys.vault_token_in_account),
        (*accounts.vault_token_out_account.key, keys.vault_token_out_account),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_in_program.key, keys.token_in_program),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.token_out_program.key, keys.token_out_program),
        (*accounts.user_token_in_account.key, keys.user_token_in_account),
        (*accounts.user_token_out_account.key, keys.user_token_out_account),
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.redemption_vault_authority.key, keys.redemption_vault_authority),
        (
            *accounts.redemption_vault_token_in_account.key,
            keys.redemption_vault_token_in_account,
        ),
        (*accounts.offer_proceeds_vault.key, keys.offer_proceeds_vault),
        (
            *accounts.offer_proceeds_token_in_account.key,
            keys.offer_proceeds_token_in_account,
        ),
        (*accounts.offer_fee_vault.key, keys.offer_fee_vault),
        (*accounts.offer_fee_token_in_account.key, keys.offer_fee_token_in_account),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.buffer_accounts_buffer_state.key, keys.buffer_accounts_buffer_state),
        (
            *accounts.buffer_accounts_reserve_vault_onyc_account.key,
            keys.buffer_accounts_reserve_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_management_fee_vault_onyc_account.key,
            keys.buffer_accounts_management_fee_vault_onyc_account,
        ),
        (
            *accounts.buffer_accounts_performance_fee_vault_onyc_account.key,
            keys.buffer_accounts_performance_fee_vault_onyc_account,
        ),
        (*accounts.market_stats.key, keys.market_stats),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.user.key, keys.user),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.main_offer.key, keys.main_offer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn take_offer_v2_verify_writable_privileges<'me, 'info>(
    accounts: TakeOfferV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.offer,
        accounts.vault_token_in_account,
        accounts.vault_token_out_account,
        accounts.token_in_mint,
        accounts.token_out_mint,
        accounts.user_token_in_account,
        accounts.user_token_out_account,
        accounts.redemption_vault_token_in_account,
        accounts.offer_proceeds_vault,
        accounts.offer_proceeds_token_in_account,
        accounts.offer_fee_vault,
        accounts.offer_fee_token_in_account,
        accounts.buffer_accounts_buffer_state,
        accounts.buffer_accounts_reserve_vault_onyc_account,
        accounts.buffer_accounts_management_fee_vault_onyc_account,
        accounts.buffer_accounts_performance_fee_vault_onyc_account,
        accounts.market_stats,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn take_offer_v2_verify_signer_privileges<'me, 'info>(
    accounts: TakeOfferV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn take_offer_v2_verify_account_privileges<'me, 'info>(
    accounts: TakeOfferV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    take_offer_v2_verify_writable_privileges(accounts)?;
    take_offer_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct TransferMintAuthorityToBossAccounts<'me, 'info> {
    pub boss: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferMintAuthorityToBossKeys {
    pub boss: Pubkey,
    pub state: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<TransferMintAuthorityToBossAccounts<'_, '_>>
for TransferMintAuthorityToBossKeys {
    fn from(accounts: TransferMintAuthorityToBossAccounts) -> Self {
        Self {
            boss: *accounts.boss.key,
            state: *accounts.state.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<TransferMintAuthorityToBossKeys>
for [AccountMeta; TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferMintAuthorityToBossKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
impl From<[Pubkey; TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_ACCOUNTS_LEN]>
for TransferMintAuthorityToBossKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            boss: pubkeys[0],
            state: pubkeys[1],
            mint: pubkeys[2],
            mint_authority: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<TransferMintAuthorityToBossAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferMintAuthorityToBossAccounts<'_, 'info>) -> Self {
        [
            accounts.boss.clone(),
            accounts.state.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_ACCOUNTS_LEN]>
for TransferMintAuthorityToBossAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            boss: &arr[0],
            state: &arr[1],
            mint: &arr[2],
            mint_authority: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_DISCM: [u8; 8usize] = [
    197, 61, 42, 52, 70, 93, 30, 125,
];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferMintAuthorityToBossIxData;
impl TransferMintAuthorityToBossIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_mint_authority_to_boss_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferMintAuthorityToBossKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_MINT_AUTHORITY_TO_BOSS_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferMintAuthorityToBossIxData.try_to_vec()?,
    })
}
pub fn transfer_mint_authority_to_boss_ix(
    keys: TransferMintAuthorityToBossKeys,
) -> std::io::Result<Instruction> {
    transfer_mint_authority_to_boss_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn transfer_mint_authority_to_boss_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferMintAuthorityToBossAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferMintAuthorityToBossKeys = accounts.into();
    let ix = transfer_mint_authority_to_boss_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_mint_authority_to_boss_invoke(
    accounts: TransferMintAuthorityToBossAccounts<'_, '_>,
) -> ProgramResult {
    transfer_mint_authority_to_boss_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts)
}
pub fn transfer_mint_authority_to_boss_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferMintAuthorityToBossAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferMintAuthorityToBossKeys = accounts.into();
    let ix = transfer_mint_authority_to_boss_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_mint_authority_to_boss_invoke_signed(
    accounts: TransferMintAuthorityToBossAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_mint_authority_to_boss_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn transfer_mint_authority_to_boss_verify_account_keys(
    accounts: TransferMintAuthorityToBossAccounts<'_, '_>,
    keys: TransferMintAuthorityToBossKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.boss.key, keys.boss),
        (*accounts.state.key, keys.state),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_mint_authority_to_boss_verify_writable_privileges<'me, 'info>(
    accounts: TransferMintAuthorityToBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_mint_authority_to_boss_verify_signer_privileges<'me, 'info>(
    accounts: TransferMintAuthorityToBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_mint_authority_to_boss_verify_account_privileges<'me, 'info>(
    accounts: TransferMintAuthorityToBossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_mint_authority_to_boss_verify_writable_privileges(accounts)?;
    transfer_mint_authority_to_boss_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct TransferMintAuthorityToProgramAccounts<'me, 'info> {
    pub boss: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferMintAuthorityToProgramKeys {
    pub boss: Pubkey,
    pub state: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<TransferMintAuthorityToProgramAccounts<'_, '_>>
for TransferMintAuthorityToProgramKeys {
    fn from(accounts: TransferMintAuthorityToProgramAccounts) -> Self {
        Self {
            boss: *accounts.boss.key,
            state: *accounts.state.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<TransferMintAuthorityToProgramKeys>
for [AccountMeta; TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferMintAuthorityToProgramKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
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
impl From<[Pubkey; TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_ACCOUNTS_LEN]>
for TransferMintAuthorityToProgramKeys {
    fn from(
        pubkeys: [Pubkey; TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            boss: pubkeys[0],
            state: pubkeys[1],
            mint: pubkeys[2],
            mint_authority: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<TransferMintAuthorityToProgramAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferMintAuthorityToProgramAccounts<'_, 'info>) -> Self {
        [
            accounts.boss.clone(),
            accounts.state.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_ACCOUNTS_LEN]>
for TransferMintAuthorityToProgramAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            boss: &arr[0],
            state: &arr[1],
            mint: &arr[2],
            mint_authority: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_DISCM: [u8; 8usize] = [
    98, 112, 50, 135, 53, 6, 149, 232,
];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferMintAuthorityToProgramIxData;
impl TransferMintAuthorityToProgramIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_mint_authority_to_program_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferMintAuthorityToProgramKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_MINT_AUTHORITY_TO_PROGRAM_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferMintAuthorityToProgramIxData.try_to_vec()?,
    })
}
pub fn transfer_mint_authority_to_program_ix(
    keys: TransferMintAuthorityToProgramKeys,
) -> std::io::Result<Instruction> {
    transfer_mint_authority_to_program_ix_with_program_id(ONREAPP_PROGRAM_ID, keys)
}
pub fn transfer_mint_authority_to_program_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferMintAuthorityToProgramAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferMintAuthorityToProgramKeys = accounts.into();
    let ix = transfer_mint_authority_to_program_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_mint_authority_to_program_invoke(
    accounts: TransferMintAuthorityToProgramAccounts<'_, '_>,
) -> ProgramResult {
    transfer_mint_authority_to_program_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
    )
}
pub fn transfer_mint_authority_to_program_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferMintAuthorityToProgramAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferMintAuthorityToProgramKeys = accounts.into();
    let ix = transfer_mint_authority_to_program_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_mint_authority_to_program_invoke_signed(
    accounts: TransferMintAuthorityToProgramAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_mint_authority_to_program_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn transfer_mint_authority_to_program_verify_account_keys(
    accounts: TransferMintAuthorityToProgramAccounts<'_, '_>,
    keys: TransferMintAuthorityToProgramKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.boss.key, keys.boss),
        (*accounts.state.key, keys.state),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_mint_authority_to_program_verify_writable_privileges<'me, 'info>(
    accounts: TransferMintAuthorityToProgramAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_mint_authority_to_program_verify_signer_privileges<'me, 'info>(
    accounts: TransferMintAuthorityToProgramAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_mint_authority_to_program_verify_account_privileges<'me, 'info>(
    accounts: TransferMintAuthorityToProgramAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_mint_authority_to_program_verify_writable_privileges(accounts)?;
    transfer_mint_authority_to_program_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCirculatingSupplyExcludedBalanceAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub excluded_accounts: &'me AccountInfo<'info>,
    pub circulating_supply_excluded_balance: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCirculatingSupplyExcludedBalanceKeys {
    pub state: Pubkey,
    pub onyc_mint: Pubkey,
    pub excluded_accounts: Pubkey,
    pub circulating_supply_excluded_balance: Pubkey,
    pub token_program: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateCirculatingSupplyExcludedBalanceAccounts<'_, '_>>
for UpdateCirculatingSupplyExcludedBalanceKeys {
    fn from(accounts: UpdateCirculatingSupplyExcludedBalanceAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            onyc_mint: *accounts.onyc_mint.key,
            excluded_accounts: *accounts.excluded_accounts.key,
            circulating_supply_excluded_balance: *accounts
                .circulating_supply_excluded_balance
                .key,
            token_program: *accounts.token_program.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateCirculatingSupplyExcludedBalanceKeys>
for [AccountMeta; UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCirculatingSupplyExcludedBalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.excluded_accounts,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.circulating_supply_excluded_balance,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
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
impl From<[Pubkey; UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_ACCOUNTS_LEN]>
for UpdateCirculatingSupplyExcludedBalanceKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: pubkeys[0],
            onyc_mint: pubkeys[1],
            excluded_accounts: pubkeys[2],
            circulating_supply_excluded_balance: pubkeys[3],
            token_program: pubkeys[4],
            signer: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<UpdateCirculatingSupplyExcludedBalanceAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.state.clone(),
            accounts.onyc_mint.clone(),
            accounts.excluded_accounts.clone(),
            accounts.circulating_supply_excluded_balance.clone(),
            accounts.token_program.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_ACCOUNTS_LEN],
> for UpdateCirculatingSupplyExcludedBalanceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            onyc_mint: &arr[1],
            excluded_accounts: &arr[2],
            circulating_supply_excluded_balance: &arr[3],
            token_program: &arr[4],
            signer: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_DISCM: [u8; 8usize] = [
    3, 26, 180, 255, 32, 146, 247, 85,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCirculatingSupplyExcludedBalanceIxData;
impl UpdateCirculatingSupplyExcludedBalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_circulating_supply_excluded_balance_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCirculatingSupplyExcludedBalanceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CIRCULATING_SUPPLY_EXCLUDED_BALANCE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateCirculatingSupplyExcludedBalanceIxData.try_to_vec()?,
    })
}
pub fn update_circulating_supply_excluded_balance_ix(
    keys: UpdateCirculatingSupplyExcludedBalanceKeys,
) -> std::io::Result<Instruction> {
    update_circulating_supply_excluded_balance_ix_with_program_id(
        ONREAPP_PROGRAM_ID,
        keys,
    )
}
pub fn update_circulating_supply_excluded_balance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateCirculatingSupplyExcludedBalanceKeys = accounts.into();
    let ix = update_circulating_supply_excluded_balance_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_circulating_supply_excluded_balance_invoke(
    accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'_, '_>,
) -> ProgramResult {
    update_circulating_supply_excluded_balance_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
    )
}
pub fn update_circulating_supply_excluded_balance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCirculatingSupplyExcludedBalanceKeys = accounts.into();
    let ix = update_circulating_supply_excluded_balance_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_circulating_supply_excluded_balance_invoke_signed(
    accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_circulating_supply_excluded_balance_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_circulating_supply_excluded_balance_verify_account_keys(
    accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'_, '_>,
    keys: UpdateCirculatingSupplyExcludedBalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.excluded_accounts.key, keys.excluded_accounts),
        (
            *accounts.circulating_supply_excluded_balance.key,
            keys.circulating_supply_excluded_balance,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_circulating_supply_excluded_balance_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.circulating_supply_excluded_balance,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_circulating_supply_excluded_balance_verify_signer_privileges<'me, 'info>(
    accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_circulating_supply_excluded_balance_verify_account_privileges<'me, 'info>(
    accounts: UpdateCirculatingSupplyExcludedBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_circulating_supply_excluded_balance_verify_writable_privileges(accounts)?;
    update_circulating_supply_excluded_balance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_OFFER_FEE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOfferFeeAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOfferFeeKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<UpdateOfferFeeAccounts<'_, '_>> for UpdateOfferFeeKeys {
    fn from(accounts: UpdateOfferFeeAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<UpdateOfferFeeKeys> for [AccountMeta; UPDATE_OFFER_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOfferFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_OFFER_FEE_IX_ACCOUNTS_LEN]> for UpdateOfferFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_OFFER_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
            state: pubkeys[3],
            boss: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateOfferFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_OFFER_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOfferFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_OFFER_FEE_IX_ACCOUNTS_LEN]>
for UpdateOfferFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_OFFER_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
            state: &arr[3],
            boss: &arr[4],
        }
    }
}
pub const UPDATE_OFFER_FEE_IX_DISCM: [u8; 8usize] = [
    254, 162, 70, 248, 117, 70, 197, 118,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOfferFeeIxArgs {
    pub new_fee_basis_points: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOfferFeeIxData(pub UpdateOfferFeeIxArgs);
impl From<UpdateOfferFeeIxArgs> for UpdateOfferFeeIxData {
    fn from(args: UpdateOfferFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOfferFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_OFFER_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateOfferFeeIxArgs {
                new_fee_basis_points,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_OFFER_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_fee_basis_points, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_offer_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOfferFeeKeys,
    args: UpdateOfferFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_OFFER_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOfferFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_offer_fee_ix(
    keys: UpdateOfferFeeKeys,
    args: UpdateOfferFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_offer_fee_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn update_offer_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOfferFeeAccounts<'_, '_>,
    args: UpdateOfferFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateOfferFeeKeys = accounts.into();
    let ix = update_offer_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_offer_fee_invoke(
    accounts: UpdateOfferFeeAccounts<'_, '_>,
    args: UpdateOfferFeeIxArgs,
) -> ProgramResult {
    update_offer_fee_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn update_offer_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOfferFeeAccounts<'_, '_>,
    args: UpdateOfferFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOfferFeeKeys = accounts.into();
    let ix = update_offer_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_offer_fee_invoke_signed(
    accounts: UpdateOfferFeeAccounts<'_, '_>,
    args: UpdateOfferFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_offer_fee_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_offer_fee_verify_account_keys(
    accounts: UpdateOfferFeeAccounts<'_, '_>,
    keys: UpdateOfferFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_offer_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOfferFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_offer_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOfferFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_offer_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateOfferFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_offer_fee_verify_writable_privileges(accounts)?;
    update_offer_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_OFFER_PERMISSIONLESS_FEE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOfferPermissionlessFeeAccounts<'me, 'info> {
    pub offer: &'me AccountInfo<'info>,
    pub token_in_mint: &'me AccountInfo<'info>,
    pub token_out_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOfferPermissionlessFeeKeys {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<UpdateOfferPermissionlessFeeAccounts<'_, '_>>
for UpdateOfferPermissionlessFeeKeys {
    fn from(accounts: UpdateOfferPermissionlessFeeAccounts) -> Self {
        Self {
            offer: *accounts.offer.key,
            token_in_mint: *accounts.token_in_mint.key,
            token_out_mint: *accounts.token_out_mint.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<UpdateOfferPermissionlessFeeKeys>
for [AccountMeta; UPDATE_OFFER_PERMISSIONLESS_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOfferPermissionlessFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_in_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_out_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_OFFER_PERMISSIONLESS_FEE_IX_ACCOUNTS_LEN]>
for UpdateOfferPermissionlessFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_OFFER_PERMISSIONLESS_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            offer: pubkeys[0],
            token_in_mint: pubkeys[1],
            token_out_mint: pubkeys[2],
            state: pubkeys[3],
            boss: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateOfferPermissionlessFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_OFFER_PERMISSIONLESS_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOfferPermissionlessFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.offer.clone(),
            accounts.token_in_mint.clone(),
            accounts.token_out_mint.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_OFFER_PERMISSIONLESS_FEE_IX_ACCOUNTS_LEN]>
for UpdateOfferPermissionlessFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_OFFER_PERMISSIONLESS_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            offer: &arr[0],
            token_in_mint: &arr[1],
            token_out_mint: &arr[2],
            state: &arr[3],
            boss: &arr[4],
        }
    }
}
pub const UPDATE_OFFER_PERMISSIONLESS_FEE_IX_DISCM: [u8; 8usize] = [
    211, 4, 141, 85, 85, 236, 195, 34,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOfferPermissionlessFeeIxArgs {
    pub new_fee_basis_points_permissionless: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOfferPermissionlessFeeIxData(pub UpdateOfferPermissionlessFeeIxArgs);
impl From<UpdateOfferPermissionlessFeeIxArgs> for UpdateOfferPermissionlessFeeIxData {
    fn from(args: UpdateOfferPermissionlessFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOfferPermissionlessFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_OFFER_PERMISSIONLESS_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_fee_basis_points_permissionless: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateOfferPermissionlessFeeIxArgs {
                new_fee_basis_points_permissionless,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_OFFER_PERMISSIONLESS_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_fee_basis_points_permissionless,
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
pub fn update_offer_permissionless_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOfferPermissionlessFeeKeys,
    args: UpdateOfferPermissionlessFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_OFFER_PERMISSIONLESS_FEE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateOfferPermissionlessFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_offer_permissionless_fee_ix(
    keys: UpdateOfferPermissionlessFeeKeys,
    args: UpdateOfferPermissionlessFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_offer_permissionless_fee_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn update_offer_permissionless_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOfferPermissionlessFeeAccounts<'_, '_>,
    args: UpdateOfferPermissionlessFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateOfferPermissionlessFeeKeys = accounts.into();
    let ix = update_offer_permissionless_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_offer_permissionless_fee_invoke(
    accounts: UpdateOfferPermissionlessFeeAccounts<'_, '_>,
    args: UpdateOfferPermissionlessFeeIxArgs,
) -> ProgramResult {
    update_offer_permissionless_fee_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_offer_permissionless_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOfferPermissionlessFeeAccounts<'_, '_>,
    args: UpdateOfferPermissionlessFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOfferPermissionlessFeeKeys = accounts.into();
    let ix = update_offer_permissionless_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_offer_permissionless_fee_invoke_signed(
    accounts: UpdateOfferPermissionlessFeeAccounts<'_, '_>,
    args: UpdateOfferPermissionlessFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_offer_permissionless_fee_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_offer_permissionless_fee_verify_account_keys(
    accounts: UpdateOfferPermissionlessFeeAccounts<'_, '_>,
    keys: UpdateOfferPermissionlessFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.offer.key, keys.offer),
        (*accounts.token_in_mint.key, keys.token_in_mint),
        (*accounts.token_out_mint.key, keys.token_out_mint),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_offer_permissionless_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOfferPermissionlessFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_offer_permissionless_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOfferPermissionlessFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_offer_permissionless_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateOfferPermissionlessFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_offer_permissionless_fee_verify_writable_privileges(accounts)?;
    update_offer_permissionless_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REDEMPTION_OFFER_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRedemptionOfferFeeAccounts<'me, 'info> {
    pub redemption_offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRedemptionOfferFeeKeys {
    pub redemption_offer: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<UpdateRedemptionOfferFeeAccounts<'_, '_>> for UpdateRedemptionOfferFeeKeys {
    fn from(accounts: UpdateRedemptionOfferFeeAccounts) -> Self {
        Self {
            redemption_offer: *accounts.redemption_offer.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<UpdateRedemptionOfferFeeKeys>
for [AccountMeta; UPDATE_REDEMPTION_OFFER_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRedemptionOfferFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REDEMPTION_OFFER_FEE_IX_ACCOUNTS_LEN]>
for UpdateRedemptionOfferFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REDEMPTION_OFFER_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            redemption_offer: pubkeys[0],
            state: pubkeys[1],
            boss: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateRedemptionOfferFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REDEMPTION_OFFER_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRedemptionOfferFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.redemption_offer.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_REDEMPTION_OFFER_FEE_IX_ACCOUNTS_LEN]>
for UpdateRedemptionOfferFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REDEMPTION_OFFER_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            redemption_offer: &arr[0],
            state: &arr[1],
            boss: &arr[2],
        }
    }
}
pub const UPDATE_REDEMPTION_OFFER_FEE_IX_DISCM: [u8; 8usize] = [
    73, 11, 35, 194, 219, 147, 159, 3,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRedemptionOfferFeeIxArgs {
    pub new_fee_basis_points: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRedemptionOfferFeeIxData(pub UpdateRedemptionOfferFeeIxArgs);
impl From<UpdateRedemptionOfferFeeIxArgs> for UpdateRedemptionOfferFeeIxData {
    fn from(args: UpdateRedemptionOfferFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRedemptionOfferFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REDEMPTION_OFFER_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRedemptionOfferFeeIxArgs {
                new_fee_basis_points,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REDEMPTION_OFFER_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_fee_basis_points, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_redemption_offer_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRedemptionOfferFeeKeys,
    args: UpdateRedemptionOfferFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REDEMPTION_OFFER_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRedemptionOfferFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_redemption_offer_fee_ix(
    keys: UpdateRedemptionOfferFeeKeys,
    args: UpdateRedemptionOfferFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_redemption_offer_fee_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn update_redemption_offer_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRedemptionOfferFeeAccounts<'_, '_>,
    args: UpdateRedemptionOfferFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateRedemptionOfferFeeKeys = accounts.into();
    let ix = update_redemption_offer_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_redemption_offer_fee_invoke(
    accounts: UpdateRedemptionOfferFeeAccounts<'_, '_>,
    args: UpdateRedemptionOfferFeeIxArgs,
) -> ProgramResult {
    update_redemption_offer_fee_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_redemption_offer_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRedemptionOfferFeeAccounts<'_, '_>,
    args: UpdateRedemptionOfferFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRedemptionOfferFeeKeys = accounts.into();
    let ix = update_redemption_offer_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_redemption_offer_fee_invoke_signed(
    accounts: UpdateRedemptionOfferFeeAccounts<'_, '_>,
    args: UpdateRedemptionOfferFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_redemption_offer_fee_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_redemption_offer_fee_verify_account_keys(
    accounts: UpdateRedemptionOfferFeeAccounts<'_, '_>,
    keys: UpdateRedemptionOfferFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.redemption_offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_redemption_offer_fee_verify_writable_privileges(accounts)?;
    update_redemption_offer_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRedemptionOfferPropAmmSellFeeAccounts<'me, 'info> {
    pub redemption_offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRedemptionOfferPropAmmSellFeeKeys {
    pub redemption_offer: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<UpdateRedemptionOfferPropAmmSellFeeAccounts<'_, '_>>
for UpdateRedemptionOfferPropAmmSellFeeKeys {
    fn from(accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts) -> Self {
        Self {
            redemption_offer: *accounts.redemption_offer.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<UpdateRedemptionOfferPropAmmSellFeeKeys>
for [AccountMeta; UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRedemptionOfferPropAmmSellFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_ACCOUNTS_LEN]>
for UpdateRedemptionOfferPropAmmSellFeeKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            redemption_offer: pubkeys[0],
            state: pubkeys[1],
            boss: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateRedemptionOfferPropAmmSellFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.redemption_offer.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_ACCOUNTS_LEN],
> for UpdateRedemptionOfferPropAmmSellFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            redemption_offer: &arr[0],
            state: &arr[1],
            boss: &arr[2],
        }
    }
}
pub const UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_DISCM: [u8; 8usize] = [
    160, 118, 162, 32, 102, 148, 239, 5,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRedemptionOfferPropAmmSellFeeIxArgs {
    pub new_fee_basis_points_prop_amm_sell: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRedemptionOfferPropAmmSellFeeIxData(
    pub UpdateRedemptionOfferPropAmmSellFeeIxArgs,
);
impl From<UpdateRedemptionOfferPropAmmSellFeeIxArgs>
for UpdateRedemptionOfferPropAmmSellFeeIxData {
    fn from(args: UpdateRedemptionOfferPropAmmSellFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRedemptionOfferPropAmmSellFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_fee_basis_points_prop_amm_sell: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateRedemptionOfferPropAmmSellFeeIxArgs {
                new_fee_basis_points_prop_amm_sell,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_fee_basis_points_prop_amm_sell,
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
pub fn update_redemption_offer_prop_amm_sell_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRedemptionOfferPropAmmSellFeeKeys,
    args: UpdateRedemptionOfferPropAmmSellFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REDEMPTION_OFFER_PROP_AMM_SELL_FEE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateRedemptionOfferPropAmmSellFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_redemption_offer_prop_amm_sell_fee_ix(
    keys: UpdateRedemptionOfferPropAmmSellFeeKeys,
    args: UpdateRedemptionOfferPropAmmSellFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_redemption_offer_prop_amm_sell_fee_ix_with_program_id(
        ONREAPP_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_redemption_offer_prop_amm_sell_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'_, '_>,
    args: UpdateRedemptionOfferPropAmmSellFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateRedemptionOfferPropAmmSellFeeKeys = accounts.into();
    let ix = update_redemption_offer_prop_amm_sell_fee_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_redemption_offer_prop_amm_sell_fee_invoke(
    accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'_, '_>,
    args: UpdateRedemptionOfferPropAmmSellFeeIxArgs,
) -> ProgramResult {
    update_redemption_offer_prop_amm_sell_fee_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_redemption_offer_prop_amm_sell_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'_, '_>,
    args: UpdateRedemptionOfferPropAmmSellFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRedemptionOfferPropAmmSellFeeKeys = accounts.into();
    let ix = update_redemption_offer_prop_amm_sell_fee_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_redemption_offer_prop_amm_sell_fee_invoke_signed(
    accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'_, '_>,
    args: UpdateRedemptionOfferPropAmmSellFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_redemption_offer_prop_amm_sell_fee_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_redemption_offer_prop_amm_sell_fee_verify_account_keys(
    accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'_, '_>,
    keys: UpdateRedemptionOfferPropAmmSellFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_prop_amm_sell_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.redemption_offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_prop_amm_sell_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_prop_amm_sell_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferPropAmmSellFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_redemption_offer_prop_amm_sell_fee_verify_writable_privileges(accounts)?;
    update_redemption_offer_prop_amm_sell_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRedemptionOfferVaultTargetAccounts<'me, 'info> {
    pub redemption_offer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRedemptionOfferVaultTargetKeys {
    pub redemption_offer: Pubkey,
    pub state: Pubkey,
    pub boss: Pubkey,
}
impl From<UpdateRedemptionOfferVaultTargetAccounts<'_, '_>>
for UpdateRedemptionOfferVaultTargetKeys {
    fn from(accounts: UpdateRedemptionOfferVaultTargetAccounts) -> Self {
        Self {
            redemption_offer: *accounts.redemption_offer.key,
            state: *accounts.state.key,
            boss: *accounts.boss.key,
        }
    }
}
impl From<UpdateRedemptionOfferVaultTargetKeys>
for [AccountMeta; UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRedemptionOfferVaultTargetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.redemption_offer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_ACCOUNTS_LEN]>
for UpdateRedemptionOfferVaultTargetKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            redemption_offer: pubkeys[0],
            state: pubkeys[1],
            boss: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateRedemptionOfferVaultTargetAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRedemptionOfferVaultTargetAccounts<'_, 'info>) -> Self {
        [
            accounts.redemption_offer.clone(),
            accounts.state.clone(),
            accounts.boss.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_ACCOUNTS_LEN]>
for UpdateRedemptionOfferVaultTargetAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            redemption_offer: &arr[0],
            state: &arr[1],
            boss: &arr[2],
        }
    }
}
pub const UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_DISCM: [u8; 8usize] = [
    182, 79, 179, 178, 68, 172, 197, 1,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRedemptionOfferVaultTargetIxArgs {
    pub new_vault_target_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRedemptionOfferVaultTargetIxData(
    pub UpdateRedemptionOfferVaultTargetIxArgs,
);
impl From<UpdateRedemptionOfferVaultTargetIxArgs>
for UpdateRedemptionOfferVaultTargetIxData {
    fn from(args: UpdateRedemptionOfferVaultTargetIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRedemptionOfferVaultTargetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_vault_target_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRedemptionOfferVaultTargetIxArgs {
                new_vault_target_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_vault_target_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_redemption_offer_vault_target_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRedemptionOfferVaultTargetKeys,
    args: UpdateRedemptionOfferVaultTargetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REDEMPTION_OFFER_VAULT_TARGET_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateRedemptionOfferVaultTargetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_redemption_offer_vault_target_ix(
    keys: UpdateRedemptionOfferVaultTargetKeys,
    args: UpdateRedemptionOfferVaultTargetIxArgs,
) -> std::io::Result<Instruction> {
    update_redemption_offer_vault_target_ix_with_program_id(
        ONREAPP_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_redemption_offer_vault_target_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRedemptionOfferVaultTargetAccounts<'_, '_>,
    args: UpdateRedemptionOfferVaultTargetIxArgs,
) -> ProgramResult {
    let keys: UpdateRedemptionOfferVaultTargetKeys = accounts.into();
    let ix = update_redemption_offer_vault_target_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_redemption_offer_vault_target_invoke(
    accounts: UpdateRedemptionOfferVaultTargetAccounts<'_, '_>,
    args: UpdateRedemptionOfferVaultTargetIxArgs,
) -> ProgramResult {
    update_redemption_offer_vault_target_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_redemption_offer_vault_target_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRedemptionOfferVaultTargetAccounts<'_, '_>,
    args: UpdateRedemptionOfferVaultTargetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRedemptionOfferVaultTargetKeys = accounts.into();
    let ix = update_redemption_offer_vault_target_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_redemption_offer_vault_target_invoke_signed(
    accounts: UpdateRedemptionOfferVaultTargetAccounts<'_, '_>,
    args: UpdateRedemptionOfferVaultTargetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_redemption_offer_vault_target_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_redemption_offer_vault_target_verify_account_keys(
    accounts: UpdateRedemptionOfferVaultTargetAccounts<'_, '_>,
    keys: UpdateRedemptionOfferVaultTargetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.redemption_offer.key, keys.redemption_offer),
        (*accounts.state.key, keys.state),
        (*accounts.boss.key, keys.boss),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_vault_target_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferVaultTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.redemption_offer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_vault_target_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferVaultTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_redemption_offer_vault_target_verify_account_privileges<'me, 'info>(
    accounts: UpdateRedemptionOfferVaultTargetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_redemption_offer_vault_target_verify_writable_privileges(accounts)?;
    update_redemption_offer_vault_target_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_CONFIGURABLE_VAULT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawConfigurableVaultAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub caller: &'me AccountInfo<'info>,
    pub configurable_vault: &'me AccountInfo<'info>,
    pub vault_token_account: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawConfigurableVaultKeys {
    pub state: Pubkey,
    pub caller: Pubkey,
    pub configurable_vault: Pubkey,
    pub vault_token_account: Pubkey,
    pub destination: Pubkey,
    pub destination_token_account: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<WithdrawConfigurableVaultAccounts<'_, '_>> for WithdrawConfigurableVaultKeys {
    fn from(accounts: WithdrawConfigurableVaultAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            caller: *accounts.caller.key,
            configurable_vault: *accounts.configurable_vault.key,
            vault_token_account: *accounts.vault_token_account.key,
            destination: *accounts.destination.key,
            destination_token_account: *accounts.destination_token_account.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<WithdrawConfigurableVaultKeys>
for [AccountMeta; WITHDRAW_CONFIGURABLE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawConfigurableVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.caller,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.configurable_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
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
impl From<[Pubkey; WITHDRAW_CONFIGURABLE_VAULT_IX_ACCOUNTS_LEN]>
for WithdrawConfigurableVaultKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_CONFIGURABLE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            caller: pubkeys[1],
            configurable_vault: pubkeys[2],
            vault_token_account: pubkeys[3],
            destination: pubkeys[4],
            destination_token_account: pubkeys[5],
            mint: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawConfigurableVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_CONFIGURABLE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawConfigurableVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.caller.clone(),
            accounts.configurable_vault.clone(),
            accounts.vault_token_account.clone(),
            accounts.destination.clone(),
            accounts.destination_token_account.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_CONFIGURABLE_VAULT_IX_ACCOUNTS_LEN]>
for WithdrawConfigurableVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_CONFIGURABLE_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            caller: &arr[1],
            configurable_vault: &arr[2],
            vault_token_account: &arr[3],
            destination: &arr[4],
            destination_token_account: &arr[5],
            mint: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const WITHDRAW_CONFIGURABLE_VAULT_IX_DISCM: [u8; 8usize] = [
    255, 81, 28, 61, 192, 180, 29, 13,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawConfigurableVaultIxArgs {
    pub kind: ConfigurableVaultKind,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawConfigurableVaultIxData(pub WithdrawConfigurableVaultIxArgs);
impl From<WithdrawConfigurableVaultIxArgs> for WithdrawConfigurableVaultIxData {
    fn from(args: WithdrawConfigurableVaultIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawConfigurableVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_CONFIGURABLE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kind: ConfigurableVaultKind = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawConfigurableVaultIxArgs {
                kind,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_CONFIGURABLE_VAULT_IX_DISCM)?;
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
pub fn withdraw_configurable_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawConfigurableVaultKeys,
    args: WithdrawConfigurableVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_CONFIGURABLE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawConfigurableVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_configurable_vault_ix(
    keys: WithdrawConfigurableVaultKeys,
    args: WithdrawConfigurableVaultIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_configurable_vault_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn withdraw_configurable_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawConfigurableVaultAccounts<'_, '_>,
    args: WithdrawConfigurableVaultIxArgs,
) -> ProgramResult {
    let keys: WithdrawConfigurableVaultKeys = accounts.into();
    let ix = withdraw_configurable_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_configurable_vault_invoke(
    accounts: WithdrawConfigurableVaultAccounts<'_, '_>,
    args: WithdrawConfigurableVaultIxArgs,
) -> ProgramResult {
    withdraw_configurable_vault_invoke_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_configurable_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawConfigurableVaultAccounts<'_, '_>,
    args: WithdrawConfigurableVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawConfigurableVaultKeys = accounts.into();
    let ix = withdraw_configurable_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_configurable_vault_invoke_signed(
    accounts: WithdrawConfigurableVaultAccounts<'_, '_>,
    args: WithdrawConfigurableVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_configurable_vault_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_configurable_vault_verify_account_keys(
    accounts: WithdrawConfigurableVaultAccounts<'_, '_>,
    keys: WithdrawConfigurableVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.caller.key, keys.caller),
        (*accounts.configurable_vault.key, keys.configurable_vault),
        (*accounts.vault_token_account.key, keys.vault_token_account),
        (*accounts.destination.key, keys.destination),
        (*accounts.destination_token_account.key, keys.destination_token_account),
        (*accounts.mint.key, keys.mint),
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
pub fn withdraw_configurable_vault_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawConfigurableVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.caller,
        accounts.vault_token_account,
        accounts.destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_configurable_vault_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawConfigurableVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.caller] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_configurable_vault_verify_account_privileges<'me, 'info>(
    accounts: WithdrawConfigurableVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_configurable_vault_verify_writable_privileges(accounts)?;
    withdraw_configurable_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_RESERVE_VAULT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawReserveVaultAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub buffer_state: &'me AccountInfo<'info>,
    pub reserve_vault_authority: &'me AccountInfo<'info>,
    pub onyc_mint: &'me AccountInfo<'info>,
    pub boss_onyc_account: &'me AccountInfo<'info>,
    pub reserve_vault_onyc_account: &'me AccountInfo<'info>,
    pub boss: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawReserveVaultKeys {
    pub state: Pubkey,
    pub buffer_state: Pubkey,
    pub reserve_vault_authority: Pubkey,
    pub onyc_mint: Pubkey,
    pub boss_onyc_account: Pubkey,
    pub reserve_vault_onyc_account: Pubkey,
    pub boss: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<WithdrawReserveVaultAccounts<'_, '_>> for WithdrawReserveVaultKeys {
    fn from(accounts: WithdrawReserveVaultAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            buffer_state: *accounts.buffer_state.key,
            reserve_vault_authority: *accounts.reserve_vault_authority.key,
            onyc_mint: *accounts.onyc_mint.key,
            boss_onyc_account: *accounts.boss_onyc_account.key,
            reserve_vault_onyc_account: *accounts.reserve_vault_onyc_account.key,
            boss: *accounts.boss.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<WithdrawReserveVaultKeys>
for [AccountMeta; WITHDRAW_RESERVE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawReserveVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buffer_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onyc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.boss_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_vault_onyc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.boss,
                is_signer: true,
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
impl From<[Pubkey; WITHDRAW_RESERVE_VAULT_IX_ACCOUNTS_LEN]>
for WithdrawReserveVaultKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_RESERVE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            buffer_state: pubkeys[1],
            reserve_vault_authority: pubkeys[2],
            onyc_mint: pubkeys[3],
            boss_onyc_account: pubkeys[4],
            reserve_vault_onyc_account: pubkeys[5],
            boss: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawReserveVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_RESERVE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawReserveVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.buffer_state.clone(),
            accounts.reserve_vault_authority.clone(),
            accounts.onyc_mint.clone(),
            accounts.boss_onyc_account.clone(),
            accounts.reserve_vault_onyc_account.clone(),
            accounts.boss.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_RESERVE_VAULT_IX_ACCOUNTS_LEN]>
for WithdrawReserveVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_RESERVE_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            buffer_state: &arr[1],
            reserve_vault_authority: &arr[2],
            onyc_mint: &arr[3],
            boss_onyc_account: &arr[4],
            reserve_vault_onyc_account: &arr[5],
            boss: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const WITHDRAW_RESERVE_VAULT_IX_DISCM: [u8; 8usize] = [
    224, 37, 127, 12, 213, 154, 179, 98,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawReserveVaultIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawReserveVaultIxData(pub WithdrawReserveVaultIxArgs);
impl From<WithdrawReserveVaultIxArgs> for WithdrawReserveVaultIxData {
    fn from(args: WithdrawReserveVaultIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawReserveVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_RESERVE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawReserveVaultIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_RESERVE_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_reserve_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawReserveVaultKeys,
    args: WithdrawReserveVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_RESERVE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawReserveVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_reserve_vault_ix(
    keys: WithdrawReserveVaultKeys,
    args: WithdrawReserveVaultIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_reserve_vault_ix_with_program_id(ONREAPP_PROGRAM_ID, keys, args)
}
pub fn withdraw_reserve_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawReserveVaultAccounts<'_, '_>,
    args: WithdrawReserveVaultIxArgs,
) -> ProgramResult {
    let keys: WithdrawReserveVaultKeys = accounts.into();
    let ix = withdraw_reserve_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_reserve_vault_invoke(
    accounts: WithdrawReserveVaultAccounts<'_, '_>,
    args: WithdrawReserveVaultIxArgs,
) -> ProgramResult {
    withdraw_reserve_vault_invoke_with_program_id(ONREAPP_PROGRAM_ID, accounts, args)
}
pub fn withdraw_reserve_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawReserveVaultAccounts<'_, '_>,
    args: WithdrawReserveVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawReserveVaultKeys = accounts.into();
    let ix = withdraw_reserve_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_reserve_vault_invoke_signed(
    accounts: WithdrawReserveVaultAccounts<'_, '_>,
    args: WithdrawReserveVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_reserve_vault_invoke_signed_with_program_id(
        ONREAPP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_reserve_vault_verify_account_keys(
    accounts: WithdrawReserveVaultAccounts<'_, '_>,
    keys: WithdrawReserveVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.state.key, keys.state),
        (*accounts.buffer_state.key, keys.buffer_state),
        (*accounts.reserve_vault_authority.key, keys.reserve_vault_authority),
        (*accounts.onyc_mint.key, keys.onyc_mint),
        (*accounts.boss_onyc_account.key, keys.boss_onyc_account),
        (*accounts.reserve_vault_onyc_account.key, keys.reserve_vault_onyc_account),
        (*accounts.boss.key, keys.boss),
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
pub fn withdraw_reserve_vault_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawReserveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.boss_onyc_account,
        accounts.reserve_vault_onyc_account,
        accounts.boss,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_reserve_vault_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawReserveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.boss] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_reserve_vault_verify_account_privileges<'me, 'info>(
    accounts: WithdrawReserveVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_reserve_vault_verify_writable_privileges(accounts)?;
    withdraw_reserve_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
