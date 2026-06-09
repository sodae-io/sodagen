use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum PerpetualsProgramIx {
    Init(InitIxArgs),
    AddPool(AddPoolIxArgs),
    AddCustody(AddCustodyIxArgs),
    SetCustodyConfig(SetCustodyConfigIxArgs),
    SetPoolConfig(SetPoolConfigIxArgs),
    SetPerpetualsConfig(SetPerpetualsConfigIxArgs),
    TransferAdmin(TransferAdminIxArgs),
    WithdrawFees2(WithdrawFees2IxArgs),
    CreateTokenMetadata(CreateTokenMetadataIxArgs),
    CreateTokenLedger,
    ReallocCustody,
    ReallocPool,
    CreateAndDelegateStakeAccount(CreateAndDelegateStakeAccountIxArgs),
    Unstake,
    WithdrawStake,
    RedeemStake,
    OperatorSetCustodyConfig(OperatorSetCustodyConfigIxArgs),
    OperatorSetPoolConfig(OperatorSetPoolConfigIxArgs),
    TestInit(TestInitIxArgs),
    SetTestTime(SetTestTimeIxArgs),
    SetTokenLedger,
    Swap2(Swap2IxArgs),
    SwapWithTokenLedger(SwapWithTokenLedgerIxArgs),
    InstantIncreasePositionPreSwap(InstantIncreasePositionPreSwapIxArgs),
    AddLiquidity2(AddLiquidity2IxArgs),
    RemoveLiquidity2(RemoveLiquidity2IxArgs),
    CreateIncreasePositionMarketRequest(CreateIncreasePositionMarketRequestIxArgs),
    CreateDecreasePositionRequest2(CreateDecreasePositionRequest2IxArgs),
    CreateDecreasePositionMarketRequest(CreateDecreasePositionMarketRequestIxArgs),
    UpdateDecreasePositionRequest2(UpdateDecreasePositionRequest2IxArgs),
    ClosePositionRequest2,
    ClosePositionRequest3,
    IncreasePosition4(IncreasePosition4IxArgs),
    IncreasePositionPreSwap(IncreasePositionPreSwapIxArgs),
    IncreasePositionWithInternalSwap(IncreasePositionWithInternalSwapIxArgs),
    DecreasePosition4(DecreasePosition4IxArgs),
    DecreasePositionWithInternalSwap(DecreasePositionWithInternalSwapIxArgs),
    DecreasePositionWithTpsl(DecreasePositionWithTpslIxArgs),
    DecreasePositionWithTpslAndInternalSwap(
        DecreasePositionWithTpslAndInternalSwapIxArgs,
    ),
    LiquidateFullPosition4(LiquidateFullPosition4IxArgs),
    RefreshAssetsUnderManagement(RefreshAssetsUnderManagementIxArgs),
    SetMaxGlobalSizes(SetMaxGlobalSizesIxArgs),
    InstantCreateTpsl(InstantCreateTpslIxArgs),
    InstantCreateLimitOrder(InstantCreateLimitOrderIxArgs),
    InstantIncreasePosition(InstantIncreasePositionIxArgs),
    InstantDecreasePosition(InstantDecreasePositionIxArgs),
    InstantDecreasePosition2(InstantDecreasePosition2IxArgs),
    InstantUpdateLimitOrder(InstantUpdateLimitOrderIxArgs),
    InstantUpdateTpsl(InstantUpdateTpslIxArgs),
    GetAddLiquidityAmountAndFee2(GetAddLiquidityAmountAndFee2IxArgs),
    GetRemoveLiquidityAmountAndFee2(GetRemoveLiquidityAmountAndFee2IxArgs),
    GetAssetsUnderManagement2(GetAssetsUnderManagement2IxArgs),
    BorrowFromCustody(BorrowFromCustodyIxArgs),
    RepayToCustody(RepayToCustodyIxArgs),
    DepositCollateralForBorrows(DepositCollateralForBorrowsIxArgs),
    WithdrawCollateralForBorrows(WithdrawCollateralForBorrowsIxArgs),
    LiquidateBorrowPosition(LiquidateBorrowPositionIxArgs),
    PartialLiquidateBorrowPosition(PartialLiquidateBorrowPositionIxArgs),
    CloseBorrowPosition(CloseBorrowPositionIxArgs),
}
impl PerpetualsProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INIT_IX_DISCM) {
            let mut reader = &buf[INIT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Init(InitIxArgs { params }));
        }
        if buf.starts_with(&ADD_POOL_IX_DISCM) {
            let mut reader = &buf[ADD_POOL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <AddPoolParams>::deserialize(&mut reader)?
            };
            return Ok(Self::AddPool(AddPoolIxArgs { params }));
        }
        if buf.starts_with(&ADD_CUSTODY_IX_DISCM) {
            let mut reader = &buf[ADD_CUSTODY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <AddCustodyParams>::deserialize(&mut reader)?
            };
            return Ok(Self::AddCustody(AddCustodyIxArgs { params }));
        }
        if buf.starts_with(&SET_CUSTODY_CONFIG_IX_DISCM) {
            let mut reader = &buf[SET_CUSTODY_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SetCustodyConfigParams>::deserialize(&mut reader)?
            };
            return Ok(Self::SetCustodyConfig(SetCustodyConfigIxArgs { params }));
        }
        if buf.starts_with(&SET_POOL_CONFIG_IX_DISCM) {
            let mut reader = &buf[SET_POOL_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SetPoolConfigParams>::deserialize(&mut reader)?
            };
            return Ok(Self::SetPoolConfig(SetPoolConfigIxArgs { params }));
        }
        if buf.starts_with(&SET_PERPETUALS_CONFIG_IX_DISCM) {
            let mut reader = &buf[SET_PERPETUALS_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SetPerpetualsConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SetPerpetualsConfig(SetPerpetualsConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&TRANSFER_ADMIN_IX_DISCM) {
            let mut reader = &buf[TRANSFER_ADMIN_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <TransferAdminParams>::deserialize(&mut reader)?
            };
            return Ok(Self::TransferAdmin(TransferAdminIxArgs { params }));
        }
        if buf.starts_with(&WITHDRAW_FEES2_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FEES2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <WithdrawFees2Params>::deserialize(&mut reader)?
            };
            return Ok(Self::WithdrawFees2(WithdrawFees2IxArgs { params }));
        }
        if buf.starts_with(&CREATE_TOKEN_METADATA_IX_DISCM) {
            let mut reader = &buf[CREATE_TOKEN_METADATA_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateTokenMetadataParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateTokenMetadata(CreateTokenMetadataIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&CREATE_TOKEN_LEDGER_IX_DISCM) {
            return Ok(Self::CreateTokenLedger);
        }
        if buf.starts_with(&REALLOC_CUSTODY_IX_DISCM) {
            return Ok(Self::ReallocCustody);
        }
        if buf.starts_with(&REALLOC_POOL_IX_DISCM) {
            return Ok(Self::ReallocPool);
        }
        if buf.starts_with(&CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateAndDelegateStakeAccountParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateAndDelegateStakeAccount(CreateAndDelegateStakeAccountIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&UNSTAKE_IX_DISCM) {
            return Ok(Self::Unstake);
        }
        if buf.starts_with(&WITHDRAW_STAKE_IX_DISCM) {
            return Ok(Self::WithdrawStake);
        }
        if buf.starts_with(&REDEEM_STAKE_IX_DISCM) {
            return Ok(Self::RedeemStake);
        }
        if buf.starts_with(&OPERATOR_SET_CUSTODY_CONFIG_IX_DISCM) {
            let mut reader = &buf[OPERATOR_SET_CUSTODY_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <OperatorSetCustodyConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::OperatorSetCustodyConfig(OperatorSetCustodyConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&OPERATOR_SET_POOL_CONFIG_IX_DISCM) {
            let mut reader = &buf[OPERATOR_SET_POOL_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <OperatorSetPoolConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::OperatorSetPoolConfig(OperatorSetPoolConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&TEST_INIT_IX_DISCM) {
            let mut reader = &buf[TEST_INIT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <TestInitParams>::deserialize(&mut reader)?
            };
            return Ok(Self::TestInit(TestInitIxArgs { params }));
        }
        if buf.starts_with(&SET_TEST_TIME_IX_DISCM) {
            let mut reader = &buf[SET_TEST_TIME_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SetTestTimeParams>::deserialize(&mut reader)?
            };
            return Ok(Self::SetTestTime(SetTestTimeIxArgs { params }));
        }
        if buf.starts_with(&SET_TOKEN_LEDGER_IX_DISCM) {
            return Ok(Self::SetTokenLedger);
        }
        if buf.starts_with(&SWAP2_IX_DISCM) {
            let mut reader = &buf[SWAP2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <Swap2Params>::deserialize(&mut reader)?
            };
            return Ok(Self::Swap2(Swap2IxArgs { params }));
        }
        if buf.starts_with(&SWAP_WITH_TOKEN_LEDGER_IX_DISCM) {
            let mut reader = &buf[SWAP_WITH_TOKEN_LEDGER_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SwapWithTokenLedgerParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SwapWithTokenLedger(SwapWithTokenLedgerIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INSTANT_INCREASE_POSITION_PRE_SWAP_IX_DISCM) {
            let mut reader = &buf[INSTANT_INCREASE_POSITION_PRE_SWAP_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InstantIncreasePositionPreSwapParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InstantIncreasePositionPreSwap(InstantIncreasePositionPreSwapIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY2_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <AddLiquidity2Params>::deserialize(&mut reader)?
            };
            return Ok(Self::AddLiquidity2(AddLiquidity2IxArgs { params }));
        }
        if buf.starts_with(&REMOVE_LIQUIDITY2_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <RemoveLiquidity2Params>::deserialize(&mut reader)?
            };
            return Ok(Self::RemoveLiquidity2(RemoveLiquidity2IxArgs { params }));
        }
        if buf.starts_with(&CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_DISCM) {
            let mut reader = &buf[CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_DISCM
                .len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateIncreasePositionMarketRequestParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateIncreasePositionMarketRequest(CreateIncreasePositionMarketRequestIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&CREATE_DECREASE_POSITION_REQUEST2_IX_DISCM) {
            let mut reader = &buf[CREATE_DECREASE_POSITION_REQUEST2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateDecreasePositionRequest2Params>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateDecreasePositionRequest2(CreateDecreasePositionRequest2IxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_DISCM) {
            let mut reader = &buf[CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_DISCM
                .len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateDecreasePositionMarketRequestParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateDecreasePositionMarketRequest(CreateDecreasePositionMarketRequestIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&UPDATE_DECREASE_POSITION_REQUEST2_IX_DISCM) {
            let mut reader = &buf[UPDATE_DECREASE_POSITION_REQUEST2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateDecreasePositionRequest2Params>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateDecreasePositionRequest2(UpdateDecreasePositionRequest2IxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&CLOSE_POSITION_REQUEST2_IX_DISCM) {
            return Ok(Self::ClosePositionRequest2);
        }
        if buf.starts_with(&CLOSE_POSITION_REQUEST3_IX_DISCM) {
            return Ok(Self::ClosePositionRequest3);
        }
        if buf.starts_with(&INCREASE_POSITION4_IX_DISCM) {
            let mut reader = &buf[INCREASE_POSITION4_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <IncreasePosition4Params>::deserialize(&mut reader)?
            };
            return Ok(Self::IncreasePosition4(IncreasePosition4IxArgs { params }));
        }
        if buf.starts_with(&INCREASE_POSITION_PRE_SWAP_IX_DISCM) {
            let mut reader = &buf[INCREASE_POSITION_PRE_SWAP_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <IncreasePositionPreSwapParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::IncreasePositionPreSwap(IncreasePositionPreSwapIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM) {
            let mut reader = &buf[INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <IncreasePositionWithInternalSwapParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::IncreasePositionWithInternalSwap(IncreasePositionWithInternalSwapIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&DECREASE_POSITION4_IX_DISCM) {
            let mut reader = &buf[DECREASE_POSITION4_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <DecreasePosition4Params>::deserialize(&mut reader)?
            };
            return Ok(Self::DecreasePosition4(DecreasePosition4IxArgs { params }));
        }
        if buf.starts_with(&DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM) {
            let mut reader = &buf[DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <DecreasePositionWithInternalSwapParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DecreasePositionWithInternalSwap(DecreasePositionWithInternalSwapIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&DECREASE_POSITION_WITH_TPSL_IX_DISCM) {
            let mut reader = &buf[DECREASE_POSITION_WITH_TPSL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <DecreasePositionWithTpslParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DecreasePositionWithTpsl(DecreasePositionWithTpslIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_DISCM) {
            let mut reader = &buf[DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_DISCM
                .len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <DecreasePositionWithTpslAndInternalSwapParams>::deserialize(
                    &mut reader,
                )?
            };
            return Ok(
                Self::DecreasePositionWithTpslAndInternalSwap(DecreasePositionWithTpslAndInternalSwapIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&LIQUIDATE_FULL_POSITION4_IX_DISCM) {
            let mut reader = &buf[LIQUIDATE_FULL_POSITION4_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <LiquidateFullPosition4Params>::deserialize(&mut reader)?
            };
            return Ok(
                Self::LiquidateFullPosition4(LiquidateFullPosition4IxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&REFRESH_ASSETS_UNDER_MANAGEMENT_IX_DISCM) {
            let mut reader = &buf[REFRESH_ASSETS_UNDER_MANAGEMENT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <RefreshAssetsUnderManagementParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::RefreshAssetsUnderManagement(RefreshAssetsUnderManagementIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&SET_MAX_GLOBAL_SIZES_IX_DISCM) {
            let mut reader = &buf[SET_MAX_GLOBAL_SIZES_IX_DISCM.len()..];
            let params = <SetMaxGlobalSizesParams>::deserialize(&mut reader)?;
            return Ok(Self::SetMaxGlobalSizes(SetMaxGlobalSizesIxArgs { params }));
        }
        if buf.starts_with(&INSTANT_CREATE_TPSL_IX_DISCM) {
            let mut reader = &buf[INSTANT_CREATE_TPSL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InstantCreateTpslParams>::deserialize(&mut reader)?
            };
            return Ok(Self::InstantCreateTpsl(InstantCreateTpslIxArgs { params }));
        }
        if buf.starts_with(&INSTANT_CREATE_LIMIT_ORDER_IX_DISCM) {
            let mut reader = &buf[INSTANT_CREATE_LIMIT_ORDER_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InstantCreateLimitOrderParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InstantCreateLimitOrder(InstantCreateLimitOrderIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INSTANT_INCREASE_POSITION_IX_DISCM) {
            let mut reader = &buf[INSTANT_INCREASE_POSITION_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InstantIncreasePositionParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InstantIncreasePosition(InstantIncreasePositionIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INSTANT_DECREASE_POSITION_IX_DISCM) {
            let mut reader = &buf[INSTANT_DECREASE_POSITION_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InstantDecreasePositionParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InstantDecreasePosition(InstantDecreasePositionIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INSTANT_DECREASE_POSITION2_IX_DISCM) {
            let mut reader = &buf[INSTANT_DECREASE_POSITION2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InstantDecreasePosition2Params>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InstantDecreasePosition2(InstantDecreasePosition2IxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INSTANT_UPDATE_LIMIT_ORDER_IX_DISCM) {
            let mut reader = &buf[INSTANT_UPDATE_LIMIT_ORDER_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InstantUpdateLimitOrderParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InstantUpdateLimitOrder(InstantUpdateLimitOrderIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INSTANT_UPDATE_TPSL_IX_DISCM) {
            let mut reader = &buf[INSTANT_UPDATE_TPSL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InstantUpdateTpslParams>::deserialize(&mut reader)?
            };
            return Ok(Self::InstantUpdateTpsl(InstantUpdateTpslIxArgs { params }));
        }
        if buf.starts_with(&GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM) {
            let mut reader = &buf[GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <GetAddLiquidityAmountAndFee2Params>::deserialize(&mut reader)?
            };
            return Ok(
                Self::GetAddLiquidityAmountAndFee2(GetAddLiquidityAmountAndFee2IxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM) {
            let mut reader = &buf[GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <GetRemoveLiquidityAmountAndFee2Params>::deserialize(&mut reader)?
            };
            return Ok(
                Self::GetRemoveLiquidityAmountAndFee2(GetRemoveLiquidityAmountAndFee2IxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&GET_ASSETS_UNDER_MANAGEMENT2_IX_DISCM) {
            let mut reader = &buf[GET_ASSETS_UNDER_MANAGEMENT2_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <GetAssetsUnderManagement2Params>::deserialize(&mut reader)?
            };
            return Ok(
                Self::GetAssetsUnderManagement2(GetAssetsUnderManagement2IxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&BORROW_FROM_CUSTODY_IX_DISCM) {
            let mut reader = &buf[BORROW_FROM_CUSTODY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <BorrowFromCustodyParams>::deserialize(&mut reader)?
            };
            return Ok(Self::BorrowFromCustody(BorrowFromCustodyIxArgs { params }));
        }
        if buf.starts_with(&REPAY_TO_CUSTODY_IX_DISCM) {
            let mut reader = &buf[REPAY_TO_CUSTODY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <RepayToCustodyParams>::deserialize(&mut reader)?
            };
            return Ok(Self::RepayToCustody(RepayToCustodyIxArgs { params }));
        }
        if buf.starts_with(&DEPOSIT_COLLATERAL_FOR_BORROWS_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_COLLATERAL_FOR_BORROWS_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <DepositParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DepositCollateralForBorrows(DepositCollateralForBorrowsIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_COLLATERAL_FOR_BORROWS_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_COLLATERAL_FOR_BORROWS_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <WithdrawParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::WithdrawCollateralForBorrows(WithdrawCollateralForBorrowsIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&LIQUIDATE_BORROW_POSITION_IX_DISCM) {
            let mut reader = &buf[LIQUIDATE_BORROW_POSITION_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <LiquidateBorrowPositionParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::LiquidateBorrowPosition(LiquidateBorrowPositionIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&PARTIAL_LIQUIDATE_BORROW_POSITION_IX_DISCM) {
            let mut reader = &buf[PARTIAL_LIQUIDATE_BORROW_POSITION_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <PartialLiquidateBorrowPositionParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::PartialLiquidateBorrowPosition(PartialLiquidateBorrowPositionIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&CLOSE_BORROW_POSITION_IX_DISCM) {
            let mut reader = &buf[CLOSE_BORROW_POSITION_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CloseBorrowPositionParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CloseBorrowPosition(CloseBorrowPositionIxArgs {
                    params,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Init(args) => {
                writer.write_all(&INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::AddPool(args) => {
                writer.write_all(&ADD_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::AddCustody(args) => {
                writer.write_all(&ADD_CUSTODY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SetCustodyConfig(args) => {
                writer.write_all(&SET_CUSTODY_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SetPoolConfig(args) => {
                writer.write_all(&SET_POOL_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SetPerpetualsConfig(args) => {
                writer.write_all(&SET_PERPETUALS_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::TransferAdmin(args) => {
                writer.write_all(&TRANSFER_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFees2(args) => {
                writer.write_all(&WITHDRAW_FEES2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CreateTokenMetadata(args) => {
                writer.write_all(&CREATE_TOKEN_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CreateTokenLedger => writer.write_all(&CREATE_TOKEN_LEDGER_IX_DISCM),
            Self::ReallocCustody => writer.write_all(&REALLOC_CUSTODY_IX_DISCM),
            Self::ReallocPool => writer.write_all(&REALLOC_POOL_IX_DISCM),
            Self::CreateAndDelegateStakeAccount(args) => {
                writer.write_all(&CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::Unstake => writer.write_all(&UNSTAKE_IX_DISCM),
            Self::WithdrawStake => writer.write_all(&WITHDRAW_STAKE_IX_DISCM),
            Self::RedeemStake => writer.write_all(&REDEEM_STAKE_IX_DISCM),
            Self::OperatorSetCustodyConfig(args) => {
                writer.write_all(&OPERATOR_SET_CUSTODY_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::OperatorSetPoolConfig(args) => {
                writer.write_all(&OPERATOR_SET_POOL_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::TestInit(args) => {
                writer.write_all(&TEST_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SetTestTime(args) => {
                writer.write_all(&SET_TEST_TIME_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SetTokenLedger => writer.write_all(&SET_TOKEN_LEDGER_IX_DISCM),
            Self::Swap2(args) => {
                writer.write_all(&SWAP2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SwapWithTokenLedger(args) => {
                writer.write_all(&SWAP_WITH_TOKEN_LEDGER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InstantIncreasePositionPreSwap(args) => {
                writer.write_all(&INSTANT_INCREASE_POSITION_PRE_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::AddLiquidity2(args) => {
                writer.write_all(&ADD_LIQUIDITY2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::RemoveLiquidity2(args) => {
                writer.write_all(&REMOVE_LIQUIDITY2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CreateIncreasePositionMarketRequest(args) => {
                writer.write_all(&CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CreateDecreasePositionRequest2(args) => {
                writer.write_all(&CREATE_DECREASE_POSITION_REQUEST2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CreateDecreasePositionMarketRequest(args) => {
                writer.write_all(&CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::UpdateDecreasePositionRequest2(args) => {
                writer.write_all(&UPDATE_DECREASE_POSITION_REQUEST2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::ClosePositionRequest2 => {
                writer.write_all(&CLOSE_POSITION_REQUEST2_IX_DISCM)
            }
            Self::ClosePositionRequest3 => {
                writer.write_all(&CLOSE_POSITION_REQUEST3_IX_DISCM)
            }
            Self::IncreasePosition4(args) => {
                writer.write_all(&INCREASE_POSITION4_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::IncreasePositionPreSwap(args) => {
                writer.write_all(&INCREASE_POSITION_PRE_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::IncreasePositionWithInternalSwap(args) => {
                writer.write_all(&INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::DecreasePosition4(args) => {
                writer.write_all(&DECREASE_POSITION4_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::DecreasePositionWithInternalSwap(args) => {
                writer.write_all(&DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::DecreasePositionWithTpsl(args) => {
                writer.write_all(&DECREASE_POSITION_WITH_TPSL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::DecreasePositionWithTpslAndInternalSwap(args) => {
                writer
                    .write_all(&DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::LiquidateFullPosition4(args) => {
                writer.write_all(&LIQUIDATE_FULL_POSITION4_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::RefreshAssetsUnderManagement(args) => {
                writer.write_all(&REFRESH_ASSETS_UNDER_MANAGEMENT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SetMaxGlobalSizes(args) => {
                writer.write_all(&SET_MAX_GLOBAL_SIZES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InstantCreateTpsl(args) => {
                writer.write_all(&INSTANT_CREATE_TPSL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InstantCreateLimitOrder(args) => {
                writer.write_all(&INSTANT_CREATE_LIMIT_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InstantIncreasePosition(args) => {
                writer.write_all(&INSTANT_INCREASE_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InstantDecreasePosition(args) => {
                writer.write_all(&INSTANT_DECREASE_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InstantDecreasePosition2(args) => {
                writer.write_all(&INSTANT_DECREASE_POSITION2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InstantUpdateLimitOrder(args) => {
                writer.write_all(&INSTANT_UPDATE_LIMIT_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InstantUpdateTpsl(args) => {
                writer.write_all(&INSTANT_UPDATE_TPSL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::GetAddLiquidityAmountAndFee2(args) => {
                writer.write_all(&GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::GetRemoveLiquidityAmountAndFee2(args) => {
                writer.write_all(&GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::GetAssetsUnderManagement2(args) => {
                writer.write_all(&GET_ASSETS_UNDER_MANAGEMENT2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::BorrowFromCustody(args) => {
                writer.write_all(&BORROW_FROM_CUSTODY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::RepayToCustody(args) => {
                writer.write_all(&REPAY_TO_CUSTODY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::DepositCollateralForBorrows(args) => {
                writer.write_all(&DEPOSIT_COLLATERAL_FOR_BORROWS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::WithdrawCollateralForBorrows(args) => {
                writer.write_all(&WITHDRAW_COLLATERAL_FOR_BORROWS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::LiquidateBorrowPosition(args) => {
                writer.write_all(&LIQUIDATE_BORROW_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::PartialLiquidateBorrowPosition(args) => {
                writer.write_all(&PARTIAL_LIQUIDATE_BORROW_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CloseBorrowPosition(args) => {
                writer.write_all(&CLOSE_BORROW_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
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
pub const INIT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitAccounts<'me, 'info> {
    pub upgrade_authority: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub perpetuals_program: &'me AccountInfo<'info>,
    pub perpetuals_program_data: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitKeys {
    pub upgrade_authority: Pubkey,
    pub admin: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub perpetuals_program: Pubkey,
    pub perpetuals_program_data: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitAccounts<'_, '_>> for InitKeys {
    fn from(accounts: InitAccounts) -> Self {
        Self {
            upgrade_authority: *accounts.upgrade_authority.key,
            admin: *accounts.admin.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            perpetuals_program: *accounts.perpetuals_program.key,
            perpetuals_program_data: *accounts.perpetuals_program_data.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitKeys> for [AccountMeta; INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.upgrade_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals_program_data,
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
impl From<[Pubkey; INIT_IX_ACCOUNTS_LEN]> for InitKeys {
    fn from(pubkeys: [Pubkey; INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            upgrade_authority: pubkeys[0],
            admin: pubkeys[1],
            transfer_authority: pubkeys[2],
            perpetuals: pubkeys[3],
            perpetuals_program: pubkeys[4],
            perpetuals_program_data: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<InitAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitAccounts<'_, 'info>) -> Self {
        [
            accounts.upgrade_authority.clone(),
            accounts.admin.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.perpetuals_program.clone(),
            accounts.perpetuals_program_data.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN]>
for InitAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            upgrade_authority: &arr[0],
            admin: &arr[1],
            transfer_authority: &arr[2],
            perpetuals: &arr[3],
            perpetuals_program: &arr[4],
            perpetuals_program_data: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const INIT_IX_DISCM: [u8; 8usize] = [220, 59, 207, 236, 108, 250, 47, 100];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitIxArgs {
    pub params: InitParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitIxData(pub InitIxArgs);
impl From<InitIxArgs> for InitIxData {
    fn from(args: InitIxArgs) -> Self {
        Self(args)
    }
}
impl InitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitParams>::deserialize(&mut reader)?
        };
        Ok(Self(InitIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_ix_with_program_id(
    program_id: Pubkey,
    keys: InitKeys,
    args: InitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_ix(keys: InitKeys, args: InitIxArgs) -> std::io::Result<Instruction> {
    init_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitAccounts<'_, '_>,
    args: InitIxArgs,
) -> ProgramResult {
    let keys: InitKeys = accounts.into();
    let ix = init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_invoke(accounts: InitAccounts<'_, '_>, args: InitIxArgs) -> ProgramResult {
    init_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitAccounts<'_, '_>,
    args: InitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitKeys = accounts.into();
    let ix = init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_invoke_signed(
    accounts: InitAccounts<'_, '_>,
    args: InitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args, seeds)
}
pub fn init_verify_account_keys(
    accounts: InitAccounts<'_, '_>,
    keys: InitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.upgrade_authority.key, keys.upgrade_authority),
        (*accounts.admin.key, keys.admin),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.perpetuals_program.key, keys.perpetuals_program),
        (*accounts.perpetuals_program_data.key, keys.perpetuals_program_data),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_verify_writable_privileges<'me, 'info>(
    accounts: InitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.upgrade_authority,
        accounts.transfer_authority,
        accounts.perpetuals,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_verify_signer_privileges<'me, 'info>(
    accounts: InitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.upgrade_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_verify_account_privileges<'me, 'info>(
    accounts: InitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_verify_writable_privileges(accounts)?;
    init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_POOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct AddPoolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddPoolKeys {
    pub admin: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub lp_token_mint: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<AddPoolAccounts<'_, '_>> for AddPoolKeys {
    fn from(accounts: AddPoolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<AddPoolKeys> for [AccountMeta; ADD_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: AddPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_POOL_IX_ACCOUNTS_LEN]> for AddPoolKeys {
    fn from(pubkeys: [Pubkey; ADD_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            transfer_authority: pubkeys[1],
            perpetuals: pubkeys[2],
            pool: pubkeys[3],
            lp_token_mint: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<AddPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.lp_token_mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_POOL_IX_ACCOUNTS_LEN]>
for AddPoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            transfer_authority: &arr[1],
            perpetuals: &arr[2],
            pool: &arr[3],
            lp_token_mint: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const ADD_POOL_IX_DISCM: [u8; 8usize] = [115, 230, 212, 211, 175, 49, 39, 169];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddPoolIxArgs {
    pub params: AddPoolParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddPoolIxData(pub AddPoolIxArgs);
impl From<AddPoolIxArgs> for AddPoolIxData {
    fn from(args: AddPoolIxArgs) -> Self {
        Self(args)
    }
}
impl AddPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <AddPoolParams>::deserialize(&mut reader)?
        };
        Ok(Self(AddPoolIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: AddPoolKeys,
    args: AddPoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddPoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_pool_ix(
    keys: AddPoolKeys,
    args: AddPoolIxArgs,
) -> std::io::Result<Instruction> {
    add_pool_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn add_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddPoolAccounts<'_, '_>,
    args: AddPoolIxArgs,
) -> ProgramResult {
    let keys: AddPoolKeys = accounts.into();
    let ix = add_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_pool_invoke(
    accounts: AddPoolAccounts<'_, '_>,
    args: AddPoolIxArgs,
) -> ProgramResult {
    add_pool_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn add_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddPoolAccounts<'_, '_>,
    args: AddPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddPoolKeys = accounts.into();
    let ix = add_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_pool_invoke_signed(
    accounts: AddPoolAccounts<'_, '_>,
    args: AddPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_pool_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_pool_verify_account_keys(
    accounts: AddPoolAccounts<'_, '_>,
    keys: AddPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
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
pub fn add_pool_verify_writable_privileges<'me, 'info>(
    accounts: AddPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.perpetuals,
        accounts.pool,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_pool_verify_signer_privileges<'me, 'info>(
    accounts: AddPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_pool_verify_account_privileges<'me, 'info>(
    accounts: AddPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_pool_verify_writable_privileges(accounts)?;
    add_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_CUSTODY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct AddCustodyAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub custody_token_mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddCustodyKeys {
    pub admin: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_token_account: Pubkey,
    pub custody_token_mint: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<AddCustodyAccounts<'_, '_>> for AddCustodyKeys {
    fn from(accounts: AddCustodyAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_token_account: *accounts.custody_token_account.key,
            custody_token_mint: *accounts.custody_token_mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<AddCustodyKeys> for [AccountMeta; ADD_CUSTODY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddCustodyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_token_mint,
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
impl From<[Pubkey; ADD_CUSTODY_IX_ACCOUNTS_LEN]> for AddCustodyKeys {
    fn from(pubkeys: [Pubkey; ADD_CUSTODY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            transfer_authority: pubkeys[1],
            perpetuals: pubkeys[2],
            pool: pubkeys[3],
            custody: pubkeys[4],
            custody_token_account: pubkeys[5],
            custody_token_mint: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            rent: pubkeys[9],
        }
    }
}
impl<'info> From<AddCustodyAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_CUSTODY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddCustodyAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_token_account.clone(),
            accounts.custody_token_mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_CUSTODY_IX_ACCOUNTS_LEN]>
for AddCustodyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_CUSTODY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            transfer_authority: &arr[1],
            perpetuals: &arr[2],
            pool: &arr[3],
            custody: &arr[4],
            custody_token_account: &arr[5],
            custody_token_mint: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            rent: &arr[9],
        }
    }
}
pub const ADD_CUSTODY_IX_DISCM: [u8; 8usize] = [247, 254, 126, 17, 26, 6, 215, 117];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddCustodyIxArgs {
    pub params: AddCustodyParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddCustodyIxData(pub AddCustodyIxArgs);
impl From<AddCustodyIxArgs> for AddCustodyIxData {
    fn from(args: AddCustodyIxArgs) -> Self {
        Self(args)
    }
}
impl AddCustodyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_CUSTODY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <AddCustodyParams>::deserialize(&mut reader)?
        };
        Ok(Self(AddCustodyIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_CUSTODY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_custody_ix_with_program_id(
    program_id: Pubkey,
    keys: AddCustodyKeys,
    args: AddCustodyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_CUSTODY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddCustodyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_custody_ix(
    keys: AddCustodyKeys,
    args: AddCustodyIxArgs,
) -> std::io::Result<Instruction> {
    add_custody_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn add_custody_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddCustodyAccounts<'_, '_>,
    args: AddCustodyIxArgs,
) -> ProgramResult {
    let keys: AddCustodyKeys = accounts.into();
    let ix = add_custody_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_custody_invoke(
    accounts: AddCustodyAccounts<'_, '_>,
    args: AddCustodyIxArgs,
) -> ProgramResult {
    add_custody_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn add_custody_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddCustodyAccounts<'_, '_>,
    args: AddCustodyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddCustodyKeys = accounts.into();
    let ix = add_custody_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_custody_invoke_signed(
    accounts: AddCustodyAccounts<'_, '_>,
    args: AddCustodyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_custody_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_custody_verify_account_keys(
    accounts: AddCustodyAccounts<'_, '_>,
    keys: AddCustodyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_token_account.key, keys.custody_token_account),
        (*accounts.custody_token_mint.key, keys.custody_token_mint),
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
pub fn add_custody_verify_writable_privileges<'me, 'info>(
    accounts: AddCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_custody_verify_signer_privileges<'me, 'info>(
    accounts: AddCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_custody_verify_account_privileges<'me, 'info>(
    accounts: AddCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_custody_verify_writable_privileges(accounts)?;
    add_custody_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetCustodyConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetCustodyConfigKeys {
    pub admin: Pubkey,
    pub perpetuals: Pubkey,
    pub custody: Pubkey,
}
impl From<SetCustodyConfigAccounts<'_, '_>> for SetCustodyConfigKeys {
    fn from(accounts: SetCustodyConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            perpetuals: *accounts.perpetuals.key,
            custody: *accounts.custody.key,
        }
    }
}
impl From<SetCustodyConfigKeys> for [AccountMeta; SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: SetCustodyConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN]> for SetCustodyConfigKeys {
    fn from(pubkeys: [Pubkey; SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            perpetuals: pubkeys[1],
            custody: pubkeys[2],
        }
    }
}
impl<'info> From<SetCustodyConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetCustodyConfigAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.perpetuals.clone(), accounts.custody.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN]>
for SetCustodyConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            perpetuals: &arr[1],
            custody: &arr[2],
        }
    }
}
pub const SET_CUSTODY_CONFIG_IX_DISCM: [u8; 8usize] = [
    133, 97, 130, 143, 215, 229, 36, 176,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetCustodyConfigIxArgs {
    pub params: SetCustodyConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetCustodyConfigIxData(pub SetCustodyConfigIxArgs);
impl From<SetCustodyConfigIxArgs> for SetCustodyConfigIxData {
    fn from(args: SetCustodyConfigIxArgs) -> Self {
        Self(args)
    }
}
impl SetCustodyConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_CUSTODY_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SetCustodyConfigParams>::deserialize(&mut reader)?
        };
        Ok(Self(SetCustodyConfigIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_CUSTODY_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_custody_config_ix_with_program_id(
    program_id: Pubkey,
    keys: SetCustodyConfigKeys,
    args: SetCustodyConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetCustodyConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_custody_config_ix(
    keys: SetCustodyConfigKeys,
    args: SetCustodyConfigIxArgs,
) -> std::io::Result<Instruction> {
    set_custody_config_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn set_custody_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetCustodyConfigAccounts<'_, '_>,
    args: SetCustodyConfigIxArgs,
) -> ProgramResult {
    let keys: SetCustodyConfigKeys = accounts.into();
    let ix = set_custody_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_custody_config_invoke(
    accounts: SetCustodyConfigAccounts<'_, '_>,
    args: SetCustodyConfigIxArgs,
) -> ProgramResult {
    set_custody_config_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn set_custody_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetCustodyConfigAccounts<'_, '_>,
    args: SetCustodyConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetCustodyConfigKeys = accounts.into();
    let ix = set_custody_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_custody_config_invoke_signed(
    accounts: SetCustodyConfigAccounts<'_, '_>,
    args: SetCustodyConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_custody_config_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_custody_config_verify_account_keys(
    accounts: SetCustodyConfigAccounts<'_, '_>,
    keys: SetCustodyConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.custody.key, keys.custody),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_custody_config_verify_writable_privileges<'me, 'info>(
    accounts: SetCustodyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.custody] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_custody_config_verify_signer_privileges<'me, 'info>(
    accounts: SetCustodyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_custody_config_verify_account_privileges<'me, 'info>(
    accounts: SetCustodyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_custody_config_verify_writable_privileges(accounts)?;
    set_custody_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolConfigKeys {
    pub admin: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
}
impl From<SetPoolConfigAccounts<'_, '_>> for SetPoolConfigKeys {
    fn from(accounts: SetPoolConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<SetPoolConfigKeys> for [AccountMeta; SET_POOL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_CONFIG_IX_ACCOUNTS_LEN]> for SetPoolConfigKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
        }
    }
}
impl<'info> From<SetPoolConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolConfigAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.perpetuals.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POOL_CONFIG_IX_ACCOUNTS_LEN]>
for SetPoolConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_POOL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
        }
    }
}
pub const SET_POOL_CONFIG_IX_DISCM: [u8; 8usize] = [
    216, 87, 65, 125, 113, 110, 185, 120,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPoolConfigIxArgs {
    pub params: SetPoolConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolConfigIxData(pub SetPoolConfigIxArgs);
impl From<SetPoolConfigIxArgs> for SetPoolConfigIxData {
    fn from(args: SetPoolConfigIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SetPoolConfigParams>::deserialize(&mut reader)?
        };
        Ok(Self(SetPoolConfigIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_config_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolConfigKeys,
    args: SetPoolConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_config_ix(
    keys: SetPoolConfigKeys,
    args: SetPoolConfigIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_config_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn set_pool_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolConfigAccounts<'_, '_>,
    args: SetPoolConfigIxArgs,
) -> ProgramResult {
    let keys: SetPoolConfigKeys = accounts.into();
    let ix = set_pool_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_config_invoke(
    accounts: SetPoolConfigAccounts<'_, '_>,
    args: SetPoolConfigIxArgs,
) -> ProgramResult {
    set_pool_config_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn set_pool_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolConfigAccounts<'_, '_>,
    args: SetPoolConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolConfigKeys = accounts.into();
    let ix = set_pool_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_config_invoke_signed(
    accounts: SetPoolConfigAccounts<'_, '_>,
    args: SetPoolConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_config_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pool_config_verify_account_keys(
    accounts: SetPoolConfigAccounts<'_, '_>,
    keys: SetPoolConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_config_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_config_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_config_verify_account_privileges<'me, 'info>(
    accounts: SetPoolConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_config_verify_writable_privileges(accounts)?;
    set_pool_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PERPETUALS_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPerpetualsConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPerpetualsConfigKeys {
    pub admin: Pubkey,
    pub perpetuals: Pubkey,
}
impl From<SetPerpetualsConfigAccounts<'_, '_>> for SetPerpetualsConfigKeys {
    fn from(accounts: SetPerpetualsConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            perpetuals: *accounts.perpetuals.key,
        }
    }
}
impl From<SetPerpetualsConfigKeys>
for [AccountMeta; SET_PERPETUALS_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPerpetualsConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_PERPETUALS_CONFIG_IX_ACCOUNTS_LEN]> for SetPerpetualsConfigKeys {
    fn from(pubkeys: [Pubkey; SET_PERPETUALS_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            perpetuals: pubkeys[1],
        }
    }
}
impl<'info> From<SetPerpetualsConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PERPETUALS_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPerpetualsConfigAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.perpetuals.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PERPETUALS_CONFIG_IX_ACCOUNTS_LEN]>
for SetPerpetualsConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PERPETUALS_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            perpetuals: &arr[1],
        }
    }
}
pub const SET_PERPETUALS_CONFIG_IX_DISCM: [u8; 8usize] = [
    80, 72, 21, 191, 29, 121, 45, 111,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPerpetualsConfigIxArgs {
    pub params: SetPerpetualsConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPerpetualsConfigIxData(pub SetPerpetualsConfigIxArgs);
impl From<SetPerpetualsConfigIxArgs> for SetPerpetualsConfigIxData {
    fn from(args: SetPerpetualsConfigIxArgs) -> Self {
        Self(args)
    }
}
impl SetPerpetualsConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PERPETUALS_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SetPerpetualsConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(SetPerpetualsConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PERPETUALS_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_perpetuals_config_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPerpetualsConfigKeys,
    args: SetPerpetualsConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PERPETUALS_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPerpetualsConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_perpetuals_config_ix(
    keys: SetPerpetualsConfigKeys,
    args: SetPerpetualsConfigIxArgs,
) -> std::io::Result<Instruction> {
    set_perpetuals_config_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn set_perpetuals_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPerpetualsConfigAccounts<'_, '_>,
    args: SetPerpetualsConfigIxArgs,
) -> ProgramResult {
    let keys: SetPerpetualsConfigKeys = accounts.into();
    let ix = set_perpetuals_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_perpetuals_config_invoke(
    accounts: SetPerpetualsConfigAccounts<'_, '_>,
    args: SetPerpetualsConfigIxArgs,
) -> ProgramResult {
    set_perpetuals_config_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn set_perpetuals_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPerpetualsConfigAccounts<'_, '_>,
    args: SetPerpetualsConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPerpetualsConfigKeys = accounts.into();
    let ix = set_perpetuals_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_perpetuals_config_invoke_signed(
    accounts: SetPerpetualsConfigAccounts<'_, '_>,
    args: SetPerpetualsConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_perpetuals_config_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_perpetuals_config_verify_account_keys(
    accounts: SetPerpetualsConfigAccounts<'_, '_>,
    keys: SetPerpetualsConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.perpetuals.key, keys.perpetuals),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_perpetuals_config_verify_writable_privileges<'me, 'info>(
    accounts: SetPerpetualsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perpetuals] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_perpetuals_config_verify_signer_privileges<'me, 'info>(
    accounts: SetPerpetualsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_perpetuals_config_verify_account_privileges<'me, 'info>(
    accounts: SetPerpetualsConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_perpetuals_config_verify_writable_privileges(accounts)?;
    set_perpetuals_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct TransferAdminAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferAdminKeys {
    pub admin: Pubkey,
    pub new_admin: Pubkey,
    pub perpetuals: Pubkey,
}
impl From<TransferAdminAccounts<'_, '_>> for TransferAdminKeys {
    fn from(accounts: TransferAdminAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            new_admin: *accounts.new_admin.key,
            perpetuals: *accounts.perpetuals.key,
        }
    }
}
impl From<TransferAdminKeys> for [AccountMeta; TRANSFER_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferAdminKeys) -> Self {
        [
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
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_ADMIN_IX_ACCOUNTS_LEN]> for TransferAdminKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            new_admin: pubkeys[1],
            perpetuals: pubkeys[2],
        }
    }
}
impl<'info> From<TransferAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferAdminAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.new_admin.clone(), accounts.perpetuals.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_ADMIN_IX_ACCOUNTS_LEN]>
for TransferAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            new_admin: &arr[1],
            perpetuals: &arr[2],
        }
    }
}
pub const TRANSFER_ADMIN_IX_DISCM: [u8; 8usize] = [42, 242, 66, 106, 228, 10, 111, 156];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferAdminIxArgs {
    pub params: TransferAdminParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferAdminIxData(pub TransferAdminIxArgs);
impl From<TransferAdminIxArgs> for TransferAdminIxData {
    fn from(args: TransferAdminIxArgs) -> Self {
        Self(args)
    }
}
impl TransferAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <TransferAdminParams>::deserialize(&mut reader)?
        };
        Ok(Self(TransferAdminIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferAdminKeys,
    args: TransferAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_admin_ix(
    keys: TransferAdminKeys,
    args: TransferAdminIxArgs,
) -> std::io::Result<Instruction> {
    transfer_admin_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn transfer_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferAdminAccounts<'_, '_>,
    args: TransferAdminIxArgs,
) -> ProgramResult {
    let keys: TransferAdminKeys = accounts.into();
    let ix = transfer_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_admin_invoke(
    accounts: TransferAdminAccounts<'_, '_>,
    args: TransferAdminIxArgs,
) -> ProgramResult {
    transfer_admin_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn transfer_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferAdminAccounts<'_, '_>,
    args: TransferAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferAdminKeys = accounts.into();
    let ix = transfer_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_admin_invoke_signed(
    accounts: TransferAdminAccounts<'_, '_>,
    args: TransferAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_admin_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn transfer_admin_verify_account_keys(
    accounts: TransferAdminAccounts<'_, '_>,
    keys: TransferAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.new_admin.key, keys.new_admin),
        (*accounts.perpetuals.key, keys.perpetuals),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_admin_verify_writable_privileges<'me, 'info>(
    accounts: TransferAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perpetuals] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_admin_verify_signer_privileges<'me, 'info>(
    accounts: TransferAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_admin_verify_account_privileges<'me, 'info>(
    accounts: TransferAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_admin_verify_writable_privileges(accounts)?;
    transfer_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FEES2_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFees2Accounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub receiving_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFees2Keys {
    pub keeper: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_token_account: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub receiving_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawFees2Accounts<'_, '_>> for WithdrawFees2Keys {
    fn from(accounts: WithdrawFees2Accounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_token_account: *accounts.custody_token_account.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            receiving_token_account: *accounts.receiving_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawFees2Keys> for [AccountMeta; WITHDRAW_FEES2_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFees2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_token_account,
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
impl From<[Pubkey; WITHDRAW_FEES2_IX_ACCOUNTS_LEN]> for WithdrawFees2Keys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FEES2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            transfer_authority: pubkeys[1],
            perpetuals: pubkeys[2],
            pool: pubkeys[3],
            custody: pubkeys[4],
            custody_token_account: pubkeys[5],
            custody_doves_price_account: pubkeys[6],
            custody_pythnet_price_account: pubkeys[7],
            receiving_token_account: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawFees2Accounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FEES2_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFees2Accounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_token_account.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.receiving_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FEES2_IX_ACCOUNTS_LEN]>
for WithdrawFees2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_FEES2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            transfer_authority: &arr[1],
            perpetuals: &arr[2],
            pool: &arr[3],
            custody: &arr[4],
            custody_token_account: &arr[5],
            custody_doves_price_account: &arr[6],
            custody_pythnet_price_account: &arr[7],
            receiving_token_account: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const WITHDRAW_FEES2_IX_DISCM: [u8; 8usize] = [
    252, 128, 143, 145, 225, 221, 159, 207,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFees2IxArgs {
    pub params: WithdrawFees2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFees2IxData(pub WithdrawFees2IxArgs);
impl From<WithdrawFees2IxArgs> for WithdrawFees2IxData {
    fn from(args: WithdrawFees2IxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFees2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FEES2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawFees2Params>::deserialize(&mut reader)?
        };
        Ok(Self(WithdrawFees2IxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FEES2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_fees2_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFees2Keys,
    args: WithdrawFees2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FEES2_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFees2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_fees2_ix(
    keys: WithdrawFees2Keys,
    args: WithdrawFees2IxArgs,
) -> std::io::Result<Instruction> {
    withdraw_fees2_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn withdraw_fees2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFees2Accounts<'_, '_>,
    args: WithdrawFees2IxArgs,
) -> ProgramResult {
    let keys: WithdrawFees2Keys = accounts.into();
    let ix = withdraw_fees2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_fees2_invoke(
    accounts: WithdrawFees2Accounts<'_, '_>,
    args: WithdrawFees2IxArgs,
) -> ProgramResult {
    withdraw_fees2_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn withdraw_fees2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFees2Accounts<'_, '_>,
    args: WithdrawFees2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFees2Keys = accounts.into();
    let ix = withdraw_fees2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_fees2_invoke_signed(
    accounts: WithdrawFees2Accounts<'_, '_>,
    args: WithdrawFees2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_fees2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_fees2_verify_account_keys(
    accounts: WithdrawFees2Accounts<'_, '_>,
    keys: WithdrawFees2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_token_account.key, keys.custody_token_account),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.receiving_token_account.key, keys.receiving_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_fees2_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFees2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.receiving_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_fees2_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFees2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_fees2_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFees2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_fees2_verify_writable_privileges(accounts)?;
    withdraw_fees2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenMetadataAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenMetadataKeys {
    pub admin: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub transfer_authority: Pubkey,
    pub metadata: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateTokenMetadataAccounts<'_, '_>> for CreateTokenMetadataKeys {
    fn from(accounts: CreateTokenMetadataAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            transfer_authority: *accounts.transfer_authority.key,
            metadata: *accounts.metadata.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateTokenMetadataKeys>
for [AccountMeta; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
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
impl From<[Pubkey; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]> for CreateTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            transfer_authority: pubkeys[3],
            metadata: pubkeys[4],
            lp_token_mint: pubkeys[5],
            token_metadata_program: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<CreateTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.transfer_authority.clone(),
            accounts.metadata.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]>
for CreateTokenMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            transfer_authority: &arr[3],
            metadata: &arr[4],
            lp_token_mint: &arr[5],
            token_metadata_program: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
        }
    }
}
pub const CREATE_TOKEN_METADATA_IX_DISCM: [u8; 8usize] = [
    221, 80, 176, 37, 153, 188, 160, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTokenMetadataIxArgs {
    pub params: CreateTokenMetadataParams,
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
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateTokenMetadataParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateTokenMetadataIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
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
    create_token_metadata_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
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
    create_token_metadata_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
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
        PERPETUALS_PROGRAM_ID,
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
        (*accounts.admin.key, keys.admin),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_token_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
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
pub const CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenLedgerAccounts<'me, 'info> {
    pub token_ledger: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenLedgerKeys {
    pub token_ledger: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTokenLedgerAccounts<'_, '_>> for CreateTokenLedgerKeys {
    fn from(accounts: CreateTokenLedgerAccounts) -> Self {
        Self {
            token_ledger: *accounts.token_ledger.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTokenLedgerKeys> for [AccountMeta; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: true,
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
impl From<[Pubkey; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]> for CreateTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: pubkeys[0],
            payer: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateTokenLedgerAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_ledger.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
for CreateTokenLedgerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_ledger: &arr[0],
            payer: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_TOKEN_LEDGER_IX_DISCM: [u8; 8usize] = [
    232, 242, 197, 253, 240, 143, 129, 52,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenLedgerIxData;
impl CreateTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_LEDGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenLedgerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateTokenLedgerIxData.try_to_vec()?,
    })
}
pub fn create_token_ledger_ix(
    keys: CreateTokenLedgerKeys,
) -> std::io::Result<Instruction> {
    create_token_ledger_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn create_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateTokenLedgerKeys = accounts.into();
    let ix = create_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_ledger_invoke(
    accounts: CreateTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    create_token_ledger_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn create_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenLedgerKeys = accounts.into();
    let ix = create_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_ledger_invoke_signed(
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_ledger_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_token_ledger_verify_account_keys(
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    keys: CreateTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_ledger, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_ledger, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_ledger_verify_writable_privileges(accounts)?;
    create_token_ledger_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REALLOC_CUSTODY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ReallocCustodyAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReallocCustodyKeys {
    pub keeper: Pubkey,
    pub custody: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<ReallocCustodyAccounts<'_, '_>> for ReallocCustodyKeys {
    fn from(accounts: ReallocCustodyAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            custody: *accounts.custody.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<ReallocCustodyKeys> for [AccountMeta; REALLOC_CUSTODY_IX_ACCOUNTS_LEN] {
    fn from(keys: ReallocCustodyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
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
impl From<[Pubkey; REALLOC_CUSTODY_IX_ACCOUNTS_LEN]> for ReallocCustodyKeys {
    fn from(pubkeys: [Pubkey; REALLOC_CUSTODY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            custody: pubkeys[1],
            system_program: pubkeys[2],
            rent: pubkeys[3],
        }
    }
}
impl<'info> From<ReallocCustodyAccounts<'_, 'info>>
for [AccountInfo<'info>; REALLOC_CUSTODY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ReallocCustodyAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.custody.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REALLOC_CUSTODY_IX_ACCOUNTS_LEN]>
for ReallocCustodyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REALLOC_CUSTODY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            custody: &arr[1],
            system_program: &arr[2],
            rent: &arr[3],
        }
    }
}
pub const REALLOC_CUSTODY_IX_DISCM: [u8; 8usize] = [123, 58, 109, 139, 133, 7, 225, 200];
#[derive(Clone, Debug, PartialEq)]
pub struct ReallocCustodyIxData;
impl ReallocCustodyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REALLOC_CUSTODY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REALLOC_CUSTODY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn realloc_custody_ix_with_program_id(
    program_id: Pubkey,
    keys: ReallocCustodyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REALLOC_CUSTODY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ReallocCustodyIxData.try_to_vec()?,
    })
}
pub fn realloc_custody_ix(keys: ReallocCustodyKeys) -> std::io::Result<Instruction> {
    realloc_custody_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn realloc_custody_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ReallocCustodyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ReallocCustodyKeys = accounts.into();
    let ix = realloc_custody_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn realloc_custody_invoke(
    accounts: ReallocCustodyAccounts<'_, '_>,
) -> ProgramResult {
    realloc_custody_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn realloc_custody_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ReallocCustodyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ReallocCustodyKeys = accounts.into();
    let ix = realloc_custody_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn realloc_custody_invoke_signed(
    accounts: ReallocCustodyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    realloc_custody_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, seeds)
}
pub fn realloc_custody_verify_account_keys(
    accounts: ReallocCustodyAccounts<'_, '_>,
    keys: ReallocCustodyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.custody.key, keys.custody),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn realloc_custody_verify_writable_privileges<'me, 'info>(
    accounts: ReallocCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.keeper, accounts.custody] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn realloc_custody_verify_signer_privileges<'me, 'info>(
    accounts: ReallocCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn realloc_custody_verify_account_privileges<'me, 'info>(
    accounts: ReallocCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    realloc_custody_verify_writable_privileges(accounts)?;
    realloc_custody_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REALLOC_POOL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ReallocPoolAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReallocPoolKeys {
    pub keeper: Pubkey,
    pub pool: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<ReallocPoolAccounts<'_, '_>> for ReallocPoolKeys {
    fn from(accounts: ReallocPoolAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            pool: *accounts.pool.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<ReallocPoolKeys> for [AccountMeta; REALLOC_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: ReallocPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
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
impl From<[Pubkey; REALLOC_POOL_IX_ACCOUNTS_LEN]> for ReallocPoolKeys {
    fn from(pubkeys: [Pubkey; REALLOC_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            pool: pubkeys[1],
            system_program: pubkeys[2],
            rent: pubkeys[3],
        }
    }
}
impl<'info> From<ReallocPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; REALLOC_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ReallocPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.pool.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REALLOC_POOL_IX_ACCOUNTS_LEN]>
for ReallocPoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REALLOC_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            pool: &arr[1],
            system_program: &arr[2],
            rent: &arr[3],
        }
    }
}
pub const REALLOC_POOL_IX_DISCM: [u8; 8usize] = [114, 128, 37, 167, 71, 227, 40, 178];
#[derive(Clone, Debug, PartialEq)]
pub struct ReallocPoolIxData;
impl ReallocPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REALLOC_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REALLOC_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn realloc_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: ReallocPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REALLOC_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ReallocPoolIxData.try_to_vec()?,
    })
}
pub fn realloc_pool_ix(keys: ReallocPoolKeys) -> std::io::Result<Instruction> {
    realloc_pool_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn realloc_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ReallocPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ReallocPoolKeys = accounts.into();
    let ix = realloc_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn realloc_pool_invoke(accounts: ReallocPoolAccounts<'_, '_>) -> ProgramResult {
    realloc_pool_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn realloc_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ReallocPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ReallocPoolKeys = accounts.into();
    let ix = realloc_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn realloc_pool_invoke_signed(
    accounts: ReallocPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    realloc_pool_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, seeds)
}
pub fn realloc_pool_verify_account_keys(
    accounts: ReallocPoolAccounts<'_, '_>,
    keys: ReallocPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.pool.key, keys.pool),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn realloc_pool_verify_writable_privileges<'me, 'info>(
    accounts: ReallocPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.keeper, accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn realloc_pool_verify_signer_privileges<'me, 'info>(
    accounts: ReallocPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn realloc_pool_verify_account_privileges<'me, 'info>(
    accounts: ReallocPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    realloc_pool_verify_writable_privileges(accounts)?;
    realloc_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct CreateAndDelegateStakeAccountAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_info: &'me AccountInfo<'info>,
    pub validator_vote_account: &'me AccountInfo<'info>,
    pub stake_config: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub temp_wsol_account: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateAndDelegateStakeAccountKeys {
    pub keeper: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_token_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub stake_account: Pubkey,
    pub stake_info: Pubkey,
    pub validator_vote_account: Pubkey,
    pub stake_config: Pubkey,
    pub wsol_mint: Pubkey,
    pub temp_wsol_account: Pubkey,
    pub rent: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<CreateAndDelegateStakeAccountAccounts<'_, '_>>
for CreateAndDelegateStakeAccountKeys {
    fn from(accounts: CreateAndDelegateStakeAccountAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_token_account: *accounts.custody_token_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            stake_account: *accounts.stake_account.key,
            stake_info: *accounts.stake_info.key,
            validator_vote_account: *accounts.validator_vote_account.key,
            stake_config: *accounts.stake_config.key,
            wsol_mint: *accounts.wsol_mint.key,
            temp_wsol_account: *accounts.temp_wsol_account.key,
            rent: *accounts.rent.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CreateAndDelegateStakeAccountKeys>
for [AccountMeta; CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateAndDelegateStakeAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.validator_vote_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.temp_wsol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
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
                pubkey: keys.stake_program,
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
impl From<[Pubkey; CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateAndDelegateStakeAccountKeys {
    fn from(
        pubkeys: [Pubkey; CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            custody_token_account: pubkeys[4],
            transfer_authority: pubkeys[5],
            stake_account: pubkeys[6],
            stake_info: pubkeys[7],
            validator_vote_account: pubkeys[8],
            stake_config: pubkeys[9],
            wsol_mint: pubkeys[10],
            temp_wsol_account: pubkeys[11],
            rent: pubkeys[12],
            clock: pubkeys[13],
            stake_history: pubkeys[14],
            stake_program: pubkeys[15],
            system_program: pubkeys[16],
            token_program: pubkeys[17],
        }
    }
}
impl<'info> From<CreateAndDelegateStakeAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAndDelegateStakeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_token_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.stake_account.clone(),
            accounts.stake_info.clone(),
            accounts.validator_vote_account.clone(),
            accounts.stake_config.clone(),
            accounts.wsol_mint.clone(),
            accounts.temp_wsol_account.clone(),
            accounts.rent.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateAndDelegateStakeAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            custody_token_account: &arr[4],
            transfer_authority: &arr[5],
            stake_account: &arr[6],
            stake_info: &arr[7],
            validator_vote_account: &arr[8],
            stake_config: &arr[9],
            wsol_mint: &arr[10],
            temp_wsol_account: &arr[11],
            rent: &arr[12],
            clock: &arr[13],
            stake_history: &arr[14],
            stake_program: &arr[15],
            system_program: &arr[16],
            token_program: &arr[17],
        }
    }
}
pub const CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    98, 209, 122, 27, 222, 137, 94, 134,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateAndDelegateStakeAccountIxArgs {
    pub params: CreateAndDelegateStakeAccountParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateAndDelegateStakeAccountIxData(pub CreateAndDelegateStakeAccountIxArgs);
impl From<CreateAndDelegateStakeAccountIxArgs> for CreateAndDelegateStakeAccountIxData {
    fn from(args: CreateAndDelegateStakeAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateAndDelegateStakeAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateAndDelegateStakeAccountParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateAndDelegateStakeAccountIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_and_delegate_stake_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateAndDelegateStakeAccountKeys,
    args: CreateAndDelegateStakeAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_AND_DELEGATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CreateAndDelegateStakeAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_and_delegate_stake_account_ix(
    keys: CreateAndDelegateStakeAccountKeys,
    args: CreateAndDelegateStakeAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_and_delegate_stake_account_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn create_and_delegate_stake_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAndDelegateStakeAccountAccounts<'_, '_>,
    args: CreateAndDelegateStakeAccountIxArgs,
) -> ProgramResult {
    let keys: CreateAndDelegateStakeAccountKeys = accounts.into();
    let ix = create_and_delegate_stake_account_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn create_and_delegate_stake_account_invoke(
    accounts: CreateAndDelegateStakeAccountAccounts<'_, '_>,
    args: CreateAndDelegateStakeAccountIxArgs,
) -> ProgramResult {
    create_and_delegate_stake_account_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_and_delegate_stake_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAndDelegateStakeAccountAccounts<'_, '_>,
    args: CreateAndDelegateStakeAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateAndDelegateStakeAccountKeys = accounts.into();
    let ix = create_and_delegate_stake_account_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_and_delegate_stake_account_invoke_signed(
    accounts: CreateAndDelegateStakeAccountAccounts<'_, '_>,
    args: CreateAndDelegateStakeAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_and_delegate_stake_account_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_and_delegate_stake_account_verify_account_keys(
    accounts: CreateAndDelegateStakeAccountAccounts<'_, '_>,
    keys: CreateAndDelegateStakeAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_token_account.key, keys.custody_token_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.stake_account.key, keys.stake_account),
        (*accounts.stake_info.key, keys.stake_info),
        (*accounts.validator_vote_account.key, keys.validator_vote_account),
        (*accounts.stake_config.key, keys.stake_config),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.temp_wsol_account.key, keys.temp_wsol_account),
        (*accounts.rent.key, keys.rent),
        (*accounts.clock.key, keys.clock),
        (*accounts.stake_history.key, keys.stake_history),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_and_delegate_stake_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateAndDelegateStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.stake_account,
        accounts.stake_info,
        accounts.temp_wsol_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_and_delegate_stake_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateAndDelegateStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_and_delegate_stake_account_verify_account_privileges<'me, 'info>(
    accounts: CreateAndDelegateStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_and_delegate_stake_account_verify_writable_privileges(accounts)?;
    create_and_delegate_stake_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNSTAKE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct UnstakeAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_info: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnstakeKeys {
    pub operator: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub transfer_authority: Pubkey,
    pub stake_account: Pubkey,
    pub stake_info: Pubkey,
    pub clock: Pubkey,
    pub stake_program: Pubkey,
}
impl From<UnstakeAccounts<'_, '_>> for UnstakeKeys {
    fn from(accounts: UnstakeAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            transfer_authority: *accounts.transfer_authority.key,
            stake_account: *accounts.stake_account.key,
            stake_info: *accounts.stake_info.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<UnstakeKeys> for [AccountMeta; UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: UnstakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNSTAKE_IX_ACCOUNTS_LEN]> for UnstakeKeys {
    fn from(pubkeys: [Pubkey; UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            transfer_authority: pubkeys[4],
            stake_account: pubkeys[5],
            stake_info: pubkeys[6],
            clock: pubkeys[7],
            stake_program: pubkeys[8],
        }
    }
}
impl<'info> From<UnstakeAccounts<'_, 'info>>
for [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.transfer_authority.clone(),
            accounts.stake_account.clone(),
            accounts.stake_info.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN]>
for UnstakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            transfer_authority: &arr[4],
            stake_account: &arr[5],
            stake_info: &arr[6],
            clock: &arr[7],
            stake_program: &arr[8],
        }
    }
}
pub const UNSTAKE_IX_DISCM: [u8; 8usize] = [90, 95, 107, 42, 205, 124, 50, 225];
#[derive(Clone, Debug, PartialEq)]
pub struct UnstakeIxData;
impl UnstakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNSTAKE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNSTAKE_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNSTAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnstakeIxData.try_to_vec()?,
    })
}
pub fn unstake_ix(keys: UnstakeKeys) -> std::io::Result<Instruction> {
    unstake_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn unstake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnstakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnstakeKeys = accounts.into();
    let ix = unstake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unstake_invoke(accounts: UnstakeAccounts<'_, '_>) -> ProgramResult {
    unstake_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn unstake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnstakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnstakeKeys = accounts.into();
    let ix = unstake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unstake_invoke_signed(
    accounts: UnstakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unstake_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, seeds)
}
pub fn unstake_verify_account_keys(
    accounts: UnstakeAccounts<'_, '_>,
    keys: UnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.stake_account.key, keys.stake_account),
        (*accounts.stake_info.key, keys.stake_info),
        (*accounts.clock.key, keys.clock),
        (*accounts.stake_program.key, keys.stake_program),
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
    for should_be_writable in [accounts.stake_account, accounts.stake_info] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unstake_verify_signer_privileges<'me, 'info>(
    accounts: UnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
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
pub const WITHDRAW_STAKE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawStakeAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_info: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawStakeKeys {
    pub keeper: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_token_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub stake_account: Pubkey,
    pub stake_info: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawStakeAccounts<'_, '_>> for WithdrawStakeKeys {
    fn from(accounts: WithdrawStakeAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_token_account: *accounts.custody_token_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            stake_account: *accounts.stake_account.key,
            stake_info: *accounts.stake_info.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawStakeKeys> for [AccountMeta; WITHDRAW_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_info,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.stake_program,
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
impl From<[Pubkey; WITHDRAW_STAKE_IX_ACCOUNTS_LEN]> for WithdrawStakeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            custody_token_account: pubkeys[4],
            transfer_authority: pubkeys[5],
            stake_account: pubkeys[6],
            stake_info: pubkeys[7],
            clock: pubkeys[8],
            stake_history: pubkeys[9],
            stake_program: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<WithdrawStakeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_STAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_token_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.stake_account.clone(),
            accounts.stake_info.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_STAKE_IX_ACCOUNTS_LEN]>
for WithdrawStakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            custody_token_account: &arr[4],
            transfer_authority: &arr[5],
            stake_account: &arr[6],
            stake_info: &arr[7],
            clock: &arr[8],
            stake_history: &arr[9],
            stake_program: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const WITHDRAW_STAKE_IX_DISCM: [u8; 8usize] = [153, 8, 22, 138, 105, 176, 87, 66];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawStakeIxData;
impl WithdrawStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_STAKE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_STAKE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawStakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawStakeIxData.try_to_vec()?,
    })
}
pub fn withdraw_stake_ix(keys: WithdrawStakeKeys) -> std::io::Result<Instruction> {
    withdraw_stake_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn withdraw_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawStakeKeys = accounts.into();
    let ix = withdraw_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_stake_invoke(accounts: WithdrawStakeAccounts<'_, '_>) -> ProgramResult {
    withdraw_stake_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn withdraw_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawStakeKeys = accounts.into();
    let ix = withdraw_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_stake_invoke_signed(
    accounts: WithdrawStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_stake_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, seeds)
}
pub fn withdraw_stake_verify_account_keys(
    accounts: WithdrawStakeAccounts<'_, '_>,
    keys: WithdrawStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_token_account.key, keys.custody_token_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.stake_account.key, keys.stake_account),
        (*accounts.stake_info.key, keys.stake_info),
        (*accounts.clock.key, keys.clock),
        (*accounts.stake_history.key, keys.stake_history),
        (*accounts.stake_program.key, keys.stake_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_stake_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.transfer_authority,
        accounts.stake_account,
        accounts.stake_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_stake_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_stake_verify_account_privileges<'me, 'info>(
    accounts: WithdrawStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_stake_verify_writable_privileges(accounts)?;
    withdraw_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_STAKE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RedeemStakeAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_info: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemStakeKeys {
    pub keeper: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub stake_account: Pubkey,
    pub stake_info: Pubkey,
}
impl From<RedeemStakeAccounts<'_, '_>> for RedeemStakeKeys {
    fn from(accounts: RedeemStakeAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            stake_account: *accounts.stake_account.key,
            stake_info: *accounts.stake_info.key,
        }
    }
}
impl From<RedeemStakeKeys> for [AccountMeta; REDEEM_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_info,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_STAKE_IX_ACCOUNTS_LEN]> for RedeemStakeKeys {
    fn from(pubkeys: [Pubkey; REDEEM_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            stake_account: pubkeys[4],
            stake_info: pubkeys[5],
        }
    }
}
impl<'info> From<RedeemStakeAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_STAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.stake_account.clone(),
            accounts.stake_info.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_STAKE_IX_ACCOUNTS_LEN]>
for RedeemStakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            stake_account: &arr[4],
            stake_info: &arr[5],
        }
    }
}
pub const REDEEM_STAKE_IX_DISCM: [u8; 8usize] = [178, 203, 250, 105, 133, 118, 255, 69];
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemStakeIxData;
impl RedeemStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_STAKE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_STAKE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemStakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RedeemStakeIxData.try_to_vec()?,
    })
}
pub fn redeem_stake_ix(keys: RedeemStakeKeys) -> std::io::Result<Instruction> {
    redeem_stake_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn redeem_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RedeemStakeKeys = accounts.into();
    let ix = redeem_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_stake_invoke(accounts: RedeemStakeAccounts<'_, '_>) -> ProgramResult {
    redeem_stake_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn redeem_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemStakeKeys = accounts.into();
    let ix = redeem_stake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_stake_invoke_signed(
    accounts: RedeemStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_stake_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, seeds)
}
pub fn redeem_stake_verify_account_keys(
    accounts: RedeemStakeAccounts<'_, '_>,
    keys: RedeemStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.stake_account.key, keys.stake_account),
        (*accounts.stake_info.key, keys.stake_info),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_stake_verify_writable_privileges<'me, 'info>(
    accounts: RedeemStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.custody, accounts.stake_info] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_stake_verify_signer_privileges<'me, 'info>(
    accounts: RedeemStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_stake_verify_account_privileges<'me, 'info>(
    accounts: RedeemStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_stake_verify_writable_privileges(accounts)?;
    redeem_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPERATOR_SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct OperatorSetCustodyConfigAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OperatorSetCustodyConfigKeys {
    pub operator: Pubkey,
    pub custody: Pubkey,
}
impl From<OperatorSetCustodyConfigAccounts<'_, '_>> for OperatorSetCustodyConfigKeys {
    fn from(accounts: OperatorSetCustodyConfigAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            custody: *accounts.custody.key,
        }
    }
}
impl From<OperatorSetCustodyConfigKeys>
for [AccountMeta; OPERATOR_SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: OperatorSetCustodyConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; OPERATOR_SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN]>
for OperatorSetCustodyConfigKeys {
    fn from(pubkeys: [Pubkey; OPERATOR_SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            custody: pubkeys[1],
        }
    }
}
impl<'info> From<OperatorSetCustodyConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; OPERATOR_SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: OperatorSetCustodyConfigAccounts<'_, 'info>) -> Self {
        [accounts.operator.clone(), accounts.custody.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; OPERATOR_SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN]>
for OperatorSetCustodyConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPERATOR_SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            custody: &arr[1],
        }
    }
}
pub const OPERATOR_SET_CUSTODY_CONFIG_IX_DISCM: [u8; 8usize] = [
    166, 137, 92, 204, 145, 224, 24, 218,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperatorSetCustodyConfigIxArgs {
    pub params: OperatorSetCustodyConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorSetCustodyConfigIxData(pub OperatorSetCustodyConfigIxArgs);
impl From<OperatorSetCustodyConfigIxArgs> for OperatorSetCustodyConfigIxData {
    fn from(args: OperatorSetCustodyConfigIxArgs) -> Self {
        Self(args)
    }
}
impl OperatorSetCustodyConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPERATOR_SET_CUSTODY_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <OperatorSetCustodyConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(OperatorSetCustodyConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPERATOR_SET_CUSTODY_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn operator_set_custody_config_ix_with_program_id(
    program_id: Pubkey,
    keys: OperatorSetCustodyConfigKeys,
    args: OperatorSetCustodyConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPERATOR_SET_CUSTODY_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: OperatorSetCustodyConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn operator_set_custody_config_ix(
    keys: OperatorSetCustodyConfigKeys,
    args: OperatorSetCustodyConfigIxArgs,
) -> std::io::Result<Instruction> {
    operator_set_custody_config_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn operator_set_custody_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetCustodyConfigAccounts<'_, '_>,
    args: OperatorSetCustodyConfigIxArgs,
) -> ProgramResult {
    let keys: OperatorSetCustodyConfigKeys = accounts.into();
    let ix = operator_set_custody_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn operator_set_custody_config_invoke(
    accounts: OperatorSetCustodyConfigAccounts<'_, '_>,
    args: OperatorSetCustodyConfigIxArgs,
) -> ProgramResult {
    operator_set_custody_config_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn operator_set_custody_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetCustodyConfigAccounts<'_, '_>,
    args: OperatorSetCustodyConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OperatorSetCustodyConfigKeys = accounts.into();
    let ix = operator_set_custody_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn operator_set_custody_config_invoke_signed(
    accounts: OperatorSetCustodyConfigAccounts<'_, '_>,
    args: OperatorSetCustodyConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    operator_set_custody_config_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn operator_set_custody_config_verify_account_keys(
    accounts: OperatorSetCustodyConfigAccounts<'_, '_>,
    keys: OperatorSetCustodyConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.custody.key, keys.custody),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn operator_set_custody_config_verify_writable_privileges<'me, 'info>(
    accounts: OperatorSetCustodyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.custody] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn operator_set_custody_config_verify_signer_privileges<'me, 'info>(
    accounts: OperatorSetCustodyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn operator_set_custody_config_verify_account_privileges<'me, 'info>(
    accounts: OperatorSetCustodyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    operator_set_custody_config_verify_writable_privileges(accounts)?;
    operator_set_custody_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPERATOR_SET_POOL_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct OperatorSetPoolConfigAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OperatorSetPoolConfigKeys {
    pub operator: Pubkey,
    pub pool: Pubkey,
}
impl From<OperatorSetPoolConfigAccounts<'_, '_>> for OperatorSetPoolConfigKeys {
    fn from(accounts: OperatorSetPoolConfigAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<OperatorSetPoolConfigKeys>
for [AccountMeta; OPERATOR_SET_POOL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: OperatorSetPoolConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; OPERATOR_SET_POOL_CONFIG_IX_ACCOUNTS_LEN]>
for OperatorSetPoolConfigKeys {
    fn from(pubkeys: [Pubkey; OPERATOR_SET_POOL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<OperatorSetPoolConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; OPERATOR_SET_POOL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: OperatorSetPoolConfigAccounts<'_, 'info>) -> Self {
        [accounts.operator.clone(), accounts.pool.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; OPERATOR_SET_POOL_CONFIG_IX_ACCOUNTS_LEN]>
for OperatorSetPoolConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPERATOR_SET_POOL_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            pool: &arr[1],
        }
    }
}
pub const OPERATOR_SET_POOL_CONFIG_IX_DISCM: [u8; 8usize] = [
    76, 201, 80, 18, 199, 92, 246, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperatorSetPoolConfigIxArgs {
    pub params: OperatorSetPoolConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorSetPoolConfigIxData(pub OperatorSetPoolConfigIxArgs);
impl From<OperatorSetPoolConfigIxArgs> for OperatorSetPoolConfigIxData {
    fn from(args: OperatorSetPoolConfigIxArgs) -> Self {
        Self(args)
    }
}
impl OperatorSetPoolConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPERATOR_SET_POOL_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <OperatorSetPoolConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(OperatorSetPoolConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPERATOR_SET_POOL_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn operator_set_pool_config_ix_with_program_id(
    program_id: Pubkey,
    keys: OperatorSetPoolConfigKeys,
    args: OperatorSetPoolConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPERATOR_SET_POOL_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: OperatorSetPoolConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn operator_set_pool_config_ix(
    keys: OperatorSetPoolConfigKeys,
    args: OperatorSetPoolConfigIxArgs,
) -> std::io::Result<Instruction> {
    operator_set_pool_config_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn operator_set_pool_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetPoolConfigAccounts<'_, '_>,
    args: OperatorSetPoolConfigIxArgs,
) -> ProgramResult {
    let keys: OperatorSetPoolConfigKeys = accounts.into();
    let ix = operator_set_pool_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn operator_set_pool_config_invoke(
    accounts: OperatorSetPoolConfigAccounts<'_, '_>,
    args: OperatorSetPoolConfigIxArgs,
) -> ProgramResult {
    operator_set_pool_config_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn operator_set_pool_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OperatorSetPoolConfigAccounts<'_, '_>,
    args: OperatorSetPoolConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OperatorSetPoolConfigKeys = accounts.into();
    let ix = operator_set_pool_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn operator_set_pool_config_invoke_signed(
    accounts: OperatorSetPoolConfigAccounts<'_, '_>,
    args: OperatorSetPoolConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    operator_set_pool_config_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn operator_set_pool_config_verify_account_keys(
    accounts: OperatorSetPoolConfigAccounts<'_, '_>,
    keys: OperatorSetPoolConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn operator_set_pool_config_verify_writable_privileges<'me, 'info>(
    accounts: OperatorSetPoolConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn operator_set_pool_config_verify_signer_privileges<'me, 'info>(
    accounts: OperatorSetPoolConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn operator_set_pool_config_verify_account_privileges<'me, 'info>(
    accounts: OperatorSetPoolConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    operator_set_pool_config_verify_writable_privileges(accounts)?;
    operator_set_pool_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TEST_INIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct TestInitAccounts<'me, 'info> {
    pub upgrade_authority: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TestInitKeys {
    pub upgrade_authority: Pubkey,
    pub admin: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<TestInitAccounts<'_, '_>> for TestInitKeys {
    fn from(accounts: TestInitAccounts) -> Self {
        Self {
            upgrade_authority: *accounts.upgrade_authority.key,
            admin: *accounts.admin.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<TestInitKeys> for [AccountMeta; TEST_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: TestInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.upgrade_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
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
impl From<[Pubkey; TEST_INIT_IX_ACCOUNTS_LEN]> for TestInitKeys {
    fn from(pubkeys: [Pubkey; TEST_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            upgrade_authority: pubkeys[0],
            admin: pubkeys[1],
            transfer_authority: pubkeys[2],
            perpetuals: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<TestInitAccounts<'_, 'info>>
for [AccountInfo<'info>; TEST_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: TestInitAccounts<'_, 'info>) -> Self {
        [
            accounts.upgrade_authority.clone(),
            accounts.admin.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TEST_INIT_IX_ACCOUNTS_LEN]>
for TestInitAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TEST_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            upgrade_authority: &arr[0],
            admin: &arr[1],
            transfer_authority: &arr[2],
            perpetuals: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const TEST_INIT_IX_DISCM: [u8; 8usize] = [48, 51, 92, 122, 81, 19, 112, 41];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TestInitIxArgs {
    pub params: TestInitParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TestInitIxData(pub TestInitIxArgs);
impl From<TestInitIxArgs> for TestInitIxData {
    fn from(args: TestInitIxArgs) -> Self {
        Self(args)
    }
}
impl TestInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TEST_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <TestInitParams>::deserialize(&mut reader)?
        };
        Ok(Self(TestInitIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TEST_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn test_init_ix_with_program_id(
    program_id: Pubkey,
    keys: TestInitKeys,
    args: TestInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TEST_INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: TestInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn test_init_ix(
    keys: TestInitKeys,
    args: TestInitIxArgs,
) -> std::io::Result<Instruction> {
    test_init_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn test_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TestInitAccounts<'_, '_>,
    args: TestInitIxArgs,
) -> ProgramResult {
    let keys: TestInitKeys = accounts.into();
    let ix = test_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn test_init_invoke(
    accounts: TestInitAccounts<'_, '_>,
    args: TestInitIxArgs,
) -> ProgramResult {
    test_init_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn test_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TestInitAccounts<'_, '_>,
    args: TestInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TestInitKeys = accounts.into();
    let ix = test_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn test_init_invoke_signed(
    accounts: TestInitAccounts<'_, '_>,
    args: TestInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    test_init_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args, seeds)
}
pub fn test_init_verify_account_keys(
    accounts: TestInitAccounts<'_, '_>,
    keys: TestInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.upgrade_authority.key, keys.upgrade_authority),
        (*accounts.admin.key, keys.admin),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn test_init_verify_writable_privileges<'me, 'info>(
    accounts: TestInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.upgrade_authority,
        accounts.transfer_authority,
        accounts.perpetuals,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn test_init_verify_signer_privileges<'me, 'info>(
    accounts: TestInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.upgrade_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn test_init_verify_account_privileges<'me, 'info>(
    accounts: TestInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    test_init_verify_writable_privileges(accounts)?;
    test_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_TEST_TIME_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetTestTimeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetTestTimeKeys {
    pub admin: Pubkey,
    pub perpetuals: Pubkey,
}
impl From<SetTestTimeAccounts<'_, '_>> for SetTestTimeKeys {
    fn from(accounts: SetTestTimeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            perpetuals: *accounts.perpetuals.key,
        }
    }
}
impl From<SetTestTimeKeys> for [AccountMeta; SET_TEST_TIME_IX_ACCOUNTS_LEN] {
    fn from(keys: SetTestTimeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_TEST_TIME_IX_ACCOUNTS_LEN]> for SetTestTimeKeys {
    fn from(pubkeys: [Pubkey; SET_TEST_TIME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            perpetuals: pubkeys[1],
        }
    }
}
impl<'info> From<SetTestTimeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_TEST_TIME_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetTestTimeAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.perpetuals.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_TEST_TIME_IX_ACCOUNTS_LEN]>
for SetTestTimeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_TEST_TIME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            perpetuals: &arr[1],
        }
    }
}
pub const SET_TEST_TIME_IX_DISCM: [u8; 8usize] = [
    242, 231, 177, 251, 126, 145, 159, 104,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetTestTimeIxArgs {
    pub params: SetTestTimeParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetTestTimeIxData(pub SetTestTimeIxArgs);
impl From<SetTestTimeIxArgs> for SetTestTimeIxData {
    fn from(args: SetTestTimeIxArgs) -> Self {
        Self(args)
    }
}
impl SetTestTimeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_TEST_TIME_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SetTestTimeParams>::deserialize(&mut reader)?
        };
        Ok(Self(SetTestTimeIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_TEST_TIME_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_test_time_ix_with_program_id(
    program_id: Pubkey,
    keys: SetTestTimeKeys,
    args: SetTestTimeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_TEST_TIME_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetTestTimeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_test_time_ix(
    keys: SetTestTimeKeys,
    args: SetTestTimeIxArgs,
) -> std::io::Result<Instruction> {
    set_test_time_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn set_test_time_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetTestTimeAccounts<'_, '_>,
    args: SetTestTimeIxArgs,
) -> ProgramResult {
    let keys: SetTestTimeKeys = accounts.into();
    let ix = set_test_time_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_test_time_invoke(
    accounts: SetTestTimeAccounts<'_, '_>,
    args: SetTestTimeIxArgs,
) -> ProgramResult {
    set_test_time_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn set_test_time_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetTestTimeAccounts<'_, '_>,
    args: SetTestTimeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetTestTimeKeys = accounts.into();
    let ix = set_test_time_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_test_time_invoke_signed(
    accounts: SetTestTimeAccounts<'_, '_>,
    args: SetTestTimeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_test_time_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_test_time_verify_account_keys(
    accounts: SetTestTimeAccounts<'_, '_>,
    keys: SetTestTimeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.perpetuals.key, keys.perpetuals),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_test_time_verify_writable_privileges<'me, 'info>(
    accounts: SetTestTimeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perpetuals] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_test_time_verify_signer_privileges<'me, 'info>(
    accounts: SetTestTimeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_test_time_verify_account_privileges<'me, 'info>(
    accounts: SetTestTimeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_test_time_verify_writable_privileges(accounts)?;
    set_test_time_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetTokenLedgerAccounts<'me, 'info> {
    pub token_ledger: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetTokenLedgerKeys {
    pub token_ledger: Pubkey,
    pub token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<SetTokenLedgerAccounts<'_, '_>> for SetTokenLedgerKeys {
    fn from(accounts: SetTokenLedgerAccounts) -> Self {
        Self {
            token_ledger: *accounts.token_ledger.key,
            token_account: *accounts.token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SetTokenLedgerKeys> for [AccountMeta; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account,
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
impl From<[Pubkey; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]> for SetTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: pubkeys[0],
            token_account: pubkeys[1],
            token_program: pubkeys[2],
        }
    }
}
impl<'info> From<SetTokenLedgerAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_ledger.clone(),
            accounts.token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
for SetTokenLedgerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: &arr[0],
            token_account: &arr[1],
            token_program: &arr[2],
        }
    }
}
pub const SET_TOKEN_LEDGER_IX_DISCM: [u8; 8usize] = [228, 85, 185, 112, 78, 79, 77, 2];
#[derive(Clone, Debug, PartialEq)]
pub struct SetTokenLedgerIxData;
impl SetTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_TOKEN_LEDGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: SetTokenLedgerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetTokenLedgerIxData.try_to_vec()?,
    })
}
pub fn set_token_ledger_ix(keys: SetTokenLedgerKeys) -> std::io::Result<Instruction> {
    set_token_ledger_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn set_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetTokenLedgerKeys = accounts.into();
    let ix = set_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_token_ledger_invoke(
    accounts: SetTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    set_token_ledger_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn set_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetTokenLedgerKeys = accounts.into();
    let ix = set_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_token_ledger_invoke_signed(
    accounts: SetTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_token_ledger_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_token_ledger_verify_account_keys(
    accounts: SetTokenLedgerAccounts<'_, '_>,
    keys: SetTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.token_account.key, keys.token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: SetTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_ledger] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: SetTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_token_ledger_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SWAP2_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct Swap2Accounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub receiving_custody: &'me AccountInfo<'info>,
    pub receiving_custody_doves_price_account: &'me AccountInfo<'info>,
    pub receiving_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub receiving_custody_token_account: &'me AccountInfo<'info>,
    pub dispensing_custody: &'me AccountInfo<'info>,
    pub dispensing_custody_doves_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Swap2Keys {
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub receiving_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub receiving_custody: Pubkey,
    pub receiving_custody_doves_price_account: Pubkey,
    pub receiving_custody_pythnet_price_account: Pubkey,
    pub receiving_custody_token_account: Pubkey,
    pub dispensing_custody: Pubkey,
    pub dispensing_custody_doves_price_account: Pubkey,
    pub dispensing_custody_pythnet_price_account: Pubkey,
    pub dispensing_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<Swap2Accounts<'_, '_>> for Swap2Keys {
    fn from(accounts: Swap2Accounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            receiving_account: *accounts.receiving_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            receiving_custody: *accounts.receiving_custody.key,
            receiving_custody_doves_price_account: *accounts
                .receiving_custody_doves_price_account
                .key,
            receiving_custody_pythnet_price_account: *accounts
                .receiving_custody_pythnet_price_account
                .key,
            receiving_custody_token_account: *accounts
                .receiving_custody_token_account
                .key,
            dispensing_custody: *accounts.dispensing_custody.key,
            dispensing_custody_doves_price_account: *accounts
                .dispensing_custody_doves_price_account
                .key,
            dispensing_custody_pythnet_price_account: *accounts
                .dispensing_custody_pythnet_price_account
                .key,
            dispensing_custody_token_account: *accounts
                .dispensing_custody_token_account
                .key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<Swap2Keys> for [AccountMeta; SWAP2_IX_ACCOUNTS_LEN] {
    fn from(keys: Swap2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP2_IX_ACCOUNTS_LEN]> for Swap2Keys {
    fn from(pubkeys: [Pubkey; SWAP2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            funding_account: pubkeys[1],
            receiving_account: pubkeys[2],
            transfer_authority: pubkeys[3],
            perpetuals: pubkeys[4],
            pool: pubkeys[5],
            receiving_custody: pubkeys[6],
            receiving_custody_doves_price_account: pubkeys[7],
            receiving_custody_pythnet_price_account: pubkeys[8],
            receiving_custody_token_account: pubkeys[9],
            dispensing_custody: pubkeys[10],
            dispensing_custody_doves_price_account: pubkeys[11],
            dispensing_custody_pythnet_price_account: pubkeys[12],
            dispensing_custody_token_account: pubkeys[13],
            token_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<Swap2Accounts<'_, 'info>>
for [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN] {
    fn from(accounts: Swap2Accounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.receiving_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.receiving_custody.clone(),
            accounts.receiving_custody_doves_price_account.clone(),
            accounts.receiving_custody_pythnet_price_account.clone(),
            accounts.receiving_custody_token_account.clone(),
            accounts.dispensing_custody.clone(),
            accounts.dispensing_custody_doves_price_account.clone(),
            accounts.dispensing_custody_pythnet_price_account.clone(),
            accounts.dispensing_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN]>
for Swap2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            funding_account: &arr[1],
            receiving_account: &arr[2],
            transfer_authority: &arr[3],
            perpetuals: &arr[4],
            pool: &arr[5],
            receiving_custody: &arr[6],
            receiving_custody_doves_price_account: &arr[7],
            receiving_custody_pythnet_price_account: &arr[8],
            receiving_custody_token_account: &arr[9],
            dispensing_custody: &arr[10],
            dispensing_custody_doves_price_account: &arr[11],
            dispensing_custody_pythnet_price_account: &arr[12],
            dispensing_custody_token_account: &arr[13],
            token_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const SWAP2_IX_DISCM: [u8; 8usize] = [65, 75, 63, 76, 235, 91, 91, 136];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Swap2IxArgs {
    pub params: Swap2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Swap2IxData(pub Swap2IxArgs);
impl From<Swap2IxArgs> for Swap2IxData {
    fn from(args: Swap2IxArgs) -> Self {
        Self(args)
    }
}
impl Swap2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <Swap2Params>::deserialize(&mut reader)?
        };
        Ok(Self(Swap2IxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap2_ix_with_program_id(
    program_id: Pubkey,
    keys: Swap2Keys,
    args: Swap2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP2_IX_ACCOUNTS_LEN] = keys.into();
    let data: Swap2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap2_ix(keys: Swap2Keys, args: Swap2IxArgs) -> std::io::Result<Instruction> {
    swap2_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn swap2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
) -> ProgramResult {
    let keys: Swap2Keys = accounts.into();
    let ix = swap2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap2_invoke(
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
) -> ProgramResult {
    swap2_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn swap2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: Swap2Keys = accounts.into();
    let ix = swap2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap2_invoke_signed(
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap2_invoke_signed_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap2_verify_account_keys(
    accounts: Swap2Accounts<'_, '_>,
    keys: Swap2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.receiving_custody.key, keys.receiving_custody),
        (
            *accounts.receiving_custody_doves_price_account.key,
            keys.receiving_custody_doves_price_account,
        ),
        (
            *accounts.receiving_custody_pythnet_price_account.key,
            keys.receiving_custody_pythnet_price_account,
        ),
        (
            *accounts.receiving_custody_token_account.key,
            keys.receiving_custody_token_account,
        ),
        (*accounts.dispensing_custody.key, keys.dispensing_custody),
        (
            *accounts.dispensing_custody_doves_price_account.key,
            keys.dispensing_custody_doves_price_account,
        ),
        (
            *accounts.dispensing_custody_pythnet_price_account.key,
            keys.dispensing_custody_pythnet_price_account,
        ),
        (
            *accounts.dispensing_custody_token_account.key,
            keys.dispensing_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap2_verify_writable_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funding_account,
        accounts.receiving_account,
        accounts.pool,
        accounts.receiving_custody,
        accounts.receiving_custody_token_account,
        accounts.dispensing_custody,
        accounts.dispensing_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap2_verify_signer_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap2_verify_account_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap2_verify_writable_privileges(accounts)?;
    swap2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SwapWithTokenLedgerAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub receiving_custody: &'me AccountInfo<'info>,
    pub receiving_custody_doves_price_account: &'me AccountInfo<'info>,
    pub receiving_custody_token_account: &'me AccountInfo<'info>,
    pub dispensing_custody: &'me AccountInfo<'info>,
    pub dispensing_custody_doves_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_token_account: &'me AccountInfo<'info>,
    pub token_ledger: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub instruction: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapWithTokenLedgerKeys {
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub receiving_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub receiving_custody: Pubkey,
    pub receiving_custody_doves_price_account: Pubkey,
    pub receiving_custody_token_account: Pubkey,
    pub dispensing_custody: Pubkey,
    pub dispensing_custody_doves_price_account: Pubkey,
    pub dispensing_custody_token_account: Pubkey,
    pub token_ledger: Pubkey,
    pub token_program: Pubkey,
    pub instruction: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapWithTokenLedgerAccounts<'_, '_>> for SwapWithTokenLedgerKeys {
    fn from(accounts: SwapWithTokenLedgerAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            receiving_account: *accounts.receiving_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            receiving_custody: *accounts.receiving_custody.key,
            receiving_custody_doves_price_account: *accounts
                .receiving_custody_doves_price_account
                .key,
            receiving_custody_token_account: *accounts
                .receiving_custody_token_account
                .key,
            dispensing_custody: *accounts.dispensing_custody.key,
            dispensing_custody_doves_price_account: *accounts
                .dispensing_custody_doves_price_account
                .key,
            dispensing_custody_token_account: *accounts
                .dispensing_custody_token_account
                .key,
            token_ledger: *accounts.token_ledger.key,
            token_program: *accounts.token_program.key,
            instruction: *accounts.instruction.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapWithTokenLedgerKeys>
for [AccountMeta; SWAP_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapWithTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]> for SwapWithTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; SWAP_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            funding_account: pubkeys[1],
            receiving_account: pubkeys[2],
            transfer_authority: pubkeys[3],
            perpetuals: pubkeys[4],
            pool: pubkeys[5],
            receiving_custody: pubkeys[6],
            receiving_custody_doves_price_account: pubkeys[7],
            receiving_custody_token_account: pubkeys[8],
            dispensing_custody: pubkeys[9],
            dispensing_custody_doves_price_account: pubkeys[10],
            dispensing_custody_token_account: pubkeys[11],
            token_ledger: pubkeys[12],
            token_program: pubkeys[13],
            instruction: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<SwapWithTokenLedgerAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapWithTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.receiving_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.receiving_custody.clone(),
            accounts.receiving_custody_doves_price_account.clone(),
            accounts.receiving_custody_token_account.clone(),
            accounts.dispensing_custody.clone(),
            accounts.dispensing_custody_doves_price_account.clone(),
            accounts.dispensing_custody_token_account.clone(),
            accounts.token_ledger.clone(),
            accounts.token_program.clone(),
            accounts.instruction.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
for SwapWithTokenLedgerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            funding_account: &arr[1],
            receiving_account: &arr[2],
            transfer_authority: &arr[3],
            perpetuals: &arr[4],
            pool: &arr[5],
            receiving_custody: &arr[6],
            receiving_custody_doves_price_account: &arr[7],
            receiving_custody_token_account: &arr[8],
            dispensing_custody: &arr[9],
            dispensing_custody_doves_price_account: &arr[10],
            dispensing_custody_token_account: &arr[11],
            token_ledger: &arr[12],
            token_program: &arr[13],
            instruction: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const SWAP_WITH_TOKEN_LEDGER_IX_DISCM: [u8; 8usize] = [
    139, 141, 238, 197, 41, 211, 172, 19,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapWithTokenLedgerIxArgs {
    pub params: SwapWithTokenLedgerParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithTokenLedgerIxData(pub SwapWithTokenLedgerIxArgs);
impl From<SwapWithTokenLedgerIxArgs> for SwapWithTokenLedgerIxData {
    fn from(args: SwapWithTokenLedgerIxArgs) -> Self {
        Self(args)
    }
}
impl SwapWithTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_WITH_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapWithTokenLedgerParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(SwapWithTokenLedgerIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_WITH_TOKEN_LEDGER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_with_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapWithTokenLedgerKeys,
    args: SwapWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapWithTokenLedgerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_with_token_ledger_ix(
    keys: SwapWithTokenLedgerKeys,
    args: SwapWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    swap_with_token_ledger_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn swap_with_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithTokenLedgerAccounts<'_, '_>,
    args: SwapWithTokenLedgerIxArgs,
) -> ProgramResult {
    let keys: SwapWithTokenLedgerKeys = accounts.into();
    let ix = swap_with_token_ledger_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_with_token_ledger_invoke(
    accounts: SwapWithTokenLedgerAccounts<'_, '_>,
    args: SwapWithTokenLedgerIxArgs,
) -> ProgramResult {
    swap_with_token_ledger_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn swap_with_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithTokenLedgerAccounts<'_, '_>,
    args: SwapWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapWithTokenLedgerKeys = accounts.into();
    let ix = swap_with_token_ledger_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_with_token_ledger_invoke_signed(
    accounts: SwapWithTokenLedgerAccounts<'_, '_>,
    args: SwapWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_with_token_ledger_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_with_token_ledger_verify_account_keys(
    accounts: SwapWithTokenLedgerAccounts<'_, '_>,
    keys: SwapWithTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.receiving_custody.key, keys.receiving_custody),
        (
            *accounts.receiving_custody_doves_price_account.key,
            keys.receiving_custody_doves_price_account,
        ),
        (
            *accounts.receiving_custody_token_account.key,
            keys.receiving_custody_token_account,
        ),
        (*accounts.dispensing_custody.key, keys.dispensing_custody),
        (
            *accounts.dispensing_custody_doves_price_account.key,
            keys.dispensing_custody_doves_price_account,
        ),
        (
            *accounts.dispensing_custody_token_account.key,
            keys.dispensing_custody_token_account,
        ),
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.instruction.key, keys.instruction),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_with_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: SwapWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funding_account,
        accounts.receiving_account,
        accounts.pool,
        accounts.receiving_custody,
        accounts.receiving_custody_token_account,
        accounts.dispensing_custody,
        accounts.dispensing_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_with_token_ledger_verify_signer_privileges<'me, 'info>(
    accounts: SwapWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_with_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: SwapWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_with_token_ledger_verify_writable_privileges(accounts)?;
    swap_with_token_ledger_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct InstantIncreasePositionPreSwapAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub receiving_custody: &'me AccountInfo<'info>,
    pub receiving_custody_doves_price_account: &'me AccountInfo<'info>,
    pub receiving_custody_token_account: &'me AccountInfo<'info>,
    pub dispensing_custody: &'me AccountInfo<'info>,
    pub dispensing_custody_doves_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub instruction: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantIncreasePositionPreSwapKeys {
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub receiving_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub receiving_custody: Pubkey,
    pub receiving_custody_doves_price_account: Pubkey,
    pub receiving_custody_token_account: Pubkey,
    pub dispensing_custody: Pubkey,
    pub dispensing_custody_doves_price_account: Pubkey,
    pub dispensing_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub instruction: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InstantIncreasePositionPreSwapAccounts<'_, '_>>
for InstantIncreasePositionPreSwapKeys {
    fn from(accounts: InstantIncreasePositionPreSwapAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            receiving_account: *accounts.receiving_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            receiving_custody: *accounts.receiving_custody.key,
            receiving_custody_doves_price_account: *accounts
                .receiving_custody_doves_price_account
                .key,
            receiving_custody_token_account: *accounts
                .receiving_custody_token_account
                .key,
            dispensing_custody: *accounts.dispensing_custody.key,
            dispensing_custody_doves_price_account: *accounts
                .dispensing_custody_doves_price_account
                .key,
            dispensing_custody_token_account: *accounts
                .dispensing_custody_token_account
                .key,
            token_program: *accounts.token_program.key,
            instruction: *accounts.instruction.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InstantIncreasePositionPreSwapKeys>
for [AccountMeta; INSTANT_INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantIncreasePositionPreSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN]>
for InstantIncreasePositionPreSwapKeys {
    fn from(
        pubkeys: [Pubkey; INSTANT_INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: pubkeys[0],
            funding_account: pubkeys[1],
            receiving_account: pubkeys[2],
            transfer_authority: pubkeys[3],
            perpetuals: pubkeys[4],
            pool: pubkeys[5],
            receiving_custody: pubkeys[6],
            receiving_custody_doves_price_account: pubkeys[7],
            receiving_custody_token_account: pubkeys[8],
            dispensing_custody: pubkeys[9],
            dispensing_custody_doves_price_account: pubkeys[10],
            dispensing_custody_token_account: pubkeys[11],
            token_program: pubkeys[12],
            instruction: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<InstantIncreasePositionPreSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantIncreasePositionPreSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.receiving_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.receiving_custody.clone(),
            accounts.receiving_custody_doves_price_account.clone(),
            accounts.receiving_custody_token_account.clone(),
            accounts.dispensing_custody.clone(),
            accounts.dispensing_custody_doves_price_account.clone(),
            accounts.dispensing_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.instruction.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INSTANT_INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN]>
for InstantIncreasePositionPreSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INSTANT_INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            funding_account: &arr[1],
            receiving_account: &arr[2],
            transfer_authority: &arr[3],
            perpetuals: &arr[4],
            pool: &arr[5],
            receiving_custody: &arr[6],
            receiving_custody_doves_price_account: &arr[7],
            receiving_custody_token_account: &arr[8],
            dispensing_custody: &arr[9],
            dispensing_custody_doves_price_account: &arr[10],
            dispensing_custody_token_account: &arr[11],
            token_program: &arr[12],
            instruction: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const INSTANT_INCREASE_POSITION_PRE_SWAP_IX_DISCM: [u8; 8usize] = [
    197, 38, 86, 165, 199, 23, 38, 234,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantIncreasePositionPreSwapIxArgs {
    pub params: InstantIncreasePositionPreSwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantIncreasePositionPreSwapIxData(
    pub InstantIncreasePositionPreSwapIxArgs,
);
impl From<InstantIncreasePositionPreSwapIxArgs>
for InstantIncreasePositionPreSwapIxData {
    fn from(args: InstantIncreasePositionPreSwapIxArgs) -> Self {
        Self(args)
    }
}
impl InstantIncreasePositionPreSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_INCREASE_POSITION_PRE_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InstantIncreasePositionPreSwapParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InstantIncreasePositionPreSwapIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_INCREASE_POSITION_PRE_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_increase_position_pre_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantIncreasePositionPreSwapKeys,
    args: InstantIncreasePositionPreSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InstantIncreasePositionPreSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_increase_position_pre_swap_ix(
    keys: InstantIncreasePositionPreSwapKeys,
    args: InstantIncreasePositionPreSwapIxArgs,
) -> std::io::Result<Instruction> {
    instant_increase_position_pre_swap_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn instant_increase_position_pre_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantIncreasePositionPreSwapAccounts<'_, '_>,
    args: InstantIncreasePositionPreSwapIxArgs,
) -> ProgramResult {
    let keys: InstantIncreasePositionPreSwapKeys = accounts.into();
    let ix = instant_increase_position_pre_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_increase_position_pre_swap_invoke(
    accounts: InstantIncreasePositionPreSwapAccounts<'_, '_>,
    args: InstantIncreasePositionPreSwapIxArgs,
) -> ProgramResult {
    instant_increase_position_pre_swap_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn instant_increase_position_pre_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantIncreasePositionPreSwapAccounts<'_, '_>,
    args: InstantIncreasePositionPreSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantIncreasePositionPreSwapKeys = accounts.into();
    let ix = instant_increase_position_pre_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_increase_position_pre_swap_invoke_signed(
    accounts: InstantIncreasePositionPreSwapAccounts<'_, '_>,
    args: InstantIncreasePositionPreSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_increase_position_pre_swap_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_increase_position_pre_swap_verify_account_keys(
    accounts: InstantIncreasePositionPreSwapAccounts<'_, '_>,
    keys: InstantIncreasePositionPreSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.receiving_custody.key, keys.receiving_custody),
        (
            *accounts.receiving_custody_doves_price_account.key,
            keys.receiving_custody_doves_price_account,
        ),
        (
            *accounts.receiving_custody_token_account.key,
            keys.receiving_custody_token_account,
        ),
        (*accounts.dispensing_custody.key, keys.dispensing_custody),
        (
            *accounts.dispensing_custody_doves_price_account.key,
            keys.dispensing_custody_doves_price_account,
        ),
        (
            *accounts.dispensing_custody_token_account.key,
            keys.dispensing_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.instruction.key, keys.instruction),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_increase_position_pre_swap_verify_writable_privileges<'me, 'info>(
    accounts: InstantIncreasePositionPreSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funding_account,
        accounts.receiving_account,
        accounts.pool,
        accounts.receiving_custody,
        accounts.receiving_custody_token_account,
        accounts.dispensing_custody,
        accounts.dispensing_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_increase_position_pre_swap_verify_signer_privileges<'me, 'info>(
    accounts: InstantIncreasePositionPreSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_increase_position_pre_swap_verify_account_privileges<'me, 'info>(
    accounts: InstantIncreasePositionPreSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_increase_position_pre_swap_verify_writable_privileges(accounts)?;
    instant_increase_position_pre_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidity2Accounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub lp_token_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidity2Keys {
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub lp_token_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub custody_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidity2Accounts<'_, '_>> for AddLiquidity2Keys {
    fn from(accounts: AddLiquidity2Accounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            lp_token_account: *accounts.lp_token_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            custody_token_account: *accounts.custody_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidity2Keys> for [AccountMeta; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidity2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN]> for AddLiquidity2Keys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            funding_account: pubkeys[1],
            lp_token_account: pubkeys[2],
            transfer_authority: pubkeys[3],
            perpetuals: pubkeys[4],
            pool: pubkeys[5],
            custody: pubkeys[6],
            custody_doves_price_account: pubkeys[7],
            custody_pythnet_price_account: pubkeys[8],
            custody_token_account: pubkeys[9],
            lp_token_mint: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<AddLiquidity2Accounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidity2Accounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.lp_token_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.custody_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN]>
for AddLiquidity2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            funding_account: &arr[1],
            lp_token_account: &arr[2],
            transfer_authority: &arr[3],
            perpetuals: &arr[4],
            pool: &arr[5],
            custody: &arr[6],
            custody_doves_price_account: &arr[7],
            custody_pythnet_price_account: &arr[8],
            custody_token_account: &arr[9],
            lp_token_mint: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const ADD_LIQUIDITY2_IX_DISCM: [u8; 8usize] = [228, 162, 78, 28, 70, 219, 116, 115];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidity2IxArgs {
    pub params: AddLiquidity2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidity2IxData(pub AddLiquidity2IxArgs);
impl From<AddLiquidity2IxArgs> for AddLiquidity2IxData {
    fn from(args: AddLiquidity2IxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidity2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <AddLiquidity2Params>::deserialize(&mut reader)?
        };
        Ok(Self(AddLiquidity2IxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity2_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidity2Keys,
    args: AddLiquidity2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidity2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity2_ix(
    keys: AddLiquidity2Keys,
    args: AddLiquidity2IxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity2_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn add_liquidity2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidity2Accounts<'_, '_>,
    args: AddLiquidity2IxArgs,
) -> ProgramResult {
    let keys: AddLiquidity2Keys = accounts.into();
    let ix = add_liquidity2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity2_invoke(
    accounts: AddLiquidity2Accounts<'_, '_>,
    args: AddLiquidity2IxArgs,
) -> ProgramResult {
    add_liquidity2_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidity2Accounts<'_, '_>,
    args: AddLiquidity2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidity2Keys = accounts.into();
    let ix = add_liquidity2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity2_invoke_signed(
    accounts: AddLiquidity2Accounts<'_, '_>,
    args: AddLiquidity2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity2_verify_account_keys(
    accounts: AddLiquidity2Accounts<'_, '_>,
    keys: AddLiquidity2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.lp_token_account.key, keys.lp_token_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.custody_token_account.key, keys.custody_token_account),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity2_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidity2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funding_account,
        accounts.lp_token_account,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity2_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidity2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity2_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidity2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity2_verify_writable_privileges(accounts)?;
    add_liquidity2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidity2Accounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub lp_token_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidity2Keys {
    pub owner: Pubkey,
    pub receiving_account: Pubkey,
    pub lp_token_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub custody_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveLiquidity2Accounts<'_, '_>> for RemoveLiquidity2Keys {
    fn from(accounts: RemoveLiquidity2Accounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            receiving_account: *accounts.receiving_account.key,
            lp_token_account: *accounts.lp_token_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            custody_token_account: *accounts.custody_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveLiquidity2Keys> for [AccountMeta; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidity2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN]> for RemoveLiquidity2Keys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            receiving_account: pubkeys[1],
            lp_token_account: pubkeys[2],
            transfer_authority: pubkeys[3],
            perpetuals: pubkeys[4],
            pool: pubkeys[5],
            custody: pubkeys[6],
            custody_doves_price_account: pubkeys[7],
            custody_pythnet_price_account: pubkeys[8],
            custody_token_account: pubkeys[9],
            lp_token_mint: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<RemoveLiquidity2Accounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidity2Accounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.receiving_account.clone(),
            accounts.lp_token_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.custody_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN]>
for RemoveLiquidity2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            receiving_account: &arr[1],
            lp_token_account: &arr[2],
            transfer_authority: &arr[3],
            perpetuals: &arr[4],
            pool: &arr[5],
            custody: &arr[6],
            custody_doves_price_account: &arr[7],
            custody_pythnet_price_account: &arr[8],
            custody_token_account: &arr[9],
            lp_token_mint: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const REMOVE_LIQUIDITY2_IX_DISCM: [u8; 8usize] = [
    230, 215, 82, 127, 241, 101, 227, 146,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidity2IxArgs {
    pub params: RemoveLiquidity2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidity2IxData(pub RemoveLiquidity2IxArgs);
impl From<RemoveLiquidity2IxArgs> for RemoveLiquidity2IxData {
    fn from(args: RemoveLiquidity2IxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidity2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <RemoveLiquidity2Params>::deserialize(&mut reader)?
        };
        Ok(Self(RemoveLiquidity2IxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity2_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidity2Keys,
    args: RemoveLiquidity2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidity2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity2_ix(
    keys: RemoveLiquidity2Keys,
    args: RemoveLiquidity2IxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity2_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn remove_liquidity2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidity2Accounts<'_, '_>,
    args: RemoveLiquidity2IxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidity2Keys = accounts.into();
    let ix = remove_liquidity2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity2_invoke(
    accounts: RemoveLiquidity2Accounts<'_, '_>,
    args: RemoveLiquidity2IxArgs,
) -> ProgramResult {
    remove_liquidity2_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn remove_liquidity2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidity2Accounts<'_, '_>,
    args: RemoveLiquidity2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidity2Keys = accounts.into();
    let ix = remove_liquidity2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity2_invoke_signed(
    accounts: RemoveLiquidity2Accounts<'_, '_>,
    args: RemoveLiquidity2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity2_verify_account_keys(
    accounts: RemoveLiquidity2Accounts<'_, '_>,
    keys: RemoveLiquidity2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.lp_token_account.key, keys.lp_token_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.custody_token_account.key, keys.custody_token_account),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity2_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidity2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.receiving_account,
        accounts.lp_token_account,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity2_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidity2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity2_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidity2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity2_verify_writable_privileges(accounts)?;
    remove_liquidity2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CreateIncreasePositionMarketRequestAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateIncreasePositionMarketRequestKeys {
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub custody: Pubkey,
    pub collateral_custody: Pubkey,
    pub input_mint: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateIncreasePositionMarketRequestAccounts<'_, '_>>
for CreateIncreasePositionMarketRequestKeys {
    fn from(accounts: CreateIncreasePositionMarketRequestAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            custody: *accounts.custody.key,
            collateral_custody: *accounts.collateral_custody.key,
            input_mint: *accounts.input_mint.key,
            referral: *accounts.referral.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateIncreasePositionMarketRequestKeys>
for [AccountMeta; CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateIncreasePositionMarketRequestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN]>
for CreateIncreasePositionMarketRequestKeys {
    fn from(
        pubkeys: [Pubkey; CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: pubkeys[0],
            funding_account: pubkeys[1],
            perpetuals: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            position_request: pubkeys[5],
            position_request_ata: pubkeys[6],
            custody: pubkeys[7],
            collateral_custody: pubkeys[8],
            input_mint: pubkeys[9],
            referral: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            system_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<CreateIncreasePositionMarketRequestAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateIncreasePositionMarketRequestAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.custody.clone(),
            accounts.collateral_custody.clone(),
            accounts.input_mint.clone(),
            accounts.referral.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN],
> for CreateIncreasePositionMarketRequestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            funding_account: &arr[1],
            perpetuals: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            position_request: &arr[5],
            position_request_ata: &arr[6],
            custody: &arr[7],
            collateral_custody: &arr[8],
            input_mint: &arr[9],
            referral: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            system_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_DISCM: [u8; 8usize] = [
    184, 85, 199, 24, 105, 171, 156, 56,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateIncreasePositionMarketRequestIxArgs {
    pub params: CreateIncreasePositionMarketRequestParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIncreasePositionMarketRequestIxData(
    pub CreateIncreasePositionMarketRequestIxArgs,
);
impl From<CreateIncreasePositionMarketRequestIxArgs>
for CreateIncreasePositionMarketRequestIxData {
    fn from(args: CreateIncreasePositionMarketRequestIxArgs) -> Self {
        Self(args)
    }
}
impl CreateIncreasePositionMarketRequestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateIncreasePositionMarketRequestParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateIncreasePositionMarketRequestIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_increase_position_market_request_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateIncreasePositionMarketRequestKeys,
    args: CreateIncreasePositionMarketRequestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_INCREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CreateIncreasePositionMarketRequestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_increase_position_market_request_ix(
    keys: CreateIncreasePositionMarketRequestKeys,
    args: CreateIncreasePositionMarketRequestIxArgs,
) -> std::io::Result<Instruction> {
    create_increase_position_market_request_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn create_increase_position_market_request_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateIncreasePositionMarketRequestAccounts<'_, '_>,
    args: CreateIncreasePositionMarketRequestIxArgs,
) -> ProgramResult {
    let keys: CreateIncreasePositionMarketRequestKeys = accounts.into();
    let ix = create_increase_position_market_request_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn create_increase_position_market_request_invoke(
    accounts: CreateIncreasePositionMarketRequestAccounts<'_, '_>,
    args: CreateIncreasePositionMarketRequestIxArgs,
) -> ProgramResult {
    create_increase_position_market_request_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_increase_position_market_request_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateIncreasePositionMarketRequestAccounts<'_, '_>,
    args: CreateIncreasePositionMarketRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateIncreasePositionMarketRequestKeys = accounts.into();
    let ix = create_increase_position_market_request_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_increase_position_market_request_invoke_signed(
    accounts: CreateIncreasePositionMarketRequestAccounts<'_, '_>,
    args: CreateIncreasePositionMarketRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_increase_position_market_request_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_increase_position_market_request_verify_account_keys(
    accounts: CreateIncreasePositionMarketRequestAccounts<'_, '_>,
    keys: CreateIncreasePositionMarketRequestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.custody.key, keys.custody),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.referral.key, keys.referral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_increase_position_market_request_verify_writable_privileges<'me, 'info>(
    accounts: CreateIncreasePositionMarketRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.funding_account,
        accounts.position,
        accounts.position_request,
        accounts.position_request_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_increase_position_market_request_verify_signer_privileges<'me, 'info>(
    accounts: CreateIncreasePositionMarketRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_increase_position_market_request_verify_account_privileges<'me, 'info>(
    accounts: CreateIncreasePositionMarketRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_increase_position_market_request_verify_writable_privileges(accounts)?;
    create_increase_position_market_request_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct CreateDecreasePositionRequest2Accounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub desired_mint: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateDecreasePositionRequest2Keys {
    pub owner: Pubkey,
    pub receiving_account: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub desired_mint: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateDecreasePositionRequest2Accounts<'_, '_>>
for CreateDecreasePositionRequest2Keys {
    fn from(accounts: CreateDecreasePositionRequest2Accounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            receiving_account: *accounts.receiving_account.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            desired_mint: *accounts.desired_mint.key,
            referral: *accounts.referral.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateDecreasePositionRequest2Keys>
for [AccountMeta; CREATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateDecreasePositionRequest2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.desired_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN]>
for CreateDecreasePositionRequest2Keys {
    fn from(
        pubkeys: [Pubkey; CREATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: pubkeys[0],
            receiving_account: pubkeys[1],
            perpetuals: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            position_request: pubkeys[5],
            position_request_ata: pubkeys[6],
            custody: pubkeys[7],
            custody_doves_price_account: pubkeys[8],
            custody_pythnet_price_account: pubkeys[9],
            collateral_custody: pubkeys[10],
            desired_mint: pubkeys[11],
            referral: pubkeys[12],
            token_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            system_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<CreateDecreasePositionRequest2Accounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateDecreasePositionRequest2Accounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.receiving_account.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.desired_mint.clone(),
            accounts.referral.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN]>
for CreateDecreasePositionRequest2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            receiving_account: &arr[1],
            perpetuals: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            position_request: &arr[5],
            position_request_ata: &arr[6],
            custody: &arr[7],
            custody_doves_price_account: &arr[8],
            custody_pythnet_price_account: &arr[9],
            collateral_custody: &arr[10],
            desired_mint: &arr[11],
            referral: &arr[12],
            token_program: &arr[13],
            associated_token_program: &arr[14],
            system_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const CREATE_DECREASE_POSITION_REQUEST2_IX_DISCM: [u8; 8usize] = [
    105, 64, 201, 82, 250, 14, 109, 77,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateDecreasePositionRequest2IxArgs {
    pub params: CreateDecreasePositionRequest2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateDecreasePositionRequest2IxData(
    pub CreateDecreasePositionRequest2IxArgs,
);
impl From<CreateDecreasePositionRequest2IxArgs>
for CreateDecreasePositionRequest2IxData {
    fn from(args: CreateDecreasePositionRequest2IxArgs) -> Self {
        Self(args)
    }
}
impl CreateDecreasePositionRequest2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_DECREASE_POSITION_REQUEST2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateDecreasePositionRequest2Params>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateDecreasePositionRequest2IxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_DECREASE_POSITION_REQUEST2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_decrease_position_request2_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateDecreasePositionRequest2Keys,
    args: CreateDecreasePositionRequest2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CreateDecreasePositionRequest2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_decrease_position_request2_ix(
    keys: CreateDecreasePositionRequest2Keys,
    args: CreateDecreasePositionRequest2IxArgs,
) -> std::io::Result<Instruction> {
    create_decrease_position_request2_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn create_decrease_position_request2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateDecreasePositionRequest2Accounts<'_, '_>,
    args: CreateDecreasePositionRequest2IxArgs,
) -> ProgramResult {
    let keys: CreateDecreasePositionRequest2Keys = accounts.into();
    let ix = create_decrease_position_request2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn create_decrease_position_request2_invoke(
    accounts: CreateDecreasePositionRequest2Accounts<'_, '_>,
    args: CreateDecreasePositionRequest2IxArgs,
) -> ProgramResult {
    create_decrease_position_request2_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_decrease_position_request2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateDecreasePositionRequest2Accounts<'_, '_>,
    args: CreateDecreasePositionRequest2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateDecreasePositionRequest2Keys = accounts.into();
    let ix = create_decrease_position_request2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_decrease_position_request2_invoke_signed(
    accounts: CreateDecreasePositionRequest2Accounts<'_, '_>,
    args: CreateDecreasePositionRequest2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_decrease_position_request2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_decrease_position_request2_verify_account_keys(
    accounts: CreateDecreasePositionRequest2Accounts<'_, '_>,
    keys: CreateDecreasePositionRequest2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (*accounts.desired_mint.key, keys.desired_mint),
        (*accounts.referral.key, keys.referral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_decrease_position_request2_verify_writable_privileges<'me, 'info>(
    accounts: CreateDecreasePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.receiving_account,
        accounts.position_request,
        accounts.position_request_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_decrease_position_request2_verify_signer_privileges<'me, 'info>(
    accounts: CreateDecreasePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_decrease_position_request2_verify_account_privileges<'me, 'info>(
    accounts: CreateDecreasePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_decrease_position_request2_verify_writable_privileges(accounts)?;
    create_decrease_position_request2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CreateDecreasePositionMarketRequestAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub desired_mint: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateDecreasePositionMarketRequestKeys {
    pub owner: Pubkey,
    pub receiving_account: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub custody: Pubkey,
    pub collateral_custody: Pubkey,
    pub desired_mint: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateDecreasePositionMarketRequestAccounts<'_, '_>>
for CreateDecreasePositionMarketRequestKeys {
    fn from(accounts: CreateDecreasePositionMarketRequestAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            receiving_account: *accounts.receiving_account.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            custody: *accounts.custody.key,
            collateral_custody: *accounts.collateral_custody.key,
            desired_mint: *accounts.desired_mint.key,
            referral: *accounts.referral.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateDecreasePositionMarketRequestKeys>
for [AccountMeta; CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateDecreasePositionMarketRequestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.desired_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN]>
for CreateDecreasePositionMarketRequestKeys {
    fn from(
        pubkeys: [Pubkey; CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: pubkeys[0],
            receiving_account: pubkeys[1],
            perpetuals: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            position_request: pubkeys[5],
            position_request_ata: pubkeys[6],
            custody: pubkeys[7],
            collateral_custody: pubkeys[8],
            desired_mint: pubkeys[9],
            referral: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            system_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<CreateDecreasePositionMarketRequestAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateDecreasePositionMarketRequestAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.receiving_account.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.custody.clone(),
            accounts.collateral_custody.clone(),
            accounts.desired_mint.clone(),
            accounts.referral.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN],
> for CreateDecreasePositionMarketRequestAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            receiving_account: &arr[1],
            perpetuals: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            position_request: &arr[5],
            position_request_ata: &arr[6],
            custody: &arr[7],
            collateral_custody: &arr[8],
            desired_mint: &arr[9],
            referral: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            system_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_DISCM: [u8; 8usize] = [
    74, 198, 195, 86, 193, 99, 1, 79,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateDecreasePositionMarketRequestIxArgs {
    pub params: CreateDecreasePositionMarketRequestParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateDecreasePositionMarketRequestIxData(
    pub CreateDecreasePositionMarketRequestIxArgs,
);
impl From<CreateDecreasePositionMarketRequestIxArgs>
for CreateDecreasePositionMarketRequestIxData {
    fn from(args: CreateDecreasePositionMarketRequestIxArgs) -> Self {
        Self(args)
    }
}
impl CreateDecreasePositionMarketRequestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateDecreasePositionMarketRequestParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateDecreasePositionMarketRequestIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_decrease_position_market_request_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateDecreasePositionMarketRequestKeys,
    args: CreateDecreasePositionMarketRequestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_DECREASE_POSITION_MARKET_REQUEST_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CreateDecreasePositionMarketRequestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_decrease_position_market_request_ix(
    keys: CreateDecreasePositionMarketRequestKeys,
    args: CreateDecreasePositionMarketRequestIxArgs,
) -> std::io::Result<Instruction> {
    create_decrease_position_market_request_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn create_decrease_position_market_request_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateDecreasePositionMarketRequestAccounts<'_, '_>,
    args: CreateDecreasePositionMarketRequestIxArgs,
) -> ProgramResult {
    let keys: CreateDecreasePositionMarketRequestKeys = accounts.into();
    let ix = create_decrease_position_market_request_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn create_decrease_position_market_request_invoke(
    accounts: CreateDecreasePositionMarketRequestAccounts<'_, '_>,
    args: CreateDecreasePositionMarketRequestIxArgs,
) -> ProgramResult {
    create_decrease_position_market_request_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_decrease_position_market_request_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateDecreasePositionMarketRequestAccounts<'_, '_>,
    args: CreateDecreasePositionMarketRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateDecreasePositionMarketRequestKeys = accounts.into();
    let ix = create_decrease_position_market_request_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_decrease_position_market_request_invoke_signed(
    accounts: CreateDecreasePositionMarketRequestAccounts<'_, '_>,
    args: CreateDecreasePositionMarketRequestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_decrease_position_market_request_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_decrease_position_market_request_verify_account_keys(
    accounts: CreateDecreasePositionMarketRequestAccounts<'_, '_>,
    keys: CreateDecreasePositionMarketRequestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.custody.key, keys.custody),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (*accounts.desired_mint.key, keys.desired_mint),
        (*accounts.referral.key, keys.referral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_decrease_position_market_request_verify_writable_privileges<'me, 'info>(
    accounts: CreateDecreasePositionMarketRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.receiving_account,
        accounts.position_request,
        accounts.position_request_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_decrease_position_market_request_verify_signer_privileges<'me, 'info>(
    accounts: CreateDecreasePositionMarketRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_decrease_position_market_request_verify_account_privileges<'me, 'info>(
    accounts: CreateDecreasePositionMarketRequestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_decrease_position_market_request_verify_writable_privileges(accounts)?;
    create_decrease_position_market_request_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDecreasePositionRequest2Accounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDecreasePositionRequest2Keys {
    pub owner: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
}
impl From<UpdateDecreasePositionRequest2Accounts<'_, '_>>
for UpdateDecreasePositionRequest2Keys {
    fn from(accounts: UpdateDecreasePositionRequest2Accounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_request: *accounts.position_request.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
        }
    }
}
impl From<UpdateDecreasePositionRequest2Keys>
for [AccountMeta; UPDATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateDecreasePositionRequest2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN]>
for UpdateDecreasePositionRequest2Keys {
    fn from(
        pubkeys: [Pubkey; UPDATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            position: pubkeys[3],
            position_request: pubkeys[4],
            custody: pubkeys[5],
            custody_doves_price_account: pubkeys[6],
            custody_pythnet_price_account: pubkeys[7],
        }
    }
}
impl<'info> From<UpdateDecreasePositionRequest2Accounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateDecreasePositionRequest2Accounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_request.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN]>
for UpdateDecreasePositionRequest2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            position: &arr[3],
            position_request: &arr[4],
            custody: &arr[5],
            custody_doves_price_account: &arr[6],
            custody_pythnet_price_account: &arr[7],
        }
    }
}
pub const UPDATE_DECREASE_POSITION_REQUEST2_IX_DISCM: [u8; 8usize] = [
    144, 200, 249, 255, 108, 217, 249, 116,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDecreasePositionRequest2IxArgs {
    pub params: UpdateDecreasePositionRequest2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDecreasePositionRequest2IxData(
    pub UpdateDecreasePositionRequest2IxArgs,
);
impl From<UpdateDecreasePositionRequest2IxArgs>
for UpdateDecreasePositionRequest2IxData {
    fn from(args: UpdateDecreasePositionRequest2IxArgs) -> Self {
        Self(args)
    }
}
impl UpdateDecreasePositionRequest2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DECREASE_POSITION_REQUEST2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateDecreasePositionRequest2Params>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateDecreasePositionRequest2IxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DECREASE_POSITION_REQUEST2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_decrease_position_request2_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateDecreasePositionRequest2Keys,
    args: UpdateDecreasePositionRequest2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_DECREASE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateDecreasePositionRequest2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_decrease_position_request2_ix(
    keys: UpdateDecreasePositionRequest2Keys,
    args: UpdateDecreasePositionRequest2IxArgs,
) -> std::io::Result<Instruction> {
    update_decrease_position_request2_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_decrease_position_request2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDecreasePositionRequest2Accounts<'_, '_>,
    args: UpdateDecreasePositionRequest2IxArgs,
) -> ProgramResult {
    let keys: UpdateDecreasePositionRequest2Keys = accounts.into();
    let ix = update_decrease_position_request2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_decrease_position_request2_invoke(
    accounts: UpdateDecreasePositionRequest2Accounts<'_, '_>,
    args: UpdateDecreasePositionRequest2IxArgs,
) -> ProgramResult {
    update_decrease_position_request2_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_decrease_position_request2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDecreasePositionRequest2Accounts<'_, '_>,
    args: UpdateDecreasePositionRequest2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateDecreasePositionRequest2Keys = accounts.into();
    let ix = update_decrease_position_request2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_decrease_position_request2_invoke_signed(
    accounts: UpdateDecreasePositionRequest2Accounts<'_, '_>,
    args: UpdateDecreasePositionRequest2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_decrease_position_request2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_decrease_position_request2_verify_account_keys(
    accounts: UpdateDecreasePositionRequest2Accounts<'_, '_>,
    keys: UpdateDecreasePositionRequest2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (*accounts.custody_pythnet_price_account.key, keys.custody_pythnet_price_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_decrease_position_request2_verify_writable_privileges<'me, 'info>(
    accounts: UpdateDecreasePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position_request] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_decrease_position_request2_verify_signer_privileges<'me, 'info>(
    accounts: UpdateDecreasePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_decrease_position_request2_verify_account_privileges<'me, 'info>(
    accounts: UpdateDecreasePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_decrease_position_request2_verify_writable_privileges(accounts)?;
    update_decrease_position_request2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_REQUEST2_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionRequest2Accounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub owner_ata: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionRequest2Keys {
    pub keeper: Pubkey,
    pub owner: Pubkey,
    pub owner_ata: Pubkey,
    pub pool: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClosePositionRequest2Accounts<'_, '_>> for ClosePositionRequest2Keys {
    fn from(accounts: ClosePositionRequest2Accounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            owner: *accounts.owner.key,
            owner_ata: *accounts.owner_ata.key,
            pool: *accounts.pool.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClosePositionRequest2Keys>
for [AccountMeta; CLOSE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionRequest2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_POSITION_REQUEST2_IX_ACCOUNTS_LEN]>
for ClosePositionRequest2Keys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_REQUEST2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            owner: pubkeys[1],
            owner_ata: pubkeys[2],
            pool: pubkeys[3],
            position_request: pubkeys[4],
            position_request_ata: pubkeys[5],
            position: pubkeys[6],
            mint: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<ClosePositionRequest2Accounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionRequest2Accounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.owner.clone(),
            accounts.owner_ata.clone(),
            accounts.pool.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_REQUEST2_IX_ACCOUNTS_LEN]>
for ClosePositionRequest2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_POSITION_REQUEST2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            owner: &arr[1],
            owner_ata: &arr[2],
            pool: &arr[3],
            position_request: &arr[4],
            position_request_ata: &arr[5],
            position: &arr[6],
            mint: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            associated_token_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const CLOSE_POSITION_REQUEST2_IX_DISCM: [u8; 8usize] = [
    121, 68, 162, 28, 216, 47, 200, 66,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionRequest2IxData;
impl ClosePositionRequest2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION_REQUEST2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_REQUEST2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_position_request2_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePositionRequest2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION_REQUEST2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePositionRequest2IxData.try_to_vec()?,
    })
}
pub fn close_position_request2_ix(
    keys: ClosePositionRequest2Keys,
) -> std::io::Result<Instruction> {
    close_position_request2_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn close_position_request2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionRequest2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePositionRequest2Keys = accounts.into();
    let ix = close_position_request2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_position_request2_invoke(
    accounts: ClosePositionRequest2Accounts<'_, '_>,
) -> ProgramResult {
    close_position_request2_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn close_position_request2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionRequest2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePositionRequest2Keys = accounts.into();
    let ix = close_position_request2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_position_request2_invoke_signed(
    accounts: ClosePositionRequest2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position_request2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_position_request2_verify_account_keys(
    accounts: ClosePositionRequest2Accounts<'_, '_>,
    keys: ClosePositionRequest2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.owner_ata.key, keys.owner_ata),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_position_request2_verify_writable_privileges<'me, 'info>(
    accounts: ClosePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper,
        accounts.owner,
        accounts.owner_ata,
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_position_request2_verify_signer_privileges<'me, 'info>(
    accounts: ClosePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_position_request2_verify_account_privileges<'me, 'info>(
    accounts: ClosePositionRequest2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_position_request2_verify_writable_privileges(accounts)?;
    close_position_request2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_REQUEST3_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionRequest3Accounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub owner_ata: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionRequest3Keys {
    pub keeper: Pubkey,
    pub owner: Pubkey,
    pub owner_ata: Pubkey,
    pub pool: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClosePositionRequest3Accounts<'_, '_>> for ClosePositionRequest3Keys {
    fn from(accounts: ClosePositionRequest3Accounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            owner: *accounts.owner.key,
            owner_ata: *accounts.owner_ata.key,
            pool: *accounts.pool.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClosePositionRequest3Keys>
for [AccountMeta; CLOSE_POSITION_REQUEST3_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionRequest3Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
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
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_POSITION_REQUEST3_IX_ACCOUNTS_LEN]>
for ClosePositionRequest3Keys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_REQUEST3_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            owner: pubkeys[1],
            owner_ata: pubkeys[2],
            pool: pubkeys[3],
            position_request: pubkeys[4],
            position_request_ata: pubkeys[5],
            position: pubkeys[6],
            custody: pubkeys[7],
            mint: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<ClosePositionRequest3Accounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_REQUEST3_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionRequest3Accounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.owner.clone(),
            accounts.owner_ata.clone(),
            accounts.pool.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_REQUEST3_IX_ACCOUNTS_LEN]>
for ClosePositionRequest3Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_POSITION_REQUEST3_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            owner: &arr[1],
            owner_ata: &arr[2],
            pool: &arr[3],
            position_request: &arr[4],
            position_request_ata: &arr[5],
            position: &arr[6],
            custody: &arr[7],
            mint: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            associated_token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CLOSE_POSITION_REQUEST3_IX_DISCM: [u8; 8usize] = [
    122, 130, 33, 18, 211, 44, 161, 58,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionRequest3IxData;
impl ClosePositionRequest3IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION_REQUEST3_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_REQUEST3_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_position_request3_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePositionRequest3Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION_REQUEST3_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePositionRequest3IxData.try_to_vec()?,
    })
}
pub fn close_position_request3_ix(
    keys: ClosePositionRequest3Keys,
) -> std::io::Result<Instruction> {
    close_position_request3_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys)
}
pub fn close_position_request3_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionRequest3Accounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePositionRequest3Keys = accounts.into();
    let ix = close_position_request3_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_position_request3_invoke(
    accounts: ClosePositionRequest3Accounts<'_, '_>,
) -> ProgramResult {
    close_position_request3_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts)
}
pub fn close_position_request3_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionRequest3Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePositionRequest3Keys = accounts.into();
    let ix = close_position_request3_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_position_request3_invoke_signed(
    accounts: ClosePositionRequest3Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position_request3_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_position_request3_verify_account_keys(
    accounts: ClosePositionRequest3Accounts<'_, '_>,
    keys: ClosePositionRequest3Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.owner_ata.key, keys.owner_ata),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_position_request3_verify_writable_privileges<'me, 'info>(
    accounts: ClosePositionRequest3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper,
        accounts.owner,
        accounts.owner_ata,
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
        accounts.custody,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_position_request3_verify_signer_privileges<'me, 'info>(
    accounts: ClosePositionRequest3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_position_request3_verify_account_privileges<'me, 'info>(
    accounts: ClosePositionRequest3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_position_request3_verify_writable_privileges(accounts)?;
    close_position_request3_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_POSITION4_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct IncreasePosition4Accounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreasePosition4Keys {
    pub keeper: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_pythnet_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<IncreasePosition4Accounts<'_, '_>> for IncreasePosition4Keys {
    fn from(accounts: IncreasePosition4Accounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_pythnet_price_account: *accounts
                .collateral_custody_pythnet_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<IncreasePosition4Keys> for [AccountMeta; INCREASE_POSITION4_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreasePosition4Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_POSITION4_IX_ACCOUNTS_LEN]> for IncreasePosition4Keys {
    fn from(pubkeys: [Pubkey; INCREASE_POSITION4_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            position_request: pubkeys[3],
            position_request_ata: pubkeys[4],
            position: pubkeys[5],
            custody: pubkeys[6],
            custody_doves_price_account: pubkeys[7],
            custody_pythnet_price_account: pubkeys[8],
            collateral_custody: pubkeys[9],
            collateral_custody_doves_price_account: pubkeys[10],
            collateral_custody_pythnet_price_account: pubkeys[11],
            collateral_custody_token_account: pubkeys[12],
            token_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<IncreasePosition4Accounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_POSITION4_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreasePosition4Accounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_pythnet_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_POSITION4_IX_ACCOUNTS_LEN]>
for IncreasePosition4Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INCREASE_POSITION4_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            position_request: &arr[3],
            position_request_ata: &arr[4],
            position: &arr[5],
            custody: &arr[6],
            custody_doves_price_account: &arr[7],
            custody_pythnet_price_account: &arr[8],
            collateral_custody: &arr[9],
            collateral_custody_doves_price_account: &arr[10],
            collateral_custody_pythnet_price_account: &arr[11],
            collateral_custody_token_account: &arr[12],
            token_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const INCREASE_POSITION4_IX_DISCM: [u8; 8usize] = [67, 147, 53, 23, 43, 57, 16, 67];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePosition4IxArgs {
    pub params: IncreasePosition4Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePosition4IxData(pub IncreasePosition4IxArgs);
impl From<IncreasePosition4IxArgs> for IncreasePosition4IxData {
    fn from(args: IncreasePosition4IxArgs) -> Self {
        Self(args)
    }
}
impl IncreasePosition4IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION4_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <IncreasePosition4Params>::deserialize(&mut reader)?
        };
        Ok(Self(IncreasePosition4IxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION4_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_position4_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreasePosition4Keys,
    args: IncreasePosition4IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_POSITION4_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreasePosition4IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_position4_ix(
    keys: IncreasePosition4Keys,
    args: IncreasePosition4IxArgs,
) -> std::io::Result<Instruction> {
    increase_position4_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn increase_position4_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePosition4Accounts<'_, '_>,
    args: IncreasePosition4IxArgs,
) -> ProgramResult {
    let keys: IncreasePosition4Keys = accounts.into();
    let ix = increase_position4_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_position4_invoke(
    accounts: IncreasePosition4Accounts<'_, '_>,
    args: IncreasePosition4IxArgs,
) -> ProgramResult {
    increase_position4_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn increase_position4_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePosition4Accounts<'_, '_>,
    args: IncreasePosition4IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreasePosition4Keys = accounts.into();
    let ix = increase_position4_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_position4_invoke_signed(
    accounts: IncreasePosition4Accounts<'_, '_>,
    args: IncreasePosition4IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_position4_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_position4_verify_account_keys(
    accounts: IncreasePosition4Accounts<'_, '_>,
    keys: IncreasePosition4Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_pythnet_price_account.key,
            keys.collateral_custody_pythnet_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_position4_verify_writable_privileges<'me, 'info>(
    accounts: IncreasePosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_position4_verify_signer_privileges<'me, 'info>(
    accounts: IncreasePosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_position4_verify_account_privileges<'me, 'info>(
    accounts: IncreasePosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_position4_verify_writable_privileges(accounts)?;
    increase_position4_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct IncreasePositionPreSwapAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub keeper_ata: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub instruction: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreasePositionPreSwapKeys {
    pub keeper: Pubkey,
    pub keeper_ata: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub instruction: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<IncreasePositionPreSwapAccounts<'_, '_>> for IncreasePositionPreSwapKeys {
    fn from(accounts: IncreasePositionPreSwapAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            keeper_ata: *accounts.keeper_ata.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            instruction: *accounts.instruction.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<IncreasePositionPreSwapKeys>
for [AccountMeta; INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreasePositionPreSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.keeper_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN]>
for IncreasePositionPreSwapKeys {
    fn from(pubkeys: [Pubkey; INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            keeper_ata: pubkeys[1],
            position_request: pubkeys[2],
            position_request_ata: pubkeys[3],
            position: pubkeys[4],
            collateral_custody: pubkeys[5],
            collateral_custody_token_account: pubkeys[6],
            instruction: pubkeys[7],
            token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<IncreasePositionPreSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreasePositionPreSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.keeper_ata.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.instruction.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN]>
for IncreasePositionPreSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            keeper_ata: &arr[1],
            position_request: &arr[2],
            position_request_ata: &arr[3],
            position: &arr[4],
            collateral_custody: &arr[5],
            collateral_custody_token_account: &arr[6],
            instruction: &arr[7],
            token_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const INCREASE_POSITION_PRE_SWAP_IX_DISCM: [u8; 8usize] = [
    26, 136, 225, 217, 22, 21, 83, 20,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionPreSwapIxArgs {
    pub params: IncreasePositionPreSwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePositionPreSwapIxData(pub IncreasePositionPreSwapIxArgs);
impl From<IncreasePositionPreSwapIxArgs> for IncreasePositionPreSwapIxData {
    fn from(args: IncreasePositionPreSwapIxArgs) -> Self {
        Self(args)
    }
}
impl IncreasePositionPreSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION_PRE_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <IncreasePositionPreSwapParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(IncreasePositionPreSwapIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION_PRE_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_position_pre_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreasePositionPreSwapKeys,
    args: IncreasePositionPreSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_POSITION_PRE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreasePositionPreSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_position_pre_swap_ix(
    keys: IncreasePositionPreSwapKeys,
    args: IncreasePositionPreSwapIxArgs,
) -> std::io::Result<Instruction> {
    increase_position_pre_swap_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn increase_position_pre_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionPreSwapAccounts<'_, '_>,
    args: IncreasePositionPreSwapIxArgs,
) -> ProgramResult {
    let keys: IncreasePositionPreSwapKeys = accounts.into();
    let ix = increase_position_pre_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_position_pre_swap_invoke(
    accounts: IncreasePositionPreSwapAccounts<'_, '_>,
    args: IncreasePositionPreSwapIxArgs,
) -> ProgramResult {
    increase_position_pre_swap_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn increase_position_pre_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionPreSwapAccounts<'_, '_>,
    args: IncreasePositionPreSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreasePositionPreSwapKeys = accounts.into();
    let ix = increase_position_pre_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_position_pre_swap_invoke_signed(
    accounts: IncreasePositionPreSwapAccounts<'_, '_>,
    args: IncreasePositionPreSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_position_pre_swap_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_position_pre_swap_verify_account_keys(
    accounts: IncreasePositionPreSwapAccounts<'_, '_>,
    keys: IncreasePositionPreSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.keeper_ata.key, keys.keeper_ata),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.instruction.key, keys.instruction),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_position_pre_swap_verify_writable_privileges<'me, 'info>(
    accounts: IncreasePositionPreSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.keeper_ata,
        accounts.position_request,
        accounts.position_request_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_position_pre_swap_verify_signer_privileges<'me, 'info>(
    accounts: IncreasePositionPreSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_position_pre_swap_verify_account_privileges<'me, 'info>(
    accounts: IncreasePositionPreSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_position_pre_swap_verify_writable_privileges(accounts)?;
    increase_position_pre_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct IncreasePositionWithInternalSwapAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub receiving_custody: &'me AccountInfo<'info>,
    pub receiving_custody_doves_price_account: &'me AccountInfo<'info>,
    pub receiving_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub receiving_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreasePositionWithInternalSwapKeys {
    pub keeper: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_pythnet_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub receiving_custody: Pubkey,
    pub receiving_custody_doves_price_account: Pubkey,
    pub receiving_custody_pythnet_price_account: Pubkey,
    pub receiving_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<IncreasePositionWithInternalSwapAccounts<'_, '_>>
for IncreasePositionWithInternalSwapKeys {
    fn from(accounts: IncreasePositionWithInternalSwapAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_pythnet_price_account: *accounts
                .collateral_custody_pythnet_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            receiving_custody: *accounts.receiving_custody.key,
            receiving_custody_doves_price_account: *accounts
                .receiving_custody_doves_price_account
                .key,
            receiving_custody_pythnet_price_account: *accounts
                .receiving_custody_pythnet_price_account
                .key,
            receiving_custody_token_account: *accounts
                .receiving_custody_token_account
                .key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<IncreasePositionWithInternalSwapKeys>
for [AccountMeta; INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreasePositionWithInternalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN]>
for IncreasePositionWithInternalSwapKeys {
    fn from(
        pubkeys: [Pubkey; INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            position_request: pubkeys[3],
            position_request_ata: pubkeys[4],
            position: pubkeys[5],
            custody: pubkeys[6],
            custody_doves_price_account: pubkeys[7],
            custody_pythnet_price_account: pubkeys[8],
            collateral_custody: pubkeys[9],
            collateral_custody_doves_price_account: pubkeys[10],
            collateral_custody_pythnet_price_account: pubkeys[11],
            collateral_custody_token_account: pubkeys[12],
            receiving_custody: pubkeys[13],
            receiving_custody_doves_price_account: pubkeys[14],
            receiving_custody_pythnet_price_account: pubkeys[15],
            receiving_custody_token_account: pubkeys[16],
            token_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<IncreasePositionWithInternalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreasePositionWithInternalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_pythnet_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.receiving_custody.clone(),
            accounts.receiving_custody_doves_price_account.clone(),
            accounts.receiving_custody_pythnet_price_account.clone(),
            accounts.receiving_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN]>
for IncreasePositionWithInternalSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            position_request: &arr[3],
            position_request_ata: &arr[4],
            position: &arr[5],
            custody: &arr[6],
            custody_doves_price_account: &arr[7],
            custody_pythnet_price_account: &arr[8],
            collateral_custody: &arr[9],
            collateral_custody_doves_price_account: &arr[10],
            collateral_custody_pythnet_price_account: &arr[11],
            collateral_custody_token_account: &arr[12],
            receiving_custody: &arr[13],
            receiving_custody_doves_price_account: &arr[14],
            receiving_custody_pythnet_price_account: &arr[15],
            receiving_custody_token_account: &arr[16],
            token_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM: [u8; 8usize] = [
    114, 55, 106, 140, 199, 221, 32, 112,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionWithInternalSwapIxArgs {
    pub params: IncreasePositionWithInternalSwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePositionWithInternalSwapIxData(
    pub IncreasePositionWithInternalSwapIxArgs,
);
impl From<IncreasePositionWithInternalSwapIxArgs>
for IncreasePositionWithInternalSwapIxData {
    fn from(args: IncreasePositionWithInternalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl IncreasePositionWithInternalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <IncreasePositionWithInternalSwapParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(IncreasePositionWithInternalSwapIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_position_with_internal_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreasePositionWithInternalSwapKeys,
    args: IncreasePositionWithInternalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: IncreasePositionWithInternalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_position_with_internal_swap_ix(
    keys: IncreasePositionWithInternalSwapKeys,
    args: IncreasePositionWithInternalSwapIxArgs,
) -> std::io::Result<Instruction> {
    increase_position_with_internal_swap_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn increase_position_with_internal_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionWithInternalSwapAccounts<'_, '_>,
    args: IncreasePositionWithInternalSwapIxArgs,
) -> ProgramResult {
    let keys: IncreasePositionWithInternalSwapKeys = accounts.into();
    let ix = increase_position_with_internal_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_position_with_internal_swap_invoke(
    accounts: IncreasePositionWithInternalSwapAccounts<'_, '_>,
    args: IncreasePositionWithInternalSwapIxArgs,
) -> ProgramResult {
    increase_position_with_internal_swap_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn increase_position_with_internal_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionWithInternalSwapAccounts<'_, '_>,
    args: IncreasePositionWithInternalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreasePositionWithInternalSwapKeys = accounts.into();
    let ix = increase_position_with_internal_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_position_with_internal_swap_invoke_signed(
    accounts: IncreasePositionWithInternalSwapAccounts<'_, '_>,
    args: IncreasePositionWithInternalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_position_with_internal_swap_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_position_with_internal_swap_verify_account_keys(
    accounts: IncreasePositionWithInternalSwapAccounts<'_, '_>,
    keys: IncreasePositionWithInternalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_pythnet_price_account.key,
            keys.collateral_custody_pythnet_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.receiving_custody.key, keys.receiving_custody),
        (
            *accounts.receiving_custody_doves_price_account.key,
            keys.receiving_custody_doves_price_account,
        ),
        (
            *accounts.receiving_custody_pythnet_price_account.key,
            keys.receiving_custody_pythnet_price_account,
        ),
        (
            *accounts.receiving_custody_token_account.key,
            keys.receiving_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_position_with_internal_swap_verify_writable_privileges<'me, 'info>(
    accounts: IncreasePositionWithInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
        accounts.receiving_custody,
        accounts.receiving_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_position_with_internal_swap_verify_signer_privileges<'me, 'info>(
    accounts: IncreasePositionWithInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_position_with_internal_swap_verify_account_privileges<'me, 'info>(
    accounts: IncreasePositionWithInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_position_with_internal_swap_verify_writable_privileges(accounts)?;
    increase_position_with_internal_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_POSITION4_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct DecreasePosition4Accounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreasePosition4Keys {
    pub keeper: Pubkey,
    pub owner: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_pythnet_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DecreasePosition4Accounts<'_, '_>> for DecreasePosition4Keys {
    fn from(accounts: DecreasePosition4Accounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            owner: *accounts.owner.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_pythnet_price_account: *accounts
                .collateral_custody_pythnet_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DecreasePosition4Keys> for [AccountMeta; DECREASE_POSITION4_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreasePosition4Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_POSITION4_IX_ACCOUNTS_LEN]> for DecreasePosition4Keys {
    fn from(pubkeys: [Pubkey; DECREASE_POSITION4_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            owner: pubkeys[1],
            transfer_authority: pubkeys[2],
            perpetuals: pubkeys[3],
            pool: pubkeys[4],
            position_request: pubkeys[5],
            position_request_ata: pubkeys[6],
            position: pubkeys[7],
            custody: pubkeys[8],
            custody_doves_price_account: pubkeys[9],
            custody_pythnet_price_account: pubkeys[10],
            collateral_custody: pubkeys[11],
            collateral_custody_doves_price_account: pubkeys[12],
            collateral_custody_pythnet_price_account: pubkeys[13],
            collateral_custody_token_account: pubkeys[14],
            token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<DecreasePosition4Accounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_POSITION4_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreasePosition4Accounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.owner.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_pythnet_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_POSITION4_IX_ACCOUNTS_LEN]>
for DecreasePosition4Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECREASE_POSITION4_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: &arr[0],
            owner: &arr[1],
            transfer_authority: &arr[2],
            perpetuals: &arr[3],
            pool: &arr[4],
            position_request: &arr[5],
            position_request_ata: &arr[6],
            position: &arr[7],
            custody: &arr[8],
            custody_doves_price_account: &arr[9],
            custody_pythnet_price_account: &arr[10],
            collateral_custody: &arr[11],
            collateral_custody_doves_price_account: &arr[12],
            collateral_custody_pythnet_price_account: &arr[13],
            collateral_custody_token_account: &arr[14],
            token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const DECREASE_POSITION4_IX_DISCM: [u8; 8usize] = [
    185, 161, 114, 175, 96, 148, 3, 170,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePosition4IxArgs {
    pub params: DecreasePosition4Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePosition4IxData(pub DecreasePosition4IxArgs);
impl From<DecreasePosition4IxArgs> for DecreasePosition4IxData {
    fn from(args: DecreasePosition4IxArgs) -> Self {
        Self(args)
    }
}
impl DecreasePosition4IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION4_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <DecreasePosition4Params>::deserialize(&mut reader)?
        };
        Ok(Self(DecreasePosition4IxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION4_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_position4_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreasePosition4Keys,
    args: DecreasePosition4IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_POSITION4_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreasePosition4IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_position4_ix(
    keys: DecreasePosition4Keys,
    args: DecreasePosition4IxArgs,
) -> std::io::Result<Instruction> {
    decrease_position4_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn decrease_position4_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePosition4Accounts<'_, '_>,
    args: DecreasePosition4IxArgs,
) -> ProgramResult {
    let keys: DecreasePosition4Keys = accounts.into();
    let ix = decrease_position4_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_position4_invoke(
    accounts: DecreasePosition4Accounts<'_, '_>,
    args: DecreasePosition4IxArgs,
) -> ProgramResult {
    decrease_position4_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn decrease_position4_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePosition4Accounts<'_, '_>,
    args: DecreasePosition4IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreasePosition4Keys = accounts.into();
    let ix = decrease_position4_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_position4_invoke_signed(
    accounts: DecreasePosition4Accounts<'_, '_>,
    args: DecreasePosition4IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_position4_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_position4_verify_account_keys(
    accounts: DecreasePosition4Accounts<'_, '_>,
    keys: DecreasePosition4Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_pythnet_price_account.key,
            keys.collateral_custody_pythnet_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_position4_verify_writable_privileges<'me, 'info>(
    accounts: DecreasePosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_position4_verify_signer_privileges<'me, 'info>(
    accounts: DecreasePosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_position4_verify_account_privileges<'me, 'info>(
    accounts: DecreasePosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_position4_verify_writable_privileges(accounts)?;
    decrease_position4_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct DecreasePositionWithInternalSwapAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub dispensing_custody: &'me AccountInfo<'info>,
    pub dispensing_custody_doves_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreasePositionWithInternalSwapKeys {
    pub keeper: Pubkey,
    pub owner: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_pythnet_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub dispensing_custody: Pubkey,
    pub dispensing_custody_doves_price_account: Pubkey,
    pub dispensing_custody_pythnet_price_account: Pubkey,
    pub dispensing_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DecreasePositionWithInternalSwapAccounts<'_, '_>>
for DecreasePositionWithInternalSwapKeys {
    fn from(accounts: DecreasePositionWithInternalSwapAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            owner: *accounts.owner.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_pythnet_price_account: *accounts
                .collateral_custody_pythnet_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            dispensing_custody: *accounts.dispensing_custody.key,
            dispensing_custody_doves_price_account: *accounts
                .dispensing_custody_doves_price_account
                .key,
            dispensing_custody_pythnet_price_account: *accounts
                .dispensing_custody_pythnet_price_account
                .key,
            dispensing_custody_token_account: *accounts
                .dispensing_custody_token_account
                .key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DecreasePositionWithInternalSwapKeys>
for [AccountMeta; DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreasePositionWithInternalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN]>
for DecreasePositionWithInternalSwapKeys {
    fn from(
        pubkeys: [Pubkey; DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: pubkeys[0],
            owner: pubkeys[1],
            transfer_authority: pubkeys[2],
            perpetuals: pubkeys[3],
            pool: pubkeys[4],
            position_request: pubkeys[5],
            position_request_ata: pubkeys[6],
            position: pubkeys[7],
            custody: pubkeys[8],
            custody_doves_price_account: pubkeys[9],
            custody_pythnet_price_account: pubkeys[10],
            collateral_custody: pubkeys[11],
            collateral_custody_doves_price_account: pubkeys[12],
            collateral_custody_pythnet_price_account: pubkeys[13],
            collateral_custody_token_account: pubkeys[14],
            dispensing_custody: pubkeys[15],
            dispensing_custody_doves_price_account: pubkeys[16],
            dispensing_custody_pythnet_price_account: pubkeys[17],
            dispensing_custody_token_account: pubkeys[18],
            token_program: pubkeys[19],
            event_authority: pubkeys[20],
            program: pubkeys[21],
        }
    }
}
impl<'info> From<DecreasePositionWithInternalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreasePositionWithInternalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.owner.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_pythnet_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.dispensing_custody.clone(),
            accounts.dispensing_custody_doves_price_account.clone(),
            accounts.dispensing_custody_pythnet_price_account.clone(),
            accounts.dispensing_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN]>
for DecreasePositionWithInternalSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            owner: &arr[1],
            transfer_authority: &arr[2],
            perpetuals: &arr[3],
            pool: &arr[4],
            position_request: &arr[5],
            position_request_ata: &arr[6],
            position: &arr[7],
            custody: &arr[8],
            custody_doves_price_account: &arr[9],
            custody_pythnet_price_account: &arr[10],
            collateral_custody: &arr[11],
            collateral_custody_doves_price_account: &arr[12],
            collateral_custody_pythnet_price_account: &arr[13],
            collateral_custody_token_account: &arr[14],
            dispensing_custody: &arr[15],
            dispensing_custody_doves_price_account: &arr[16],
            dispensing_custody_pythnet_price_account: &arr[17],
            dispensing_custody_token_account: &arr[18],
            token_program: &arr[19],
            event_authority: &arr[20],
            program: &arr[21],
        }
    }
}
pub const DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM: [u8; 8usize] = [
    131, 17, 153, 110, 119, 100, 97, 38,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionWithInternalSwapIxArgs {
    pub params: DecreasePositionWithInternalSwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePositionWithInternalSwapIxData(
    pub DecreasePositionWithInternalSwapIxArgs,
);
impl From<DecreasePositionWithInternalSwapIxArgs>
for DecreasePositionWithInternalSwapIxData {
    fn from(args: DecreasePositionWithInternalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl DecreasePositionWithInternalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <DecreasePositionWithInternalSwapParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(DecreasePositionWithInternalSwapIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_position_with_internal_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreasePositionWithInternalSwapKeys,
    args: DecreasePositionWithInternalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_POSITION_WITH_INTERNAL_SWAP_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DecreasePositionWithInternalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_position_with_internal_swap_ix(
    keys: DecreasePositionWithInternalSwapKeys,
    args: DecreasePositionWithInternalSwapIxArgs,
) -> std::io::Result<Instruction> {
    decrease_position_with_internal_swap_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn decrease_position_with_internal_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionWithInternalSwapAccounts<'_, '_>,
    args: DecreasePositionWithInternalSwapIxArgs,
) -> ProgramResult {
    let keys: DecreasePositionWithInternalSwapKeys = accounts.into();
    let ix = decrease_position_with_internal_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_position_with_internal_swap_invoke(
    accounts: DecreasePositionWithInternalSwapAccounts<'_, '_>,
    args: DecreasePositionWithInternalSwapIxArgs,
) -> ProgramResult {
    decrease_position_with_internal_swap_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn decrease_position_with_internal_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionWithInternalSwapAccounts<'_, '_>,
    args: DecreasePositionWithInternalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreasePositionWithInternalSwapKeys = accounts.into();
    let ix = decrease_position_with_internal_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_position_with_internal_swap_invoke_signed(
    accounts: DecreasePositionWithInternalSwapAccounts<'_, '_>,
    args: DecreasePositionWithInternalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_position_with_internal_swap_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_position_with_internal_swap_verify_account_keys(
    accounts: DecreasePositionWithInternalSwapAccounts<'_, '_>,
    keys: DecreasePositionWithInternalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_pythnet_price_account.key,
            keys.collateral_custody_pythnet_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.dispensing_custody.key, keys.dispensing_custody),
        (
            *accounts.dispensing_custody_doves_price_account.key,
            keys.dispensing_custody_doves_price_account,
        ),
        (
            *accounts.dispensing_custody_pythnet_price_account.key,
            keys.dispensing_custody_pythnet_price_account,
        ),
        (
            *accounts.dispensing_custody_token_account.key,
            keys.dispensing_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_position_with_internal_swap_verify_writable_privileges<'me, 'info>(
    accounts: DecreasePositionWithInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
        accounts.dispensing_custody,
        accounts.dispensing_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_position_with_internal_swap_verify_signer_privileges<'me, 'info>(
    accounts: DecreasePositionWithInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_position_with_internal_swap_verify_account_privileges<'me, 'info>(
    accounts: DecreasePositionWithInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_position_with_internal_swap_verify_writable_privileges(accounts)?;
    decrease_position_with_internal_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_POSITION_WITH_TPSL_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct DecreasePositionWithTpslAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreasePositionWithTpslKeys {
    pub keeper: Pubkey,
    pub owner: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DecreasePositionWithTpslAccounts<'_, '_>> for DecreasePositionWithTpslKeys {
    fn from(accounts: DecreasePositionWithTpslAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            owner: *accounts.owner.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DecreasePositionWithTpslKeys>
for [AccountMeta; DECREASE_POSITION_WITH_TPSL_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreasePositionWithTpslKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_POSITION_WITH_TPSL_IX_ACCOUNTS_LEN]>
for DecreasePositionWithTpslKeys {
    fn from(pubkeys: [Pubkey; DECREASE_POSITION_WITH_TPSL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            owner: pubkeys[1],
            transfer_authority: pubkeys[2],
            perpetuals: pubkeys[3],
            pool: pubkeys[4],
            position_request: pubkeys[5],
            position_request_ata: pubkeys[6],
            position: pubkeys[7],
            custody: pubkeys[8],
            custody_doves_price_account: pubkeys[9],
            collateral_custody: pubkeys[10],
            collateral_custody_doves_price_account: pubkeys[11],
            collateral_custody_token_account: pubkeys[12],
            token_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<DecreasePositionWithTpslAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_POSITION_WITH_TPSL_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreasePositionWithTpslAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.owner.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DECREASE_POSITION_WITH_TPSL_IX_ACCOUNTS_LEN]>
for DecreasePositionWithTpslAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DECREASE_POSITION_WITH_TPSL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            owner: &arr[1],
            transfer_authority: &arr[2],
            perpetuals: &arr[3],
            pool: &arr[4],
            position_request: &arr[5],
            position_request_ata: &arr[6],
            position: &arr[7],
            custody: &arr[8],
            custody_doves_price_account: &arr[9],
            collateral_custody: &arr[10],
            collateral_custody_doves_price_account: &arr[11],
            collateral_custody_token_account: &arr[12],
            token_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const DECREASE_POSITION_WITH_TPSL_IX_DISCM: [u8; 8usize] = [
    108, 18, 203, 209, 227, 103, 65, 165,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionWithTpslIxArgs {
    pub params: DecreasePositionWithTpslParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePositionWithTpslIxData(pub DecreasePositionWithTpslIxArgs);
impl From<DecreasePositionWithTpslIxArgs> for DecreasePositionWithTpslIxData {
    fn from(args: DecreasePositionWithTpslIxArgs) -> Self {
        Self(args)
    }
}
impl DecreasePositionWithTpslIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION_WITH_TPSL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <DecreasePositionWithTpslParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(DecreasePositionWithTpslIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION_WITH_TPSL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_position_with_tpsl_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreasePositionWithTpslKeys,
    args: DecreasePositionWithTpslIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_POSITION_WITH_TPSL_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreasePositionWithTpslIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_position_with_tpsl_ix(
    keys: DecreasePositionWithTpslKeys,
    args: DecreasePositionWithTpslIxArgs,
) -> std::io::Result<Instruction> {
    decrease_position_with_tpsl_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn decrease_position_with_tpsl_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionWithTpslAccounts<'_, '_>,
    args: DecreasePositionWithTpslIxArgs,
) -> ProgramResult {
    let keys: DecreasePositionWithTpslKeys = accounts.into();
    let ix = decrease_position_with_tpsl_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_position_with_tpsl_invoke(
    accounts: DecreasePositionWithTpslAccounts<'_, '_>,
    args: DecreasePositionWithTpslIxArgs,
) -> ProgramResult {
    decrease_position_with_tpsl_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn decrease_position_with_tpsl_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionWithTpslAccounts<'_, '_>,
    args: DecreasePositionWithTpslIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreasePositionWithTpslKeys = accounts.into();
    let ix = decrease_position_with_tpsl_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_position_with_tpsl_invoke_signed(
    accounts: DecreasePositionWithTpslAccounts<'_, '_>,
    args: DecreasePositionWithTpslIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_position_with_tpsl_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_position_with_tpsl_verify_account_keys(
    accounts: DecreasePositionWithTpslAccounts<'_, '_>,
    keys: DecreasePositionWithTpslKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_position_with_tpsl_verify_writable_privileges<'me, 'info>(
    accounts: DecreasePositionWithTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_position_with_tpsl_verify_signer_privileges<'me, 'info>(
    accounts: DecreasePositionWithTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_position_with_tpsl_verify_account_privileges<'me, 'info>(
    accounts: DecreasePositionWithTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_position_with_tpsl_verify_writable_privileges(accounts)?;
    decrease_position_with_tpsl_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct DecreasePositionWithTpslAndInternalSwapAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub dispensing_custody: &'me AccountInfo<'info>,
    pub dispensing_custody_doves_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreasePositionWithTpslAndInternalSwapKeys {
    pub keeper: Pubkey,
    pub owner: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub dispensing_custody: Pubkey,
    pub dispensing_custody_doves_price_account: Pubkey,
    pub dispensing_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DecreasePositionWithTpslAndInternalSwapAccounts<'_, '_>>
for DecreasePositionWithTpslAndInternalSwapKeys {
    fn from(accounts: DecreasePositionWithTpslAndInternalSwapAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            owner: *accounts.owner.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            dispensing_custody: *accounts.dispensing_custody.key,
            dispensing_custody_doves_price_account: *accounts
                .dispensing_custody_doves_price_account
                .key,
            dispensing_custody_token_account: *accounts
                .dispensing_custody_token_account
                .key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DecreasePositionWithTpslAndInternalSwapKeys>
for [AccountMeta; DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreasePositionWithTpslAndInternalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_ACCOUNTS_LEN]>
for DecreasePositionWithTpslAndInternalSwapKeys {
    fn from(
        pubkeys: [Pubkey; DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: pubkeys[0],
            owner: pubkeys[1],
            transfer_authority: pubkeys[2],
            perpetuals: pubkeys[3],
            pool: pubkeys[4],
            position_request: pubkeys[5],
            position_request_ata: pubkeys[6],
            position: pubkeys[7],
            custody: pubkeys[8],
            custody_doves_price_account: pubkeys[9],
            collateral_custody: pubkeys[10],
            collateral_custody_doves_price_account: pubkeys[11],
            collateral_custody_token_account: pubkeys[12],
            dispensing_custody: pubkeys[13],
            dispensing_custody_doves_price_account: pubkeys[14],
            dispensing_custody_token_account: pubkeys[15],
            token_program: pubkeys[16],
            event_authority: pubkeys[17],
            program: pubkeys[18],
        }
    }
}
impl<'info> From<DecreasePositionWithTpslAndInternalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.keeper.clone(),
            accounts.owner.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.dispensing_custody.clone(),
            accounts.dispensing_custody_doves_price_account.clone(),
            accounts.dispensing_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_ACCOUNTS_LEN],
> for DecreasePositionWithTpslAndInternalSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            owner: &arr[1],
            transfer_authority: &arr[2],
            perpetuals: &arr[3],
            pool: &arr[4],
            position_request: &arr[5],
            position_request_ata: &arr[6],
            position: &arr[7],
            custody: &arr[8],
            custody_doves_price_account: &arr[9],
            collateral_custody: &arr[10],
            collateral_custody_doves_price_account: &arr[11],
            collateral_custody_token_account: &arr[12],
            dispensing_custody: &arr[13],
            dispensing_custody_doves_price_account: &arr[14],
            dispensing_custody_token_account: &arr[15],
            token_program: &arr[16],
            event_authority: &arr[17],
            program: &arr[18],
        }
    }
}
pub const DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_DISCM: [u8; 8usize] = [
    2, 111, 200, 231, 35, 65, 123, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionWithTpslAndInternalSwapIxArgs {
    pub params: DecreasePositionWithTpslAndInternalSwapParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePositionWithTpslAndInternalSwapIxData(
    pub DecreasePositionWithTpslAndInternalSwapIxArgs,
);
impl From<DecreasePositionWithTpslAndInternalSwapIxArgs>
for DecreasePositionWithTpslAndInternalSwapIxData {
    fn from(args: DecreasePositionWithTpslAndInternalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl DecreasePositionWithTpslAndInternalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <DecreasePositionWithTpslAndInternalSwapParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(DecreasePositionWithTpslAndInternalSwapIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_position_with_tpsl_and_internal_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreasePositionWithTpslAndInternalSwapKeys,
    args: DecreasePositionWithTpslAndInternalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_POSITION_WITH_TPSL_AND_INTERNAL_SWAP_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DecreasePositionWithTpslAndInternalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_position_with_tpsl_and_internal_swap_ix(
    keys: DecreasePositionWithTpslAndInternalSwapKeys,
    args: DecreasePositionWithTpslAndInternalSwapIxArgs,
) -> std::io::Result<Instruction> {
    decrease_position_with_tpsl_and_internal_swap_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn decrease_position_with_tpsl_and_internal_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'_, '_>,
    args: DecreasePositionWithTpslAndInternalSwapIxArgs,
) -> ProgramResult {
    let keys: DecreasePositionWithTpslAndInternalSwapKeys = accounts.into();
    let ix = decrease_position_with_tpsl_and_internal_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_position_with_tpsl_and_internal_swap_invoke(
    accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'_, '_>,
    args: DecreasePositionWithTpslAndInternalSwapIxArgs,
) -> ProgramResult {
    decrease_position_with_tpsl_and_internal_swap_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn decrease_position_with_tpsl_and_internal_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'_, '_>,
    args: DecreasePositionWithTpslAndInternalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreasePositionWithTpslAndInternalSwapKeys = accounts.into();
    let ix = decrease_position_with_tpsl_and_internal_swap_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_position_with_tpsl_and_internal_swap_invoke_signed(
    accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'_, '_>,
    args: DecreasePositionWithTpslAndInternalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_position_with_tpsl_and_internal_swap_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_position_with_tpsl_and_internal_swap_verify_account_keys(
    accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'_, '_>,
    keys: DecreasePositionWithTpslAndInternalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.dispensing_custody.key, keys.dispensing_custody),
        (
            *accounts.dispensing_custody_doves_price_account.key,
            keys.dispensing_custody_doves_price_account,
        ),
        (
            *accounts.dispensing_custody_token_account.key,
            keys.dispensing_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_position_with_tpsl_and_internal_swap_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
        accounts.dispensing_custody,
        accounts.dispensing_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_position_with_tpsl_and_internal_swap_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_position_with_tpsl_and_internal_swap_verify_account_privileges<
    'me,
    'info,
>(
    accounts: DecreasePositionWithTpslAndInternalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_position_with_tpsl_and_internal_swap_verify_writable_privileges(accounts)?;
    decrease_position_with_tpsl_and_internal_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUIDATE_FULL_POSITION4_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateFullPosition4Accounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateFullPosition4Keys {
    pub signer: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_pythnet_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<LiquidateFullPosition4Accounts<'_, '_>> for LiquidateFullPosition4Keys {
    fn from(accounts: LiquidateFullPosition4Accounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_pythnet_price_account: *accounts
                .collateral_custody_pythnet_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<LiquidateFullPosition4Keys>
for [AccountMeta; LIQUIDATE_FULL_POSITION4_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidateFullPosition4Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUIDATE_FULL_POSITION4_IX_ACCOUNTS_LEN]>
for LiquidateFullPosition4Keys {
    fn from(pubkeys: [Pubkey; LIQUIDATE_FULL_POSITION4_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            position: pubkeys[3],
            custody: pubkeys[4],
            custody_doves_price_account: pubkeys[5],
            custody_pythnet_price_account: pubkeys[6],
            collateral_custody: pubkeys[7],
            collateral_custody_doves_price_account: pubkeys[8],
            collateral_custody_pythnet_price_account: pubkeys[9],
            collateral_custody_token_account: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<LiquidateFullPosition4Accounts<'_, 'info>>
for [AccountInfo<'info>; LIQUIDATE_FULL_POSITION4_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidateFullPosition4Accounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_pythnet_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LIQUIDATE_FULL_POSITION4_IX_ACCOUNTS_LEN]>
for LiquidateFullPosition4Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUIDATE_FULL_POSITION4_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            position: &arr[3],
            custody: &arr[4],
            custody_doves_price_account: &arr[5],
            custody_pythnet_price_account: &arr[6],
            collateral_custody: &arr[7],
            collateral_custody_doves_price_account: &arr[8],
            collateral_custody_pythnet_price_account: &arr[9],
            collateral_custody_token_account: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const LIQUIDATE_FULL_POSITION4_IX_DISCM: [u8; 8usize] = [
    64, 176, 88, 51, 168, 188, 156, 175,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidateFullPosition4IxArgs {
    pub params: LiquidateFullPosition4Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateFullPosition4IxData(pub LiquidateFullPosition4IxArgs);
impl From<LiquidateFullPosition4IxArgs> for LiquidateFullPosition4IxData {
    fn from(args: LiquidateFullPosition4IxArgs) -> Self {
        Self(args)
    }
}
impl LiquidateFullPosition4IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_FULL_POSITION4_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidateFullPosition4Params>::deserialize(&mut reader)?
        };
        Ok(
            Self(LiquidateFullPosition4IxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_FULL_POSITION4_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_full_position4_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidateFullPosition4Keys,
    args: LiquidateFullPosition4IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUIDATE_FULL_POSITION4_IX_ACCOUNTS_LEN] = keys.into();
    let data: LiquidateFullPosition4IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_full_position4_ix(
    keys: LiquidateFullPosition4Keys,
    args: LiquidateFullPosition4IxArgs,
) -> std::io::Result<Instruction> {
    liquidate_full_position4_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn liquidate_full_position4_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateFullPosition4Accounts<'_, '_>,
    args: LiquidateFullPosition4IxArgs,
) -> ProgramResult {
    let keys: LiquidateFullPosition4Keys = accounts.into();
    let ix = liquidate_full_position4_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquidate_full_position4_invoke(
    accounts: LiquidateFullPosition4Accounts<'_, '_>,
    args: LiquidateFullPosition4IxArgs,
) -> ProgramResult {
    liquidate_full_position4_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn liquidate_full_position4_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateFullPosition4Accounts<'_, '_>,
    args: LiquidateFullPosition4IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidateFullPosition4Keys = accounts.into();
    let ix = liquidate_full_position4_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquidate_full_position4_invoke_signed(
    accounts: LiquidateFullPosition4Accounts<'_, '_>,
    args: LiquidateFullPosition4IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquidate_full_position4_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquidate_full_position4_verify_account_keys(
    accounts: LiquidateFullPosition4Accounts<'_, '_>,
    keys: LiquidateFullPosition4Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_pythnet_price_account.key,
            keys.collateral_custody_pythnet_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquidate_full_position4_verify_writable_privileges<'me, 'info>(
    accounts: LiquidateFullPosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquidate_full_position4_verify_signer_privileges<'me, 'info>(
    accounts: LiquidateFullPosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquidate_full_position4_verify_account_privileges<'me, 'info>(
    accounts: LiquidateFullPosition4Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquidate_full_position4_verify_writable_privileges(accounts)?;
    liquidate_full_position4_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_ASSETS_UNDER_MANAGEMENT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RefreshAssetsUnderManagementAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshAssetsUnderManagementKeys {
    pub keeper: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub lp_token_mint: Pubkey,
}
impl From<RefreshAssetsUnderManagementAccounts<'_, '_>>
for RefreshAssetsUnderManagementKeys {
    fn from(accounts: RefreshAssetsUnderManagementAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            lp_token_mint: *accounts.lp_token_mint.key,
        }
    }
}
impl From<RefreshAssetsUnderManagementKeys>
for [AccountMeta; REFRESH_ASSETS_UNDER_MANAGEMENT_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshAssetsUnderManagementKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REFRESH_ASSETS_UNDER_MANAGEMENT_IX_ACCOUNTS_LEN]>
for RefreshAssetsUnderManagementKeys {
    fn from(pubkeys: [Pubkey; REFRESH_ASSETS_UNDER_MANAGEMENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            lp_token_mint: pubkeys[3],
        }
    }
}
impl<'info> From<RefreshAssetsUnderManagementAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_ASSETS_UNDER_MANAGEMENT_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshAssetsUnderManagementAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.lp_token_mint.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REFRESH_ASSETS_UNDER_MANAGEMENT_IX_ACCOUNTS_LEN]>
for RefreshAssetsUnderManagementAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REFRESH_ASSETS_UNDER_MANAGEMENT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            lp_token_mint: &arr[3],
        }
    }
}
pub const REFRESH_ASSETS_UNDER_MANAGEMENT_IX_DISCM: [u8; 8usize] = [
    162, 0, 215, 55, 225, 15, 185, 0,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefreshAssetsUnderManagementIxArgs {
    pub params: RefreshAssetsUnderManagementParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshAssetsUnderManagementIxData(pub RefreshAssetsUnderManagementIxArgs);
impl From<RefreshAssetsUnderManagementIxArgs> for RefreshAssetsUnderManagementIxData {
    fn from(args: RefreshAssetsUnderManagementIxArgs) -> Self {
        Self(args)
    }
}
impl RefreshAssetsUnderManagementIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_ASSETS_UNDER_MANAGEMENT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <RefreshAssetsUnderManagementParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(RefreshAssetsUnderManagementIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_ASSETS_UNDER_MANAGEMENT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_assets_under_management_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshAssetsUnderManagementKeys,
    args: RefreshAssetsUnderManagementIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_ASSETS_UNDER_MANAGEMENT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: RefreshAssetsUnderManagementIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn refresh_assets_under_management_ix(
    keys: RefreshAssetsUnderManagementKeys,
    args: RefreshAssetsUnderManagementIxArgs,
) -> std::io::Result<Instruction> {
    refresh_assets_under_management_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn refresh_assets_under_management_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshAssetsUnderManagementAccounts<'_, '_>,
    args: RefreshAssetsUnderManagementIxArgs,
) -> ProgramResult {
    let keys: RefreshAssetsUnderManagementKeys = accounts.into();
    let ix = refresh_assets_under_management_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_assets_under_management_invoke(
    accounts: RefreshAssetsUnderManagementAccounts<'_, '_>,
    args: RefreshAssetsUnderManagementIxArgs,
) -> ProgramResult {
    refresh_assets_under_management_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn refresh_assets_under_management_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshAssetsUnderManagementAccounts<'_, '_>,
    args: RefreshAssetsUnderManagementIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshAssetsUnderManagementKeys = accounts.into();
    let ix = refresh_assets_under_management_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_assets_under_management_invoke_signed(
    accounts: RefreshAssetsUnderManagementAccounts<'_, '_>,
    args: RefreshAssetsUnderManagementIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_assets_under_management_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn refresh_assets_under_management_verify_account_keys(
    accounts: RefreshAssetsUnderManagementAccounts<'_, '_>,
    keys: RefreshAssetsUnderManagementKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_assets_under_management_verify_writable_privileges<'me, 'info>(
    accounts: RefreshAssetsUnderManagementAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_assets_under_management_verify_signer_privileges<'me, 'info>(
    accounts: RefreshAssetsUnderManagementAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn refresh_assets_under_management_verify_account_privileges<'me, 'info>(
    accounts: RefreshAssetsUnderManagementAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_assets_under_management_verify_writable_privileges(accounts)?;
    refresh_assets_under_management_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MAX_GLOBAL_SIZES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetMaxGlobalSizesAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMaxGlobalSizesKeys {
    pub keeper: Pubkey,
    pub custody: Pubkey,
    pub pool: Pubkey,
}
impl From<SetMaxGlobalSizesAccounts<'_, '_>> for SetMaxGlobalSizesKeys {
    fn from(accounts: SetMaxGlobalSizesAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            custody: *accounts.custody.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<SetMaxGlobalSizesKeys>
for [AccountMeta; SET_MAX_GLOBAL_SIZES_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMaxGlobalSizesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_MAX_GLOBAL_SIZES_IX_ACCOUNTS_LEN]> for SetMaxGlobalSizesKeys {
    fn from(pubkeys: [Pubkey; SET_MAX_GLOBAL_SIZES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            custody: pubkeys[1],
            pool: pubkeys[2],
        }
    }
}
impl<'info> From<SetMaxGlobalSizesAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MAX_GLOBAL_SIZES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMaxGlobalSizesAccounts<'_, 'info>) -> Self {
        [accounts.keeper.clone(), accounts.custody.clone(), accounts.pool.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MAX_GLOBAL_SIZES_IX_ACCOUNTS_LEN]>
for SetMaxGlobalSizesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_MAX_GLOBAL_SIZES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            custody: &arr[1],
            pool: &arr[2],
        }
    }
}
pub const SET_MAX_GLOBAL_SIZES_IX_DISCM: [u8; 8usize] = [
    89, 2, 210, 24, 167, 227, 13, 214,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetMaxGlobalSizesIxArgs {
    pub params: SetMaxGlobalSizesParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxGlobalSizesIxData(pub SetMaxGlobalSizesIxArgs);
impl From<SetMaxGlobalSizesIxArgs> for SetMaxGlobalSizesIxData {
    fn from(args: SetMaxGlobalSizesIxArgs) -> Self {
        Self(args)
    }
}
impl SetMaxGlobalSizesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MAX_GLOBAL_SIZES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = <SetMaxGlobalSizesParams>::deserialize(&mut reader)?;
        Ok(Self(SetMaxGlobalSizesIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MAX_GLOBAL_SIZES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_max_global_sizes_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMaxGlobalSizesKeys,
    args: SetMaxGlobalSizesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MAX_GLOBAL_SIZES_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMaxGlobalSizesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_max_global_sizes_ix(
    keys: SetMaxGlobalSizesKeys,
    args: SetMaxGlobalSizesIxArgs,
) -> std::io::Result<Instruction> {
    set_max_global_sizes_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn set_max_global_sizes_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMaxGlobalSizesAccounts<'_, '_>,
    args: SetMaxGlobalSizesIxArgs,
) -> ProgramResult {
    let keys: SetMaxGlobalSizesKeys = accounts.into();
    let ix = set_max_global_sizes_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_max_global_sizes_invoke(
    accounts: SetMaxGlobalSizesAccounts<'_, '_>,
    args: SetMaxGlobalSizesIxArgs,
) -> ProgramResult {
    set_max_global_sizes_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn set_max_global_sizes_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMaxGlobalSizesAccounts<'_, '_>,
    args: SetMaxGlobalSizesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMaxGlobalSizesKeys = accounts.into();
    let ix = set_max_global_sizes_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_max_global_sizes_invoke_signed(
    accounts: SetMaxGlobalSizesAccounts<'_, '_>,
    args: SetMaxGlobalSizesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_max_global_sizes_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_max_global_sizes_verify_account_keys(
    accounts: SetMaxGlobalSizesAccounts<'_, '_>,
    keys: SetMaxGlobalSizesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.custody.key, keys.custody),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_max_global_sizes_verify_writable_privileges<'me, 'info>(
    accounts: SetMaxGlobalSizesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.custody] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_max_global_sizes_verify_signer_privileges<'me, 'info>(
    accounts: SetMaxGlobalSizesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_max_global_sizes_verify_account_privileges<'me, 'info>(
    accounts: SetMaxGlobalSizesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_max_global_sizes_verify_writable_privileges(accounts)?;
    set_max_global_sizes_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_CREATE_TPSL_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct InstantCreateTpslAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub api_keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub desired_mint: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantCreateTpslKeys {
    pub keeper: Pubkey,
    pub api_keeper: Pubkey,
    pub owner: Pubkey,
    pub receiving_account: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub desired_mint: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InstantCreateTpslAccounts<'_, '_>> for InstantCreateTpslKeys {
    fn from(accounts: InstantCreateTpslAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            api_keeper: *accounts.api_keeper.key,
            owner: *accounts.owner.key,
            receiving_account: *accounts.receiving_account.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            desired_mint: *accounts.desired_mint.key,
            referral: *accounts.referral.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InstantCreateTpslKeys> for [AccountMeta; INSTANT_CREATE_TPSL_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantCreateTpslKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.api_keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.desired_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_CREATE_TPSL_IX_ACCOUNTS_LEN]> for InstantCreateTpslKeys {
    fn from(pubkeys: [Pubkey; INSTANT_CREATE_TPSL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            api_keeper: pubkeys[1],
            owner: pubkeys[2],
            receiving_account: pubkeys[3],
            perpetuals: pubkeys[4],
            pool: pubkeys[5],
            position: pubkeys[6],
            position_request: pubkeys[7],
            position_request_ata: pubkeys[8],
            custody: pubkeys[9],
            custody_doves_price_account: pubkeys[10],
            custody_pythnet_price_account: pubkeys[11],
            collateral_custody: pubkeys[12],
            desired_mint: pubkeys[13],
            referral: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            system_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<InstantCreateTpslAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_CREATE_TPSL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantCreateTpslAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.api_keeper.clone(),
            accounts.owner.clone(),
            accounts.receiving_account.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.desired_mint.clone(),
            accounts.referral.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INSTANT_CREATE_TPSL_IX_ACCOUNTS_LEN]>
for InstantCreateTpslAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_CREATE_TPSL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            api_keeper: &arr[1],
            owner: &arr[2],
            receiving_account: &arr[3],
            perpetuals: &arr[4],
            pool: &arr[5],
            position: &arr[6],
            position_request: &arr[7],
            position_request_ata: &arr[8],
            custody: &arr[9],
            custody_doves_price_account: &arr[10],
            custody_pythnet_price_account: &arr[11],
            collateral_custody: &arr[12],
            desired_mint: &arr[13],
            referral: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            system_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const INSTANT_CREATE_TPSL_IX_DISCM: [u8; 8usize] = [
    117, 98, 66, 127, 30, 50, 73, 185,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantCreateTpslIxArgs {
    pub params: InstantCreateTpslParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantCreateTpslIxData(pub InstantCreateTpslIxArgs);
impl From<InstantCreateTpslIxArgs> for InstantCreateTpslIxData {
    fn from(args: InstantCreateTpslIxArgs) -> Self {
        Self(args)
    }
}
impl InstantCreateTpslIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_CREATE_TPSL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InstantCreateTpslParams>::deserialize(&mut reader)?
        };
        Ok(Self(InstantCreateTpslIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_CREATE_TPSL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_create_tpsl_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantCreateTpslKeys,
    args: InstantCreateTpslIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_CREATE_TPSL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantCreateTpslIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_create_tpsl_ix(
    keys: InstantCreateTpslKeys,
    args: InstantCreateTpslIxArgs,
) -> std::io::Result<Instruction> {
    instant_create_tpsl_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn instant_create_tpsl_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantCreateTpslAccounts<'_, '_>,
    args: InstantCreateTpslIxArgs,
) -> ProgramResult {
    let keys: InstantCreateTpslKeys = accounts.into();
    let ix = instant_create_tpsl_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_create_tpsl_invoke(
    accounts: InstantCreateTpslAccounts<'_, '_>,
    args: InstantCreateTpslIxArgs,
) -> ProgramResult {
    instant_create_tpsl_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn instant_create_tpsl_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantCreateTpslAccounts<'_, '_>,
    args: InstantCreateTpslIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantCreateTpslKeys = accounts.into();
    let ix = instant_create_tpsl_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_create_tpsl_invoke_signed(
    accounts: InstantCreateTpslAccounts<'_, '_>,
    args: InstantCreateTpslIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_create_tpsl_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_create_tpsl_verify_account_keys(
    accounts: InstantCreateTpslAccounts<'_, '_>,
    keys: InstantCreateTpslKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.api_keeper.key, keys.api_keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (*accounts.desired_mint.key, keys.desired_mint),
        (*accounts.referral.key, keys.referral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_create_tpsl_verify_writable_privileges<'me, 'info>(
    accounts: InstantCreateTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.receiving_account,
        accounts.pool,
        accounts.position_request,
        accounts.position_request_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_create_tpsl_verify_signer_privileges<'me, 'info>(
    accounts: InstantCreateTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper, accounts.api_keeper, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_create_tpsl_verify_account_privileges<'me, 'info>(
    accounts: InstantCreateTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_create_tpsl_verify_writable_privileges(accounts)?;
    instant_create_tpsl_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_CREATE_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct InstantCreateLimitOrderAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub api_keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub input_mint: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantCreateLimitOrderKeys {
    pub keeper: Pubkey,
    pub api_keeper: Pubkey,
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub input_mint: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InstantCreateLimitOrderAccounts<'_, '_>> for InstantCreateLimitOrderKeys {
    fn from(accounts: InstantCreateLimitOrderAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            api_keeper: *accounts.api_keeper.key,
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            input_mint: *accounts.input_mint.key,
            referral: *accounts.referral.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InstantCreateLimitOrderKeys>
for [AccountMeta; INSTANT_CREATE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantCreateLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.api_keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_CREATE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for InstantCreateLimitOrderKeys {
    fn from(pubkeys: [Pubkey; INSTANT_CREATE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            api_keeper: pubkeys[1],
            owner: pubkeys[2],
            funding_account: pubkeys[3],
            perpetuals: pubkeys[4],
            pool: pubkeys[5],
            position: pubkeys[6],
            position_request: pubkeys[7],
            position_request_ata: pubkeys[8],
            custody: pubkeys[9],
            custody_doves_price_account: pubkeys[10],
            custody_pythnet_price_account: pubkeys[11],
            collateral_custody: pubkeys[12],
            input_mint: pubkeys[13],
            referral: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            system_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<InstantCreateLimitOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_CREATE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantCreateLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.api_keeper.clone(),
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.input_mint.clone(),
            accounts.referral.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INSTANT_CREATE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for InstantCreateLimitOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_CREATE_LIMIT_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            api_keeper: &arr[1],
            owner: &arr[2],
            funding_account: &arr[3],
            perpetuals: &arr[4],
            pool: &arr[5],
            position: &arr[6],
            position_request: &arr[7],
            position_request_ata: &arr[8],
            custody: &arr[9],
            custody_doves_price_account: &arr[10],
            custody_pythnet_price_account: &arr[11],
            collateral_custody: &arr[12],
            input_mint: &arr[13],
            referral: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            system_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const INSTANT_CREATE_LIMIT_ORDER_IX_DISCM: [u8; 8usize] = [
    194, 37, 195, 123, 40, 127, 126, 156,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantCreateLimitOrderIxArgs {
    pub params: InstantCreateLimitOrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantCreateLimitOrderIxData(pub InstantCreateLimitOrderIxArgs);
impl From<InstantCreateLimitOrderIxArgs> for InstantCreateLimitOrderIxData {
    fn from(args: InstantCreateLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl InstantCreateLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_CREATE_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InstantCreateLimitOrderParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InstantCreateLimitOrderIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_CREATE_LIMIT_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_create_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantCreateLimitOrderKeys,
    args: InstantCreateLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_CREATE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantCreateLimitOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_create_limit_order_ix(
    keys: InstantCreateLimitOrderKeys,
    args: InstantCreateLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    instant_create_limit_order_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn instant_create_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantCreateLimitOrderAccounts<'_, '_>,
    args: InstantCreateLimitOrderIxArgs,
) -> ProgramResult {
    let keys: InstantCreateLimitOrderKeys = accounts.into();
    let ix = instant_create_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_create_limit_order_invoke(
    accounts: InstantCreateLimitOrderAccounts<'_, '_>,
    args: InstantCreateLimitOrderIxArgs,
) -> ProgramResult {
    instant_create_limit_order_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn instant_create_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantCreateLimitOrderAccounts<'_, '_>,
    args: InstantCreateLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantCreateLimitOrderKeys = accounts.into();
    let ix = instant_create_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_create_limit_order_invoke_signed(
    accounts: InstantCreateLimitOrderAccounts<'_, '_>,
    args: InstantCreateLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_create_limit_order_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_create_limit_order_verify_account_keys(
    accounts: InstantCreateLimitOrderAccounts<'_, '_>,
    keys: InstantCreateLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.api_keeper.key, keys.api_keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (*accounts.input_mint.key, keys.input_mint),
        (*accounts.referral.key, keys.referral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_create_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: InstantCreateLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.funding_account,
        accounts.position,
        accounts.position_request,
        accounts.position_request_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_create_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: InstantCreateLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper, accounts.api_keeper, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_create_limit_order_verify_account_privileges<'me, 'info>(
    accounts: InstantCreateLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_create_limit_order_verify_writable_privileges(accounts)?;
    instant_create_limit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_INCREASE_POSITION_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct InstantIncreasePositionAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub api_keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub token_ledger: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantIncreasePositionKeys {
    pub keeper: Pubkey,
    pub api_keeper: Pubkey,
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_pythnet_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub token_ledger: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InstantIncreasePositionAccounts<'_, '_>> for InstantIncreasePositionKeys {
    fn from(accounts: InstantIncreasePositionAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            api_keeper: *accounts.api_keeper.key,
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_pythnet_price_account: *accounts
                .collateral_custody_pythnet_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            token_ledger: *accounts.token_ledger.key,
            referral: *accounts.referral.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InstantIncreasePositionKeys>
for [AccountMeta; INSTANT_INCREASE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantIncreasePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.api_keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_INCREASE_POSITION_IX_ACCOUNTS_LEN]>
for InstantIncreasePositionKeys {
    fn from(pubkeys: [Pubkey; INSTANT_INCREASE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            api_keeper: pubkeys[1],
            owner: pubkeys[2],
            funding_account: pubkeys[3],
            perpetuals: pubkeys[4],
            pool: pubkeys[5],
            position: pubkeys[6],
            custody: pubkeys[7],
            custody_doves_price_account: pubkeys[8],
            custody_pythnet_price_account: pubkeys[9],
            collateral_custody: pubkeys[10],
            collateral_custody_doves_price_account: pubkeys[11],
            collateral_custody_pythnet_price_account: pubkeys[12],
            collateral_custody_token_account: pubkeys[13],
            token_ledger: pubkeys[14],
            referral: pubkeys[15],
            token_program: pubkeys[16],
            system_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<InstantIncreasePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_INCREASE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantIncreasePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.api_keeper.clone(),
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_pythnet_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.token_ledger.clone(),
            accounts.referral.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INSTANT_INCREASE_POSITION_IX_ACCOUNTS_LEN]>
for InstantIncreasePositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_INCREASE_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            api_keeper: &arr[1],
            owner: &arr[2],
            funding_account: &arr[3],
            perpetuals: &arr[4],
            pool: &arr[5],
            position: &arr[6],
            custody: &arr[7],
            custody_doves_price_account: &arr[8],
            custody_pythnet_price_account: &arr[9],
            collateral_custody: &arr[10],
            collateral_custody_doves_price_account: &arr[11],
            collateral_custody_pythnet_price_account: &arr[12],
            collateral_custody_token_account: &arr[13],
            token_ledger: &arr[14],
            referral: &arr[15],
            token_program: &arr[16],
            system_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const INSTANT_INCREASE_POSITION_IX_DISCM: [u8; 8usize] = [
    164, 126, 68, 182, 223, 166, 64, 183,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantIncreasePositionIxArgs {
    pub params: InstantIncreasePositionParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantIncreasePositionIxData(pub InstantIncreasePositionIxArgs);
impl From<InstantIncreasePositionIxArgs> for InstantIncreasePositionIxData {
    fn from(args: InstantIncreasePositionIxArgs) -> Self {
        Self(args)
    }
}
impl InstantIncreasePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_INCREASE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InstantIncreasePositionParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InstantIncreasePositionIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_INCREASE_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_increase_position_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantIncreasePositionKeys,
    args: InstantIncreasePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_INCREASE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantIncreasePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_increase_position_ix(
    keys: InstantIncreasePositionKeys,
    args: InstantIncreasePositionIxArgs,
) -> std::io::Result<Instruction> {
    instant_increase_position_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn instant_increase_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantIncreasePositionAccounts<'_, '_>,
    args: InstantIncreasePositionIxArgs,
) -> ProgramResult {
    let keys: InstantIncreasePositionKeys = accounts.into();
    let ix = instant_increase_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_increase_position_invoke(
    accounts: InstantIncreasePositionAccounts<'_, '_>,
    args: InstantIncreasePositionIxArgs,
) -> ProgramResult {
    instant_increase_position_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn instant_increase_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantIncreasePositionAccounts<'_, '_>,
    args: InstantIncreasePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantIncreasePositionKeys = accounts.into();
    let ix = instant_increase_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_increase_position_invoke_signed(
    accounts: InstantIncreasePositionAccounts<'_, '_>,
    args: InstantIncreasePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_increase_position_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_increase_position_verify_account_keys(
    accounts: InstantIncreasePositionAccounts<'_, '_>,
    keys: InstantIncreasePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.api_keeper.key, keys.api_keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_pythnet_price_account.key,
            keys.collateral_custody_pythnet_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.referral.key, keys.referral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_increase_position_verify_writable_privileges<'me, 'info>(
    accounts: InstantIncreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.funding_account,
        accounts.pool,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_increase_position_verify_signer_privileges<'me, 'info>(
    accounts: InstantIncreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper, accounts.api_keeper, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_increase_position_verify_account_privileges<'me, 'info>(
    accounts: InstantIncreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_increase_position_verify_writable_privileges(accounts)?;
    instant_increase_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_DECREASE_POSITION_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct InstantDecreasePositionAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub api_keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub desired_mint: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantDecreasePositionKeys {
    pub keeper: Pubkey,
    pub api_keeper: Pubkey,
    pub owner: Pubkey,
    pub receiving_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_pythnet_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub desired_mint: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InstantDecreasePositionAccounts<'_, '_>> for InstantDecreasePositionKeys {
    fn from(accounts: InstantDecreasePositionAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            api_keeper: *accounts.api_keeper.key,
            owner: *accounts.owner.key,
            receiving_account: *accounts.receiving_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_pythnet_price_account: *accounts
                .collateral_custody_pythnet_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            desired_mint: *accounts.desired_mint.key,
            referral: *accounts.referral.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InstantDecreasePositionKeys>
for [AccountMeta; INSTANT_DECREASE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantDecreasePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.api_keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.desired_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_DECREASE_POSITION_IX_ACCOUNTS_LEN]>
for InstantDecreasePositionKeys {
    fn from(pubkeys: [Pubkey; INSTANT_DECREASE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            api_keeper: pubkeys[1],
            owner: pubkeys[2],
            receiving_account: pubkeys[3],
            transfer_authority: pubkeys[4],
            perpetuals: pubkeys[5],
            pool: pubkeys[6],
            position: pubkeys[7],
            custody: pubkeys[8],
            custody_doves_price_account: pubkeys[9],
            custody_pythnet_price_account: pubkeys[10],
            collateral_custody: pubkeys[11],
            collateral_custody_doves_price_account: pubkeys[12],
            collateral_custody_pythnet_price_account: pubkeys[13],
            collateral_custody_token_account: pubkeys[14],
            desired_mint: pubkeys[15],
            referral: pubkeys[16],
            token_program: pubkeys[17],
            associated_token_program: pubkeys[18],
            system_program: pubkeys[19],
            event_authority: pubkeys[20],
            program: pubkeys[21],
        }
    }
}
impl<'info> From<InstantDecreasePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_DECREASE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantDecreasePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.api_keeper.clone(),
            accounts.owner.clone(),
            accounts.receiving_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_pythnet_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.desired_mint.clone(),
            accounts.referral.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INSTANT_DECREASE_POSITION_IX_ACCOUNTS_LEN]>
for InstantDecreasePositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_DECREASE_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            api_keeper: &arr[1],
            owner: &arr[2],
            receiving_account: &arr[3],
            transfer_authority: &arr[4],
            perpetuals: &arr[5],
            pool: &arr[6],
            position: &arr[7],
            custody: &arr[8],
            custody_doves_price_account: &arr[9],
            custody_pythnet_price_account: &arr[10],
            collateral_custody: &arr[11],
            collateral_custody_doves_price_account: &arr[12],
            collateral_custody_pythnet_price_account: &arr[13],
            collateral_custody_token_account: &arr[14],
            desired_mint: &arr[15],
            referral: &arr[16],
            token_program: &arr[17],
            associated_token_program: &arr[18],
            system_program: &arr[19],
            event_authority: &arr[20],
            program: &arr[21],
        }
    }
}
pub const INSTANT_DECREASE_POSITION_IX_DISCM: [u8; 8usize] = [
    46, 23, 240, 44, 30, 138, 94, 140,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantDecreasePositionIxArgs {
    pub params: InstantDecreasePositionParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantDecreasePositionIxData(pub InstantDecreasePositionIxArgs);
impl From<InstantDecreasePositionIxArgs> for InstantDecreasePositionIxData {
    fn from(args: InstantDecreasePositionIxArgs) -> Self {
        Self(args)
    }
}
impl InstantDecreasePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_DECREASE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InstantDecreasePositionParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InstantDecreasePositionIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_DECREASE_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_decrease_position_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantDecreasePositionKeys,
    args: InstantDecreasePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_DECREASE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantDecreasePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_decrease_position_ix(
    keys: InstantDecreasePositionKeys,
    args: InstantDecreasePositionIxArgs,
) -> std::io::Result<Instruction> {
    instant_decrease_position_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn instant_decrease_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantDecreasePositionAccounts<'_, '_>,
    args: InstantDecreasePositionIxArgs,
) -> ProgramResult {
    let keys: InstantDecreasePositionKeys = accounts.into();
    let ix = instant_decrease_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_decrease_position_invoke(
    accounts: InstantDecreasePositionAccounts<'_, '_>,
    args: InstantDecreasePositionIxArgs,
) -> ProgramResult {
    instant_decrease_position_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn instant_decrease_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantDecreasePositionAccounts<'_, '_>,
    args: InstantDecreasePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantDecreasePositionKeys = accounts.into();
    let ix = instant_decrease_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_decrease_position_invoke_signed(
    accounts: InstantDecreasePositionAccounts<'_, '_>,
    args: InstantDecreasePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_decrease_position_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_decrease_position_verify_account_keys(
    accounts: InstantDecreasePositionAccounts<'_, '_>,
    keys: InstantDecreasePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.api_keeper.key, keys.api_keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_pythnet_price_account.key,
            keys.collateral_custody_pythnet_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.desired_mint.key, keys.desired_mint),
        (*accounts.referral.key, keys.referral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_decrease_position_verify_writable_privileges<'me, 'info>(
    accounts: InstantDecreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.receiving_account,
        accounts.pool,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_decrease_position_verify_signer_privileges<'me, 'info>(
    accounts: InstantDecreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper, accounts.api_keeper, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_decrease_position_verify_account_privileges<'me, 'info>(
    accounts: InstantDecreasePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_decrease_position_verify_writable_privileges(accounts)?;
    instant_decrease_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_DECREASE_POSITION2_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct InstantDecreasePosition2Accounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub api_keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody: &'me AccountInfo<'info>,
    pub collateral_custody_doves_price_account: &'me AccountInfo<'info>,
    pub collateral_custody_token_account: &'me AccountInfo<'info>,
    pub desired_mint: &'me AccountInfo<'info>,
    pub referral: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub position_request_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantDecreasePosition2Keys {
    pub keeper: Pubkey,
    pub api_keeper: Pubkey,
    pub owner: Pubkey,
    pub receiving_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub collateral_custody: Pubkey,
    pub collateral_custody_doves_price_account: Pubkey,
    pub collateral_custody_token_account: Pubkey,
    pub desired_mint: Pubkey,
    pub referral: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InstantDecreasePosition2Accounts<'_, '_>> for InstantDecreasePosition2Keys {
    fn from(accounts: InstantDecreasePosition2Accounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            api_keeper: *accounts.api_keeper.key,
            owner: *accounts.owner.key,
            receiving_account: *accounts.receiving_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            collateral_custody: *accounts.collateral_custody.key,
            collateral_custody_doves_price_account: *accounts
                .collateral_custody_doves_price_account
                .key,
            collateral_custody_token_account: *accounts
                .collateral_custody_token_account
                .key,
            desired_mint: *accounts.desired_mint.key,
            referral: *accounts.referral.key,
            position_request: *accounts.position_request.key,
            position_request_ata: *accounts.position_request_ata.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InstantDecreasePosition2Keys>
for [AccountMeta; INSTANT_DECREASE_POSITION2_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantDecreasePosition2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.api_keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.desired_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_request_ata,
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
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_DECREASE_POSITION2_IX_ACCOUNTS_LEN]>
for InstantDecreasePosition2Keys {
    fn from(pubkeys: [Pubkey; INSTANT_DECREASE_POSITION2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            api_keeper: pubkeys[1],
            owner: pubkeys[2],
            receiving_account: pubkeys[3],
            transfer_authority: pubkeys[4],
            perpetuals: pubkeys[5],
            pool: pubkeys[6],
            position: pubkeys[7],
            custody: pubkeys[8],
            custody_doves_price_account: pubkeys[9],
            collateral_custody: pubkeys[10],
            collateral_custody_doves_price_account: pubkeys[11],
            collateral_custody_token_account: pubkeys[12],
            desired_mint: pubkeys[13],
            referral: pubkeys[14],
            position_request: pubkeys[15],
            position_request_ata: pubkeys[16],
            token_program: pubkeys[17],
            associated_token_program: pubkeys[18],
            system_program: pubkeys[19],
            event_authority: pubkeys[20],
            program: pubkeys[21],
        }
    }
}
impl<'info> From<InstantDecreasePosition2Accounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_DECREASE_POSITION2_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantDecreasePosition2Accounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.api_keeper.clone(),
            accounts.owner.clone(),
            accounts.receiving_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.collateral_custody.clone(),
            accounts.collateral_custody_doves_price_account.clone(),
            accounts.collateral_custody_token_account.clone(),
            accounts.desired_mint.clone(),
            accounts.referral.clone(),
            accounts.position_request.clone(),
            accounts.position_request_ata.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INSTANT_DECREASE_POSITION2_IX_ACCOUNTS_LEN]>
for InstantDecreasePosition2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_DECREASE_POSITION2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            api_keeper: &arr[1],
            owner: &arr[2],
            receiving_account: &arr[3],
            transfer_authority: &arr[4],
            perpetuals: &arr[5],
            pool: &arr[6],
            position: &arr[7],
            custody: &arr[8],
            custody_doves_price_account: &arr[9],
            collateral_custody: &arr[10],
            collateral_custody_doves_price_account: &arr[11],
            collateral_custody_token_account: &arr[12],
            desired_mint: &arr[13],
            referral: &arr[14],
            position_request: &arr[15],
            position_request_ata: &arr[16],
            token_program: &arr[17],
            associated_token_program: &arr[18],
            system_program: &arr[19],
            event_authority: &arr[20],
            program: &arr[21],
        }
    }
}
pub const INSTANT_DECREASE_POSITION2_IX_DISCM: [u8; 8usize] = [
    162, 191, 200, 62, 139, 62, 176, 17,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantDecreasePosition2IxArgs {
    pub params: InstantDecreasePosition2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantDecreasePosition2IxData(pub InstantDecreasePosition2IxArgs);
impl From<InstantDecreasePosition2IxArgs> for InstantDecreasePosition2IxData {
    fn from(args: InstantDecreasePosition2IxArgs) -> Self {
        Self(args)
    }
}
impl InstantDecreasePosition2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_DECREASE_POSITION2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InstantDecreasePosition2Params>::deserialize(&mut reader)?
        };
        Ok(
            Self(InstantDecreasePosition2IxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_DECREASE_POSITION2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_decrease_position2_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantDecreasePosition2Keys,
    args: InstantDecreasePosition2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_DECREASE_POSITION2_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantDecreasePosition2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_decrease_position2_ix(
    keys: InstantDecreasePosition2Keys,
    args: InstantDecreasePosition2IxArgs,
) -> std::io::Result<Instruction> {
    instant_decrease_position2_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn instant_decrease_position2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantDecreasePosition2Accounts<'_, '_>,
    args: InstantDecreasePosition2IxArgs,
) -> ProgramResult {
    let keys: InstantDecreasePosition2Keys = accounts.into();
    let ix = instant_decrease_position2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_decrease_position2_invoke(
    accounts: InstantDecreasePosition2Accounts<'_, '_>,
    args: InstantDecreasePosition2IxArgs,
) -> ProgramResult {
    instant_decrease_position2_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn instant_decrease_position2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantDecreasePosition2Accounts<'_, '_>,
    args: InstantDecreasePosition2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantDecreasePosition2Keys = accounts.into();
    let ix = instant_decrease_position2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_decrease_position2_invoke_signed(
    accounts: InstantDecreasePosition2Accounts<'_, '_>,
    args: InstantDecreasePosition2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_decrease_position2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_decrease_position2_verify_account_keys(
    accounts: InstantDecreasePosition2Accounts<'_, '_>,
    keys: InstantDecreasePosition2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.api_keeper.key, keys.api_keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (*accounts.collateral_custody.key, keys.collateral_custody),
        (
            *accounts.collateral_custody_doves_price_account.key,
            keys.collateral_custody_doves_price_account,
        ),
        (
            *accounts.collateral_custody_token_account.key,
            keys.collateral_custody_token_account,
        ),
        (*accounts.desired_mint.key, keys.desired_mint),
        (*accounts.referral.key, keys.referral),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.position_request_ata.key, keys.position_request_ata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_decrease_position2_verify_writable_privileges<'me, 'info>(
    accounts: InstantDecreasePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.receiving_account,
        accounts.pool,
        accounts.position,
        accounts.custody,
        accounts.collateral_custody,
        accounts.collateral_custody_token_account,
        accounts.position_request,
        accounts.position_request_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_decrease_position2_verify_signer_privileges<'me, 'info>(
    accounts: InstantDecreasePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper, accounts.api_keeper, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_decrease_position2_verify_account_privileges<'me, 'info>(
    accounts: InstantDecreasePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_decrease_position2_verify_writable_privileges(accounts)?;
    instant_decrease_position2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_UPDATE_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InstantUpdateLimitOrderAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub api_keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantUpdateLimitOrderKeys {
    pub keeper: Pubkey,
    pub api_keeper: Pubkey,
    pub owner: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
}
impl From<InstantUpdateLimitOrderAccounts<'_, '_>> for InstantUpdateLimitOrderKeys {
    fn from(accounts: InstantUpdateLimitOrderAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            api_keeper: *accounts.api_keeper.key,
            owner: *accounts.owner.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_request: *accounts.position_request.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
        }
    }
}
impl From<InstantUpdateLimitOrderKeys>
for [AccountMeta; INSTANT_UPDATE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantUpdateLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.api_keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_UPDATE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for InstantUpdateLimitOrderKeys {
    fn from(pubkeys: [Pubkey; INSTANT_UPDATE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            api_keeper: pubkeys[1],
            owner: pubkeys[2],
            perpetuals: pubkeys[3],
            pool: pubkeys[4],
            position: pubkeys[5],
            position_request: pubkeys[6],
            custody: pubkeys[7],
            custody_doves_price_account: pubkeys[8],
            custody_pythnet_price_account: pubkeys[9],
        }
    }
}
impl<'info> From<InstantUpdateLimitOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_UPDATE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantUpdateLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.api_keeper.clone(),
            accounts.owner.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_request.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INSTANT_UPDATE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for InstantUpdateLimitOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_UPDATE_LIMIT_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            api_keeper: &arr[1],
            owner: &arr[2],
            perpetuals: &arr[3],
            pool: &arr[4],
            position: &arr[5],
            position_request: &arr[6],
            custody: &arr[7],
            custody_doves_price_account: &arr[8],
            custody_pythnet_price_account: &arr[9],
        }
    }
}
pub const INSTANT_UPDATE_LIMIT_ORDER_IX_DISCM: [u8; 8usize] = [
    136, 245, 229, 58, 121, 141, 12, 207,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantUpdateLimitOrderIxArgs {
    pub params: InstantUpdateLimitOrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantUpdateLimitOrderIxData(pub InstantUpdateLimitOrderIxArgs);
impl From<InstantUpdateLimitOrderIxArgs> for InstantUpdateLimitOrderIxData {
    fn from(args: InstantUpdateLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl InstantUpdateLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_UPDATE_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InstantUpdateLimitOrderParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InstantUpdateLimitOrderIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_UPDATE_LIMIT_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_update_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantUpdateLimitOrderKeys,
    args: InstantUpdateLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_UPDATE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantUpdateLimitOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_update_limit_order_ix(
    keys: InstantUpdateLimitOrderKeys,
    args: InstantUpdateLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    instant_update_limit_order_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn instant_update_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantUpdateLimitOrderAccounts<'_, '_>,
    args: InstantUpdateLimitOrderIxArgs,
) -> ProgramResult {
    let keys: InstantUpdateLimitOrderKeys = accounts.into();
    let ix = instant_update_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_update_limit_order_invoke(
    accounts: InstantUpdateLimitOrderAccounts<'_, '_>,
    args: InstantUpdateLimitOrderIxArgs,
) -> ProgramResult {
    instant_update_limit_order_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn instant_update_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantUpdateLimitOrderAccounts<'_, '_>,
    args: InstantUpdateLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantUpdateLimitOrderKeys = accounts.into();
    let ix = instant_update_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_update_limit_order_invoke_signed(
    accounts: InstantUpdateLimitOrderAccounts<'_, '_>,
    args: InstantUpdateLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_update_limit_order_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_update_limit_order_verify_account_keys(
    accounts: InstantUpdateLimitOrderAccounts<'_, '_>,
    keys: InstantUpdateLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.api_keeper.key, keys.api_keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (*accounts.custody_pythnet_price_account.key, keys.custody_pythnet_price_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_update_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: InstantUpdateLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position_request] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_update_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: InstantUpdateLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper, accounts.api_keeper, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_update_limit_order_verify_account_privileges<'me, 'info>(
    accounts: InstantUpdateLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_update_limit_order_verify_writable_privileges(accounts)?;
    instant_update_limit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_UPDATE_TPSL_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InstantUpdateTpslAccounts<'me, 'info> {
    pub keeper: &'me AccountInfo<'info>,
    pub api_keeper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_request: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantUpdateTpslKeys {
    pub keeper: Pubkey,
    pub api_keeper: Pubkey,
    pub owner: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InstantUpdateTpslAccounts<'_, '_>> for InstantUpdateTpslKeys {
    fn from(accounts: InstantUpdateTpslAccounts) -> Self {
        Self {
            keeper: *accounts.keeper.key,
            api_keeper: *accounts.api_keeper.key,
            owner: *accounts.owner.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_request: *accounts.position_request.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InstantUpdateTpslKeys> for [AccountMeta; INSTANT_UPDATE_TPSL_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantUpdateTpslKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.api_keeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_UPDATE_TPSL_IX_ACCOUNTS_LEN]> for InstantUpdateTpslKeys {
    fn from(pubkeys: [Pubkey; INSTANT_UPDATE_TPSL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            keeper: pubkeys[0],
            api_keeper: pubkeys[1],
            owner: pubkeys[2],
            perpetuals: pubkeys[3],
            pool: pubkeys[4],
            position: pubkeys[5],
            position_request: pubkeys[6],
            custody: pubkeys[7],
            custody_doves_price_account: pubkeys[8],
            custody_pythnet_price_account: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<InstantUpdateTpslAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_UPDATE_TPSL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantUpdateTpslAccounts<'_, 'info>) -> Self {
        [
            accounts.keeper.clone(),
            accounts.api_keeper.clone(),
            accounts.owner.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_request.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INSTANT_UPDATE_TPSL_IX_ACCOUNTS_LEN]>
for InstantUpdateTpslAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSTANT_UPDATE_TPSL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            keeper: &arr[0],
            api_keeper: &arr[1],
            owner: &arr[2],
            perpetuals: &arr[3],
            pool: &arr[4],
            position: &arr[5],
            position_request: &arr[6],
            custody: &arr[7],
            custody_doves_price_account: &arr[8],
            custody_pythnet_price_account: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const INSTANT_UPDATE_TPSL_IX_DISCM: [u8; 8usize] = [
    144, 228, 114, 37, 165, 242, 111, 101,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantUpdateTpslIxArgs {
    pub params: InstantUpdateTpslParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantUpdateTpslIxData(pub InstantUpdateTpslIxArgs);
impl From<InstantUpdateTpslIxArgs> for InstantUpdateTpslIxData {
    fn from(args: InstantUpdateTpslIxArgs) -> Self {
        Self(args)
    }
}
impl InstantUpdateTpslIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_UPDATE_TPSL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InstantUpdateTpslParams>::deserialize(&mut reader)?
        };
        Ok(Self(InstantUpdateTpslIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_UPDATE_TPSL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_update_tpsl_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantUpdateTpslKeys,
    args: InstantUpdateTpslIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_UPDATE_TPSL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantUpdateTpslIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_update_tpsl_ix(
    keys: InstantUpdateTpslKeys,
    args: InstantUpdateTpslIxArgs,
) -> std::io::Result<Instruction> {
    instant_update_tpsl_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn instant_update_tpsl_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantUpdateTpslAccounts<'_, '_>,
    args: InstantUpdateTpslIxArgs,
) -> ProgramResult {
    let keys: InstantUpdateTpslKeys = accounts.into();
    let ix = instant_update_tpsl_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_update_tpsl_invoke(
    accounts: InstantUpdateTpslAccounts<'_, '_>,
    args: InstantUpdateTpslIxArgs,
) -> ProgramResult {
    instant_update_tpsl_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn instant_update_tpsl_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantUpdateTpslAccounts<'_, '_>,
    args: InstantUpdateTpslIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantUpdateTpslKeys = accounts.into();
    let ix = instant_update_tpsl_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_update_tpsl_invoke_signed(
    accounts: InstantUpdateTpslAccounts<'_, '_>,
    args: InstantUpdateTpslIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_update_tpsl_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_update_tpsl_verify_account_keys(
    accounts: InstantUpdateTpslAccounts<'_, '_>,
    keys: InstantUpdateTpslKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.keeper.key, keys.keeper),
        (*accounts.api_keeper.key, keys.api_keeper),
        (*accounts.owner.key, keys.owner),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_request.key, keys.position_request),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_update_tpsl_verify_writable_privileges<'me, 'info>(
    accounts: InstantUpdateTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position_request] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_update_tpsl_verify_signer_privileges<'me, 'info>(
    accounts: InstantUpdateTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.keeper, accounts.api_keeper, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_update_tpsl_verify_account_privileges<'me, 'info>(
    accounts: InstantUpdateTpslAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_update_tpsl_verify_writable_privileges(accounts)?;
    instant_update_tpsl_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GetAddLiquidityAmountAndFee2Accounts<'me, 'info> {
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetAddLiquidityAmountAndFee2Keys {
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub lp_token_mint: Pubkey,
}
impl From<GetAddLiquidityAmountAndFee2Accounts<'_, '_>>
for GetAddLiquidityAmountAndFee2Keys {
    fn from(accounts: GetAddLiquidityAmountAndFee2Accounts) -> Self {
        Self {
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
        }
    }
}
impl From<GetAddLiquidityAmountAndFee2Keys>
for [AccountMeta; GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN] {
    fn from(keys: GetAddLiquidityAmountAndFee2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN]>
for GetAddLiquidityAmountAndFee2Keys {
    fn from(
        pubkeys: [Pubkey; GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            perpetuals: pubkeys[0],
            pool: pubkeys[1],
            custody: pubkeys[2],
            custody_doves_price_account: pubkeys[3],
            custody_pythnet_price_account: pubkeys[4],
            lp_token_mint: pubkeys[5],
        }
    }
}
impl<'info> From<GetAddLiquidityAmountAndFee2Accounts<'_, 'info>>
for [AccountInfo<'info>; GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetAddLiquidityAmountAndFee2Accounts<'_, 'info>) -> Self {
        [
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.lp_token_mint.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN]>
for GetAddLiquidityAmountAndFee2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            perpetuals: &arr[0],
            pool: &arr[1],
            custody: &arr[2],
            custody_doves_price_account: &arr[3],
            custody_pythnet_price_account: &arr[4],
            lp_token_mint: &arr[5],
        }
    }
}
pub const GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM: [u8; 8usize] = [
    109, 157, 55, 169, 8, 81, 4, 118,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetAddLiquidityAmountAndFee2IxArgs {
    pub params: GetAddLiquidityAmountAndFee2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetAddLiquidityAmountAndFee2IxData(pub GetAddLiquidityAmountAndFee2IxArgs);
impl From<GetAddLiquidityAmountAndFee2IxArgs> for GetAddLiquidityAmountAndFee2IxData {
    fn from(args: GetAddLiquidityAmountAndFee2IxArgs) -> Self {
        Self(args)
    }
}
impl GetAddLiquidityAmountAndFee2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <GetAddLiquidityAmountAndFee2Params>::deserialize(&mut reader)?
        };
        Ok(
            Self(GetAddLiquidityAmountAndFee2IxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_add_liquidity_amount_and_fee2_ix_with_program_id(
    program_id: Pubkey,
    keys: GetAddLiquidityAmountAndFee2Keys,
    args: GetAddLiquidityAmountAndFee2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_ADD_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: GetAddLiquidityAmountAndFee2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn get_add_liquidity_amount_and_fee2_ix(
    keys: GetAddLiquidityAmountAndFee2Keys,
    args: GetAddLiquidityAmountAndFee2IxArgs,
) -> std::io::Result<Instruction> {
    get_add_liquidity_amount_and_fee2_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn get_add_liquidity_amount_and_fee2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetAddLiquidityAmountAndFee2Accounts<'_, '_>,
    args: GetAddLiquidityAmountAndFee2IxArgs,
) -> ProgramResult {
    let keys: GetAddLiquidityAmountAndFee2Keys = accounts.into();
    let ix = get_add_liquidity_amount_and_fee2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn get_add_liquidity_amount_and_fee2_invoke(
    accounts: GetAddLiquidityAmountAndFee2Accounts<'_, '_>,
    args: GetAddLiquidityAmountAndFee2IxArgs,
) -> ProgramResult {
    get_add_liquidity_amount_and_fee2_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn get_add_liquidity_amount_and_fee2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetAddLiquidityAmountAndFee2Accounts<'_, '_>,
    args: GetAddLiquidityAmountAndFee2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetAddLiquidityAmountAndFee2Keys = accounts.into();
    let ix = get_add_liquidity_amount_and_fee2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_add_liquidity_amount_and_fee2_invoke_signed(
    accounts: GetAddLiquidityAmountAndFee2Accounts<'_, '_>,
    args: GetAddLiquidityAmountAndFee2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_add_liquidity_amount_and_fee2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn get_add_liquidity_amount_and_fee2_verify_account_keys(
    accounts: GetAddLiquidityAmountAndFee2Accounts<'_, '_>,
    keys: GetAddLiquidityAmountAndFee2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GetRemoveLiquidityAmountAndFee2Accounts<'me, 'info> {
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetRemoveLiquidityAmountAndFee2Keys {
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub lp_token_mint: Pubkey,
}
impl From<GetRemoveLiquidityAmountAndFee2Accounts<'_, '_>>
for GetRemoveLiquidityAmountAndFee2Keys {
    fn from(accounts: GetRemoveLiquidityAmountAndFee2Accounts) -> Self {
        Self {
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
        }
    }
}
impl From<GetRemoveLiquidityAmountAndFee2Keys>
for [AccountMeta; GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN] {
    fn from(keys: GetRemoveLiquidityAmountAndFee2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN]>
for GetRemoveLiquidityAmountAndFee2Keys {
    fn from(
        pubkeys: [Pubkey; GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            perpetuals: pubkeys[0],
            pool: pubkeys[1],
            custody: pubkeys[2],
            custody_doves_price_account: pubkeys[3],
            custody_pythnet_price_account: pubkeys[4],
            lp_token_mint: pubkeys[5],
        }
    }
}
impl<'info> From<GetRemoveLiquidityAmountAndFee2Accounts<'_, 'info>>
for [AccountInfo<'info>; GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetRemoveLiquidityAmountAndFee2Accounts<'_, 'info>) -> Self {
        [
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.lp_token_mint.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN]>
for GetRemoveLiquidityAmountAndFee2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            perpetuals: &arr[0],
            pool: &arr[1],
            custody: &arr[2],
            custody_doves_price_account: &arr[3],
            custody_pythnet_price_account: &arr[4],
            lp_token_mint: &arr[5],
        }
    }
}
pub const GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM: [u8; 8usize] = [
    183, 59, 72, 110, 223, 243, 150, 142,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetRemoveLiquidityAmountAndFee2IxArgs {
    pub params: GetRemoveLiquidityAmountAndFee2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetRemoveLiquidityAmountAndFee2IxData(
    pub GetRemoveLiquidityAmountAndFee2IxArgs,
);
impl From<GetRemoveLiquidityAmountAndFee2IxArgs>
for GetRemoveLiquidityAmountAndFee2IxData {
    fn from(args: GetRemoveLiquidityAmountAndFee2IxArgs) -> Self {
        Self(args)
    }
}
impl GetRemoveLiquidityAmountAndFee2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <GetRemoveLiquidityAmountAndFee2Params>::deserialize(&mut reader)?
        };
        Ok(
            Self(GetRemoveLiquidityAmountAndFee2IxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_remove_liquidity_amount_and_fee2_ix_with_program_id(
    program_id: Pubkey,
    keys: GetRemoveLiquidityAmountAndFee2Keys,
    args: GetRemoveLiquidityAmountAndFee2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_REMOVE_LIQUIDITY_AMOUNT_AND_FEE2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: GetRemoveLiquidityAmountAndFee2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn get_remove_liquidity_amount_and_fee2_ix(
    keys: GetRemoveLiquidityAmountAndFee2Keys,
    args: GetRemoveLiquidityAmountAndFee2IxArgs,
) -> std::io::Result<Instruction> {
    get_remove_liquidity_amount_and_fee2_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn get_remove_liquidity_amount_and_fee2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetRemoveLiquidityAmountAndFee2Accounts<'_, '_>,
    args: GetRemoveLiquidityAmountAndFee2IxArgs,
) -> ProgramResult {
    let keys: GetRemoveLiquidityAmountAndFee2Keys = accounts.into();
    let ix = get_remove_liquidity_amount_and_fee2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn get_remove_liquidity_amount_and_fee2_invoke(
    accounts: GetRemoveLiquidityAmountAndFee2Accounts<'_, '_>,
    args: GetRemoveLiquidityAmountAndFee2IxArgs,
) -> ProgramResult {
    get_remove_liquidity_amount_and_fee2_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn get_remove_liquidity_amount_and_fee2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetRemoveLiquidityAmountAndFee2Accounts<'_, '_>,
    args: GetRemoveLiquidityAmountAndFee2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetRemoveLiquidityAmountAndFee2Keys = accounts.into();
    let ix = get_remove_liquidity_amount_and_fee2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_remove_liquidity_amount_and_fee2_invoke_signed(
    accounts: GetRemoveLiquidityAmountAndFee2Accounts<'_, '_>,
    args: GetRemoveLiquidityAmountAndFee2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_remove_liquidity_amount_and_fee2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn get_remove_liquidity_amount_and_fee2_verify_account_keys(
    accounts: GetRemoveLiquidityAmountAndFee2Accounts<'_, '_>,
    keys: GetRemoveLiquidityAmountAndFee2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.custody_doves_price_account.key, keys.custody_doves_price_account),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const GET_ASSETS_UNDER_MANAGEMENT2_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct GetAssetsUnderManagement2Accounts<'me, 'info> {
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetAssetsUnderManagement2Keys {
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
}
impl From<GetAssetsUnderManagement2Accounts<'_, '_>> for GetAssetsUnderManagement2Keys {
    fn from(accounts: GetAssetsUnderManagement2Accounts) -> Self {
        Self {
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
        }
    }
}
impl From<GetAssetsUnderManagement2Keys>
for [AccountMeta; GET_ASSETS_UNDER_MANAGEMENT2_IX_ACCOUNTS_LEN] {
    fn from(keys: GetAssetsUnderManagement2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_ASSETS_UNDER_MANAGEMENT2_IX_ACCOUNTS_LEN]>
for GetAssetsUnderManagement2Keys {
    fn from(pubkeys: [Pubkey; GET_ASSETS_UNDER_MANAGEMENT2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            perpetuals: pubkeys[0],
            pool: pubkeys[1],
        }
    }
}
impl<'info> From<GetAssetsUnderManagement2Accounts<'_, 'info>>
for [AccountInfo<'info>; GET_ASSETS_UNDER_MANAGEMENT2_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetAssetsUnderManagement2Accounts<'_, 'info>) -> Self {
        [accounts.perpetuals.clone(), accounts.pool.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GET_ASSETS_UNDER_MANAGEMENT2_IX_ACCOUNTS_LEN]>
for GetAssetsUnderManagement2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GET_ASSETS_UNDER_MANAGEMENT2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            perpetuals: &arr[0],
            pool: &arr[1],
        }
    }
}
pub const GET_ASSETS_UNDER_MANAGEMENT2_IX_DISCM: [u8; 8usize] = [
    193, 210, 13, 249, 113, 149, 29, 84,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetAssetsUnderManagement2IxArgs {
    pub params: GetAssetsUnderManagement2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GetAssetsUnderManagement2IxData(pub GetAssetsUnderManagement2IxArgs);
impl From<GetAssetsUnderManagement2IxArgs> for GetAssetsUnderManagement2IxData {
    fn from(args: GetAssetsUnderManagement2IxArgs) -> Self {
        Self(args)
    }
}
impl GetAssetsUnderManagement2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_ASSETS_UNDER_MANAGEMENT2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <GetAssetsUnderManagement2Params>::deserialize(&mut reader)?
        };
        Ok(
            Self(GetAssetsUnderManagement2IxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_ASSETS_UNDER_MANAGEMENT2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_assets_under_management2_ix_with_program_id(
    program_id: Pubkey,
    keys: GetAssetsUnderManagement2Keys,
    args: GetAssetsUnderManagement2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_ASSETS_UNDER_MANAGEMENT2_IX_ACCOUNTS_LEN] = keys.into();
    let data: GetAssetsUnderManagement2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn get_assets_under_management2_ix(
    keys: GetAssetsUnderManagement2Keys,
    args: GetAssetsUnderManagement2IxArgs,
) -> std::io::Result<Instruction> {
    get_assets_under_management2_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn get_assets_under_management2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetAssetsUnderManagement2Accounts<'_, '_>,
    args: GetAssetsUnderManagement2IxArgs,
) -> ProgramResult {
    let keys: GetAssetsUnderManagement2Keys = accounts.into();
    let ix = get_assets_under_management2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_assets_under_management2_invoke(
    accounts: GetAssetsUnderManagement2Accounts<'_, '_>,
    args: GetAssetsUnderManagement2IxArgs,
) -> ProgramResult {
    get_assets_under_management2_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn get_assets_under_management2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetAssetsUnderManagement2Accounts<'_, '_>,
    args: GetAssetsUnderManagement2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetAssetsUnderManagement2Keys = accounts.into();
    let ix = get_assets_under_management2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_assets_under_management2_invoke_signed(
    accounts: GetAssetsUnderManagement2Accounts<'_, '_>,
    args: GetAssetsUnderManagement2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_assets_under_management2_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn get_assets_under_management2_verify_account_keys(
    accounts: GetAssetsUnderManagement2Accounts<'_, '_>,
    keys: GetAssetsUnderManagement2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const BORROW_FROM_CUSTODY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct BorrowFromCustodyAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub borrow_position: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowFromCustodyKeys {
    pub owner: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub transfer_authority: Pubkey,
    pub borrow_position: Pubkey,
    pub custody_token_account: Pubkey,
    pub user_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BorrowFromCustodyAccounts<'_, '_>> for BorrowFromCustodyKeys {
    fn from(accounts: BorrowFromCustodyAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            transfer_authority: *accounts.transfer_authority.key,
            borrow_position: *accounts.borrow_position.key,
            custody_token_account: *accounts.custody_token_account.key,
            user_token_account: *accounts.user_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BorrowFromCustodyKeys> for [AccountMeta; BORROW_FROM_CUSTODY_IX_ACCOUNTS_LEN] {
    fn from(keys: BorrowFromCustodyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BORROW_FROM_CUSTODY_IX_ACCOUNTS_LEN]> for BorrowFromCustodyKeys {
    fn from(pubkeys: [Pubkey; BORROW_FROM_CUSTODY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            transfer_authority: pubkeys[4],
            borrow_position: pubkeys[5],
            custody_token_account: pubkeys[6],
            user_token_account: pubkeys[7],
            lp_token_mint: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<BorrowFromCustodyAccounts<'_, 'info>>
for [AccountInfo<'info>; BORROW_FROM_CUSTODY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BorrowFromCustodyAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.transfer_authority.clone(),
            accounts.borrow_position.clone(),
            accounts.custody_token_account.clone(),
            accounts.user_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BORROW_FROM_CUSTODY_IX_ACCOUNTS_LEN]>
for BorrowFromCustodyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BORROW_FROM_CUSTODY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            transfer_authority: &arr[4],
            borrow_position: &arr[5],
            custody_token_account: &arr[6],
            user_token_account: &arr[7],
            lp_token_mint: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const BORROW_FROM_CUSTODY_IX_DISCM: [u8; 8usize] = [
    153, 183, 65, 65, 113, 33, 249, 45,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowFromCustodyIxArgs {
    pub params: BorrowFromCustodyParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowFromCustodyIxData(pub BorrowFromCustodyIxArgs);
impl From<BorrowFromCustodyIxArgs> for BorrowFromCustodyIxData {
    fn from(args: BorrowFromCustodyIxArgs) -> Self {
        Self(args)
    }
}
impl BorrowFromCustodyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_FROM_CUSTODY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowFromCustodyParams>::deserialize(&mut reader)?
        };
        Ok(Self(BorrowFromCustodyIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_FROM_CUSTODY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn borrow_from_custody_ix_with_program_id(
    program_id: Pubkey,
    keys: BorrowFromCustodyKeys,
    args: BorrowFromCustodyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BORROW_FROM_CUSTODY_IX_ACCOUNTS_LEN] = keys.into();
    let data: BorrowFromCustodyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn borrow_from_custody_ix(
    keys: BorrowFromCustodyKeys,
    args: BorrowFromCustodyIxArgs,
) -> std::io::Result<Instruction> {
    borrow_from_custody_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn borrow_from_custody_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BorrowFromCustodyAccounts<'_, '_>,
    args: BorrowFromCustodyIxArgs,
) -> ProgramResult {
    let keys: BorrowFromCustodyKeys = accounts.into();
    let ix = borrow_from_custody_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn borrow_from_custody_invoke(
    accounts: BorrowFromCustodyAccounts<'_, '_>,
    args: BorrowFromCustodyIxArgs,
) -> ProgramResult {
    borrow_from_custody_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn borrow_from_custody_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BorrowFromCustodyAccounts<'_, '_>,
    args: BorrowFromCustodyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BorrowFromCustodyKeys = accounts.into();
    let ix = borrow_from_custody_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn borrow_from_custody_invoke_signed(
    accounts: BorrowFromCustodyAccounts<'_, '_>,
    args: BorrowFromCustodyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    borrow_from_custody_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn borrow_from_custody_verify_account_keys(
    accounts: BorrowFromCustodyAccounts<'_, '_>,
    keys: BorrowFromCustodyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.borrow_position.key, keys.borrow_position),
        (*accounts.custody_token_account.key, keys.custody_token_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn borrow_from_custody_verify_writable_privileges<'me, 'info>(
    accounts: BorrowFromCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.custody,
        accounts.borrow_position,
        accounts.custody_token_account,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn borrow_from_custody_verify_signer_privileges<'me, 'info>(
    accounts: BorrowFromCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn borrow_from_custody_verify_account_privileges<'me, 'info>(
    accounts: BorrowFromCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    borrow_from_custody_verify_writable_privileges(accounts)?;
    borrow_from_custody_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REPAY_TO_CUSTODY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RepayToCustodyAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub borrow_position: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RepayToCustodyKeys {
    pub owner: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub borrow_position: Pubkey,
    pub custody_token_account: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RepayToCustodyAccounts<'_, '_>> for RepayToCustodyKeys {
    fn from(accounts: RepayToCustodyAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            borrow_position: *accounts.borrow_position.key,
            custody_token_account: *accounts.custody_token_account.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RepayToCustodyKeys> for [AccountMeta; REPAY_TO_CUSTODY_IX_ACCOUNTS_LEN] {
    fn from(keys: RepayToCustodyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REPAY_TO_CUSTODY_IX_ACCOUNTS_LEN]> for RepayToCustodyKeys {
    fn from(pubkeys: [Pubkey; REPAY_TO_CUSTODY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            borrow_position: pubkeys[4],
            custody_token_account: pubkeys[5],
            user_token_account: pubkeys[6],
            token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<RepayToCustodyAccounts<'_, 'info>>
for [AccountInfo<'info>; REPAY_TO_CUSTODY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RepayToCustodyAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.borrow_position.clone(),
            accounts.custody_token_account.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REPAY_TO_CUSTODY_IX_ACCOUNTS_LEN]>
for RepayToCustodyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REPAY_TO_CUSTODY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            borrow_position: &arr[4],
            custody_token_account: &arr[5],
            user_token_account: &arr[6],
            token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const REPAY_TO_CUSTODY_IX_DISCM: [u8; 8usize] = [211, 219, 183, 222, 248, 74, 5, 26];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RepayToCustodyIxArgs {
    pub params: RepayToCustodyParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RepayToCustodyIxData(pub RepayToCustodyIxArgs);
impl From<RepayToCustodyIxArgs> for RepayToCustodyIxData {
    fn from(args: RepayToCustodyIxArgs) -> Self {
        Self(args)
    }
}
impl RepayToCustodyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REPAY_TO_CUSTODY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <RepayToCustodyParams>::deserialize(&mut reader)?
        };
        Ok(Self(RepayToCustodyIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REPAY_TO_CUSTODY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn repay_to_custody_ix_with_program_id(
    program_id: Pubkey,
    keys: RepayToCustodyKeys,
    args: RepayToCustodyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REPAY_TO_CUSTODY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RepayToCustodyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn repay_to_custody_ix(
    keys: RepayToCustodyKeys,
    args: RepayToCustodyIxArgs,
) -> std::io::Result<Instruction> {
    repay_to_custody_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn repay_to_custody_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RepayToCustodyAccounts<'_, '_>,
    args: RepayToCustodyIxArgs,
) -> ProgramResult {
    let keys: RepayToCustodyKeys = accounts.into();
    let ix = repay_to_custody_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn repay_to_custody_invoke(
    accounts: RepayToCustodyAccounts<'_, '_>,
    args: RepayToCustodyIxArgs,
) -> ProgramResult {
    repay_to_custody_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn repay_to_custody_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RepayToCustodyAccounts<'_, '_>,
    args: RepayToCustodyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RepayToCustodyKeys = accounts.into();
    let ix = repay_to_custody_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn repay_to_custody_invoke_signed(
    accounts: RepayToCustodyAccounts<'_, '_>,
    args: RepayToCustodyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    repay_to_custody_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn repay_to_custody_verify_account_keys(
    accounts: RepayToCustodyAccounts<'_, '_>,
    keys: RepayToCustodyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.borrow_position.key, keys.borrow_position),
        (*accounts.custody_token_account.key, keys.custody_token_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn repay_to_custody_verify_writable_privileges<'me, 'info>(
    accounts: RepayToCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.custody,
        accounts.borrow_position,
        accounts.custody_token_account,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn repay_to_custody_verify_signer_privileges<'me, 'info>(
    accounts: RepayToCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn repay_to_custody_verify_account_privileges<'me, 'info>(
    accounts: RepayToCustodyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    repay_to_custody_verify_writable_privileges(accounts)?;
    repay_to_custody_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct DepositCollateralForBorrowsAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub borrow_position: &'me AccountInfo<'info>,
    pub collateral_token_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositCollateralForBorrowsKeys {
    pub owner: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub transfer_authority: Pubkey,
    pub borrow_position: Pubkey,
    pub collateral_token_account: Pubkey,
    pub user_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DepositCollateralForBorrowsAccounts<'_, '_>>
for DepositCollateralForBorrowsKeys {
    fn from(accounts: DepositCollateralForBorrowsAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            transfer_authority: *accounts.transfer_authority.key,
            borrow_position: *accounts.borrow_position.key,
            collateral_token_account: *accounts.collateral_token_account.key,
            user_token_account: *accounts.user_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DepositCollateralForBorrowsKeys>
for [AccountMeta; DEPOSIT_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositCollateralForBorrowsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
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
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN]>
for DepositCollateralForBorrowsKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            transfer_authority: pubkeys[4],
            borrow_position: pubkeys[5],
            collateral_token_account: pubkeys[6],
            user_token_account: pubkeys[7],
            lp_token_mint: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<DepositCollateralForBorrowsAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositCollateralForBorrowsAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.transfer_authority.clone(),
            accounts.borrow_position.clone(),
            accounts.collateral_token_account.clone(),
            accounts.user_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPOSIT_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN]>
for DepositCollateralForBorrowsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            transfer_authority: &arr[4],
            borrow_position: &arr[5],
            collateral_token_account: &arr[6],
            user_token_account: &arr[7],
            lp_token_mint: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const DEPOSIT_COLLATERAL_FOR_BORROWS_IX_DISCM: [u8; 8usize] = [
    17, 2, 195, 190, 76, 16, 238, 74,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositCollateralForBorrowsIxArgs {
    pub params: DepositParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositCollateralForBorrowsIxData(pub DepositCollateralForBorrowsIxArgs);
impl From<DepositCollateralForBorrowsIxArgs> for DepositCollateralForBorrowsIxData {
    fn from(args: DepositCollateralForBorrowsIxArgs) -> Self {
        Self(args)
    }
}
impl DepositCollateralForBorrowsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_COLLATERAL_FOR_BORROWS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <DepositParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(DepositCollateralForBorrowsIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_COLLATERAL_FOR_BORROWS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_collateral_for_borrows_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositCollateralForBorrowsKeys,
    args: DepositCollateralForBorrowsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DepositCollateralForBorrowsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_collateral_for_borrows_ix(
    keys: DepositCollateralForBorrowsKeys,
    args: DepositCollateralForBorrowsIxArgs,
) -> std::io::Result<Instruction> {
    deposit_collateral_for_borrows_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn deposit_collateral_for_borrows_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositCollateralForBorrowsAccounts<'_, '_>,
    args: DepositCollateralForBorrowsIxArgs,
) -> ProgramResult {
    let keys: DepositCollateralForBorrowsKeys = accounts.into();
    let ix = deposit_collateral_for_borrows_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_collateral_for_borrows_invoke(
    accounts: DepositCollateralForBorrowsAccounts<'_, '_>,
    args: DepositCollateralForBorrowsIxArgs,
) -> ProgramResult {
    deposit_collateral_for_borrows_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn deposit_collateral_for_borrows_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositCollateralForBorrowsAccounts<'_, '_>,
    args: DepositCollateralForBorrowsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositCollateralForBorrowsKeys = accounts.into();
    let ix = deposit_collateral_for_borrows_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_collateral_for_borrows_invoke_signed(
    accounts: DepositCollateralForBorrowsAccounts<'_, '_>,
    args: DepositCollateralForBorrowsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_collateral_for_borrows_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_collateral_for_borrows_verify_account_keys(
    accounts: DepositCollateralForBorrowsAccounts<'_, '_>,
    keys: DepositCollateralForBorrowsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.borrow_position.key, keys.borrow_position),
        (*accounts.collateral_token_account.key, keys.collateral_token_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_collateral_for_borrows_verify_writable_privileges<'me, 'info>(
    accounts: DepositCollateralForBorrowsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.borrow_position,
        accounts.collateral_token_account,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_collateral_for_borrows_verify_signer_privileges<'me, 'info>(
    accounts: DepositCollateralForBorrowsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_collateral_for_borrows_verify_account_privileges<'me, 'info>(
    accounts: DepositCollateralForBorrowsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_collateral_for_borrows_verify_writable_privileges(accounts)?;
    deposit_collateral_for_borrows_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawCollateralForBorrowsAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub borrow_position: &'me AccountInfo<'info>,
    pub collateral_token_account: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawCollateralForBorrowsKeys {
    pub owner: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub transfer_authority: Pubkey,
    pub borrow_position: Pubkey,
    pub collateral_token_account: Pubkey,
    pub user_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawCollateralForBorrowsAccounts<'_, '_>>
for WithdrawCollateralForBorrowsKeys {
    fn from(accounts: WithdrawCollateralForBorrowsAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            transfer_authority: *accounts.transfer_authority.key,
            borrow_position: *accounts.borrow_position.key,
            collateral_token_account: *accounts.collateral_token_account.key,
            user_token_account: *accounts.user_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawCollateralForBorrowsKeys>
for [AccountMeta; WITHDRAW_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawCollateralForBorrowsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN]>
for WithdrawCollateralForBorrowsKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            transfer_authority: pubkeys[4],
            borrow_position: pubkeys[5],
            collateral_token_account: pubkeys[6],
            user_token_account: pubkeys[7],
            lp_token_mint: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawCollateralForBorrowsAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawCollateralForBorrowsAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.transfer_authority.clone(),
            accounts.borrow_position.clone(),
            accounts.collateral_token_account.clone(),
            accounts.user_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN]>
for WithdrawCollateralForBorrowsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            transfer_authority: &arr[4],
            borrow_position: &arr[5],
            collateral_token_account: &arr[6],
            user_token_account: &arr[7],
            lp_token_mint: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const WITHDRAW_COLLATERAL_FOR_BORROWS_IX_DISCM: [u8; 8usize] = [
    117, 160, 60, 82, 237, 233, 46, 182,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawCollateralForBorrowsIxArgs {
    pub params: WithdrawParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawCollateralForBorrowsIxData(pub WithdrawCollateralForBorrowsIxArgs);
impl From<WithdrawCollateralForBorrowsIxArgs> for WithdrawCollateralForBorrowsIxData {
    fn from(args: WithdrawCollateralForBorrowsIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawCollateralForBorrowsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_COLLATERAL_FOR_BORROWS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(WithdrawCollateralForBorrowsIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_COLLATERAL_FOR_BORROWS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_collateral_for_borrows_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawCollateralForBorrowsKeys,
    args: WithdrawCollateralForBorrowsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_COLLATERAL_FOR_BORROWS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawCollateralForBorrowsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_collateral_for_borrows_ix(
    keys: WithdrawCollateralForBorrowsKeys,
    args: WithdrawCollateralForBorrowsIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_collateral_for_borrows_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn withdraw_collateral_for_borrows_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawCollateralForBorrowsAccounts<'_, '_>,
    args: WithdrawCollateralForBorrowsIxArgs,
) -> ProgramResult {
    let keys: WithdrawCollateralForBorrowsKeys = accounts.into();
    let ix = withdraw_collateral_for_borrows_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_collateral_for_borrows_invoke(
    accounts: WithdrawCollateralForBorrowsAccounts<'_, '_>,
    args: WithdrawCollateralForBorrowsIxArgs,
) -> ProgramResult {
    withdraw_collateral_for_borrows_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_collateral_for_borrows_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawCollateralForBorrowsAccounts<'_, '_>,
    args: WithdrawCollateralForBorrowsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawCollateralForBorrowsKeys = accounts.into();
    let ix = withdraw_collateral_for_borrows_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_collateral_for_borrows_invoke_signed(
    accounts: WithdrawCollateralForBorrowsAccounts<'_, '_>,
    args: WithdrawCollateralForBorrowsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_collateral_for_borrows_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_collateral_for_borrows_verify_account_keys(
    accounts: WithdrawCollateralForBorrowsAccounts<'_, '_>,
    keys: WithdrawCollateralForBorrowsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.borrow_position.key, keys.borrow_position),
        (*accounts.collateral_token_account.key, keys.collateral_token_account),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_for_borrows_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawCollateralForBorrowsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.custody,
        accounts.borrow_position,
        accounts.collateral_token_account,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_for_borrows_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawCollateralForBorrowsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_for_borrows_verify_account_privileges<'me, 'info>(
    accounts: WithdrawCollateralForBorrowsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_collateral_for_borrows_verify_writable_privileges(accounts)?;
    withdraw_collateral_for_borrows_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateBorrowPositionAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub borrow_position: &'me AccountInfo<'info>,
    pub collateral_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateBorrowPositionKeys {
    pub signer: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub transfer_authority: Pubkey,
    pub borrow_position: Pubkey,
    pub collateral_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<LiquidateBorrowPositionAccounts<'_, '_>> for LiquidateBorrowPositionKeys {
    fn from(accounts: LiquidateBorrowPositionAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            transfer_authority: *accounts.transfer_authority.key,
            borrow_position: *accounts.borrow_position.key,
            collateral_token_account: *accounts.collateral_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<LiquidateBorrowPositionKeys>
for [AccountMeta; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidateBorrowPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN]>
for LiquidateBorrowPositionKeys {
    fn from(pubkeys: [Pubkey; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            transfer_authority: pubkeys[4],
            borrow_position: pubkeys[5],
            collateral_token_account: pubkeys[6],
            lp_token_mint: pubkeys[7],
            token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<LiquidateBorrowPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidateBorrowPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.transfer_authority.clone(),
            accounts.borrow_position.clone(),
            accounts.collateral_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN]>
for LiquidateBorrowPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            transfer_authority: &arr[4],
            borrow_position: &arr[5],
            collateral_token_account: &arr[6],
            lp_token_mint: &arr[7],
            token_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const LIQUIDATE_BORROW_POSITION_IX_DISCM: [u8; 8usize] = [
    235, 201, 17, 133, 234, 72, 84, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidateBorrowPositionIxArgs {
    pub params: LiquidateBorrowPositionParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateBorrowPositionIxData(pub LiquidateBorrowPositionIxArgs);
impl From<LiquidateBorrowPositionIxArgs> for LiquidateBorrowPositionIxData {
    fn from(args: LiquidateBorrowPositionIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidateBorrowPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_BORROW_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidateBorrowPositionParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(LiquidateBorrowPositionIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_BORROW_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_borrow_position_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidateBorrowPositionKeys,
    args: LiquidateBorrowPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: LiquidateBorrowPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_borrow_position_ix(
    keys: LiquidateBorrowPositionKeys,
    args: LiquidateBorrowPositionIxArgs,
) -> std::io::Result<Instruction> {
    liquidate_borrow_position_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn liquidate_borrow_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateBorrowPositionAccounts<'_, '_>,
    args: LiquidateBorrowPositionIxArgs,
) -> ProgramResult {
    let keys: LiquidateBorrowPositionKeys = accounts.into();
    let ix = liquidate_borrow_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquidate_borrow_position_invoke(
    accounts: LiquidateBorrowPositionAccounts<'_, '_>,
    args: LiquidateBorrowPositionIxArgs,
) -> ProgramResult {
    liquidate_borrow_position_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn liquidate_borrow_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateBorrowPositionAccounts<'_, '_>,
    args: LiquidateBorrowPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidateBorrowPositionKeys = accounts.into();
    let ix = liquidate_borrow_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquidate_borrow_position_invoke_signed(
    accounts: LiquidateBorrowPositionAccounts<'_, '_>,
    args: LiquidateBorrowPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquidate_borrow_position_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquidate_borrow_position_verify_account_keys(
    accounts: LiquidateBorrowPositionAccounts<'_, '_>,
    keys: LiquidateBorrowPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.borrow_position.key, keys.borrow_position),
        (*accounts.collateral_token_account.key, keys.collateral_token_account),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquidate_borrow_position_verify_writable_privileges<'me, 'info>(
    accounts: LiquidateBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.custody,
        accounts.borrow_position,
        accounts.collateral_token_account,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquidate_borrow_position_verify_signer_privileges<'me, 'info>(
    accounts: LiquidateBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquidate_borrow_position_verify_account_privileges<'me, 'info>(
    accounts: LiquidateBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquidate_borrow_position_verify_writable_privileges(accounts)?;
    liquidate_borrow_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PARTIAL_LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct PartialLiquidateBorrowPositionAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub borrow_position: &'me AccountInfo<'info>,
    pub collateral_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PartialLiquidateBorrowPositionKeys {
    pub signer: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub transfer_authority: Pubkey,
    pub borrow_position: Pubkey,
    pub collateral_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PartialLiquidateBorrowPositionAccounts<'_, '_>>
for PartialLiquidateBorrowPositionKeys {
    fn from(accounts: PartialLiquidateBorrowPositionAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            transfer_authority: *accounts.transfer_authority.key,
            borrow_position: *accounts.borrow_position.key,
            collateral_token_account: *accounts.collateral_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PartialLiquidateBorrowPositionKeys>
for [AccountMeta; PARTIAL_LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: PartialLiquidateBorrowPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PARTIAL_LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN]>
for PartialLiquidateBorrowPositionKeys {
    fn from(
        pubkeys: [Pubkey; PARTIAL_LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: pubkeys[0],
            perpetuals: pubkeys[1],
            pool: pubkeys[2],
            custody: pubkeys[3],
            transfer_authority: pubkeys[4],
            borrow_position: pubkeys[5],
            collateral_token_account: pubkeys[6],
            lp_token_mint: pubkeys[7],
            token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<PartialLiquidateBorrowPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; PARTIAL_LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: PartialLiquidateBorrowPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.transfer_authority.clone(),
            accounts.borrow_position.clone(),
            accounts.collateral_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PARTIAL_LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN]>
for PartialLiquidateBorrowPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PARTIAL_LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            perpetuals: &arr[1],
            pool: &arr[2],
            custody: &arr[3],
            transfer_authority: &arr[4],
            borrow_position: &arr[5],
            collateral_token_account: &arr[6],
            lp_token_mint: &arr[7],
            token_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const PARTIAL_LIQUIDATE_BORROW_POSITION_IX_DISCM: [u8; 8usize] = [
    250, 166, 13, 74, 97, 204, 130, 209,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartialLiquidateBorrowPositionIxArgs {
    pub params: PartialLiquidateBorrowPositionParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PartialLiquidateBorrowPositionIxData(
    pub PartialLiquidateBorrowPositionIxArgs,
);
impl From<PartialLiquidateBorrowPositionIxArgs>
for PartialLiquidateBorrowPositionIxData {
    fn from(args: PartialLiquidateBorrowPositionIxArgs) -> Self {
        Self(args)
    }
}
impl PartialLiquidateBorrowPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PARTIAL_LIQUIDATE_BORROW_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <PartialLiquidateBorrowPositionParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(PartialLiquidateBorrowPositionIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PARTIAL_LIQUIDATE_BORROW_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn partial_liquidate_borrow_position_ix_with_program_id(
    program_id: Pubkey,
    keys: PartialLiquidateBorrowPositionKeys,
    args: PartialLiquidateBorrowPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PARTIAL_LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: PartialLiquidateBorrowPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn partial_liquidate_borrow_position_ix(
    keys: PartialLiquidateBorrowPositionKeys,
    args: PartialLiquidateBorrowPositionIxArgs,
) -> std::io::Result<Instruction> {
    partial_liquidate_borrow_position_ix_with_program_id(
        PERPETUALS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn partial_liquidate_borrow_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PartialLiquidateBorrowPositionAccounts<'_, '_>,
    args: PartialLiquidateBorrowPositionIxArgs,
) -> ProgramResult {
    let keys: PartialLiquidateBorrowPositionKeys = accounts.into();
    let ix = partial_liquidate_borrow_position_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn partial_liquidate_borrow_position_invoke(
    accounts: PartialLiquidateBorrowPositionAccounts<'_, '_>,
    args: PartialLiquidateBorrowPositionIxArgs,
) -> ProgramResult {
    partial_liquidate_borrow_position_invoke_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn partial_liquidate_borrow_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PartialLiquidateBorrowPositionAccounts<'_, '_>,
    args: PartialLiquidateBorrowPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PartialLiquidateBorrowPositionKeys = accounts.into();
    let ix = partial_liquidate_borrow_position_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn partial_liquidate_borrow_position_invoke_signed(
    accounts: PartialLiquidateBorrowPositionAccounts<'_, '_>,
    args: PartialLiquidateBorrowPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    partial_liquidate_borrow_position_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn partial_liquidate_borrow_position_verify_account_keys(
    accounts: PartialLiquidateBorrowPositionAccounts<'_, '_>,
    keys: PartialLiquidateBorrowPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.borrow_position.key, keys.borrow_position),
        (*accounts.collateral_token_account.key, keys.collateral_token_account),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn partial_liquidate_borrow_position_verify_writable_privileges<'me, 'info>(
    accounts: PartialLiquidateBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.custody,
        accounts.borrow_position,
        accounts.collateral_token_account,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn partial_liquidate_borrow_position_verify_signer_privileges<'me, 'info>(
    accounts: PartialLiquidateBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn partial_liquidate_borrow_position_verify_account_privileges<'me, 'info>(
    accounts: PartialLiquidateBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    partial_liquidate_borrow_position_verify_writable_privileges(accounts)?;
    partial_liquidate_borrow_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_BORROW_POSITION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseBorrowPositionAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub borrow_position: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseBorrowPositionKeys {
    pub owner: Pubkey,
    pub borrow_position: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseBorrowPositionAccounts<'_, '_>> for CloseBorrowPositionKeys {
    fn from(accounts: CloseBorrowPositionAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            borrow_position: *accounts.borrow_position.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseBorrowPositionKeys>
for [AccountMeta; CLOSE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseBorrowPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_BORROW_POSITION_IX_ACCOUNTS_LEN]> for CloseBorrowPositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_BORROW_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            borrow_position: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseBorrowPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseBorrowPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.borrow_position.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_BORROW_POSITION_IX_ACCOUNTS_LEN]>
for CloseBorrowPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_BORROW_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            borrow_position: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLOSE_BORROW_POSITION_IX_DISCM: [u8; 8usize] = [
    204, 226, 145, 205, 232, 37, 3, 140,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloseBorrowPositionIxArgs {
    pub params: CloseBorrowPositionParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseBorrowPositionIxData(pub CloseBorrowPositionIxArgs);
impl From<CloseBorrowPositionIxArgs> for CloseBorrowPositionIxData {
    fn from(args: CloseBorrowPositionIxArgs) -> Self {
        Self(args)
    }
}
impl CloseBorrowPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_BORROW_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CloseBorrowPositionParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(CloseBorrowPositionIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_BORROW_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_borrow_position_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseBorrowPositionKeys,
    args: CloseBorrowPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_BORROW_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseBorrowPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_borrow_position_ix(
    keys: CloseBorrowPositionKeys,
    args: CloseBorrowPositionIxArgs,
) -> std::io::Result<Instruction> {
    close_borrow_position_ix_with_program_id(PERPETUALS_PROGRAM_ID, keys, args)
}
pub fn close_borrow_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseBorrowPositionAccounts<'_, '_>,
    args: CloseBorrowPositionIxArgs,
) -> ProgramResult {
    let keys: CloseBorrowPositionKeys = accounts.into();
    let ix = close_borrow_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_borrow_position_invoke(
    accounts: CloseBorrowPositionAccounts<'_, '_>,
    args: CloseBorrowPositionIxArgs,
) -> ProgramResult {
    close_borrow_position_invoke_with_program_id(PERPETUALS_PROGRAM_ID, accounts, args)
}
pub fn close_borrow_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseBorrowPositionAccounts<'_, '_>,
    args: CloseBorrowPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseBorrowPositionKeys = accounts.into();
    let ix = close_borrow_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_borrow_position_invoke_signed(
    accounts: CloseBorrowPositionAccounts<'_, '_>,
    args: CloseBorrowPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_borrow_position_invoke_signed_with_program_id(
        PERPETUALS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn close_borrow_position_verify_account_keys(
    accounts: CloseBorrowPositionAccounts<'_, '_>,
    keys: CloseBorrowPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.borrow_position.key, keys.borrow_position),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_borrow_position_verify_writable_privileges<'me, 'info>(
    accounts: CloseBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.borrow_position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_borrow_position_verify_signer_privileges<'me, 'info>(
    accounts: CloseBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_borrow_position_verify_account_privileges<'me, 'info>(
    accounts: CloseBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_borrow_position_verify_writable_privileges(accounts)?;
    close_borrow_position_verify_signer_privileges(accounts)?;
    Ok(())
}
