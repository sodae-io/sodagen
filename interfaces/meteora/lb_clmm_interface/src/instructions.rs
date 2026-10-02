use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum LbClmmProgramIx {
    AddLiquidity(AddLiquidityIxArgs),
    AddLiquidity2(AddLiquidity2IxArgs),
    AddLiquidityByStrategy(AddLiquidityByStrategyIxArgs),
    AddLiquidityByStrategy2(AddLiquidityByStrategy2IxArgs),
    AddLiquidityByStrategyOneSide(AddLiquidityByStrategyOneSideIxArgs),
    AddLiquidityByWeight(AddLiquidityByWeightIxArgs),
    AddLiquidityByWeight2(AddLiquidityByWeight2IxArgs),
    AddLiquidityOneSide(AddLiquidityOneSideIxArgs),
    AddLiquidityOneSidePrecise(AddLiquidityOneSidePreciseIxArgs),
    AddLiquidityOneSidePrecise2(AddLiquidityOneSidePrecise2IxArgs),
    CancelLimitOrder(CancelLimitOrderIxArgs),
    ClaimFee,
    ClaimFee2(ClaimFee2IxArgs),
    ClaimReward(ClaimRewardIxArgs),
    ClaimReward2(ClaimReward2IxArgs),
    CloseBinArray,
    CloseClaimFeeOperatorAccount,
    CloseLimitOrderIfEmpty,
    CloseOperatorAccount,
    ClosePosition,
    ClosePosition2,
    ClosePositionIfEmpty,
    ClosePresetParameter,
    ClosePresetParameter2,
    CloseTokenBadge,
    CreateOperatorAccount(CreateOperatorAccountIxArgs),
    DecreasePositionLength(DecreasePositionLengthIxArgs),
    ForIdlTypeGenerationDoNotCall(ForIdlTypeGenerationDoNotCallIxArgs),
    FundReward(FundRewardIxArgs),
    GoToABin(GoToABinIxArgs),
    IncreaseOracleLength(IncreaseOracleLengthIxArgs),
    IncreasePositionLength(IncreasePositionLengthIxArgs),
    IncreasePositionLength2(IncreasePositionLength2IxArgs),
    InitializeBinArray(InitializeBinArrayIxArgs),
    InitializeBinArrayBitmapExtension,
    InitializeCustomizablePermissionlessLbPair(
        InitializeCustomizablePermissionlessLbPairIxArgs,
    ),
    InitializeCustomizablePermissionlessLbPair2(
        InitializeCustomizablePermissionlessLbPair2IxArgs,
    ),
    InitializeLbPair(InitializeLbPairIxArgs),
    InitializeLbPair2(InitializeLbPair2IxArgs),
    InitializePermissionLbPair(InitializePermissionLbPairIxArgs),
    InitializePosition(InitializePositionIxArgs),
    InitializePosition2(InitializePosition2IxArgs),
    InitializePositionByOperator(InitializePositionByOperatorIxArgs),
    InitializePositionPda(InitializePositionPdaIxArgs),
    InitializePresetParameter(InitializePresetParameterIxArgs),
    InitializeReward(InitializeRewardIxArgs),
    InitializeTokenBadge,
    PlaceLimitOrder(PlaceLimitOrderIxArgs),
    RebalanceLiquidity(RebalanceLiquidityIxArgs),
    RemoveAllLiquidity,
    RemoveLiquidity(RemoveLiquidityIxArgs),
    RemoveLiquidity2(RemoveLiquidity2IxArgs),
    RemoveLiquidityByRange(RemoveLiquidityByRangeIxArgs),
    RemoveLiquidityByRange2(RemoveLiquidityByRange2IxArgs),
    SetActivationPoint(SetActivationPointIxArgs),
    SetPairStatus(SetPairStatusIxArgs),
    SetPairStatusPermissionless(SetPairStatusPermissionlessIxArgs),
    SetPermissionlessOperationBits(SetPermissionlessOperationBitsIxArgs),
    SetPreActivationDuration(SetPreActivationDurationIxArgs),
    SetPreActivationSwapAddress(SetPreActivationSwapAddressIxArgs),
    Swap(SwapIxArgs),
    Swap2(Swap2IxArgs),
    SwapExactOut(SwapExactOutIxArgs),
    SwapExactOut2(SwapExactOut2IxArgs),
    SwapWithPriceImpact(SwapWithPriceImpactIxArgs),
    SwapWithPriceImpact2(SwapWithPriceImpact2IxArgs),
    UpdateBaseFeeParameters(UpdateBaseFeeParametersIxArgs),
    UpdateDynamicFeeParameters(UpdateDynamicFeeParametersIxArgs),
    UpdateFeesAndReward2(UpdateFeesAndReward2IxArgs),
    UpdateFeesAndRewards,
    UpdatePositionOperator(UpdatePositionOperatorIxArgs),
    UpdateRewardDuration(UpdateRewardDurationIxArgs),
    UpdateRewardFunder(UpdateRewardFunderIxArgs),
    WithdrawIneligibleReward(WithdrawIneligibleRewardIxArgs),
    WithdrawProtocolFee(WithdrawProtocolFeeIxArgs),
    ZapProtocolFee(ZapProtocolFeeIxArgs),
}
impl LbClmmProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_IX_DISCM.len()..];
            let liquidity_parameter = if reader.is_empty() {
                Default::default()
            } else {
                <LiquidityParameter>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddLiquidity(AddLiquidityIxArgs {
                    liquidity_parameter,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY2_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY2_IX_DISCM.len()..];
            let liquidity_parameter = if reader.is_empty() {
                Default::default()
            } else {
                <LiquidityParameter>::deserialize(&mut reader)?
            };
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddLiquidity2(AddLiquidity2IxArgs {
                    liquidity_parameter,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM.len()..];
            let liquidity_parameter = <LiquidityParameterByStrategy>::deserialize(
                &mut reader,
            )?;
            return Ok(
                Self::AddLiquidityByStrategy(AddLiquidityByStrategyIxArgs {
                    liquidity_parameter,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_BY_STRATEGY2_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_BY_STRATEGY2_IX_DISCM.len()..];
            let liquidity_parameter = <LiquidityParameterByStrategy>::deserialize(
                &mut reader,
            )?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddLiquidityByStrategy2(AddLiquidityByStrategy2IxArgs {
                    liquidity_parameter,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM.len()..];
            let liquidity_parameter = <LiquidityParameterByStrategyOneSide>::deserialize(
                &mut reader,
            )?;
            return Ok(
                Self::AddLiquidityByStrategyOneSide(AddLiquidityByStrategyOneSideIxArgs {
                    liquidity_parameter,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM.len()..];
            let liquidity_parameter = if reader.is_empty() {
                Default::default()
            } else {
                <LiquidityParameterByWeight>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddLiquidityByWeight(AddLiquidityByWeightIxArgs {
                    liquidity_parameter,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_BY_WEIGHT2_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_BY_WEIGHT2_IX_DISCM.len()..];
            let liquidity_parameter = if reader.is_empty() {
                Default::default()
            } else {
                <LiquidityParameterByWeight>::deserialize(&mut reader)?
            };
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddLiquidityByWeight2(AddLiquidityByWeight2IxArgs {
                    liquidity_parameter,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_ONE_SIDE_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_ONE_SIDE_IX_DISCM.len()..];
            let liquidity_parameter = if reader.is_empty() {
                Default::default()
            } else {
                <LiquidityOneSideParameter>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddLiquidityOneSide(AddLiquidityOneSideIxArgs {
                    liquidity_parameter,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM.len()..];
            let parameter = if reader.is_empty() {
                Default::default()
            } else {
                <AddLiquiditySingleSidePreciseParameter>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddLiquidityOneSidePrecise(AddLiquidityOneSidePreciseIxArgs {
                    parameter,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_DISCM.len()..];
            let liquidity_parameter = if reader.is_empty() {
                Default::default()
            } else {
                <AddLiquiditySingleSidePreciseParameter2>::deserialize(&mut reader)?
            };
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::AddLiquidityOneSidePrecise2(AddLiquidityOneSidePrecise2IxArgs {
                    liquidity_parameter,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&CANCEL_LIMIT_ORDER_IX_DISCM) {
            let mut reader = &buf[CANCEL_LIMIT_ORDER_IX_DISCM.len()..];
            let bins: Vec<i32> = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CancelLimitOrder(CancelLimitOrderIxArgs {
                    bins,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&CLAIM_FEE_IX_DISCM) {
            return Ok(Self::ClaimFee);
        }
        if buf.starts_with(&CLAIM_FEE2_IX_DISCM) {
            let mut reader = &buf[CLAIM_FEE2_IX_DISCM.len()..];
            let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ClaimFee2(ClaimFee2IxArgs {
                    min_bin_id,
                    max_bin_id,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&CLAIM_REWARD_IX_DISCM) {
            let mut reader = &buf[CLAIM_REWARD_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ClaimReward(ClaimRewardIxArgs { reward_index }));
        }
        if buf.starts_with(&CLAIM_REWARD2_IX_DISCM) {
            let mut reader = &buf[CLAIM_REWARD2_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ClaimReward2(ClaimReward2IxArgs {
                    reward_index,
                    min_bin_id,
                    max_bin_id,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&CLOSE_BIN_ARRAY_IX_DISCM) {
            return Ok(Self::CloseBinArray);
        }
        if buf.starts_with(&CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseClaimFeeOperatorAccount);
        }
        if buf.starts_with(&CLOSE_LIMIT_ORDER_IF_EMPTY_IX_DISCM) {
            return Ok(Self::CloseLimitOrderIfEmpty);
        }
        if buf.starts_with(&CLOSE_OPERATOR_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseOperatorAccount);
        }
        if buf.starts_with(&CLOSE_POSITION_IX_DISCM) {
            return Ok(Self::ClosePosition);
        }
        if buf.starts_with(&CLOSE_POSITION2_IX_DISCM) {
            return Ok(Self::ClosePosition2);
        }
        if buf.starts_with(&CLOSE_POSITION_IF_EMPTY_IX_DISCM) {
            return Ok(Self::ClosePositionIfEmpty);
        }
        if buf.starts_with(&CLOSE_PRESET_PARAMETER_IX_DISCM) {
            return Ok(Self::ClosePresetParameter);
        }
        if buf.starts_with(&CLOSE_PRESET_PARAMETER2_IX_DISCM) {
            return Ok(Self::ClosePresetParameter2);
        }
        if buf.starts_with(&CLOSE_TOKEN_BADGE_IX_DISCM) {
            return Ok(Self::CloseTokenBadge);
        }
        if buf.starts_with(&CREATE_OPERATOR_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CREATE_OPERATOR_ACCOUNT_IX_DISCM.len()..];
            let permission: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateOperatorAccount(CreateOperatorAccountIxArgs {
                    permission,
                }),
            );
        }
        if buf.starts_with(&DECREASE_POSITION_LENGTH_IX_DISCM) {
            let mut reader = &buf[DECREASE_POSITION_LENGTH_IX_DISCM.len()..];
            let length_to_remove: u16 = crate::borsh_de_or_default(&mut reader)?;
            let side: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DecreasePositionLength(DecreasePositionLengthIxArgs {
                    length_to_remove,
                    side,
                }),
            );
        }
        if buf.starts_with(&FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_DISCM) {
            let mut reader = &buf[FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_DISCM.len()..];
            let ix = if reader.is_empty() {
                Default::default()
            } else {
                <DummyIx>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ForIdlTypeGenerationDoNotCall(ForIdlTypeGenerationDoNotCallIxArgs {
                    ix,
                }),
            );
        }
        if buf.starts_with(&FUND_REWARD_IX_DISCM) {
            let mut reader = &buf[FUND_REWARD_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let carry_forward: bool = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::FundReward(FundRewardIxArgs {
                    reward_index,
                    amount,
                    carry_forward,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&GO_TO_A_BIN_IX_DISCM) {
            let mut reader = &buf[GO_TO_A_BIN_IX_DISCM.len()..];
            let bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::GoToABin(GoToABinIxArgs { bin_id }));
        }
        if buf.starts_with(&INCREASE_ORACLE_LENGTH_IX_DISCM) {
            let mut reader = &buf[INCREASE_ORACLE_LENGTH_IX_DISCM.len()..];
            let length_to_add: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IncreaseOracleLength(IncreaseOracleLengthIxArgs {
                    length_to_add,
                }),
            );
        }
        if buf.starts_with(&INCREASE_POSITION_LENGTH_IX_DISCM) {
            let mut reader = &buf[INCREASE_POSITION_LENGTH_IX_DISCM.len()..];
            let length_to_add: u16 = crate::borsh_de_or_default(&mut reader)?;
            let side: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IncreasePositionLength(IncreasePositionLengthIxArgs {
                    length_to_add,
                    side,
                }),
            );
        }
        if buf.starts_with(&INCREASE_POSITION_LENGTH2_IX_DISCM) {
            let mut reader = &buf[INCREASE_POSITION_LENGTH2_IX_DISCM.len()..];
            let minimum_upper_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::IncreasePositionLength2(IncreasePositionLength2IxArgs {
                    minimum_upper_bin_id,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_BIN_ARRAY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_BIN_ARRAY_IX_DISCM.len()..];
            let index: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::InitializeBinArray(InitializeBinArrayIxArgs { index }));
        }
        if buf.starts_with(&INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM) {
            return Ok(Self::InitializeBinArrayBitmapExtension);
        }
        if buf.starts_with(&INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM
                .len()..];
            let params = <CustomizableParams>::deserialize(&mut reader)?;
            return Ok(
                Self::InitializeCustomizablePermissionlessLbPair(InitializeCustomizablePermissionlessLbPairIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_DISCM
                .len()..];
            let params = <CustomizableParams>::deserialize(&mut reader)?;
            return Ok(
                Self::InitializeCustomizablePermissionlessLbPair2(InitializeCustomizablePermissionlessLbPair2IxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_LB_PAIR_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_LB_PAIR_IX_DISCM.len()..];
            let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeLbPair(InitializeLbPairIxArgs {
                    active_id,
                    bin_step,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_LB_PAIR2_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_LB_PAIR2_IX_DISCM.len()..];
            let params = <InitializeLbPair2Params>::deserialize(&mut reader)?;
            return Ok(Self::InitializeLbPair2(InitializeLbPair2IxArgs { params }));
        }
        if buf.starts_with(&INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM.len()..];
            let ix_data = if reader.is_empty() {
                Default::default()
            } else {
                <InitPermissionPairIx>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializePermissionLbPair(InitializePermissionLbPairIxArgs {
                    ix_data,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POSITION_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_POSITION_IX_DISCM.len()..];
            let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let width: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializePosition(InitializePositionIxArgs {
                    lower_bin_id,
                    width,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POSITION2_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_POSITION2_IX_DISCM.len()..];
            let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let width: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializePosition2(InitializePosition2IxArgs {
                    lower_bin_id,
                    width,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM.len()..];
            let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let width: i32 = crate::borsh_de_or_default(&mut reader)?;
            let fee_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let lock_release_point: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializePositionByOperator(InitializePositionByOperatorIxArgs {
                    lower_bin_id,
                    width,
                    fee_owner,
                    lock_release_point,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POSITION_PDA_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_POSITION_PDA_IX_DISCM.len()..];
            let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let width: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializePositionPda(InitializePositionPdaIxArgs {
                    lower_bin_id,
                    width,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_PRESET_PARAMETER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_PRESET_PARAMETER_IX_DISCM.len()..];
            let ix = if reader.is_empty() {
                Default::default()
            } else {
                <InitPresetParametersIx>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializePresetParameter(InitializePresetParameterIxArgs {
                    ix,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_REWARD_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_REWARD_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
            let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeReward(InitializeRewardIxArgs {
                    reward_index,
                    reward_duration,
                    funder,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_TOKEN_BADGE_IX_DISCM) {
            return Ok(Self::InitializeTokenBadge);
        }
        if buf.starts_with(&PLACE_LIMIT_ORDER_IX_DISCM) {
            let mut reader = &buf[PLACE_LIMIT_ORDER_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <PlaceLimitOrderParams>::deserialize(&mut reader)?
            };
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::PlaceLimitOrder(PlaceLimitOrderIxArgs {
                    params,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&REBALANCE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[REBALANCE_LIQUIDITY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <RebalanceLiquidityParams>::deserialize(&mut reader)?
            };
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::RebalanceLiquidity(RebalanceLiquidityIxArgs {
                    params,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&REMOVE_ALL_LIQUIDITY_IX_DISCM) {
            return Ok(Self::RemoveAllLiquidity);
        }
        if buf.starts_with(&REMOVE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY_IX_DISCM.len()..];
            let bin_liquidity_removal: Vec<BinLiquidityReduction> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::RemoveLiquidity(RemoveLiquidityIxArgs {
                    bin_liquidity_removal,
                }),
            );
        }
        if buf.starts_with(&REMOVE_LIQUIDITY2_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY2_IX_DISCM.len()..];
            let bin_liquidity_removal: Vec<BinLiquidityReduction> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::RemoveLiquidity2(RemoveLiquidity2IxArgs {
                    bin_liquidity_removal,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM.len()..];
            let from_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let to_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let bps_to_remove: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveLiquidityByRange(RemoveLiquidityByRangeIxArgs {
                    from_bin_id,
                    to_bin_id,
                    bps_to_remove,
                }),
            );
        }
        if buf.starts_with(&REMOVE_LIQUIDITY_BY_RANGE2_IX_DISCM) {
            let mut reader = &buf[REMOVE_LIQUIDITY_BY_RANGE2_IX_DISCM.len()..];
            let from_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let to_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let bps_to_remove: u16 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::RemoveLiquidityByRange2(RemoveLiquidityByRange2IxArgs {
                    from_bin_id,
                    to_bin_id,
                    bps_to_remove,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&SET_ACTIVATION_POINT_IX_DISCM) {
            let mut reader = &buf[SET_ACTIVATION_POINT_IX_DISCM.len()..];
            let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetActivationPoint(SetActivationPointIxArgs {
                    activation_point,
                }),
            );
        }
        if buf.starts_with(&SET_PAIR_STATUS_IX_DISCM) {
            let mut reader = &buf[SET_PAIR_STATUS_IX_DISCM.len()..];
            let status: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetPairStatus(SetPairStatusIxArgs { status }));
        }
        if buf.starts_with(&SET_PAIR_STATUS_PERMISSIONLESS_IX_DISCM) {
            let mut reader = &buf[SET_PAIR_STATUS_PERMISSIONLESS_IX_DISCM.len()..];
            let status: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPairStatusPermissionless(SetPairStatusPermissionlessIxArgs {
                    status,
                }),
            );
        }
        if buf.starts_with(&SET_PERMISSIONLESS_OPERATION_BITS_IX_DISCM) {
            let mut reader = &buf[SET_PERMISSIONLESS_OPERATION_BITS_IX_DISCM.len()..];
            let bits: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPermissionlessOperationBits(SetPermissionlessOperationBitsIxArgs {
                    bits,
                }),
            );
        }
        if buf.starts_with(&SET_PRE_ACTIVATION_DURATION_IX_DISCM) {
            let mut reader = &buf[SET_PRE_ACTIVATION_DURATION_IX_DISCM.len()..];
            let pre_activation_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPreActivationDuration(SetPreActivationDurationIxArgs {
                    pre_activation_duration,
                }),
            );
        }
        if buf.starts_with(&SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM) {
            let mut reader = &buf[SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM.len()..];
            let pre_activation_swap_address: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetPreActivationSwapAddress(SetPreActivationSwapAddressIxArgs {
                    pre_activation_swap_address,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&SWAP2_IX_DISCM) {
            let mut reader = &buf[SWAP2_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::Swap2(Swap2IxArgs {
                    amount_in,
                    min_amount_out,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&SWAP_EXACT_OUT_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_OUT_IX_DISCM.len()..];
            let max_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapExactOut(SwapExactOutIxArgs {
                    max_in_amount,
                    out_amount,
                }),
            );
        }
        if buf.starts_with(&SWAP_EXACT_OUT2_IX_DISCM) {
            let mut reader = &buf[SWAP_EXACT_OUT2_IX_DISCM.len()..];
            let max_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SwapExactOut2(SwapExactOut2IxArgs {
                    max_in_amount,
                    out_amount,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&SWAP_WITH_PRICE_IMPACT_IX_DISCM) {
            let mut reader = &buf[SWAP_WITH_PRICE_IMPACT_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let active_id: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
            let max_price_impact_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapWithPriceImpact(SwapWithPriceImpactIxArgs {
                    amount_in,
                    active_id,
                    max_price_impact_bps,
                }),
            );
        }
        if buf.starts_with(&SWAP_WITH_PRICE_IMPACT2_IX_DISCM) {
            let mut reader = &buf[SWAP_WITH_PRICE_IMPACT2_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let active_id: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
            let max_price_impact_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SwapWithPriceImpact2(SwapWithPriceImpact2IxArgs {
                    amount_in,
                    active_id,
                    max_price_impact_bps,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&UPDATE_BASE_FEE_PARAMETERS_IX_DISCM) {
            let mut reader = &buf[UPDATE_BASE_FEE_PARAMETERS_IX_DISCM.len()..];
            let fee_parameter = if reader.is_empty() {
                Default::default()
            } else {
                <BaseFeeParameter>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateBaseFeeParameters(UpdateBaseFeeParametersIxArgs {
                    fee_parameter,
                }),
            );
        }
        if buf.starts_with(&UPDATE_DYNAMIC_FEE_PARAMETERS_IX_DISCM) {
            let mut reader = &buf[UPDATE_DYNAMIC_FEE_PARAMETERS_IX_DISCM.len()..];
            let fee_parameter = if reader.is_empty() {
                Default::default()
            } else {
                <DynamicFeeParameter>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateDynamicFeeParameters(UpdateDynamicFeeParametersIxArgs {
                    fee_parameter,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FEES_AND_REWARD2_IX_DISCM) {
            let mut reader = &buf[UPDATE_FEES_AND_REWARD2_IX_DISCM.len()..];
            let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateFeesAndReward2(UpdateFeesAndReward2IxArgs {
                    min_bin_id,
                    max_bin_id,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FEES_AND_REWARDS_IX_DISCM) {
            return Ok(Self::UpdateFeesAndRewards);
        }
        if buf.starts_with(&UPDATE_POSITION_OPERATOR_IX_DISCM) {
            let mut reader = &buf[UPDATE_POSITION_OPERATOR_IX_DISCM.len()..];
            let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdatePositionOperator(UpdatePositionOperatorIxArgs {
                    operator,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REWARD_DURATION_IX_DISCM) {
            let mut reader = &buf[UPDATE_REWARD_DURATION_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let new_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRewardDuration(UpdateRewardDurationIxArgs {
                    reward_index,
                    new_duration,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REWARD_FUNDER_IX_DISCM) {
            let mut reader = &buf[UPDATE_REWARD_FUNDER_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let new_funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRewardFunder(UpdateRewardFunderIxArgs {
                    reward_index,
                    new_funder,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_INELIGIBLE_REWARD_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_INELIGIBLE_REWARD_IX_DISCM.len()..];
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::WithdrawIneligibleReward(WithdrawIneligibleRewardIxArgs {
                    reward_index,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_PROTOCOL_FEE_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_PROTOCOL_FEE_IX_DISCM.len()..];
            let max_amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::WithdrawProtocolFee(WithdrawProtocolFeeIxArgs {
                    max_amount_x,
                    max_amount_y,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&ZAP_PROTOCOL_FEE_IX_DISCM) {
            let mut reader = &buf[ZAP_PROTOCOL_FEE_IX_DISCM.len()..];
            let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info = if reader.is_empty() {
                Default::default()
            } else {
                <RemainingAccountsInfo>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ZapProtocolFee(ZapProtocolFeeIxArgs {
                    max_amount,
                    remaining_accounts_info,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddLiquidity(args) => {
                writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddLiquidity2(args) => {
                writer.write_all(&ADD_LIQUIDITY2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddLiquidityByStrategy(args) => {
                writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddLiquidityByStrategy2(args) => {
                writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddLiquidityByStrategyOneSide(args) => {
                writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddLiquidityByWeight(args) => {
                writer.write_all(&ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddLiquidityByWeight2(args) => {
                writer.write_all(&ADD_LIQUIDITY_BY_WEIGHT2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddLiquidityOneSide(args) => {
                writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddLiquidityOneSidePrecise(args) => {
                writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.parameter, &mut writer)?;
                Ok(())
            }
            Self::AddLiquidityOneSidePrecise2(args) => {
                writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_parameter,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CancelLimitOrder(args) => {
                writer.write_all(&CANCEL_LIMIT_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bins, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ClaimFee => writer.write_all(&CLAIM_FEE_IX_DISCM),
            Self::ClaimFee2(args) => {
                writer.write_all(&CLAIM_FEE2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.min_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ClaimReward(args) => {
                writer.write_all(&CLAIM_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                Ok(())
            }
            Self::ClaimReward2(args) => {
                writer.write_all(&CLAIM_REWARD2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CloseBinArray => writer.write_all(&CLOSE_BIN_ARRAY_IX_DISCM),
            Self::CloseClaimFeeOperatorAccount => {
                writer.write_all(&CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_DISCM)
            }
            Self::CloseLimitOrderIfEmpty => {
                writer.write_all(&CLOSE_LIMIT_ORDER_IF_EMPTY_IX_DISCM)
            }
            Self::CloseOperatorAccount => {
                writer.write_all(&CLOSE_OPERATOR_ACCOUNT_IX_DISCM)
            }
            Self::ClosePosition => writer.write_all(&CLOSE_POSITION_IX_DISCM),
            Self::ClosePosition2 => writer.write_all(&CLOSE_POSITION2_IX_DISCM),
            Self::ClosePositionIfEmpty => {
                writer.write_all(&CLOSE_POSITION_IF_EMPTY_IX_DISCM)
            }
            Self::ClosePresetParameter => {
                writer.write_all(&CLOSE_PRESET_PARAMETER_IX_DISCM)
            }
            Self::ClosePresetParameter2 => {
                writer.write_all(&CLOSE_PRESET_PARAMETER2_IX_DISCM)
            }
            Self::CloseTokenBadge => writer.write_all(&CLOSE_TOKEN_BADGE_IX_DISCM),
            Self::CreateOperatorAccount(args) => {
                writer.write_all(&CREATE_OPERATOR_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.permission, &mut writer)?;
                Ok(())
            }
            Self::DecreasePositionLength(args) => {
                writer.write_all(&DECREASE_POSITION_LENGTH_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.length_to_remove, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.side, &mut writer)?;
                Ok(())
            }
            Self::ForIdlTypeGenerationDoNotCall(args) => {
                writer.write_all(&FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.ix, &mut writer)?;
                Ok(())
            }
            Self::FundReward(args) => {
                writer.write_all(&FUND_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.carry_forward, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::GoToABin(args) => {
                writer.write_all(&GO_TO_A_BIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bin_id, &mut writer)?;
                Ok(())
            }
            Self::IncreaseOracleLength(args) => {
                writer.write_all(&INCREASE_ORACLE_LENGTH_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.length_to_add, &mut writer)?;
                Ok(())
            }
            Self::IncreasePositionLength(args) => {
                writer.write_all(&INCREASE_POSITION_LENGTH_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.length_to_add, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.side, &mut writer)?;
                Ok(())
            }
            Self::IncreasePositionLength2(args) => {
                writer.write_all(&INCREASE_POSITION_LENGTH2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.minimum_upper_bin_id,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeBinArray(args) => {
                writer.write_all(&INITIALIZE_BIN_ARRAY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                Ok(())
            }
            Self::InitializeBinArrayBitmapExtension => {
                writer.write_all(&INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM)
            }
            Self::InitializeCustomizablePermissionlessLbPair(args) => {
                writer
                    .write_all(
                        &INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM,
                    )?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeCustomizablePermissionlessLbPair2(args) => {
                writer
                    .write_all(
                        &INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_DISCM,
                    )?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeLbPair(args) => {
                writer.write_all(&INITIALIZE_LB_PAIR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.active_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bin_step, &mut writer)?;
                Ok(())
            }
            Self::InitializeLbPair2(args) => {
                writer.write_all(&INITIALIZE_LB_PAIR2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializePermissionLbPair(args) => {
                writer.write_all(&INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.ix_data, &mut writer)?;
                Ok(())
            }
            Self::InitializePosition(args) => {
                writer.write_all(&INITIALIZE_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lower_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.width, &mut writer)?;
                Ok(())
            }
            Self::InitializePosition2(args) => {
                writer.write_all(&INITIALIZE_POSITION2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lower_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.width, &mut writer)?;
                Ok(())
            }
            Self::InitializePositionByOperator(args) => {
                writer.write_all(&INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lower_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.width, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_owner, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.lock_release_point, &mut writer)?;
                Ok(())
            }
            Self::InitializePositionPda(args) => {
                writer.write_all(&INITIALIZE_POSITION_PDA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lower_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.width, &mut writer)?;
                Ok(())
            }
            Self::InitializePresetParameter(args) => {
                writer.write_all(&INITIALIZE_PRESET_PARAMETER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.ix, &mut writer)?;
                Ok(())
            }
            Self::InitializeReward(args) => {
                writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.reward_duration, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.funder, &mut writer)?;
                Ok(())
            }
            Self::InitializeTokenBadge => {
                writer.write_all(&INITIALIZE_TOKEN_BADGE_IX_DISCM)
            }
            Self::PlaceLimitOrder(args) => {
                writer.write_all(&PLACE_LIMIT_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::RebalanceLiquidity(args) => {
                writer.write_all(&REBALANCE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::RemoveAllLiquidity => writer.write_all(&REMOVE_ALL_LIQUIDITY_IX_DISCM),
            Self::RemoveLiquidity(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.bin_liquidity_removal,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::RemoveLiquidity2(args) => {
                writer.write_all(&REMOVE_LIQUIDITY2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.bin_liquidity_removal,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::RemoveLiquidityByRange(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.from_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.to_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bps_to_remove, &mut writer)?;
                Ok(())
            }
            Self::RemoveLiquidityByRange2(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_BY_RANGE2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.from_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.to_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bps_to_remove, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetActivationPoint(args) => {
                writer.write_all(&SET_ACTIVATION_POINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.activation_point, &mut writer)?;
                Ok(())
            }
            Self::SetPairStatus(args) => {
                writer.write_all(&SET_PAIR_STATUS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.status, &mut writer)?;
                Ok(())
            }
            Self::SetPairStatusPermissionless(args) => {
                writer.write_all(&SET_PAIR_STATUS_PERMISSIONLESS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.status, &mut writer)?;
                Ok(())
            }
            Self::SetPermissionlessOperationBits(args) => {
                writer.write_all(&SET_PERMISSIONLESS_OPERATION_BITS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bits, &mut writer)?;
                Ok(())
            }
            Self::SetPreActivationDuration(args) => {
                writer.write_all(&SET_PRE_ACTIVATION_DURATION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.pre_activation_duration,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetPreActivationSwapAddress(args) => {
                writer.write_all(&SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.pre_activation_swap_address,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::Swap2(args) => {
                writer.write_all(&SWAP2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SwapExactOut(args) => {
                writer.write_all(&SWAP_EXACT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.out_amount, &mut writer)?;
                Ok(())
            }
            Self::SwapExactOut2(args) => {
                writer.write_all(&SWAP_EXACT_OUT2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_in_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.out_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SwapWithPriceImpact(args) => {
                writer.write_all(&SWAP_WITH_PRICE_IMPACT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.active_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_price_impact_bps,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SwapWithPriceImpact2(args) => {
                writer.write_all(&SWAP_WITH_PRICE_IMPACT2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.active_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_price_impact_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateBaseFeeParameters(args) => {
                writer.write_all(&UPDATE_BASE_FEE_PARAMETERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_parameter, &mut writer)?;
                Ok(())
            }
            Self::UpdateDynamicFeeParameters(args) => {
                writer.write_all(&UPDATE_DYNAMIC_FEE_PARAMETERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_parameter, &mut writer)?;
                Ok(())
            }
            Self::UpdateFeesAndReward2(args) => {
                writer.write_all(&UPDATE_FEES_AND_REWARD2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.min_bin_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_bin_id, &mut writer)?;
                Ok(())
            }
            Self::UpdateFeesAndRewards => {
                writer.write_all(&UPDATE_FEES_AND_REWARDS_IX_DISCM)
            }
            Self::UpdatePositionOperator(args) => {
                writer.write_all(&UPDATE_POSITION_OPERATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.operator, &mut writer)?;
                Ok(())
            }
            Self::UpdateRewardDuration(args) => {
                writer.write_all(&UPDATE_REWARD_DURATION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_duration, &mut writer)?;
                Ok(())
            }
            Self::UpdateRewardFunder(args) => {
                writer.write_all(&UPDATE_REWARD_FUNDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_funder, &mut writer)?;
                Ok(())
            }
            Self::WithdrawIneligibleReward(args) => {
                writer.write_all(&WITHDRAW_INELIGIBLE_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::WithdrawProtocolFee(args) => {
                writer.write_all(&WITHDRAW_PROTOCOL_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_amount_x, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_amount_y, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ZapProtocolFee(args) => {
                writer.write_all(&ZAP_PROTOCOL_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
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
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: AddLiquidityAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<AddLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8usize] = [181, 157, 89, 67, 143, 182, 52, 72];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityIxArgs {
    pub liquidity_parameter: LiquidityParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityIxData(pub AddLiquidityIxArgs);
impl From<AddLiquidityIxArgs> for AddLiquidityIxData {
    fn from(args: AddLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_parameter = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityParameter>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddLiquidityIxArgs {
                liquidity_parameter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_ix(
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_invoke(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    add_liquidity_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_invoke_signed(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_verify_account_keys(
    accounts: AddLiquidityAccounts<'_, '_>,
    keys: AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_verify_writable_privileges(accounts)?;
    add_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidity2Accounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidity2Keys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidity2Accounts<'_, '_>> for AddLiquidity2Keys {
    fn from(accounts: AddLiquidity2Accounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidity2Keys> for [AccountMeta; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidity2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            sender: pubkeys[9],
            token_x_program: pubkeys[10],
            token_y_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<AddLiquidity2Accounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidity2Accounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN]>
for AddLiquidity2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            sender: &arr[9],
            token_x_program: &arr[10],
            token_y_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const ADD_LIQUIDITY2_IX_DISCM: [u8; 8usize] = [228, 162, 78, 28, 70, 219, 116, 115];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidity2IxArgs {
    pub liquidity_parameter: LiquidityParameter,
    pub remaining_accounts_info: RemainingAccountsInfo,
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
        let liquidity_parameter = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityParameter>::deserialize(&mut reader)?
        };
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddLiquidity2IxArgs {
                liquidity_parameter,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
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
    add_liquidity2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
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
    add_liquidity2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
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
        LB_CLMM_PROGRAM_ID,
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
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
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
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
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
    for should_be_signer in [accounts.sender] {
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
pub const ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityByStrategyAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategyKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityByStrategyAccounts<'_, '_>> for AddLiquidityByStrategyKeys {
    fn from(accounts: AddLiquidityByStrategyAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityByStrategyKeys>
for [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityByStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategyKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<AddLiquidityByStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityByStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM: [u8; 8usize] = [
    7, 3, 150, 127, 148, 40, 61, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityByStrategyIxArgs {
    pub liquidity_parameter: LiquidityParameterByStrategy,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategyIxData(pub AddLiquidityByStrategyIxArgs);
impl From<AddLiquidityByStrategyIxArgs> for AddLiquidityByStrategyIxData {
    fn from(args: AddLiquidityByStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityByStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_parameter = <LiquidityParameterByStrategy>::deserialize(
            &mut reader,
        )?;
        Ok(
            Self(AddLiquidityByStrategyIxArgs {
                liquidity_parameter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_by_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityByStrategyKeys,
    args: AddLiquidityByStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityByStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_by_strategy_ix(
    keys: AddLiquidityByStrategyKeys,
    args: AddLiquidityByStrategyIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_by_strategy_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_by_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    args: AddLiquidityByStrategyIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityByStrategyKeys = accounts.into();
    let ix = add_liquidity_by_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_by_strategy_invoke(
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    args: AddLiquidityByStrategyIxArgs,
) -> ProgramResult {
    add_liquidity_by_strategy_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_by_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    args: AddLiquidityByStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityByStrategyKeys = accounts.into();
    let ix = add_liquidity_by_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_by_strategy_invoke_signed(
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    args: AddLiquidityByStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_by_strategy_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_by_strategy_verify_account_keys(
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    keys: AddLiquidityByStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_by_strategy_verify_writable_privileges(accounts)?;
    add_liquidity_by_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_BY_STRATEGY2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityByStrategy2Accounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategy2Keys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityByStrategy2Accounts<'_, '_>> for AddLiquidityByStrategy2Keys {
    fn from(accounts: AddLiquidityByStrategy2Accounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityByStrategy2Keys>
for [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY2_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityByStrategy2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; ADD_LIQUIDITY_BY_STRATEGY2_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategy2Keys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_BY_STRATEGY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            sender: pubkeys[9],
            token_x_program: pubkeys[10],
            token_y_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<AddLiquidityByStrategy2Accounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY2_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityByStrategy2Accounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY2_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategy2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            sender: &arr[9],
            token_x_program: &arr[10],
            token_y_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const ADD_LIQUIDITY_BY_STRATEGY2_IX_DISCM: [u8; 8usize] = [
    3, 221, 149, 218, 111, 141, 118, 213,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityByStrategy2IxArgs {
    pub liquidity_parameter: LiquidityParameterByStrategy,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategy2IxData(pub AddLiquidityByStrategy2IxArgs);
impl From<AddLiquidityByStrategy2IxArgs> for AddLiquidityByStrategy2IxData {
    fn from(args: AddLiquidityByStrategy2IxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityByStrategy2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_BY_STRATEGY2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_parameter = <LiquidityParameterByStrategy>::deserialize(
            &mut reader,
        )?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddLiquidityByStrategy2IxArgs {
                liquidity_parameter,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_by_strategy2_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityByStrategy2Keys,
    args: AddLiquidityByStrategy2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY2_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityByStrategy2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_by_strategy2_ix(
    keys: AddLiquidityByStrategy2Keys,
    args: AddLiquidityByStrategy2IxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_by_strategy2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_by_strategy2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategy2Accounts<'_, '_>,
    args: AddLiquidityByStrategy2IxArgs,
) -> ProgramResult {
    let keys: AddLiquidityByStrategy2Keys = accounts.into();
    let ix = add_liquidity_by_strategy2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_by_strategy2_invoke(
    accounts: AddLiquidityByStrategy2Accounts<'_, '_>,
    args: AddLiquidityByStrategy2IxArgs,
) -> ProgramResult {
    add_liquidity_by_strategy2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_by_strategy2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategy2Accounts<'_, '_>,
    args: AddLiquidityByStrategy2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityByStrategy2Keys = accounts.into();
    let ix = add_liquidity_by_strategy2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_by_strategy2_invoke_signed(
    accounts: AddLiquidityByStrategy2Accounts<'_, '_>,
    args: AddLiquidityByStrategy2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_by_strategy2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_by_strategy2_verify_account_keys(
    accounts: AddLiquidityByStrategy2Accounts<'_, '_>,
    keys: AddLiquidityByStrategy2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy2_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategy2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy2_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategy2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy2_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategy2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_by_strategy2_verify_writable_privileges(accounts)?;
    add_liquidity_by_strategy2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityByStrategyOneSideAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategyOneSideKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityByStrategyOneSideAccounts<'_, '_>>
for AddLiquidityByStrategyOneSideKeys {
    fn from(accounts: AddLiquidityByStrategyOneSideAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token: *accounts.user_token.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityByStrategyOneSideKeys>
for [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityByStrategyOneSideKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
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
impl From<[Pubkey; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategyOneSideKeys {
    fn from(
        pubkeys: [Pubkey; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token: pubkeys[3],
            reserve: pubkeys[4],
            token_mint: pubkeys[5],
            bin_array_lower: pubkeys[6],
            bin_array_upper: pubkeys[7],
            sender: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<AddLiquidityByStrategyOneSideAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityByStrategyOneSideAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategyOneSideAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token: &arr[3],
            reserve: &arr[4],
            token_mint: &arr[5],
            bin_array_lower: &arr[6],
            bin_array_upper: &arr[7],
            sender: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM: [u8; 8usize] = [
    41, 5, 238, 175, 100, 225, 6, 205,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityByStrategyOneSideIxArgs {
    pub liquidity_parameter: LiquidityParameterByStrategyOneSide,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategyOneSideIxData(pub AddLiquidityByStrategyOneSideIxArgs);
impl From<AddLiquidityByStrategyOneSideIxArgs> for AddLiquidityByStrategyOneSideIxData {
    fn from(args: AddLiquidityByStrategyOneSideIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityByStrategyOneSideIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_parameter = <LiquidityParameterByStrategyOneSide>::deserialize(
            &mut reader,
        )?;
        Ok(
            Self(AddLiquidityByStrategyOneSideIxArgs {
                liquidity_parameter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_by_strategy_one_side_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityByStrategyOneSideKeys,
    args: AddLiquidityByStrategyOneSideIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AddLiquidityByStrategyOneSideIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_by_strategy_one_side_ix(
    keys: AddLiquidityByStrategyOneSideKeys,
    args: AddLiquidityByStrategyOneSideIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_by_strategy_one_side_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_by_strategy_one_side_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    args: AddLiquidityByStrategyOneSideIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityByStrategyOneSideKeys = accounts.into();
    let ix = add_liquidity_by_strategy_one_side_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_by_strategy_one_side_invoke(
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    args: AddLiquidityByStrategyOneSideIxArgs,
) -> ProgramResult {
    add_liquidity_by_strategy_one_side_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn add_liquidity_by_strategy_one_side_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    args: AddLiquidityByStrategyOneSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityByStrategyOneSideKeys = accounts.into();
    let ix = add_liquidity_by_strategy_one_side_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_by_strategy_one_side_invoke_signed(
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    args: AddLiquidityByStrategyOneSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_by_strategy_one_side_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_by_strategy_one_side_verify_account_keys(
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    keys: AddLiquidityByStrategyOneSideKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
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
pub fn add_liquidity_by_strategy_one_side_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token,
        accounts.reserve,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_one_side_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_one_side_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_by_strategy_one_side_verify_writable_privileges(accounts)?;
    add_liquidity_by_strategy_one_side_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityByWeightAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityByWeightKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityByWeightAccounts<'_, '_>> for AddLiquidityByWeightKeys {
    fn from(accounts: AddLiquidityByWeightAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityByWeightKeys>
for [AccountMeta; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityByWeightKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN]>
for AddLiquidityByWeightKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<AddLiquidityByWeightAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityByWeightAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN]>
for AddLiquidityByWeightAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM: [u8; 8usize] = [
    28, 140, 238, 99, 231, 162, 21, 149,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityByWeightIxArgs {
    pub liquidity_parameter: LiquidityParameterByWeight,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityByWeightIxData(pub AddLiquidityByWeightIxArgs);
impl From<AddLiquidityByWeightIxArgs> for AddLiquidityByWeightIxData {
    fn from(args: AddLiquidityByWeightIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityByWeightIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_parameter = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityParameterByWeight>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddLiquidityByWeightIxArgs {
                liquidity_parameter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_by_weight_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityByWeightKeys,
    args: AddLiquidityByWeightIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityByWeightIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_by_weight_ix(
    keys: AddLiquidityByWeightKeys,
    args: AddLiquidityByWeightIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_by_weight_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_by_weight_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    args: AddLiquidityByWeightIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityByWeightKeys = accounts.into();
    let ix = add_liquidity_by_weight_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_by_weight_invoke(
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    args: AddLiquidityByWeightIxArgs,
) -> ProgramResult {
    add_liquidity_by_weight_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_by_weight_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    args: AddLiquidityByWeightIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityByWeightKeys = accounts.into();
    let ix = add_liquidity_by_weight_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_by_weight_invoke_signed(
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    args: AddLiquidityByWeightIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_by_weight_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_by_weight_verify_account_keys(
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    keys: AddLiquidityByWeightKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityByWeightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityByWeightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityByWeightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_by_weight_verify_writable_privileges(accounts)?;
    add_liquidity_by_weight_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_BY_WEIGHT2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityByWeight2Accounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityByWeight2Keys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityByWeight2Accounts<'_, '_>> for AddLiquidityByWeight2Keys {
    fn from(accounts: AddLiquidityByWeight2Accounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityByWeight2Keys>
for [AccountMeta; ADD_LIQUIDITY_BY_WEIGHT2_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityByWeight2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; ADD_LIQUIDITY_BY_WEIGHT2_IX_ACCOUNTS_LEN]>
for AddLiquidityByWeight2Keys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_BY_WEIGHT2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            sender: pubkeys[9],
            token_x_program: pubkeys[10],
            token_y_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<AddLiquidityByWeight2Accounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT2_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityByWeight2Accounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT2_IX_ACCOUNTS_LEN]>
for AddLiquidityByWeight2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            sender: &arr[9],
            token_x_program: &arr[10],
            token_y_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const ADD_LIQUIDITY_BY_WEIGHT2_IX_DISCM: [u8; 8usize] = [
    209, 59, 63, 91, 111, 200, 153, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityByWeight2IxArgs {
    pub liquidity_parameter: LiquidityParameterByWeight,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityByWeight2IxData(pub AddLiquidityByWeight2IxArgs);
impl From<AddLiquidityByWeight2IxArgs> for AddLiquidityByWeight2IxData {
    fn from(args: AddLiquidityByWeight2IxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityByWeight2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_BY_WEIGHT2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_parameter = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityParameterByWeight>::deserialize(&mut reader)?
        };
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddLiquidityByWeight2IxArgs {
                liquidity_parameter,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_BY_WEIGHT2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_by_weight2_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityByWeight2Keys,
    args: AddLiquidityByWeight2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_BY_WEIGHT2_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityByWeight2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_by_weight2_ix(
    keys: AddLiquidityByWeight2Keys,
    args: AddLiquidityByWeight2IxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_by_weight2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_by_weight2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByWeight2Accounts<'_, '_>,
    args: AddLiquidityByWeight2IxArgs,
) -> ProgramResult {
    let keys: AddLiquidityByWeight2Keys = accounts.into();
    let ix = add_liquidity_by_weight2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_by_weight2_invoke(
    accounts: AddLiquidityByWeight2Accounts<'_, '_>,
    args: AddLiquidityByWeight2IxArgs,
) -> ProgramResult {
    add_liquidity_by_weight2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_by_weight2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByWeight2Accounts<'_, '_>,
    args: AddLiquidityByWeight2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityByWeight2Keys = accounts.into();
    let ix = add_liquidity_by_weight2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_by_weight2_invoke_signed(
    accounts: AddLiquidityByWeight2Accounts<'_, '_>,
    args: AddLiquidityByWeight2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_by_weight2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_by_weight2_verify_account_keys(
    accounts: AddLiquidityByWeight2Accounts<'_, '_>,
    keys: AddLiquidityByWeight2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight2_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityByWeight2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight2_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityByWeight2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight2_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityByWeight2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_by_weight2_verify_writable_privileges(accounts)?;
    add_liquidity_by_weight2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityOneSideAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSideKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityOneSideAccounts<'_, '_>> for AddLiquidityOneSideKeys {
    fn from(accounts: AddLiquidityOneSideAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token: *accounts.user_token.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityOneSideKeys>
for [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityOneSideKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
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
impl From<[Pubkey; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN]> for AddLiquidityOneSideKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token: pubkeys[3],
            reserve: pubkeys[4],
            token_mint: pubkeys[5],
            bin_array_lower: pubkeys[6],
            bin_array_upper: pubkeys[7],
            sender: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<AddLiquidityOneSideAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityOneSideAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN]>
for AddLiquidityOneSideAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token: &arr[3],
            reserve: &arr[4],
            token_mint: &arr[5],
            bin_array_lower: &arr[6],
            bin_array_upper: &arr[7],
            sender: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const ADD_LIQUIDITY_ONE_SIDE_IX_DISCM: [u8; 8usize] = [
    94, 155, 103, 151, 70, 95, 220, 165,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityOneSideIxArgs {
    pub liquidity_parameter: LiquidityOneSideParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSideIxData(pub AddLiquidityOneSideIxArgs);
impl From<AddLiquidityOneSideIxArgs> for AddLiquidityOneSideIxData {
    fn from(args: AddLiquidityOneSideIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityOneSideIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_ONE_SIDE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_parameter = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityOneSideParameter>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddLiquidityOneSideIxArgs {
                liquidity_parameter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_one_side_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityOneSideKeys,
    args: AddLiquidityOneSideIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityOneSideIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_one_side_ix(
    keys: AddLiquidityOneSideKeys,
    args: AddLiquidityOneSideIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_one_side_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_one_side_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    args: AddLiquidityOneSideIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityOneSideKeys = accounts.into();
    let ix = add_liquidity_one_side_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_one_side_invoke(
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    args: AddLiquidityOneSideIxArgs,
) -> ProgramResult {
    add_liquidity_one_side_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_one_side_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    args: AddLiquidityOneSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityOneSideKeys = accounts.into();
    let ix = add_liquidity_one_side_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_one_side_invoke_signed(
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    args: AddLiquidityOneSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_one_side_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_one_side_verify_account_keys(
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    keys: AddLiquidityOneSideKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
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
pub fn add_liquidity_one_side_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token,
        accounts.reserve,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_one_side_verify_writable_privileges(accounts)?;
    add_liquidity_one_side_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityOneSidePreciseAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSidePreciseKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityOneSidePreciseAccounts<'_, '_>>
for AddLiquidityOneSidePreciseKeys {
    fn from(accounts: AddLiquidityOneSidePreciseAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token: *accounts.user_token.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityOneSidePreciseKeys>
for [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityOneSidePreciseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
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
impl From<[Pubkey; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN]>
for AddLiquidityOneSidePreciseKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token: pubkeys[3],
            reserve: pubkeys[4],
            token_mint: pubkeys[5],
            bin_array_lower: pubkeys[6],
            bin_array_upper: pubkeys[7],
            sender: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<AddLiquidityOneSidePreciseAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityOneSidePreciseAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN]>
for AddLiquidityOneSidePreciseAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token: &arr[3],
            reserve: &arr[4],
            token_mint: &arr[5],
            bin_array_lower: &arr[6],
            bin_array_upper: &arr[7],
            sender: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM: [u8; 8usize] = [
    161, 194, 103, 84, 171, 71, 250, 154,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityOneSidePreciseIxArgs {
    pub parameter: AddLiquiditySingleSidePreciseParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSidePreciseIxData(pub AddLiquidityOneSidePreciseIxArgs);
impl From<AddLiquidityOneSidePreciseIxArgs> for AddLiquidityOneSidePreciseIxData {
    fn from(args: AddLiquidityOneSidePreciseIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityOneSidePreciseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let parameter = if reader.is_empty() {
            Default::default()
        } else {
            <AddLiquiditySingleSidePreciseParameter>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddLiquidityOneSidePreciseIxArgs {
                parameter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.parameter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_one_side_precise_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityOneSidePreciseKeys,
    args: AddLiquidityOneSidePreciseIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AddLiquidityOneSidePreciseIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_one_side_precise_ix(
    keys: AddLiquidityOneSidePreciseKeys,
    args: AddLiquidityOneSidePreciseIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_one_side_precise_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_one_side_precise_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    args: AddLiquidityOneSidePreciseIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityOneSidePreciseKeys = accounts.into();
    let ix = add_liquidity_one_side_precise_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_one_side_precise_invoke(
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    args: AddLiquidityOneSidePreciseIxArgs,
) -> ProgramResult {
    add_liquidity_one_side_precise_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn add_liquidity_one_side_precise_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    args: AddLiquidityOneSidePreciseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityOneSidePreciseKeys = accounts.into();
    let ix = add_liquidity_one_side_precise_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_one_side_precise_invoke_signed(
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    args: AddLiquidityOneSidePreciseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_one_side_precise_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_one_side_precise_verify_account_keys(
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    keys: AddLiquidityOneSidePreciseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
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
pub fn add_liquidity_one_side_precise_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePreciseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token,
        accounts.reserve,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_precise_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePreciseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_precise_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePreciseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_one_side_precise_verify_writable_privileges(accounts)?;
    add_liquidity_one_side_precise_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityOneSidePrecise2Accounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSidePrecise2Keys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityOneSidePrecise2Accounts<'_, '_>>
for AddLiquidityOneSidePrecise2Keys {
    fn from(accounts: AddLiquidityOneSidePrecise2Accounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token: *accounts.user_token.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityOneSidePrecise2Keys>
for [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityOneSidePrecise2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
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
impl From<[Pubkey; ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_ACCOUNTS_LEN]>
for AddLiquidityOneSidePrecise2Keys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token: pubkeys[3],
            reserve: pubkeys[4],
            token_mint: pubkeys[5],
            sender: pubkeys[6],
            token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<AddLiquidityOneSidePrecise2Accounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityOneSidePrecise2Accounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_ACCOUNTS_LEN]>
for AddLiquidityOneSidePrecise2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token: &arr[3],
            reserve: &arr[4],
            token_mint: &arr[5],
            sender: &arr[6],
            token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_DISCM: [u8; 8usize] = [
    33, 51, 163, 201, 117, 98, 125, 231,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityOneSidePrecise2IxArgs {
    pub liquidity_parameter: AddLiquiditySingleSidePreciseParameter2,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSidePrecise2IxData(pub AddLiquidityOneSidePrecise2IxArgs);
impl From<AddLiquidityOneSidePrecise2IxArgs> for AddLiquidityOneSidePrecise2IxData {
    fn from(args: AddLiquidityOneSidePrecise2IxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityOneSidePrecise2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_parameter = if reader.is_empty() {
            Default::default()
        } else {
            <AddLiquiditySingleSidePreciseParameter2>::deserialize(&mut reader)?
        };
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(AddLiquidityOneSidePrecise2IxArgs {
                liquidity_parameter,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_parameter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_one_side_precise2_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityOneSidePrecise2Keys,
    args: AddLiquidityOneSidePrecise2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_PRECISE2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AddLiquidityOneSidePrecise2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_one_side_precise2_ix(
    keys: AddLiquidityOneSidePrecise2Keys,
    args: AddLiquidityOneSidePrecise2IxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_one_side_precise2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_one_side_precise2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSidePrecise2Accounts<'_, '_>,
    args: AddLiquidityOneSidePrecise2IxArgs,
) -> ProgramResult {
    let keys: AddLiquidityOneSidePrecise2Keys = accounts.into();
    let ix = add_liquidity_one_side_precise2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_one_side_precise2_invoke(
    accounts: AddLiquidityOneSidePrecise2Accounts<'_, '_>,
    args: AddLiquidityOneSidePrecise2IxArgs,
) -> ProgramResult {
    add_liquidity_one_side_precise2_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn add_liquidity_one_side_precise2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSidePrecise2Accounts<'_, '_>,
    args: AddLiquidityOneSidePrecise2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityOneSidePrecise2Keys = accounts.into();
    let ix = add_liquidity_one_side_precise2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_one_side_precise2_invoke_signed(
    accounts: AddLiquidityOneSidePrecise2Accounts<'_, '_>,
    args: AddLiquidityOneSidePrecise2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_one_side_precise2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_one_side_precise2_verify_account_keys(
    accounts: AddLiquidityOneSidePrecise2Accounts<'_, '_>,
    keys: AddLiquidityOneSidePrecise2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.sender.key, keys.sender),
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
pub fn add_liquidity_one_side_precise2_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePrecise2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token,
        accounts.reserve,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_precise2_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePrecise2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_precise2_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePrecise2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_one_side_precise2_verify_writable_privileges(accounts)?;
    add_liquidity_one_side_precise2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct CancelLimitOrderAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub limit_order: &'me AccountInfo<'info>,
    pub owner_token_x: &'me AccountInfo<'info>,
    pub owner_token_y: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelLimitOrderKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub limit_order: Pubkey,
    pub owner_token_x: Pubkey,
    pub owner_token_y: Pubkey,
    pub owner: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CancelLimitOrderAccounts<'_, '_>> for CancelLimitOrderKeys {
    fn from(accounts: CancelLimitOrderAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            limit_order: *accounts.limit_order.key,
            owner_token_x: *accounts.owner_token_x.key,
            owner_token_y: *accounts.owner_token_y.key,
            owner: *accounts.owner.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CancelLimitOrderKeys> for [AccountMeta; CANCEL_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.limit_order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
impl From<[Pubkey; CANCEL_LIMIT_ORDER_IX_ACCOUNTS_LEN]> for CancelLimitOrderKeys {
    fn from(pubkeys: [Pubkey; CANCEL_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            token_x_mint: pubkeys[4],
            token_y_mint: pubkeys[5],
            limit_order: pubkeys[6],
            owner_token_x: pubkeys[7],
            owner_token_y: pubkeys[8],
            owner: pubkeys[9],
            token_x_program: pubkeys[10],
            token_y_program: pubkeys[11],
            memo_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<CancelLimitOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.limit_order.clone(),
            accounts.owner_token_x.clone(),
            accounts.owner_token_y.clone(),
            accounts.owner.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for CancelLimitOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            token_x_mint: &arr[4],
            token_y_mint: &arr[5],
            limit_order: &arr[6],
            owner_token_x: &arr[7],
            owner_token_y: &arr[8],
            owner: &arr[9],
            token_x_program: &arr[10],
            token_y_program: &arr[11],
            memo_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const CANCEL_LIMIT_ORDER_IX_DISCM: [u8; 8usize] = [
    132, 156, 132, 31, 67, 40, 232, 97,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelLimitOrderIxArgs {
    pub bins: Vec<i32>,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelLimitOrderIxData(pub CancelLimitOrderIxArgs);
impl From<CancelLimitOrderIxArgs> for CancelLimitOrderIxData {
    fn from(args: CancelLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl CancelLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bins: Vec<i32> = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(CancelLimitOrderIxArgs {
                bins,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_LIMIT_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bins, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelLimitOrderKeys,
    args: CancelLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelLimitOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_limit_order_ix(
    keys: CancelLimitOrderKeys,
    args: CancelLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    cancel_limit_order_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn cancel_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelLimitOrderAccounts<'_, '_>,
    args: CancelLimitOrderIxArgs,
) -> ProgramResult {
    let keys: CancelLimitOrderKeys = accounts.into();
    let ix = cancel_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_limit_order_invoke(
    accounts: CancelLimitOrderAccounts<'_, '_>,
    args: CancelLimitOrderIxArgs,
) -> ProgramResult {
    cancel_limit_order_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn cancel_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelLimitOrderAccounts<'_, '_>,
    args: CancelLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelLimitOrderKeys = accounts.into();
    let ix = cancel_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_limit_order_invoke_signed(
    accounts: CancelLimitOrderAccounts<'_, '_>,
    args: CancelLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_limit_order_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_limit_order_verify_account_keys(
    accounts: CancelLimitOrderAccounts<'_, '_>,
    keys: CancelLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.limit_order.key, keys.limit_order),
        (*accounts.owner_token_x.key, keys.owner_token_x),
        (*accounts.owner_token_y.key, keys.owner_token_y),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: CancelLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.limit_order,
        accounts.owner_token_x,
        accounts.owner_token_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: CancelLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_limit_order_verify_account_privileges<'me, 'info>(
    accounts: CancelLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_limit_order_verify_writable_privileges(accounts)?;
    cancel_limit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeKeys {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimFeeAccounts<'_, '_>> for ClaimFeeKeys {
    fn from(accounts: ClaimFeeAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimFeeKeys> for [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
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
impl From<[Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]> for ClaimFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            position: pubkeys[1],
            bin_array_lower: pubkeys[2],
            bin_array_upper: pubkeys[3],
            sender: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            user_token_x: pubkeys[7],
            user_token_y: pubkeys[8],
            token_x_mint: pubkeys[9],
            token_y_mint: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<ClaimFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]>
for ClaimFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            position: &arr[1],
            bin_array_lower: &arr[2],
            bin_array_upper: &arr[3],
            sender: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            user_token_x: &arr[7],
            user_token_y: &arr[8],
            token_x_mint: &arr[9],
            token_y_mint: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CLAIM_FEE_IX_DISCM: [u8; 8usize] = [169, 32, 79, 137, 136, 232, 70, 137];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeIxData;
impl ClaimFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimFeeIxData.try_to_vec()?,
    })
}
pub fn claim_fee_ix(keys: ClaimFeeKeys) -> std::io::Result<Instruction> {
    claim_fee_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn claim_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee_invoke(accounts: ClaimFeeAccounts<'_, '_>) -> ProgramResult {
    claim_fee_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn claim_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee_invoke_signed(
    accounts: ClaimFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, seeds)
}
pub fn claim_fee_verify_account_keys(
    accounts: ClaimFeeAccounts<'_, '_>,
    keys: ClaimFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
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
pub fn claim_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.position,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_x,
        accounts.user_token_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fee_verify_writable_privileges(accounts)?;
    claim_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEE2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFee2Accounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFee2Keys {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub sender: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimFee2Accounts<'_, '_>> for ClaimFee2Keys {
    fn from(accounts: ClaimFee2Accounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            sender: *accounts.sender.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimFee2Keys> for [AccountMeta; CLAIM_FEE2_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFee2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
impl From<[Pubkey; CLAIM_FEE2_IX_ACCOUNTS_LEN]> for ClaimFee2Keys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            position: pubkeys[1],
            sender: pubkeys[2],
            reserve_x: pubkeys[3],
            reserve_y: pubkeys[4],
            user_token_x: pubkeys[5],
            user_token_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            token_program_x: pubkeys[9],
            token_program_y: pubkeys[10],
            memo_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<ClaimFee2Accounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFee2Accounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.sender.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE2_IX_ACCOUNTS_LEN]>
for ClaimFee2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            position: &arr[1],
            sender: &arr[2],
            reserve_x: &arr[3],
            reserve_y: &arr[4],
            user_token_x: &arr[5],
            user_token_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            token_program_x: &arr[9],
            token_program_y: &arr[10],
            memo_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CLAIM_FEE2_IX_DISCM: [u8; 8usize] = [112, 191, 101, 171, 28, 144, 127, 187];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimFee2IxArgs {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFee2IxData(pub ClaimFee2IxArgs);
impl From<ClaimFee2IxArgs> for ClaimFee2IxData {
    fn from(args: ClaimFee2IxArgs) -> Self {
        Self(args)
    }
}
impl ClaimFee2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(ClaimFee2IxArgs {
                min_bin_id,
                max_bin_id,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.min_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fee2_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFee2Keys,
    args: ClaimFee2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE2_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimFee2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_fee2_ix(
    keys: ClaimFee2Keys,
    args: ClaimFee2IxArgs,
) -> std::io::Result<Instruction> {
    claim_fee2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn claim_fee2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFee2Accounts<'_, '_>,
    args: ClaimFee2IxArgs,
) -> ProgramResult {
    let keys: ClaimFee2Keys = accounts.into();
    let ix = claim_fee2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee2_invoke(
    accounts: ClaimFee2Accounts<'_, '_>,
    args: ClaimFee2IxArgs,
) -> ProgramResult {
    claim_fee2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn claim_fee2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFee2Accounts<'_, '_>,
    args: ClaimFee2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFee2Keys = accounts.into();
    let ix = claim_fee2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee2_invoke_signed(
    accounts: ClaimFee2Accounts<'_, '_>,
    args: ClaimFee2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee2_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn claim_fee2_verify_account_keys(
    accounts: ClaimFee2Accounts<'_, '_>,
    keys: ClaimFee2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.sender.key, keys.sender),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_fee2_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFee2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.position,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_x,
        accounts.user_token_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fee2_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFee2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fee2_verify_account_privileges<'me, 'info>(
    accounts: ClaimFee2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fee2_verify_writable_privileges(accounts)?;
    claim_fee2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REWARD_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ClaimRewardAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimRewardKeys {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimRewardAccounts<'_, '_>> for ClaimRewardKeys {
    fn from(accounts: ClaimRewardAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimRewardKeys> for [AccountMeta; CLAIM_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CLAIM_REWARD_IX_ACCOUNTS_LEN]> for ClaimRewardKeys {
    fn from(pubkeys: [Pubkey; CLAIM_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            position: pubkeys[1],
            bin_array_lower: pubkeys[2],
            bin_array_upper: pubkeys[3],
            sender: pubkeys[4],
            reward_vault: pubkeys[5],
            reward_mint: pubkeys[6],
            user_token_account: pubkeys[7],
            token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<ClaimRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN]>
for ClaimRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            position: &arr[1],
            bin_array_lower: &arr[2],
            bin_array_upper: &arr[3],
            sender: &arr[4],
            reward_vault: &arr[5],
            reward_mint: &arr[6],
            user_token_account: &arr[7],
            token_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const CLAIM_REWARD_IX_DISCM: [u8; 8usize] = [149, 95, 181, 242, 94, 90, 158, 162];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimRewardIxArgs {
    pub reward_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRewardIxData(pub ClaimRewardIxArgs);
impl From<ClaimRewardIxArgs> for ClaimRewardIxData {
    fn from(args: ClaimRewardIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ClaimRewardIxArgs { reward_index }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimRewardKeys,
    args: ClaimRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_reward_ix(
    keys: ClaimRewardKeys,
    args: ClaimRewardIxArgs,
) -> std::io::Result<Instruction> {
    claim_reward_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn claim_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimRewardAccounts<'_, '_>,
    args: ClaimRewardIxArgs,
) -> ProgramResult {
    let keys: ClaimRewardKeys = accounts.into();
    let ix = claim_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_reward_invoke(
    accounts: ClaimRewardAccounts<'_, '_>,
    args: ClaimRewardIxArgs,
) -> ProgramResult {
    claim_reward_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn claim_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimRewardAccounts<'_, '_>,
    args: ClaimRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimRewardKeys = accounts.into();
    let ix = claim_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_reward_invoke_signed(
    accounts: ClaimRewardAccounts<'_, '_>,
    args: ClaimRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_reward_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn claim_reward_verify_account_keys(
    accounts: ClaimRewardAccounts<'_, '_>,
    keys: ClaimRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
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
pub fn claim_reward_verify_writable_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.position,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.reward_vault,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_reward_verify_signer_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_reward_verify_account_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_reward_verify_writable_privileges(accounts)?;
    claim_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REWARD2_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimReward2Accounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimReward2Keys {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub sender: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimReward2Accounts<'_, '_>> for ClaimReward2Keys {
    fn from(accounts: ClaimReward2Accounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            sender: *accounts.sender.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimReward2Keys> for [AccountMeta; CLAIM_REWARD2_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimReward2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.memo_program,
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
impl From<[Pubkey; CLAIM_REWARD2_IX_ACCOUNTS_LEN]> for ClaimReward2Keys {
    fn from(pubkeys: [Pubkey; CLAIM_REWARD2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            position: pubkeys[1],
            sender: pubkeys[2],
            reward_vault: pubkeys[3],
            reward_mint: pubkeys[4],
            user_token_account: pubkeys[5],
            token_program: pubkeys[6],
            memo_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<ClaimReward2Accounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_REWARD2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimReward2Accounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.sender.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REWARD2_IX_ACCOUNTS_LEN]>
for ClaimReward2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_REWARD2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            position: &arr[1],
            sender: &arr[2],
            reward_vault: &arr[3],
            reward_mint: &arr[4],
            user_token_account: &arr[5],
            token_program: &arr[6],
            memo_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const CLAIM_REWARD2_IX_DISCM: [u8; 8usize] = [190, 3, 127, 119, 178, 87, 157, 183];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimReward2IxArgs {
    pub reward_index: u64,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimReward2IxData(pub ClaimReward2IxArgs);
impl From<ClaimReward2IxArgs> for ClaimReward2IxData {
    fn from(args: ClaimReward2IxArgs) -> Self {
        Self(args)
    }
}
impl ClaimReward2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REWARD2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(ClaimReward2IxArgs {
                reward_index,
                min_bin_id,
                max_bin_id,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REWARD2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_reward2_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimReward2Keys,
    args: ClaimReward2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_REWARD2_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimReward2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_reward2_ix(
    keys: ClaimReward2Keys,
    args: ClaimReward2IxArgs,
) -> std::io::Result<Instruction> {
    claim_reward2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn claim_reward2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimReward2Accounts<'_, '_>,
    args: ClaimReward2IxArgs,
) -> ProgramResult {
    let keys: ClaimReward2Keys = accounts.into();
    let ix = claim_reward2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_reward2_invoke(
    accounts: ClaimReward2Accounts<'_, '_>,
    args: ClaimReward2IxArgs,
) -> ProgramResult {
    claim_reward2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn claim_reward2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimReward2Accounts<'_, '_>,
    args: ClaimReward2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimReward2Keys = accounts.into();
    let ix = claim_reward2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_reward2_invoke_signed(
    accounts: ClaimReward2Accounts<'_, '_>,
    args: ClaimReward2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_reward2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn claim_reward2_verify_account_keys(
    accounts: ClaimReward2Accounts<'_, '_>,
    keys: ClaimReward2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.sender.key, keys.sender),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_reward2_verify_writable_privileges<'me, 'info>(
    accounts: ClaimReward2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.position,
        accounts.reward_vault,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_reward2_verify_signer_privileges<'me, 'info>(
    accounts: ClaimReward2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_reward2_verify_account_privileges<'me, 'info>(
    accounts: ClaimReward2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_reward2_verify_writable_privileges(accounts)?;
    claim_reward2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_BIN_ARRAY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseBinArrayAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseBinArrayKeys {
    pub lb_pair: Pubkey,
    pub bin_array: Pubkey,
    pub rent_receiver: Pubkey,
    pub signer: Pubkey,
}
impl From<CloseBinArrayAccounts<'_, '_>> for CloseBinArrayKeys {
    fn from(accounts: CloseBinArrayAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array: *accounts.bin_array.key,
            rent_receiver: *accounts.rent_receiver.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<CloseBinArrayKeys> for [AccountMeta; CLOSE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseBinArrayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: true,
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
impl From<[Pubkey; CLOSE_BIN_ARRAY_IX_ACCOUNTS_LEN]> for CloseBinArrayKeys {
    fn from(pubkeys: [Pubkey; CLOSE_BIN_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array: pubkeys[1],
            rent_receiver: pubkeys[2],
            signer: pubkeys[3],
        }
    }
}
impl<'info> From<CloseBinArrayAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseBinArrayAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array.clone(),
            accounts.rent_receiver.clone(),
            accounts.signer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_BIN_ARRAY_IX_ACCOUNTS_LEN]>
for CloseBinArrayAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_BIN_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array: &arr[1],
            rent_receiver: &arr[2],
            signer: &arr[3],
        }
    }
}
pub const CLOSE_BIN_ARRAY_IX_DISCM: [u8; 8usize] = [68, 174, 88, 80, 181, 204, 19, 224];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseBinArrayIxData;
impl CloseBinArrayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_BIN_ARRAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_BIN_ARRAY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_bin_array_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseBinArrayKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_BIN_ARRAY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseBinArrayIxData.try_to_vec()?,
    })
}
pub fn close_bin_array_ix(keys: CloseBinArrayKeys) -> std::io::Result<Instruction> {
    close_bin_array_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_bin_array_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseBinArrayAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseBinArrayKeys = accounts.into();
    let ix = close_bin_array_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_bin_array_invoke(accounts: CloseBinArrayAccounts<'_, '_>) -> ProgramResult {
    close_bin_array_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_bin_array_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseBinArrayAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseBinArrayKeys = accounts.into();
    let ix = close_bin_array_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_bin_array_invoke_signed(
    accounts: CloseBinArrayAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_bin_array_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, seeds)
}
pub fn close_bin_array_verify_account_keys(
    accounts: CloseBinArrayAccounts<'_, '_>,
    keys: CloseBinArrayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_bin_array_verify_writable_privileges<'me, 'info>(
    accounts: CloseBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bin_array, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_bin_array_verify_signer_privileges<'me, 'info>(
    accounts: CloseBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.rent_receiver, accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_bin_array_verify_account_privileges<'me, 'info>(
    accounts: CloseBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_bin_array_verify_writable_privileges(accounts)?;
    close_bin_array_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseClaimFeeOperatorAccountAccounts<'me, 'info> {
    pub claim_fee_operator: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseClaimFeeOperatorAccountKeys {
    pub claim_fee_operator: Pubkey,
    pub rent_receiver: Pubkey,
    pub signer: Pubkey,
}
impl From<CloseClaimFeeOperatorAccountAccounts<'_, '_>>
for CloseClaimFeeOperatorAccountKeys {
    fn from(accounts: CloseClaimFeeOperatorAccountAccounts) -> Self {
        Self {
            claim_fee_operator: *accounts.claim_fee_operator.key,
            rent_receiver: *accounts.rent_receiver.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<CloseClaimFeeOperatorAccountKeys>
for [AccountMeta; CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseClaimFeeOperatorAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.claim_fee_operator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseClaimFeeOperatorAccountKeys {
    fn from(
        pubkeys: [Pubkey; CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            claim_fee_operator: pubkeys[0],
            rent_receiver: pubkeys[1],
            signer: pubkeys[2],
        }
    }
}
impl<'info> From<CloseClaimFeeOperatorAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseClaimFeeOperatorAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.claim_fee_operator.clone(),
            accounts.rent_receiver.clone(),
            accounts.signer.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseClaimFeeOperatorAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            claim_fee_operator: &arr[0],
            rent_receiver: &arr[1],
            signer: &arr[2],
        }
    }
}
pub const CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    184, 213, 88, 31, 179, 101, 130, 36,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseClaimFeeOperatorAccountIxData;
impl CloseClaimFeeOperatorAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_claim_fee_operator_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseClaimFeeOperatorAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_CLAIM_FEE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseClaimFeeOperatorAccountIxData.try_to_vec()?,
    })
}
pub fn close_claim_fee_operator_account_ix(
    keys: CloseClaimFeeOperatorAccountKeys,
) -> std::io::Result<Instruction> {
    close_claim_fee_operator_account_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_claim_fee_operator_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseClaimFeeOperatorAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseClaimFeeOperatorAccountKeys = accounts.into();
    let ix = close_claim_fee_operator_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_claim_fee_operator_account_invoke(
    accounts: CloseClaimFeeOperatorAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_claim_fee_operator_account_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_claim_fee_operator_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseClaimFeeOperatorAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseClaimFeeOperatorAccountKeys = accounts.into();
    let ix = close_claim_fee_operator_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_claim_fee_operator_account_invoke_signed(
    accounts: CloseClaimFeeOperatorAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_claim_fee_operator_account_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_claim_fee_operator_account_verify_account_keys(
    accounts: CloseClaimFeeOperatorAccountAccounts<'_, '_>,
    keys: CloseClaimFeeOperatorAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.claim_fee_operator.key, keys.claim_fee_operator),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_claim_fee_operator_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.claim_fee_operator, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_claim_fee_operator_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_claim_fee_operator_account_verify_account_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_claim_fee_operator_account_verify_writable_privileges(accounts)?;
    close_claim_fee_operator_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_LIMIT_ORDER_IF_EMPTY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseLimitOrderIfEmptyAccounts<'me, 'info> {
    pub limit_order: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseLimitOrderIfEmptyKeys {
    pub limit_order: Pubkey,
    pub owner: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseLimitOrderIfEmptyAccounts<'_, '_>> for CloseLimitOrderIfEmptyKeys {
    fn from(accounts: CloseLimitOrderIfEmptyAccounts) -> Self {
        Self {
            limit_order: *accounts.limit_order.key,
            owner: *accounts.owner.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseLimitOrderIfEmptyKeys>
for [AccountMeta; CLOSE_LIMIT_ORDER_IF_EMPTY_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseLimitOrderIfEmptyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.limit_order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_LIMIT_ORDER_IF_EMPTY_IX_ACCOUNTS_LEN]>
for CloseLimitOrderIfEmptyKeys {
    fn from(pubkeys: [Pubkey; CLOSE_LIMIT_ORDER_IF_EMPTY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            limit_order: pubkeys[0],
            owner: pubkeys[1],
            rent_receiver: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseLimitOrderIfEmptyAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_LIMIT_ORDER_IF_EMPTY_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseLimitOrderIfEmptyAccounts<'_, 'info>) -> Self {
        [
            accounts.limit_order.clone(),
            accounts.owner.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_LIMIT_ORDER_IF_EMPTY_IX_ACCOUNTS_LEN]>
for CloseLimitOrderIfEmptyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_LIMIT_ORDER_IF_EMPTY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            limit_order: &arr[0],
            owner: &arr[1],
            rent_receiver: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLOSE_LIMIT_ORDER_IF_EMPTY_IX_DISCM: [u8; 8usize] = [
    57, 124, 36, 155, 126, 249, 93, 171,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseLimitOrderIfEmptyIxData;
impl CloseLimitOrderIfEmptyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_LIMIT_ORDER_IF_EMPTY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_LIMIT_ORDER_IF_EMPTY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_limit_order_if_empty_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseLimitOrderIfEmptyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_LIMIT_ORDER_IF_EMPTY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseLimitOrderIfEmptyIxData.try_to_vec()?,
    })
}
pub fn close_limit_order_if_empty_ix(
    keys: CloseLimitOrderIfEmptyKeys,
) -> std::io::Result<Instruction> {
    close_limit_order_if_empty_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_limit_order_if_empty_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseLimitOrderIfEmptyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseLimitOrderIfEmptyKeys = accounts.into();
    let ix = close_limit_order_if_empty_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_limit_order_if_empty_invoke(
    accounts: CloseLimitOrderIfEmptyAccounts<'_, '_>,
) -> ProgramResult {
    close_limit_order_if_empty_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_limit_order_if_empty_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseLimitOrderIfEmptyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseLimitOrderIfEmptyKeys = accounts.into();
    let ix = close_limit_order_if_empty_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_limit_order_if_empty_invoke_signed(
    accounts: CloseLimitOrderIfEmptyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_limit_order_if_empty_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_limit_order_if_empty_verify_account_keys(
    accounts: CloseLimitOrderIfEmptyAccounts<'_, '_>,
    keys: CloseLimitOrderIfEmptyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.limit_order.key, keys.limit_order),
        (*accounts.owner.key, keys.owner),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_limit_order_if_empty_verify_writable_privileges<'me, 'info>(
    accounts: CloseLimitOrderIfEmptyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.limit_order, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_limit_order_if_empty_verify_signer_privileges<'me, 'info>(
    accounts: CloseLimitOrderIfEmptyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_limit_order_if_empty_verify_account_privileges<'me, 'info>(
    accounts: CloseLimitOrderIfEmptyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_limit_order_if_empty_verify_writable_privileges(accounts)?;
    close_limit_order_if_empty_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseOperatorAccountAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseOperatorAccountKeys {
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub rent_receiver: Pubkey,
}
impl From<CloseOperatorAccountAccounts<'_, '_>> for CloseOperatorAccountKeys {
    fn from(accounts: CloseOperatorAccountAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            rent_receiver: *accounts.rent_receiver.key,
        }
    }
}
impl From<CloseOperatorAccountKeys>
for [AccountMeta; CLOSE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseOperatorAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseOperatorAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            signer: pubkeys[1],
            rent_receiver: pubkeys[2],
        }
    }
}
impl<'info> From<CloseOperatorAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseOperatorAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.rent_receiver.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseOperatorAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            signer: &arr[1],
            rent_receiver: &arr[2],
        }
    }
}
pub const CLOSE_OPERATOR_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    171, 9, 213, 74, 120, 23, 3, 29,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseOperatorAccountIxData;
impl CloseOperatorAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_OPERATOR_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_OPERATOR_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_operator_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseOperatorAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseOperatorAccountIxData.try_to_vec()?,
    })
}
pub fn close_operator_account_ix(
    keys: CloseOperatorAccountKeys,
) -> std::io::Result<Instruction> {
    close_operator_account_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_operator_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseOperatorAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseOperatorAccountKeys = accounts.into();
    let ix = close_operator_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_operator_account_invoke(
    accounts: CloseOperatorAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_operator_account_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_operator_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseOperatorAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseOperatorAccountKeys = accounts.into();
    let ix = close_operator_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_operator_account_invoke_signed(
    accounts: CloseOperatorAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_operator_account_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_operator_account_verify_account_keys(
    accounts: CloseOperatorAccountAccounts<'_, '_>,
    keys: CloseOperatorAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.rent_receiver.key, keys.rent_receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_operator_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.operator, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_operator_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_operator_account_verify_account_privileges<'me, 'info>(
    accounts: CloseOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_operator_account_verify_writable_privileges(accounts)?;
    close_operator_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClosePositionAccounts<'_, '_>> for ClosePositionKeys {
    fn from(accounts: ClosePositionAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClosePositionKeys> for [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_lower: pubkeys[2],
            bin_array_upper: pubkeys[3],
            sender: pubkeys[4],
            rent_receiver: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<ClosePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]>
for ClosePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_lower: &arr[2],
            bin_array_upper: &arr[3],
            sender: &arr[4],
            rent_receiver: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const CLOSE_POSITION_IX_DISCM: [u8; 8usize] = [123, 134, 81, 0, 49, 68, 98, 98];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionIxData;
impl ClosePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_position_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePositionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePositionIxData.try_to_vec()?,
    })
}
pub fn close_position_ix(keys: ClosePositionKeys) -> std::io::Result<Instruction> {
    close_position_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_position_invoke(accounts: ClosePositionAccounts<'_, '_>) -> ProgramResult {
    close_position_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_position_invoke_signed(
    accounts: ClosePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, seeds)
}
pub fn close_position_verify_account_keys(
    accounts: ClosePositionAccounts<'_, '_>,
    keys: ClosePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_position_verify_writable_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.rent_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_position_verify_signer_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_position_verify_account_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_position_verify_writable_privileges(accounts)?;
    close_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION2_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ClosePosition2Accounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePosition2Keys {
    pub position: Pubkey,
    pub sender: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClosePosition2Accounts<'_, '_>> for ClosePosition2Keys {
    fn from(accounts: ClosePosition2Accounts) -> Self {
        Self {
            position: *accounts.position.key,
            sender: *accounts.sender.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClosePosition2Keys> for [AccountMeta; CLOSE_POSITION2_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePosition2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_POSITION2_IX_ACCOUNTS_LEN]> for ClosePosition2Keys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            sender: pubkeys[1],
            rent_receiver: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<ClosePosition2Accounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePosition2Accounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.sender.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION2_IX_ACCOUNTS_LEN]>
for ClosePosition2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            sender: &arr[1],
            rent_receiver: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLOSE_POSITION2_IX_DISCM: [u8; 8usize] = [174, 90, 35, 115, 186, 40, 147, 226];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePosition2IxData;
impl ClosePosition2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_position2_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePosition2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePosition2IxData.try_to_vec()?,
    })
}
pub fn close_position2_ix(keys: ClosePosition2Keys) -> std::io::Result<Instruction> {
    close_position2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_position2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePosition2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePosition2Keys = accounts.into();
    let ix = close_position2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_position2_invoke(
    accounts: ClosePosition2Accounts<'_, '_>,
) -> ProgramResult {
    close_position2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_position2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePosition2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePosition2Keys = accounts.into();
    let ix = close_position2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_position2_invoke_signed(
    accounts: ClosePosition2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position2_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, seeds)
}
pub fn close_position2_verify_account_keys(
    accounts: ClosePosition2Accounts<'_, '_>,
    keys: ClosePosition2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.sender.key, keys.sender),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_position2_verify_writable_privileges<'me, 'info>(
    accounts: ClosePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_position2_verify_signer_privileges<'me, 'info>(
    accounts: ClosePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_position2_verify_account_privileges<'me, 'info>(
    accounts: ClosePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_position2_verify_writable_privileges(accounts)?;
    close_position2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_IF_EMPTY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionIfEmptyAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionIfEmptyKeys {
    pub position: Pubkey,
    pub sender: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClosePositionIfEmptyAccounts<'_, '_>> for ClosePositionIfEmptyKeys {
    fn from(accounts: ClosePositionIfEmptyAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            sender: *accounts.sender.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClosePositionIfEmptyKeys>
for [AccountMeta; CLOSE_POSITION_IF_EMPTY_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionIfEmptyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_POSITION_IF_EMPTY_IX_ACCOUNTS_LEN]>
for ClosePositionIfEmptyKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IF_EMPTY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            sender: pubkeys[1],
            rent_receiver: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<ClosePositionIfEmptyAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_IF_EMPTY_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionIfEmptyAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.sender.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IF_EMPTY_IX_ACCOUNTS_LEN]>
for ClosePositionIfEmptyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IF_EMPTY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            sender: &arr[1],
            rent_receiver: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLOSE_POSITION_IF_EMPTY_IX_DISCM: [u8; 8usize] = [
    59, 124, 212, 118, 91, 152, 110, 157,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionIfEmptyIxData;
impl ClosePositionIfEmptyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION_IF_EMPTY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_IF_EMPTY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_position_if_empty_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePositionIfEmptyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION_IF_EMPTY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePositionIfEmptyIxData.try_to_vec()?,
    })
}
pub fn close_position_if_empty_ix(
    keys: ClosePositionIfEmptyKeys,
) -> std::io::Result<Instruction> {
    close_position_if_empty_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_position_if_empty_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionIfEmptyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePositionIfEmptyKeys = accounts.into();
    let ix = close_position_if_empty_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_position_if_empty_invoke(
    accounts: ClosePositionIfEmptyAccounts<'_, '_>,
) -> ProgramResult {
    close_position_if_empty_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_position_if_empty_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionIfEmptyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePositionIfEmptyKeys = accounts.into();
    let ix = close_position_if_empty_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_position_if_empty_invoke_signed(
    accounts: ClosePositionIfEmptyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position_if_empty_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_position_if_empty_verify_account_keys(
    accounts: ClosePositionIfEmptyAccounts<'_, '_>,
    keys: ClosePositionIfEmptyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.sender.key, keys.sender),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_position_if_empty_verify_writable_privileges<'me, 'info>(
    accounts: ClosePositionIfEmptyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_position_if_empty_verify_signer_privileges<'me, 'info>(
    accounts: ClosePositionIfEmptyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_position_if_empty_verify_account_privileges<'me, 'info>(
    accounts: ClosePositionIfEmptyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_position_if_empty_verify_writable_privileges(accounts)?;
    close_position_if_empty_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ClosePresetParameterAccounts<'me, 'info> {
    pub preset_parameter: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePresetParameterKeys {
    pub preset_parameter: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub rent_receiver: Pubkey,
}
impl From<ClosePresetParameterAccounts<'_, '_>> for ClosePresetParameterKeys {
    fn from(accounts: ClosePresetParameterAccounts) -> Self {
        Self {
            preset_parameter: *accounts.preset_parameter.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            rent_receiver: *accounts.rent_receiver.key,
        }
    }
}
impl From<ClosePresetParameterKeys>
for [AccountMeta; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePresetParameterKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.preset_parameter,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]>
for ClosePresetParameterKeys {
    fn from(pubkeys: [Pubkey; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            preset_parameter: pubkeys[0],
            operator: pubkeys[1],
            signer: pubkeys[2],
            rent_receiver: pubkeys[3],
        }
    }
}
impl<'info> From<ClosePresetParameterAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePresetParameterAccounts<'_, 'info>) -> Self {
        [
            accounts.preset_parameter.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.rent_receiver.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]>
for ClosePresetParameterAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            preset_parameter: &arr[0],
            operator: &arr[1],
            signer: &arr[2],
            rent_receiver: &arr[3],
        }
    }
}
pub const CLOSE_PRESET_PARAMETER_IX_DISCM: [u8; 8usize] = [
    4, 148, 145, 100, 134, 26, 181, 61,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePresetParameterIxData;
impl ClosePresetParameterIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PRESET_PARAMETER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PRESET_PARAMETER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_preset_parameter_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePresetParameterKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePresetParameterIxData.try_to_vec()?,
    })
}
pub fn close_preset_parameter_ix(
    keys: ClosePresetParameterKeys,
) -> std::io::Result<Instruction> {
    close_preset_parameter_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_preset_parameter_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePresetParameterAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePresetParameterKeys = accounts.into();
    let ix = close_preset_parameter_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_preset_parameter_invoke(
    accounts: ClosePresetParameterAccounts<'_, '_>,
) -> ProgramResult {
    close_preset_parameter_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_preset_parameter_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePresetParameterAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePresetParameterKeys = accounts.into();
    let ix = close_preset_parameter_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_preset_parameter_invoke_signed(
    accounts: ClosePresetParameterAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_preset_parameter_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_preset_parameter_verify_account_keys(
    accounts: ClosePresetParameterAccounts<'_, '_>,
    keys: ClosePresetParameterKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.preset_parameter.key, keys.preset_parameter),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.rent_receiver.key, keys.rent_receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_preset_parameter_verify_writable_privileges<'me, 'info>(
    accounts: ClosePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.preset_parameter, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_preset_parameter_verify_signer_privileges<'me, 'info>(
    accounts: ClosePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_preset_parameter_verify_account_privileges<'me, 'info>(
    accounts: ClosePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_preset_parameter_verify_writable_privileges(accounts)?;
    close_preset_parameter_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_PRESET_PARAMETER2_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ClosePresetParameter2Accounts<'me, 'info> {
    pub preset_parameter: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePresetParameter2Keys {
    pub preset_parameter: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub rent_receiver: Pubkey,
}
impl From<ClosePresetParameter2Accounts<'_, '_>> for ClosePresetParameter2Keys {
    fn from(accounts: ClosePresetParameter2Accounts) -> Self {
        Self {
            preset_parameter: *accounts.preset_parameter.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            rent_receiver: *accounts.rent_receiver.key,
        }
    }
}
impl From<ClosePresetParameter2Keys>
for [AccountMeta; CLOSE_PRESET_PARAMETER2_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePresetParameter2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.preset_parameter,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_PRESET_PARAMETER2_IX_ACCOUNTS_LEN]>
for ClosePresetParameter2Keys {
    fn from(pubkeys: [Pubkey; CLOSE_PRESET_PARAMETER2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            preset_parameter: pubkeys[0],
            operator: pubkeys[1],
            signer: pubkeys[2],
            rent_receiver: pubkeys[3],
        }
    }
}
impl<'info> From<ClosePresetParameter2Accounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PRESET_PARAMETER2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePresetParameter2Accounts<'_, 'info>) -> Self {
        [
            accounts.preset_parameter.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.rent_receiver.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_PRESET_PARAMETER2_IX_ACCOUNTS_LEN]>
for ClosePresetParameter2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PRESET_PARAMETER2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            preset_parameter: &arr[0],
            operator: &arr[1],
            signer: &arr[2],
            rent_receiver: &arr[3],
        }
    }
}
pub const CLOSE_PRESET_PARAMETER2_IX_DISCM: [u8; 8usize] = [
    39, 25, 95, 107, 116, 17, 115, 28,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePresetParameter2IxData;
impl ClosePresetParameter2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PRESET_PARAMETER2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PRESET_PARAMETER2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_preset_parameter2_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePresetParameter2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PRESET_PARAMETER2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePresetParameter2IxData.try_to_vec()?,
    })
}
pub fn close_preset_parameter2_ix(
    keys: ClosePresetParameter2Keys,
) -> std::io::Result<Instruction> {
    close_preset_parameter2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_preset_parameter2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePresetParameter2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePresetParameter2Keys = accounts.into();
    let ix = close_preset_parameter2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_preset_parameter2_invoke(
    accounts: ClosePresetParameter2Accounts<'_, '_>,
) -> ProgramResult {
    close_preset_parameter2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_preset_parameter2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePresetParameter2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePresetParameter2Keys = accounts.into();
    let ix = close_preset_parameter2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_preset_parameter2_invoke_signed(
    accounts: ClosePresetParameter2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_preset_parameter2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_preset_parameter2_verify_account_keys(
    accounts: ClosePresetParameter2Accounts<'_, '_>,
    keys: ClosePresetParameter2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.preset_parameter.key, keys.preset_parameter),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.rent_receiver.key, keys.rent_receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_preset_parameter2_verify_writable_privileges<'me, 'info>(
    accounts: ClosePresetParameter2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.preset_parameter, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_preset_parameter2_verify_signer_privileges<'me, 'info>(
    accounts: ClosePresetParameter2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_preset_parameter2_verify_account_privileges<'me, 'info>(
    accounts: ClosePresetParameter2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_preset_parameter2_verify_writable_privileges(accounts)?;
    close_preset_parameter2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_TOKEN_BADGE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseTokenBadgeAccounts<'me, 'info> {
    pub token_badge: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseTokenBadgeKeys {
    pub token_badge: Pubkey,
    pub rent_receiver: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
}
impl From<CloseTokenBadgeAccounts<'_, '_>> for CloseTokenBadgeKeys {
    fn from(accounts: CloseTokenBadgeAccounts) -> Self {
        Self {
            token_badge: *accounts.token_badge.key,
            rent_receiver: *accounts.rent_receiver.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<CloseTokenBadgeKeys> for [AccountMeta; CLOSE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseTokenBadgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_badge,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
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
impl From<[Pubkey; CLOSE_TOKEN_BADGE_IX_ACCOUNTS_LEN]> for CloseTokenBadgeKeys {
    fn from(pubkeys: [Pubkey; CLOSE_TOKEN_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_badge: pubkeys[0],
            rent_receiver: pubkeys[1],
            operator: pubkeys[2],
            signer: pubkeys[3],
        }
    }
}
impl<'info> From<CloseTokenBadgeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseTokenBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.token_badge.clone(),
            accounts.rent_receiver.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_TOKEN_BADGE_IX_ACCOUNTS_LEN]>
for CloseTokenBadgeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_TOKEN_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_badge: &arr[0],
            rent_receiver: &arr[1],
            operator: &arr[2],
            signer: &arr[3],
        }
    }
}
pub const CLOSE_TOKEN_BADGE_IX_DISCM: [u8; 8usize] = [
    108, 146, 86, 110, 179, 254, 10, 104,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseTokenBadgeIxData;
impl CloseTokenBadgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_TOKEN_BADGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_TOKEN_BADGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_token_badge_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseTokenBadgeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_TOKEN_BADGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseTokenBadgeIxData.try_to_vec()?,
    })
}
pub fn close_token_badge_ix(keys: CloseTokenBadgeKeys) -> std::io::Result<Instruction> {
    close_token_badge_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn close_token_badge_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenBadgeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseTokenBadgeKeys = accounts.into();
    let ix = close_token_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_token_badge_invoke(
    accounts: CloseTokenBadgeAccounts<'_, '_>,
) -> ProgramResult {
    close_token_badge_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn close_token_badge_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseTokenBadgeKeys = accounts.into();
    let ix = close_token_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_token_badge_invoke_signed(
    accounts: CloseTokenBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_token_badge_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, seeds)
}
pub fn close_token_badge_verify_account_keys(
    accounts: CloseTokenBadgeAccounts<'_, '_>,
    keys: CloseTokenBadgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_badge.key, keys.token_badge),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_token_badge_verify_writable_privileges<'me, 'info>(
    accounts: CloseTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_badge, accounts.rent_receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_token_badge_verify_signer_privileges<'me, 'info>(
    accounts: CloseTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_token_badge_verify_account_privileges<'me, 'info>(
    accounts: CloseTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_token_badge_verify_writable_privileges(accounts)?;
    close_token_badge_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateOperatorAccountAccounts<'me, 'info> {
    pub operator: &'me AccountInfo<'info>,
    pub whitelisted_signer: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOperatorAccountKeys {
    pub operator: Pubkey,
    pub whitelisted_signer: Pubkey,
    pub signer: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateOperatorAccountAccounts<'_, '_>> for CreateOperatorAccountKeys {
    fn from(accounts: CreateOperatorAccountAccounts) -> Self {
        Self {
            operator: *accounts.operator.key,
            whitelisted_signer: *accounts.whitelisted_signer.key,
            signer: *accounts.signer.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateOperatorAccountKeys>
for [AccountMeta; CREATE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOperatorAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.whitelisted_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
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
impl From<[Pubkey; CREATE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateOperatorAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            operator: pubkeys[0],
            whitelisted_signer: pubkeys[1],
            signer: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateOperatorAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOperatorAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.operator.clone(),
            accounts.whitelisted_signer.clone(),
            accounts.signer.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateOperatorAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            operator: &arr[0],
            whitelisted_signer: &arr[1],
            signer: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_OPERATOR_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    221, 64, 246, 149, 240, 153, 229, 163,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOperatorAccountIxArgs {
    pub permission: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOperatorAccountIxData(pub CreateOperatorAccountIxArgs);
impl From<CreateOperatorAccountIxArgs> for CreateOperatorAccountIxData {
    fn from(args: CreateOperatorAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateOperatorAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_OPERATOR_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let permission: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateOperatorAccountIxArgs {
                permission,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_OPERATOR_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.permission, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_operator_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOperatorAccountKeys,
    args: CreateOperatorAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_OPERATOR_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateOperatorAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_operator_account_ix(
    keys: CreateOperatorAccountKeys,
    args: CreateOperatorAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_operator_account_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn create_operator_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOperatorAccountAccounts<'_, '_>,
    args: CreateOperatorAccountIxArgs,
) -> ProgramResult {
    let keys: CreateOperatorAccountKeys = accounts.into();
    let ix = create_operator_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_operator_account_invoke(
    accounts: CreateOperatorAccountAccounts<'_, '_>,
    args: CreateOperatorAccountIxArgs,
) -> ProgramResult {
    create_operator_account_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn create_operator_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOperatorAccountAccounts<'_, '_>,
    args: CreateOperatorAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOperatorAccountKeys = accounts.into();
    let ix = create_operator_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_operator_account_invoke_signed(
    accounts: CreateOperatorAccountAccounts<'_, '_>,
    args: CreateOperatorAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_operator_account_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_operator_account_verify_account_keys(
    accounts: CreateOperatorAccountAccounts<'_, '_>,
    keys: CreateOperatorAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.operator.key, keys.operator),
        (*accounts.whitelisted_signer.key, keys.whitelisted_signer),
        (*accounts.signer.key, keys.signer),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_operator_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.operator, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_operator_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_operator_account_verify_account_privileges<'me, 'info>(
    accounts: CreateOperatorAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_operator_account_verify_writable_privileges(accounts)?;
    create_operator_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct DecreasePositionLengthAccounts<'me, 'info> {
    pub rent_receiver: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreasePositionLengthKeys {
    pub rent_receiver: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DecreasePositionLengthAccounts<'_, '_>> for DecreasePositionLengthKeys {
    fn from(accounts: DecreasePositionLengthAccounts) -> Self {
        Self {
            rent_receiver: *accounts.rent_receiver.key,
            position: *accounts.position.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DecreasePositionLengthKeys>
for [AccountMeta; DECREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreasePositionLengthKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; DECREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN]>
for DecreasePositionLengthKeys {
    fn from(pubkeys: [Pubkey; DECREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            rent_receiver: pubkeys[0],
            position: pubkeys[1],
            owner: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<DecreasePositionLengthAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreasePositionLengthAccounts<'_, 'info>) -> Self {
        [
            accounts.rent_receiver.clone(),
            accounts.position.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DECREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN]>
for DecreasePositionLengthAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DECREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            rent_receiver: &arr[0],
            position: &arr[1],
            owner: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const DECREASE_POSITION_LENGTH_IX_DISCM: [u8; 8usize] = [
    194, 219, 136, 32, 25, 96, 105, 37,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionLengthIxArgs {
    pub length_to_remove: u16,
    pub side: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePositionLengthIxData(pub DecreasePositionLengthIxArgs);
impl From<DecreasePositionLengthIxArgs> for DecreasePositionLengthIxData {
    fn from(args: DecreasePositionLengthIxArgs) -> Self {
        Self(args)
    }
}
impl DecreasePositionLengthIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION_LENGTH_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let length_to_remove: u16 = crate::borsh_de_or_default(&mut reader)?;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DecreasePositionLengthIxArgs {
                length_to_remove,
                side,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION_LENGTH_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.length_to_remove, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.side, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_position_length_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreasePositionLengthKeys,
    args: DecreasePositionLengthIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreasePositionLengthIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_position_length_ix(
    keys: DecreasePositionLengthKeys,
    args: DecreasePositionLengthIxArgs,
) -> std::io::Result<Instruction> {
    decrease_position_length_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn decrease_position_length_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionLengthAccounts<'_, '_>,
    args: DecreasePositionLengthIxArgs,
) -> ProgramResult {
    let keys: DecreasePositionLengthKeys = accounts.into();
    let ix = decrease_position_length_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_position_length_invoke(
    accounts: DecreasePositionLengthAccounts<'_, '_>,
    args: DecreasePositionLengthIxArgs,
) -> ProgramResult {
    decrease_position_length_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn decrease_position_length_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreasePositionLengthAccounts<'_, '_>,
    args: DecreasePositionLengthIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreasePositionLengthKeys = accounts.into();
    let ix = decrease_position_length_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_position_length_invoke_signed(
    accounts: DecreasePositionLengthAccounts<'_, '_>,
    args: DecreasePositionLengthIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_position_length_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_position_length_verify_account_keys(
    accounts: DecreasePositionLengthAccounts<'_, '_>,
    keys: DecreasePositionLengthKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.position.key, keys.position),
        (*accounts.owner.key, keys.owner),
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
pub fn decrease_position_length_verify_writable_privileges<'me, 'info>(
    accounts: DecreasePositionLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.rent_receiver, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_position_length_verify_signer_privileges<'me, 'info>(
    accounts: DecreasePositionLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_position_length_verify_account_privileges<'me, 'info>(
    accounts: DecreasePositionLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_position_length_verify_writable_privileges(accounts)?;
    decrease_position_length_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct ForIdlTypeGenerationDoNotCallAccounts<'me, 'info> {
    pub dummy_zc_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ForIdlTypeGenerationDoNotCallKeys {
    pub dummy_zc_account: Pubkey,
}
impl From<ForIdlTypeGenerationDoNotCallAccounts<'_, '_>>
for ForIdlTypeGenerationDoNotCallKeys {
    fn from(accounts: ForIdlTypeGenerationDoNotCallAccounts) -> Self {
        Self {
            dummy_zc_account: *accounts.dummy_zc_account.key,
        }
    }
}
impl From<ForIdlTypeGenerationDoNotCallKeys>
for [AccountMeta; FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_ACCOUNTS_LEN] {
    fn from(keys: ForIdlTypeGenerationDoNotCallKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dummy_zc_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_ACCOUNTS_LEN]>
for ForIdlTypeGenerationDoNotCallKeys {
    fn from(
        pubkeys: [Pubkey; FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            dummy_zc_account: pubkeys[0],
        }
    }
}
impl<'info> From<ForIdlTypeGenerationDoNotCallAccounts<'_, 'info>>
for [AccountInfo<'info>; FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ForIdlTypeGenerationDoNotCallAccounts<'_, 'info>) -> Self {
        [accounts.dummy_zc_account.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_ACCOUNTS_LEN]>
for ForIdlTypeGenerationDoNotCallAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self { dummy_zc_account: &arr[0] }
    }
}
pub const FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_DISCM: [u8; 8usize] = [
    180, 105, 69, 80, 95, 50, 73, 108,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ForIdlTypeGenerationDoNotCallIxArgs {
    pub ix: DummyIx,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ForIdlTypeGenerationDoNotCallIxData(pub ForIdlTypeGenerationDoNotCallIxArgs);
impl From<ForIdlTypeGenerationDoNotCallIxArgs> for ForIdlTypeGenerationDoNotCallIxData {
    fn from(args: ForIdlTypeGenerationDoNotCallIxArgs) -> Self {
        Self(args)
    }
}
impl ForIdlTypeGenerationDoNotCallIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let ix = if reader.is_empty() {
            Default::default()
        } else {
            <DummyIx>::deserialize(&mut reader)?
        };
        Ok(
            Self(ForIdlTypeGenerationDoNotCallIxArgs {
                ix,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.ix, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn for_idl_type_generation_do_not_call_ix_with_program_id(
    program_id: Pubkey,
    keys: ForIdlTypeGenerationDoNotCallKeys,
    args: ForIdlTypeGenerationDoNotCallIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FOR_IDL_TYPE_GENERATION_DO_NOT_CALL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ForIdlTypeGenerationDoNotCallIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn for_idl_type_generation_do_not_call_ix(
    keys: ForIdlTypeGenerationDoNotCallKeys,
    args: ForIdlTypeGenerationDoNotCallIxArgs,
) -> std::io::Result<Instruction> {
    for_idl_type_generation_do_not_call_ix_with_program_id(
        LB_CLMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn for_idl_type_generation_do_not_call_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ForIdlTypeGenerationDoNotCallAccounts<'_, '_>,
    args: ForIdlTypeGenerationDoNotCallIxArgs,
) -> ProgramResult {
    let keys: ForIdlTypeGenerationDoNotCallKeys = accounts.into();
    let ix = for_idl_type_generation_do_not_call_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn for_idl_type_generation_do_not_call_invoke(
    accounts: ForIdlTypeGenerationDoNotCallAccounts<'_, '_>,
    args: ForIdlTypeGenerationDoNotCallIxArgs,
) -> ProgramResult {
    for_idl_type_generation_do_not_call_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn for_idl_type_generation_do_not_call_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ForIdlTypeGenerationDoNotCallAccounts<'_, '_>,
    args: ForIdlTypeGenerationDoNotCallIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ForIdlTypeGenerationDoNotCallKeys = accounts.into();
    let ix = for_idl_type_generation_do_not_call_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn for_idl_type_generation_do_not_call_invoke_signed(
    accounts: ForIdlTypeGenerationDoNotCallAccounts<'_, '_>,
    args: ForIdlTypeGenerationDoNotCallIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    for_idl_type_generation_do_not_call_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn for_idl_type_generation_do_not_call_verify_account_keys(
    accounts: ForIdlTypeGenerationDoNotCallAccounts<'_, '_>,
    keys: ForIdlTypeGenerationDoNotCallKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.dummy_zc_account.key, keys.dummy_zc_account)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const FUND_REWARD_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct FundRewardAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FundRewardKeys {
    pub lb_pair: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub funder_token_account: Pubkey,
    pub funder: Pubkey,
    pub bin_array: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FundRewardAccounts<'_, '_>> for FundRewardKeys {
    fn from(accounts: FundRewardAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            funder_token_account: *accounts.funder_token_account.key,
            funder: *accounts.funder.key,
            bin_array: *accounts.bin_array.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FundRewardKeys> for [AccountMeta; FUND_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: FundRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
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
impl From<[Pubkey; FUND_REWARD_IX_ACCOUNTS_LEN]> for FundRewardKeys {
    fn from(pubkeys: [Pubkey; FUND_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reward_vault: pubkeys[1],
            reward_mint: pubkeys[2],
            funder_token_account: pubkeys[3],
            funder: pubkeys[4],
            bin_array: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<FundRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: FundRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.funder_token_account.clone(),
            accounts.funder.clone(),
            accounts.bin_array.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN]>
for FundRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            reward_vault: &arr[1],
            reward_mint: &arr[2],
            funder_token_account: &arr[3],
            funder: &arr[4],
            bin_array: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const FUND_REWARD_IX_DISCM: [u8; 8usize] = [188, 50, 249, 165, 93, 151, 38, 63];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundRewardIxArgs {
    pub reward_index: u64,
    pub amount: u64,
    pub carry_forward: bool,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundRewardIxData(pub FundRewardIxArgs);
impl From<FundRewardIxArgs> for FundRewardIxData {
    fn from(args: FundRewardIxArgs) -> Self {
        Self(args)
    }
}
impl FundRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUND_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let carry_forward: bool = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(FundRewardIxArgs {
                reward_index,
                amount,
                carry_forward,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUND_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.carry_forward, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fund_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: FundRewardKeys,
    args: FundRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FUND_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: FundRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fund_reward_ix(
    keys: FundRewardKeys,
    args: FundRewardIxArgs,
) -> std::io::Result<Instruction> {
    fund_reward_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn fund_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FundRewardAccounts<'_, '_>,
    args: FundRewardIxArgs,
) -> ProgramResult {
    let keys: FundRewardKeys = accounts.into();
    let ix = fund_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fund_reward_invoke(
    accounts: FundRewardAccounts<'_, '_>,
    args: FundRewardIxArgs,
) -> ProgramResult {
    fund_reward_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn fund_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FundRewardAccounts<'_, '_>,
    args: FundRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FundRewardKeys = accounts.into();
    let ix = fund_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fund_reward_invoke_signed(
    accounts: FundRewardAccounts<'_, '_>,
    args: FundRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fund_reward_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn fund_reward_verify_account_keys(
    accounts: FundRewardAccounts<'_, '_>,
    keys: FundRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.funder.key, keys.funder),
        (*accounts.bin_array.key, keys.bin_array),
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
pub fn fund_reward_verify_writable_privileges<'me, 'info>(
    accounts: FundRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reward_vault,
        accounts.funder_token_account,
        accounts.bin_array,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fund_reward_verify_signer_privileges<'me, 'info>(
    accounts: FundRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fund_reward_verify_account_privileges<'me, 'info>(
    accounts: FundRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fund_reward_verify_writable_privileges(accounts)?;
    fund_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GO_TO_A_BIN_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GoToABinAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub from_bin_array: &'me AccountInfo<'info>,
    pub to_bin_array: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoToABinKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub from_bin_array: Pubkey,
    pub to_bin_array: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<GoToABinAccounts<'_, '_>> for GoToABinKeys {
    fn from(accounts: GoToABinAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            from_bin_array: *accounts.from_bin_array.key,
            to_bin_array: *accounts.to_bin_array.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<GoToABinKeys> for [AccountMeta; GO_TO_A_BIN_IX_ACCOUNTS_LEN] {
    fn from(keys: GoToABinKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_bin_array,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_bin_array,
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
impl From<[Pubkey; GO_TO_A_BIN_IX_ACCOUNTS_LEN]> for GoToABinKeys {
    fn from(pubkeys: [Pubkey; GO_TO_A_BIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            from_bin_array: pubkeys[2],
            to_bin_array: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<GoToABinAccounts<'_, 'info>>
for [AccountInfo<'info>; GO_TO_A_BIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: GoToABinAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.from_bin_array.clone(),
            accounts.to_bin_array.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GO_TO_A_BIN_IX_ACCOUNTS_LEN]>
for GoToABinAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GO_TO_A_BIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            from_bin_array: &arr[2],
            to_bin_array: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const GO_TO_A_BIN_IX_DISCM: [u8; 8usize] = [146, 72, 174, 224, 40, 253, 84, 174];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoToABinIxArgs {
    pub bin_id: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GoToABinIxData(pub GoToABinIxArgs);
impl From<GoToABinIxArgs> for GoToABinIxData {
    fn from(args: GoToABinIxArgs) -> Self {
        Self(args)
    }
}
impl GoToABinIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GO_TO_A_BIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(GoToABinIxArgs { bin_id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GO_TO_A_BIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bin_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn go_to_a_bin_ix_with_program_id(
    program_id: Pubkey,
    keys: GoToABinKeys,
    args: GoToABinIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GO_TO_A_BIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: GoToABinIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn go_to_a_bin_ix(
    keys: GoToABinKeys,
    args: GoToABinIxArgs,
) -> std::io::Result<Instruction> {
    go_to_a_bin_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn go_to_a_bin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GoToABinAccounts<'_, '_>,
    args: GoToABinIxArgs,
) -> ProgramResult {
    let keys: GoToABinKeys = accounts.into();
    let ix = go_to_a_bin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn go_to_a_bin_invoke(
    accounts: GoToABinAccounts<'_, '_>,
    args: GoToABinIxArgs,
) -> ProgramResult {
    go_to_a_bin_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn go_to_a_bin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GoToABinAccounts<'_, '_>,
    args: GoToABinIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GoToABinKeys = accounts.into();
    let ix = go_to_a_bin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn go_to_a_bin_invoke_signed(
    accounts: GoToABinAccounts<'_, '_>,
    args: GoToABinIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    go_to_a_bin_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn go_to_a_bin_verify_account_keys(
    accounts: GoToABinAccounts<'_, '_>,
    keys: GoToABinKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.from_bin_array.key, keys.from_bin_array),
        (*accounts.to_bin_array.key, keys.to_bin_array),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn go_to_a_bin_verify_writable_privileges<'me, 'info>(
    accounts: GoToABinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn go_to_a_bin_verify_account_privileges<'me, 'info>(
    accounts: GoToABinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    go_to_a_bin_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseOracleLengthAccounts<'me, 'info> {
    pub oracle: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseOracleLengthKeys {
    pub oracle: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<IncreaseOracleLengthAccounts<'_, '_>> for IncreaseOracleLengthKeys {
    fn from(accounts: IncreaseOracleLengthAccounts) -> Self {
        Self {
            oracle: *accounts.oracle.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<IncreaseOracleLengthKeys>
for [AccountMeta; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseOracleLengthKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
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
impl From<[Pubkey; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN]>
for IncreaseOracleLengthKeys {
    fn from(pubkeys: [Pubkey; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            oracle: pubkeys[0],
            funder: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<IncreaseOracleLengthAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseOracleLengthAccounts<'_, 'info>) -> Self {
        [
            accounts.oracle.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN]>
for IncreaseOracleLengthAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            oracle: &arr[0],
            funder: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const INCREASE_ORACLE_LENGTH_IX_DISCM: [u8; 8usize] = [
    190, 61, 125, 87, 103, 79, 158, 173,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseOracleLengthIxArgs {
    pub length_to_add: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseOracleLengthIxData(pub IncreaseOracleLengthIxArgs);
impl From<IncreaseOracleLengthIxArgs> for IncreaseOracleLengthIxData {
    fn from(args: IncreaseOracleLengthIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseOracleLengthIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_ORACLE_LENGTH_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let length_to_add: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IncreaseOracleLengthIxArgs {
                length_to_add,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_ORACLE_LENGTH_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.length_to_add, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_oracle_length_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseOracleLengthKeys,
    args: IncreaseOracleLengthIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseOracleLengthIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_oracle_length_ix(
    keys: IncreaseOracleLengthKeys,
    args: IncreaseOracleLengthIxArgs,
) -> std::io::Result<Instruction> {
    increase_oracle_length_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn increase_oracle_length_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    args: IncreaseOracleLengthIxArgs,
) -> ProgramResult {
    let keys: IncreaseOracleLengthKeys = accounts.into();
    let ix = increase_oracle_length_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_oracle_length_invoke(
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    args: IncreaseOracleLengthIxArgs,
) -> ProgramResult {
    increase_oracle_length_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn increase_oracle_length_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    args: IncreaseOracleLengthIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseOracleLengthKeys = accounts.into();
    let ix = increase_oracle_length_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_oracle_length_invoke_signed(
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    args: IncreaseOracleLengthIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_oracle_length_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_oracle_length_verify_account_keys(
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    keys: IncreaseOracleLengthKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.oracle.key, keys.oracle),
        (*accounts.funder.key, keys.funder),
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
pub fn increase_oracle_length_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseOracleLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.oracle, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_oracle_length_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseOracleLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_oracle_length_verify_account_privileges<'me, 'info>(
    accounts: IncreaseOracleLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_oracle_length_verify_writable_privileges(accounts)?;
    increase_oracle_length_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct IncreasePositionLengthAccounts<'me, 'info> {
    pub funder: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreasePositionLengthKeys {
    pub funder: Pubkey,
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<IncreasePositionLengthAccounts<'_, '_>> for IncreasePositionLengthKeys {
    fn from(accounts: IncreasePositionLengthAccounts) -> Self {
        Self {
            funder: *accounts.funder.key,
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<IncreasePositionLengthKeys>
for [AccountMeta; INCREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreasePositionLengthKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; INCREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN]>
for IncreasePositionLengthKeys {
    fn from(pubkeys: [Pubkey; INCREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: pubkeys[0],
            lb_pair: pubkeys[1],
            position: pubkeys[2],
            owner: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<IncreasePositionLengthAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreasePositionLengthAccounts<'_, 'info>) -> Self {
        [
            accounts.funder.clone(),
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INCREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN]>
for IncreasePositionLengthAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            funder: &arr[0],
            lb_pair: &arr[1],
            position: &arr[2],
            owner: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const INCREASE_POSITION_LENGTH_IX_DISCM: [u8; 8usize] = [
    80, 83, 117, 211, 66, 13, 33, 149,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionLengthIxArgs {
    pub length_to_add: u16,
    pub side: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePositionLengthIxData(pub IncreasePositionLengthIxArgs);
impl From<IncreasePositionLengthIxArgs> for IncreasePositionLengthIxData {
    fn from(args: IncreasePositionLengthIxArgs) -> Self {
        Self(args)
    }
}
impl IncreasePositionLengthIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION_LENGTH_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let length_to_add: u16 = crate::borsh_de_or_default(&mut reader)?;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IncreasePositionLengthIxArgs {
                length_to_add,
                side,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION_LENGTH_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.length_to_add, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.side, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_position_length_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreasePositionLengthKeys,
    args: IncreasePositionLengthIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_POSITION_LENGTH_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreasePositionLengthIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_position_length_ix(
    keys: IncreasePositionLengthKeys,
    args: IncreasePositionLengthIxArgs,
) -> std::io::Result<Instruction> {
    increase_position_length_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn increase_position_length_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionLengthAccounts<'_, '_>,
    args: IncreasePositionLengthIxArgs,
) -> ProgramResult {
    let keys: IncreasePositionLengthKeys = accounts.into();
    let ix = increase_position_length_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_position_length_invoke(
    accounts: IncreasePositionLengthAccounts<'_, '_>,
    args: IncreasePositionLengthIxArgs,
) -> ProgramResult {
    increase_position_length_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn increase_position_length_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionLengthAccounts<'_, '_>,
    args: IncreasePositionLengthIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreasePositionLengthKeys = accounts.into();
    let ix = increase_position_length_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_position_length_invoke_signed(
    accounts: IncreasePositionLengthAccounts<'_, '_>,
    args: IncreasePositionLengthIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_position_length_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_position_length_verify_account_keys(
    accounts: IncreasePositionLengthAccounts<'_, '_>,
    keys: IncreasePositionLengthKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.funder.key, keys.funder),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.owner.key, keys.owner),
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
pub fn increase_position_length_verify_writable_privileges<'me, 'info>(
    accounts: IncreasePositionLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.funder, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_position_length_verify_signer_privileges<'me, 'info>(
    accounts: IncreasePositionLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_position_length_verify_account_privileges<'me, 'info>(
    accounts: IncreasePositionLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_position_length_verify_writable_privileges(accounts)?;
    increase_position_length_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_POSITION_LENGTH2_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct IncreasePositionLength2Accounts<'me, 'info> {
    pub funder: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreasePositionLength2Keys {
    pub funder: Pubkey,
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<IncreasePositionLength2Accounts<'_, '_>> for IncreasePositionLength2Keys {
    fn from(accounts: IncreasePositionLength2Accounts) -> Self {
        Self {
            funder: *accounts.funder.key,
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<IncreasePositionLength2Keys>
for [AccountMeta; INCREASE_POSITION_LENGTH2_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreasePositionLength2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; INCREASE_POSITION_LENGTH2_IX_ACCOUNTS_LEN]>
for IncreasePositionLength2Keys {
    fn from(pubkeys: [Pubkey; INCREASE_POSITION_LENGTH2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: pubkeys[0],
            lb_pair: pubkeys[1],
            position: pubkeys[2],
            owner: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<IncreasePositionLength2Accounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_POSITION_LENGTH2_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreasePositionLength2Accounts<'_, 'info>) -> Self {
        [
            accounts.funder.clone(),
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INCREASE_POSITION_LENGTH2_IX_ACCOUNTS_LEN]>
for IncreasePositionLength2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_POSITION_LENGTH2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            funder: &arr[0],
            lb_pair: &arr[1],
            position: &arr[2],
            owner: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const INCREASE_POSITION_LENGTH2_IX_DISCM: [u8; 8usize] = [
    255, 210, 204, 71, 115, 137, 225, 113,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionLength2IxArgs {
    pub minimum_upper_bin_id: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePositionLength2IxData(pub IncreasePositionLength2IxArgs);
impl From<IncreasePositionLength2IxArgs> for IncreasePositionLength2IxData {
    fn from(args: IncreasePositionLength2IxArgs) -> Self {
        Self(args)
    }
}
impl IncreasePositionLength2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION_LENGTH2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let minimum_upper_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(IncreasePositionLength2IxArgs {
                minimum_upper_bin_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION_LENGTH2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_upper_bin_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_position_length2_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreasePositionLength2Keys,
    args: IncreasePositionLength2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_POSITION_LENGTH2_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreasePositionLength2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_position_length2_ix(
    keys: IncreasePositionLength2Keys,
    args: IncreasePositionLength2IxArgs,
) -> std::io::Result<Instruction> {
    increase_position_length2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn increase_position_length2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionLength2Accounts<'_, '_>,
    args: IncreasePositionLength2IxArgs,
) -> ProgramResult {
    let keys: IncreasePositionLength2Keys = accounts.into();
    let ix = increase_position_length2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_position_length2_invoke(
    accounts: IncreasePositionLength2Accounts<'_, '_>,
    args: IncreasePositionLength2IxArgs,
) -> ProgramResult {
    increase_position_length2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn increase_position_length2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreasePositionLength2Accounts<'_, '_>,
    args: IncreasePositionLength2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreasePositionLength2Keys = accounts.into();
    let ix = increase_position_length2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_position_length2_invoke_signed(
    accounts: IncreasePositionLength2Accounts<'_, '_>,
    args: IncreasePositionLength2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_position_length2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_position_length2_verify_account_keys(
    accounts: IncreasePositionLength2Accounts<'_, '_>,
    keys: IncreasePositionLength2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.funder.key, keys.funder),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.owner.key, keys.owner),
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
pub fn increase_position_length2_verify_writable_privileges<'me, 'info>(
    accounts: IncreasePositionLength2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.funder, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_position_length2_verify_signer_privileges<'me, 'info>(
    accounts: IncreasePositionLength2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_position_length2_verify_account_privileges<'me, 'info>(
    accounts: IncreasePositionLength2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_position_length2_verify_writable_privileges(accounts)?;
    increase_position_length2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeBinArrayAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeBinArrayKeys {
    pub lb_pair: Pubkey,
    pub bin_array: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeBinArrayAccounts<'_, '_>> for InitializeBinArrayKeys {
    fn from(accounts: InitializeBinArrayAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array: *accounts.bin_array.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeBinArrayKeys>
for [AccountMeta; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeBinArrayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
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
impl From<[Pubkey; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]> for InitializeBinArrayKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array: pubkeys[1],
            funder: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeBinArrayAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeBinArrayAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]>
for InitializeBinArrayAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array: &arr[1],
            funder: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_BIN_ARRAY_IX_DISCM: [u8; 8usize] = [
    35, 86, 19, 185, 78, 212, 75, 211,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeBinArrayIxArgs {
    pub index: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeBinArrayIxData(pub InitializeBinArrayIxArgs);
impl From<InitializeBinArrayIxArgs> for InitializeBinArrayIxData {
    fn from(args: InitializeBinArrayIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeBinArrayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_BIN_ARRAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitializeBinArrayIxArgs { index }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_BIN_ARRAY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_bin_array_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeBinArrayKeys,
    args: InitializeBinArrayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeBinArrayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_bin_array_ix(
    keys: InitializeBinArrayKeys,
    args: InitializeBinArrayIxArgs,
) -> std::io::Result<Instruction> {
    initialize_bin_array_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_bin_array_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
) -> ProgramResult {
    let keys: InitializeBinArrayKeys = accounts.into();
    let ix = initialize_bin_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_bin_array_invoke(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
) -> ProgramResult {
    initialize_bin_array_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_bin_array_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeBinArrayKeys = accounts.into();
    let ix = initialize_bin_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_bin_array_invoke_signed(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_bin_array_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_bin_array_verify_account_keys(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    keys: InitializeBinArrayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.funder.key, keys.funder),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_verify_writable_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bin_array, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_verify_signer_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_verify_account_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_bin_array_verify_writable_privileges(accounts)?;
    initialize_bin_array_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeBinArrayBitmapExtensionAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeBinArrayBitmapExtensionKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeBinArrayBitmapExtensionAccounts<'_, '_>>
for InitializeBinArrayBitmapExtensionKeys {
    fn from(accounts: InitializeBinArrayBitmapExtensionAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeBinArrayBitmapExtensionKeys>
for [AccountMeta; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeBinArrayBitmapExtensionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN]>
for InitializeBinArrayBitmapExtensionKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            funder: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeBinArrayBitmapExtensionAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeBinArrayBitmapExtensionAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN]>
for InitializeBinArrayBitmapExtensionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            funder: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
        }
    }
}
pub const INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM: [u8; 8usize] = [
    47, 157, 226, 180, 12, 240, 33, 71,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeBinArrayBitmapExtensionIxData;
impl InitializeBinArrayBitmapExtensionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_bin_array_bitmap_extension_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeBinArrayBitmapExtensionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeBinArrayBitmapExtensionIxData.try_to_vec()?,
    })
}
pub fn initialize_bin_array_bitmap_extension_ix(
    keys: InitializeBinArrayBitmapExtensionKeys,
) -> std::io::Result<Instruction> {
    initialize_bin_array_bitmap_extension_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn initialize_bin_array_bitmap_extension_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeBinArrayBitmapExtensionKeys = accounts.into();
    let ix = initialize_bin_array_bitmap_extension_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_bin_array_bitmap_extension_invoke(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
) -> ProgramResult {
    initialize_bin_array_bitmap_extension_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_bin_array_bitmap_extension_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeBinArrayBitmapExtensionKeys = accounts.into();
    let ix = initialize_bin_array_bitmap_extension_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_bin_array_bitmap_extension_invoke_signed(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_bin_array_bitmap_extension_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_bin_array_bitmap_extension_verify_account_keys(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
    keys: InitializeBinArrayBitmapExtensionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.funder.key, keys.funder),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_bitmap_extension_verify_writable_privileges<'me, 'info>(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bin_array_bitmap_extension, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_bitmap_extension_verify_signer_privileges<'me, 'info>(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_bitmap_extension_verify_account_privileges<'me, 'info>(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_bin_array_bitmap_extension_verify_writable_privileges(accounts)?;
    initialize_bin_array_bitmap_extension_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePermissionlessLbPairKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub user_token_x: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub user_token_y: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>>
for InitializeCustomizablePermissionlessLbPairKeys {
    fn from(accounts: InitializeCustomizablePermissionlessLbPairAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            oracle: *accounts.oracle.key,
            user_token_x: *accounts.user_token_x.key,
            funder: *accounts.funder.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            user_token_y: *accounts.user_token_y.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeCustomizablePermissionlessLbPairKeys>
for [AccountMeta; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCustomizablePermissionlessLbPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
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
                pubkey: keys.user_token_y,
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
impl From<[Pubkey; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN]>
for InitializeCustomizablePermissionlessLbPairKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            token_mint_x: pubkeys[2],
            token_mint_y: pubkeys[3],
            reserve_x: pubkeys[4],
            reserve_y: pubkeys[5],
            oracle: pubkeys[6],
            user_token_x: pubkeys[7],
            funder: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            user_token_y: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<InitializeCustomizablePermissionlessLbPairAccounts<'_, 'info>>
for [AccountInfo<
    'info,
>; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.oracle.clone(),
            accounts.user_token_x.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.user_token_y.clone(),
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
    >; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN],
> for InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            token_mint_x: &arr[2],
            token_mint_y: &arr[3],
            reserve_x: &arr[4],
            reserve_y: &arr[5],
            oracle: &arr[6],
            user_token_x: &arr[7],
            funder: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            user_token_y: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM: [u8; 8usize] = [
    46, 39, 41, 135, 111, 183, 200, 64,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeCustomizablePermissionlessLbPairIxArgs {
    pub params: CustomizableParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePermissionlessLbPairIxData(
    pub InitializeCustomizablePermissionlessLbPairIxArgs,
);
impl From<InitializeCustomizablePermissionlessLbPairIxArgs>
for InitializeCustomizablePermissionlessLbPairIxData {
    fn from(args: InitializeCustomizablePermissionlessLbPairIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeCustomizablePermissionlessLbPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = <CustomizableParams>::deserialize(&mut reader)?;
        Ok(
            Self(InitializeCustomizablePermissionlessLbPairIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_customizable_permissionless_lb_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCustomizablePermissionlessLbPairKeys,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeCustomizablePermissionlessLbPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_customizable_permissionless_lb_pair_ix(
    keys: InitializeCustomizablePermissionlessLbPairKeys,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
) -> std::io::Result<Instruction> {
    initialize_customizable_permissionless_lb_pair_ix_with_program_id(
        LB_CLMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_customizable_permissionless_lb_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
) -> ProgramResult {
    let keys: InitializeCustomizablePermissionlessLbPairKeys = accounts.into();
    let ix = initialize_customizable_permissionless_lb_pair_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_customizable_permissionless_lb_pair_invoke(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
) -> ProgramResult {
    initialize_customizable_permissionless_lb_pair_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_customizable_permissionless_lb_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCustomizablePermissionlessLbPairKeys = accounts.into();
    let ix = initialize_customizable_permissionless_lb_pair_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_customizable_permissionless_lb_pair_invoke_signed(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_customizable_permissionless_lb_pair_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_customizable_permissionless_lb_pair_verify_account_keys(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    keys: InitializeCustomizablePermissionlessLbPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.funder.key, keys.funder),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair_verify_account_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_customizable_permissionless_lb_pair_verify_writable_privileges(accounts)?;
    initialize_customizable_permissionless_lb_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct InitializeCustomizablePermissionlessLbPair2Accounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_badge_x: &'me AccountInfo<'info>,
    pub token_badge_y: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePermissionlessLbPair2Keys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub user_token_x: Pubkey,
    pub funder: Pubkey,
    pub token_badge_x: Pubkey,
    pub token_badge_y: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub system_program: Pubkey,
    pub user_token_y: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeCustomizablePermissionlessLbPair2Accounts<'_, '_>>
for InitializeCustomizablePermissionlessLbPair2Keys {
    fn from(accounts: InitializeCustomizablePermissionlessLbPair2Accounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            oracle: *accounts.oracle.key,
            user_token_x: *accounts.user_token_x.key,
            funder: *accounts.funder.key,
            token_badge_x: *accounts.token_badge_x.key,
            token_badge_y: *accounts.token_badge_y.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            system_program: *accounts.system_program.key,
            user_token_y: *accounts.user_token_y.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeCustomizablePermissionlessLbPair2Keys>
for [AccountMeta; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCustomizablePermissionlessLbPair2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_badge_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
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
impl From<[Pubkey; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_ACCOUNTS_LEN]>
for InitializeCustomizablePermissionlessLbPair2Keys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            token_mint_x: pubkeys[2],
            token_mint_y: pubkeys[3],
            reserve_x: pubkeys[4],
            reserve_y: pubkeys[5],
            oracle: pubkeys[6],
            user_token_x: pubkeys[7],
            funder: pubkeys[8],
            token_badge_x: pubkeys[9],
            token_badge_y: pubkeys[10],
            token_program_x: pubkeys[11],
            token_program_y: pubkeys[12],
            system_program: pubkeys[13],
            user_token_y: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<InitializeCustomizablePermissionlessLbPair2Accounts<'_, 'info>>
for [AccountInfo<
    'info,
>; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'_, 'info>,
    ) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.oracle.clone(),
            accounts.user_token_x.clone(),
            accounts.funder.clone(),
            accounts.token_badge_x.clone(),
            accounts.token_badge_y.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.system_program.clone(),
            accounts.user_token_y.clone(),
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
    >; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_ACCOUNTS_LEN],
> for InitializeCustomizablePermissionlessLbPair2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            token_mint_x: &arr[2],
            token_mint_y: &arr[3],
            reserve_x: &arr[4],
            reserve_y: &arr[5],
            oracle: &arr[6],
            user_token_x: &arr[7],
            funder: &arr[8],
            token_badge_x: &arr[9],
            token_badge_y: &arr[10],
            token_program_x: &arr[11],
            token_program_y: &arr[12],
            system_program: &arr[13],
            user_token_y: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_DISCM: [u8; 8usize] = [
    243, 73, 129, 126, 51, 19, 241, 107,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeCustomizablePermissionlessLbPair2IxArgs {
    pub params: CustomizableParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePermissionlessLbPair2IxData(
    pub InitializeCustomizablePermissionlessLbPair2IxArgs,
);
impl From<InitializeCustomizablePermissionlessLbPair2IxArgs>
for InitializeCustomizablePermissionlessLbPair2IxData {
    fn from(args: InitializeCustomizablePermissionlessLbPair2IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeCustomizablePermissionlessLbPair2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = <CustomizableParams>::deserialize(&mut reader)?;
        Ok(
            Self(InitializeCustomizablePermissionlessLbPair2IxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_customizable_permissionless_lb_pair2_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCustomizablePermissionlessLbPair2Keys,
    args: InitializeCustomizablePermissionlessLbPair2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeCustomizablePermissionlessLbPair2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_customizable_permissionless_lb_pair2_ix(
    keys: InitializeCustomizablePermissionlessLbPair2Keys,
    args: InitializeCustomizablePermissionlessLbPair2IxArgs,
) -> std::io::Result<Instruction> {
    initialize_customizable_permissionless_lb_pair2_ix_with_program_id(
        LB_CLMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_customizable_permissionless_lb_pair2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPair2IxArgs,
) -> ProgramResult {
    let keys: InitializeCustomizablePermissionlessLbPair2Keys = accounts.into();
    let ix = initialize_customizable_permissionless_lb_pair2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_customizable_permissionless_lb_pair2_invoke(
    accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPair2IxArgs,
) -> ProgramResult {
    initialize_customizable_permissionless_lb_pair2_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_customizable_permissionless_lb_pair2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPair2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCustomizablePermissionlessLbPair2Keys = accounts.into();
    let ix = initialize_customizable_permissionless_lb_pair2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_customizable_permissionless_lb_pair2_invoke_signed(
    accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPair2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_customizable_permissionless_lb_pair2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_customizable_permissionless_lb_pair2_verify_account_keys(
    accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'_, '_>,
    keys: InitializeCustomizablePermissionlessLbPair2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.funder.key, keys.funder),
        (*accounts.token_badge_x.key, keys.token_badge_x),
        (*accounts.token_badge_y.key, keys.token_badge_y),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair2_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair2_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair2_verify_account_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPair2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_customizable_permissionless_lb_pair2_verify_writable_privileges(
        accounts,
    )?;
    initialize_customizable_permissionless_lb_pair2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLbPairAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub preset_parameter: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeLbPairKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub preset_parameter: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeLbPairAccounts<'_, '_>> for InitializeLbPairKeys {
    fn from(accounts: InitializeLbPairAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            oracle: *accounts.oracle.key,
            preset_parameter: *accounts.preset_parameter.key,
            funder: *accounts.funder.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeLbPairKeys> for [AccountMeta; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLbPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.preset_parameter,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
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
                pubkey: keys.rent,
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
impl From<[Pubkey; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN]> for InitializeLbPairKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            token_mint_x: pubkeys[2],
            token_mint_y: pubkeys[3],
            reserve_x: pubkeys[4],
            reserve_y: pubkeys[5],
            oracle: pubkeys[6],
            preset_parameter: pubkeys[7],
            funder: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<InitializeLbPairAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLbPairAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.oracle.clone(),
            accounts.preset_parameter.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN]>
for InitializeLbPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            token_mint_x: &arr[2],
            token_mint_y: &arr[3],
            reserve_x: &arr[4],
            reserve_y: &arr[5],
            oracle: &arr[6],
            preset_parameter: &arr[7],
            funder: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const INITIALIZE_LB_PAIR_IX_DISCM: [u8; 8usize] = [
    45, 154, 237, 210, 221, 15, 166, 92,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeLbPairIxArgs {
    pub active_id: i32,
    pub bin_step: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLbPairIxData(pub InitializeLbPairIxArgs);
impl From<InitializeLbPairIxArgs> for InitializeLbPairIxData {
    fn from(args: InitializeLbPairIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeLbPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LB_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeLbPairIxArgs {
                active_id,
                bin_step,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LB_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bin_step, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_lb_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLbPairKeys,
    args: InitializeLbPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeLbPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_lb_pair_ix(
    keys: InitializeLbPairKeys,
    args: InitializeLbPairIxArgs,
) -> std::io::Result<Instruction> {
    initialize_lb_pair_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_lb_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLbPairAccounts<'_, '_>,
    args: InitializeLbPairIxArgs,
) -> ProgramResult {
    let keys: InitializeLbPairKeys = accounts.into();
    let ix = initialize_lb_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_lb_pair_invoke(
    accounts: InitializeLbPairAccounts<'_, '_>,
    args: InitializeLbPairIxArgs,
) -> ProgramResult {
    initialize_lb_pair_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_lb_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLbPairAccounts<'_, '_>,
    args: InitializeLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLbPairKeys = accounts.into();
    let ix = initialize_lb_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_lb_pair_invoke_signed(
    accounts: InitializeLbPairAccounts<'_, '_>,
    args: InitializeLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_lb_pair_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_lb_pair_verify_account_keys(
    accounts: InitializeLbPairAccounts<'_, '_>,
    keys: InitializeLbPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.preset_parameter.key, keys.preset_parameter),
        (*accounts.funder.key, keys.funder),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_lb_pair_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_lb_pair_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_lb_pair_verify_account_privileges<'me, 'info>(
    accounts: InitializeLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_lb_pair_verify_writable_privileges(accounts)?;
    initialize_lb_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_LB_PAIR2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLbPair2Accounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub preset_parameter: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_badge_x: &'me AccountInfo<'info>,
    pub token_badge_y: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeLbPair2Keys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub preset_parameter: Pubkey,
    pub funder: Pubkey,
    pub token_badge_x: Pubkey,
    pub token_badge_y: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeLbPair2Accounts<'_, '_>> for InitializeLbPair2Keys {
    fn from(accounts: InitializeLbPair2Accounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            oracle: *accounts.oracle.key,
            preset_parameter: *accounts.preset_parameter.key,
            funder: *accounts.funder.key,
            token_badge_x: *accounts.token_badge_x.key,
            token_badge_y: *accounts.token_badge_y.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeLbPair2Keys> for [AccountMeta; INITIALIZE_LB_PAIR2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLbPair2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.preset_parameter,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_badge_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
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
impl From<[Pubkey; INITIALIZE_LB_PAIR2_IX_ACCOUNTS_LEN]> for InitializeLbPair2Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_LB_PAIR2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            token_mint_x: pubkeys[2],
            token_mint_y: pubkeys[3],
            reserve_x: pubkeys[4],
            reserve_y: pubkeys[5],
            oracle: pubkeys[6],
            preset_parameter: pubkeys[7],
            funder: pubkeys[8],
            token_badge_x: pubkeys[9],
            token_badge_y: pubkeys[10],
            token_program_x: pubkeys[11],
            token_program_y: pubkeys[12],
            system_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<InitializeLbPair2Accounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LB_PAIR2_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLbPair2Accounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.oracle.clone(),
            accounts.preset_parameter.clone(),
            accounts.funder.clone(),
            accounts.token_badge_x.clone(),
            accounts.token_badge_y.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_LB_PAIR2_IX_ACCOUNTS_LEN]>
for InitializeLbPair2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_LB_PAIR2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            token_mint_x: &arr[2],
            token_mint_y: &arr[3],
            reserve_x: &arr[4],
            reserve_y: &arr[5],
            oracle: &arr[6],
            preset_parameter: &arr[7],
            funder: &arr[8],
            token_badge_x: &arr[9],
            token_badge_y: &arr[10],
            token_program_x: &arr[11],
            token_program_y: &arr[12],
            system_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const INITIALIZE_LB_PAIR2_IX_DISCM: [u8; 8usize] = [
    73, 59, 36, 120, 237, 83, 108, 198,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeLbPair2IxArgs {
    pub params: InitializeLbPair2Params,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLbPair2IxData(pub InitializeLbPair2IxArgs);
impl From<InitializeLbPair2IxArgs> for InitializeLbPair2IxData {
    fn from(args: InitializeLbPair2IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeLbPair2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LB_PAIR2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = <InitializeLbPair2Params>::deserialize(&mut reader)?;
        Ok(Self(InitializeLbPair2IxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LB_PAIR2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_lb_pair2_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLbPair2Keys,
    args: InitializeLbPair2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LB_PAIR2_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeLbPair2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_lb_pair2_ix(
    keys: InitializeLbPair2Keys,
    args: InitializeLbPair2IxArgs,
) -> std::io::Result<Instruction> {
    initialize_lb_pair2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_lb_pair2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLbPair2Accounts<'_, '_>,
    args: InitializeLbPair2IxArgs,
) -> ProgramResult {
    let keys: InitializeLbPair2Keys = accounts.into();
    let ix = initialize_lb_pair2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_lb_pair2_invoke(
    accounts: InitializeLbPair2Accounts<'_, '_>,
    args: InitializeLbPair2IxArgs,
) -> ProgramResult {
    initialize_lb_pair2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_lb_pair2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLbPair2Accounts<'_, '_>,
    args: InitializeLbPair2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLbPair2Keys = accounts.into();
    let ix = initialize_lb_pair2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_lb_pair2_invoke_signed(
    accounts: InitializeLbPair2Accounts<'_, '_>,
    args: InitializeLbPair2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_lb_pair2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_lb_pair2_verify_account_keys(
    accounts: InitializeLbPair2Accounts<'_, '_>,
    keys: InitializeLbPair2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.preset_parameter.key, keys.preset_parameter),
        (*accounts.funder.key, keys.funder),
        (*accounts.token_badge_x.key, keys.token_badge_x),
        (*accounts.token_badge_y.key, keys.token_badge_y),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
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
pub fn initialize_lb_pair2_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLbPair2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_lb_pair2_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLbPair2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_lb_pair2_verify_account_privileges<'me, 'info>(
    accounts: InitializeLbPair2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_lb_pair2_verify_writable_privileges(accounts)?;
    initialize_lb_pair2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct InitializePermissionLbPairAccounts<'me, 'info> {
    pub base: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub token_badge_x: &'me AccountInfo<'info>,
    pub token_badge_y: &'me AccountInfo<'info>,
    pub token_program_x: &'me AccountInfo<'info>,
    pub token_program_y: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePermissionLbPairKeys {
    pub base: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub payer: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub token_badge_x: Pubkey,
    pub token_badge_y: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePermissionLbPairAccounts<'_, '_>>
for InitializePermissionLbPairKeys {
    fn from(accounts: InitializePermissionLbPairAccounts) -> Self {
        Self {
            base: *accounts.base.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            oracle: *accounts.oracle.key,
            payer: *accounts.payer.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            token_badge_x: *accounts.token_badge_x.key,
            token_badge_y: *accounts.token_badge_y.key,
            token_program_x: *accounts.token_program_x.key,
            token_program_y: *accounts.token_program_y.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePermissionLbPairKeys>
for [AccountMeta; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePermissionLbPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_y,
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
impl From<[Pubkey; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN]>
for InitializePermissionLbPairKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            token_mint_x: pubkeys[3],
            token_mint_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            oracle: pubkeys[7],
            payer: pubkeys[8],
            operator: pubkeys[9],
            signer: pubkeys[10],
            token_badge_x: pubkeys[11],
            token_badge_y: pubkeys[12],
            token_program_x: pubkeys[13],
            token_program_y: pubkeys[14],
            system_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<InitializePermissionLbPairAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePermissionLbPairAccounts<'_, 'info>) -> Self {
        [
            accounts.base.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.oracle.clone(),
            accounts.payer.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.token_badge_x.clone(),
            accounts.token_badge_y.clone(),
            accounts.token_program_x.clone(),
            accounts.token_program_y.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN]>
for InitializePermissionLbPairAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            base: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            token_mint_x: &arr[3],
            token_mint_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            oracle: &arr[7],
            payer: &arr[8],
            operator: &arr[9],
            signer: &arr[10],
            token_badge_x: &arr[11],
            token_badge_y: &arr[12],
            token_program_x: &arr[13],
            token_program_y: &arr[14],
            system_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM: [u8; 8usize] = [
    108, 102, 213, 85, 251, 3, 53, 21,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePermissionLbPairIxArgs {
    pub ix_data: InitPermissionPairIx,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionLbPairIxData(pub InitializePermissionLbPairIxArgs);
impl From<InitializePermissionLbPairIxArgs> for InitializePermissionLbPairIxData {
    fn from(args: InitializePermissionLbPairIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePermissionLbPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let ix_data = if reader.is_empty() {
            Default::default()
        } else {
            <InitPermissionPairIx>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializePermissionLbPairIxArgs {
                ix_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.ix_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_permission_lb_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePermissionLbPairKeys,
    args: InitializePermissionLbPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePermissionLbPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_permission_lb_pair_ix(
    keys: InitializePermissionLbPairKeys,
    args: InitializePermissionLbPairIxArgs,
) -> std::io::Result<Instruction> {
    initialize_permission_lb_pair_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_permission_lb_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    args: InitializePermissionLbPairIxArgs,
) -> ProgramResult {
    let keys: InitializePermissionLbPairKeys = accounts.into();
    let ix = initialize_permission_lb_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_permission_lb_pair_invoke(
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    args: InitializePermissionLbPairIxArgs,
) -> ProgramResult {
    initialize_permission_lb_pair_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_permission_lb_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    args: InitializePermissionLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePermissionLbPairKeys = accounts.into();
    let ix = initialize_permission_lb_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_permission_lb_pair_invoke_signed(
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    args: InitializePermissionLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_permission_lb_pair_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_permission_lb_pair_verify_account_keys(
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    keys: InitializePermissionLbPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.base.key, keys.base),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.payer.key, keys.payer),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.token_badge_x.key, keys.token_badge_x),
        (*accounts.token_badge_y.key, keys.token_badge_y),
        (*accounts.token_program_x.key, keys.token_program_x),
        (*accounts.token_program_y.key, keys.token_program_y),
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
pub fn initialize_permission_lb_pair_verify_writable_privileges<'me, 'info>(
    accounts: InitializePermissionLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_permission_lb_pair_verify_signer_privileges<'me, 'info>(
    accounts: InitializePermissionLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base, accounts.payer, accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_permission_lb_pair_verify_account_privileges<'me, 'info>(
    accounts: InitializePermissionLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_permission_lb_pair_verify_writable_privileges(accounts)?;
    initialize_permission_lb_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionKeys {
    pub payer: Pubkey,
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePositionAccounts<'_, '_>> for InitializePositionKeys {
    fn from(accounts: InitializePositionAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePositionKeys>
for [AccountMeta; INITIALIZE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_POSITION_IX_ACCOUNTS_LEN]> for InitializePositionKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            position: pubkeys[1],
            lb_pair: pubkeys[2],
            owner: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION_IX_ACCOUNTS_LEN]>
for InitializePositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            position: &arr[1],
            lb_pair: &arr[2],
            owner: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const INITIALIZE_POSITION_IX_DISCM: [u8; 8usize] = [
    219, 192, 234, 71, 190, 191, 102, 80,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePositionIxArgs {
    pub lower_bin_id: i32,
    pub width: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionIxData(pub InitializePositionIxArgs);
impl From<InitializePositionIxArgs> for InitializePositionIxData {
    fn from(args: InitializePositionIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let width: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializePositionIxArgs {
                lower_bin_id,
                width,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lower_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.width, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionKeys,
    args: InitializePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_position_ix(
    keys: InitializePositionKeys,
    args: InitializePositionIxArgs,
) -> std::io::Result<Instruction> {
    initialize_position_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionAccounts<'_, '_>,
    args: InitializePositionIxArgs,
) -> ProgramResult {
    let keys: InitializePositionKeys = accounts.into();
    let ix = initialize_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_invoke(
    accounts: InitializePositionAccounts<'_, '_>,
    args: InitializePositionIxArgs,
) -> ProgramResult {
    initialize_position_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionAccounts<'_, '_>,
    args: InitializePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionKeys = accounts.into();
    let ix = initialize_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_invoke_signed(
    accounts: InitializePositionAccounts<'_, '_>,
    args: InitializePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_position_verify_account_keys(
    accounts: InitializePositionAccounts<'_, '_>,
    keys: InitializePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.position, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_verify_writable_privileges(accounts)?;
    initialize_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION2_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializePosition2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePosition2Keys {
    pub payer: Pubkey,
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePosition2Accounts<'_, '_>> for InitializePosition2Keys {
    fn from(accounts: InitializePosition2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePosition2Keys>
for [AccountMeta; INITIALIZE_POSITION2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePosition2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_POSITION2_IX_ACCOUNTS_LEN]> for InitializePosition2Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            position: pubkeys[1],
            lb_pair: pubkeys[2],
            owner: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializePosition2Accounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION2_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePosition2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION2_IX_ACCOUNTS_LEN]>
for InitializePosition2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            position: &arr[1],
            lb_pair: &arr[2],
            owner: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const INITIALIZE_POSITION2_IX_DISCM: [u8; 8usize] = [
    143, 19, 242, 145, 213, 15, 104, 115,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePosition2IxArgs {
    pub lower_bin_id: i32,
    pub width: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePosition2IxData(pub InitializePosition2IxArgs);
impl From<InitializePosition2IxArgs> for InitializePosition2IxData {
    fn from(args: InitializePosition2IxArgs) -> Self {
        Self(args)
    }
}
impl InitializePosition2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let width: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializePosition2IxArgs {
                lower_bin_id,
                width,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lower_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.width, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position2_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePosition2Keys,
    args: InitializePosition2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION2_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePosition2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_position2_ix(
    keys: InitializePosition2Keys,
    args: InitializePosition2IxArgs,
) -> std::io::Result<Instruction> {
    initialize_position2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_position2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePosition2Accounts<'_, '_>,
    args: InitializePosition2IxArgs,
) -> ProgramResult {
    let keys: InitializePosition2Keys = accounts.into();
    let ix = initialize_position2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position2_invoke(
    accounts: InitializePosition2Accounts<'_, '_>,
    args: InitializePosition2IxArgs,
) -> ProgramResult {
    initialize_position2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_position2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePosition2Accounts<'_, '_>,
    args: InitializePosition2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePosition2Keys = accounts.into();
    let ix = initialize_position2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position2_invoke_signed(
    accounts: InitializePosition2Accounts<'_, '_>,
    args: InitializePosition2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_position2_verify_account_keys(
    accounts: InitializePosition2Accounts<'_, '_>,
    keys: InitializePosition2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.owner.key, keys.owner),
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
pub fn initialize_position2_verify_writable_privileges<'me, 'info>(
    accounts: InitializePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position2_verify_signer_privileges<'me, 'info>(
    accounts: InitializePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.position, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position2_verify_account_privileges<'me, 'info>(
    accounts: InitializePosition2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position2_verify_writable_privileges(accounts)?;
    initialize_position2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionByOperatorAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub operator_token_x: &'me AccountInfo<'info>,
    pub owner_token_x: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionByOperatorKeys {
    pub payer: Pubkey,
    pub base: Pubkey,
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub operator: Pubkey,
    pub operator_token_x: Pubkey,
    pub owner_token_x: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePositionByOperatorAccounts<'_, '_>>
for InitializePositionByOperatorKeys {
    fn from(accounts: InitializePositionByOperatorAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            base: *accounts.base.key,
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            owner: *accounts.owner.key,
            operator: *accounts.operator.key,
            operator_token_x: *accounts.operator_token_x.key,
            owner_token_x: *accounts.owner_token_x.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePositionByOperatorKeys>
for [AccountMeta; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionByOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator_token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner_token_x,
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
impl From<[Pubkey; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN]>
for InitializePositionByOperatorKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            base: pubkeys[1],
            position: pubkeys[2],
            lb_pair: pubkeys[3],
            owner: pubkeys[4],
            operator: pubkeys[5],
            operator_token_x: pubkeys[6],
            owner_token_x: pubkeys[7],
            system_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<InitializePositionByOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionByOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.base.clone(),
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.owner.clone(),
            accounts.operator.clone(),
            accounts.operator_token_x.clone(),
            accounts.owner_token_x.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN]>
for InitializePositionByOperatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            base: &arr[1],
            position: &arr[2],
            lb_pair: &arr[3],
            owner: &arr[4],
            operator: &arr[5],
            operator_token_x: &arr[6],
            owner_token_x: &arr[7],
            system_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM: [u8; 8usize] = [
    251, 189, 190, 244, 117, 254, 35, 148,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePositionByOperatorIxArgs {
    pub lower_bin_id: i32,
    pub width: i32,
    pub fee_owner: Pubkey,
    pub lock_release_point: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionByOperatorIxData(pub InitializePositionByOperatorIxArgs);
impl From<InitializePositionByOperatorIxArgs> for InitializePositionByOperatorIxData {
    fn from(args: InitializePositionByOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePositionByOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let width: i32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lock_release_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializePositionByOperatorIxArgs {
                lower_bin_id,
                width,
                fee_owner,
                lock_release_point,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lower_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.width, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lock_release_point, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_by_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionByOperatorKeys,
    args: InitializePositionByOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePositionByOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_position_by_operator_ix(
    keys: InitializePositionByOperatorKeys,
    args: InitializePositionByOperatorIxArgs,
) -> std::io::Result<Instruction> {
    initialize_position_by_operator_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_position_by_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    args: InitializePositionByOperatorIxArgs,
) -> ProgramResult {
    let keys: InitializePositionByOperatorKeys = accounts.into();
    let ix = initialize_position_by_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_by_operator_invoke(
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    args: InitializePositionByOperatorIxArgs,
) -> ProgramResult {
    initialize_position_by_operator_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_position_by_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    args: InitializePositionByOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionByOperatorKeys = accounts.into();
    let ix = initialize_position_by_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_by_operator_invoke_signed(
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    args: InitializePositionByOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_by_operator_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_position_by_operator_verify_account_keys(
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    keys: InitializePositionByOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.base.key, keys.base),
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.owner.key, keys.owner),
        (*accounts.operator.key, keys.operator),
        (*accounts.operator_token_x.key, keys.operator_token_x),
        (*accounts.owner_token_x.key, keys.owner_token_x),
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
pub fn initialize_position_by_operator_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionByOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_by_operator_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionByOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base, accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_by_operator_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionByOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_by_operator_verify_writable_privileges(accounts)?;
    initialize_position_by_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionPdaAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionPdaKeys {
    pub payer: Pubkey,
    pub base: Pubkey,
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePositionPdaAccounts<'_, '_>> for InitializePositionPdaKeys {
    fn from(accounts: InitializePositionPdaAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            base: *accounts.base.key,
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePositionPdaKeys>
for [AccountMeta; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionPdaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN]>
for InitializePositionPdaKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            base: pubkeys[1],
            position: pubkeys[2],
            lb_pair: pubkeys[3],
            owner: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializePositionPdaAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionPdaAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.base.clone(),
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN]>
for InitializePositionPdaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            base: &arr[1],
            position: &arr[2],
            lb_pair: &arr[3],
            owner: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const INITIALIZE_POSITION_PDA_IX_DISCM: [u8; 8usize] = [
    46, 82, 125, 146, 85, 141, 228, 153,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePositionPdaIxArgs {
    pub lower_bin_id: i32,
    pub width: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionPdaIxData(pub InitializePositionPdaIxArgs);
impl From<InitializePositionPdaIxArgs> for InitializePositionPdaIxData {
    fn from(args: InitializePositionPdaIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePositionPdaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_PDA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let width: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializePositionPdaIxArgs {
                lower_bin_id,
                width,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_PDA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lower_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.width, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_pda_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionPdaKeys,
    args: InitializePositionPdaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePositionPdaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_position_pda_ix(
    keys: InitializePositionPdaKeys,
    args: InitializePositionPdaIxArgs,
) -> std::io::Result<Instruction> {
    initialize_position_pda_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_position_pda_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionPdaAccounts<'_, '_>,
    args: InitializePositionPdaIxArgs,
) -> ProgramResult {
    let keys: InitializePositionPdaKeys = accounts.into();
    let ix = initialize_position_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_pda_invoke(
    accounts: InitializePositionPdaAccounts<'_, '_>,
    args: InitializePositionPdaIxArgs,
) -> ProgramResult {
    initialize_position_pda_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_position_pda_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionPdaAccounts<'_, '_>,
    args: InitializePositionPdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionPdaKeys = accounts.into();
    let ix = initialize_position_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_pda_invoke_signed(
    accounts: InitializePositionPdaAccounts<'_, '_>,
    args: InitializePositionPdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_pda_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_position_pda_verify_account_keys(
    accounts: InitializePositionPdaAccounts<'_, '_>,
    keys: InitializePositionPdaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.base.key, keys.base),
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_pda_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_pda_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_pda_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_pda_verify_writable_privileges(accounts)?;
    initialize_position_pda_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializePresetParameterAccounts<'me, 'info> {
    pub preset_parameter: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePresetParameterKeys {
    pub preset_parameter: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePresetParameterAccounts<'_, '_>> for InitializePresetParameterKeys {
    fn from(accounts: InitializePresetParameterAccounts) -> Self {
        Self {
            preset_parameter: *accounts.preset_parameter.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePresetParameterKeys>
for [AccountMeta; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePresetParameterKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.preset_parameter,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
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
impl From<[Pubkey; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]>
for InitializePresetParameterKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            preset_parameter: pubkeys[0],
            operator: pubkeys[1],
            signer: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializePresetParameterAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePresetParameterAccounts<'_, 'info>) -> Self {
        [
            accounts.preset_parameter.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]>
for InitializePresetParameterAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            preset_parameter: &arr[0],
            operator: &arr[1],
            signer: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INITIALIZE_PRESET_PARAMETER_IX_DISCM: [u8; 8usize] = [
    66, 188, 71, 211, 98, 109, 14, 186,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePresetParameterIxArgs {
    pub ix: InitPresetParametersIx,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePresetParameterIxData(pub InitializePresetParameterIxArgs);
impl From<InitializePresetParameterIxArgs> for InitializePresetParameterIxData {
    fn from(args: InitializePresetParameterIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePresetParameterIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PRESET_PARAMETER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let ix = if reader.is_empty() {
            Default::default()
        } else {
            <InitPresetParametersIx>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializePresetParameterIxArgs {
                ix,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PRESET_PARAMETER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.ix, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_preset_parameter_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePresetParameterKeys,
    args: InitializePresetParameterIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePresetParameterIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_preset_parameter_ix(
    keys: InitializePresetParameterKeys,
    args: InitializePresetParameterIxArgs,
) -> std::io::Result<Instruction> {
    initialize_preset_parameter_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_preset_parameter_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePresetParameterAccounts<'_, '_>,
    args: InitializePresetParameterIxArgs,
) -> ProgramResult {
    let keys: InitializePresetParameterKeys = accounts.into();
    let ix = initialize_preset_parameter_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_preset_parameter_invoke(
    accounts: InitializePresetParameterAccounts<'_, '_>,
    args: InitializePresetParameterIxArgs,
) -> ProgramResult {
    initialize_preset_parameter_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_preset_parameter_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePresetParameterAccounts<'_, '_>,
    args: InitializePresetParameterIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePresetParameterKeys = accounts.into();
    let ix = initialize_preset_parameter_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_preset_parameter_invoke_signed(
    accounts: InitializePresetParameterAccounts<'_, '_>,
    args: InitializePresetParameterIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_preset_parameter_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_preset_parameter_verify_account_keys(
    accounts: InitializePresetParameterAccounts<'_, '_>,
    keys: InitializePresetParameterKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.preset_parameter.key, keys.preset_parameter),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_preset_parameter_verify_writable_privileges<'me, 'info>(
    accounts: InitializePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.preset_parameter, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_preset_parameter_verify_signer_privileges<'me, 'info>(
    accounts: InitializePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_preset_parameter_verify_account_privileges<'me, 'info>(
    accounts: InitializePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_preset_parameter_verify_writable_privileges(accounts)?;
    initialize_preset_parameter_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_REWARD_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeRewardAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub token_badge: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRewardKeys {
    pub lb_pair: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub token_badge: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeRewardAccounts<'_, '_>> for InitializeRewardKeys {
    fn from(accounts: InitializeRewardAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            token_badge: *accounts.token_badge.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeRewardKeys> for [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]> for InitializeRewardKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reward_vault: pubkeys[1],
            reward_mint: pubkeys[2],
            token_badge: pubkeys[3],
            operator: pubkeys[4],
            signer: pubkeys[5],
            payer: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<InitializeRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.token_badge.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]>
for InitializeRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            reward_vault: &arr[1],
            reward_mint: &arr[2],
            token_badge: &arr[3],
            operator: &arr[4],
            signer: &arr[5],
            payer: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const INITIALIZE_REWARD_IX_DISCM: [u8; 8usize] = [
    95, 135, 192, 196, 242, 129, 230, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeRewardIxArgs {
    pub reward_index: u64,
    pub reward_duration: u64,
    pub funder: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRewardIxData(pub InitializeRewardIxArgs);
impl From<InitializeRewardIxArgs> for InitializeRewardIxData {
    fn from(args: InitializeRewardIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeRewardIxArgs {
                reward_index,
                reward_duration,
                funder,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reward_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.funder, &mut writer)?;
        Ok(())
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
    args: InitializeRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_reward_ix(
    keys: InitializeRewardKeys,
    args: InitializeRewardIxArgs,
) -> std::io::Result<Instruction> {
    initialize_reward_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn initialize_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_reward_invoke(
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
) -> ProgramResult {
    initialize_reward_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_reward_invoke_signed(
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_reward_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_reward_verify_account_keys(
    accounts: InitializeRewardAccounts<'_, '_>,
    keys: InitializeRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.token_badge.key, keys.token_badge),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.payer.key, keys.payer),
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
pub fn initialize_reward_verify_writable_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair, accounts.reward_vault, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_signer_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.payer] {
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
pub const INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTokenBadgeAccounts<'me, 'info> {
    pub token_mint: &'me AccountInfo<'info>,
    pub token_badge: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTokenBadgeKeys {
    pub token_mint: Pubkey,
    pub token_badge: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeTokenBadgeAccounts<'_, '_>> for InitializeTokenBadgeKeys {
    fn from(accounts: InitializeTokenBadgeAccounts) -> Self {
        Self {
            token_mint: *accounts.token_mint.key,
            token_badge: *accounts.token_badge.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeTokenBadgeKeys>
for [AccountMeta; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTokenBadgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
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
impl From<[Pubkey; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN]>
for InitializeTokenBadgeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_mint: pubkeys[0],
            token_badge: pubkeys[1],
            operator: pubkeys[2],
            signer: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeTokenBadgeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTokenBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.token_mint.clone(),
            accounts.token_badge.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN]>
for InitializeTokenBadgeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_mint: &arr[0],
            token_badge: &arr[1],
            operator: &arr[2],
            signer: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INITIALIZE_TOKEN_BADGE_IX_DISCM: [u8; 8usize] = [
    253, 77, 205, 95, 27, 224, 89, 223,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTokenBadgeIxData;
impl InitializeTokenBadgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_TOKEN_BADGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_TOKEN_BADGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_token_badge_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeTokenBadgeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeTokenBadgeIxData.try_to_vec()?,
    })
}
pub fn initialize_token_badge_ix(
    keys: InitializeTokenBadgeKeys,
) -> std::io::Result<Instruction> {
    initialize_token_badge_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn initialize_token_badge_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTokenBadgeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeTokenBadgeKeys = accounts.into();
    let ix = initialize_token_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_token_badge_invoke(
    accounts: InitializeTokenBadgeAccounts<'_, '_>,
) -> ProgramResult {
    initialize_token_badge_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn initialize_token_badge_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTokenBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeTokenBadgeKeys = accounts.into();
    let ix = initialize_token_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_token_badge_invoke_signed(
    accounts: InitializeTokenBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_token_badge_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_token_badge_verify_account_keys(
    accounts: InitializeTokenBadgeAccounts<'_, '_>,
    keys: InitializeTokenBadgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_badge.key, keys.token_badge),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_token_badge_verify_writable_privileges<'me, 'info>(
    accounts: InitializeTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_badge, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_token_badge_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_token_badge_verify_account_privileges<'me, 'info>(
    accounts: InitializeTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_token_badge_verify_writable_privileges(accounts)?;
    initialize_token_badge_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct PlaceLimitOrderAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub limit_order: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceLimitOrderKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub limit_order: Pubkey,
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub user_token: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PlaceLimitOrderAccounts<'_, '_>> for PlaceLimitOrderKeys {
    fn from(accounts: PlaceLimitOrderAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            limit_order: *accounts.limit_order.key,
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            user_token: *accounts.user_token.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PlaceLimitOrderKeys> for [AccountMeta; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: PlaceLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.limit_order,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
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
impl From<[Pubkey; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]> for PlaceLimitOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve: pubkeys[2],
            token_mint: pubkeys[3],
            limit_order: pubkeys[4],
            payer: pubkeys[5],
            owner: pubkeys[6],
            user_token: pubkeys[7],
            sender: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<PlaceLimitOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: PlaceLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.limit_order.clone(),
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.user_token.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for PlaceLimitOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve: &arr[2],
            token_mint: &arr[3],
            limit_order: &arr[4],
            payer: &arr[5],
            owner: &arr[6],
            user_token: &arr[7],
            sender: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const PLACE_LIMIT_ORDER_IX_DISCM: [u8; 8usize] = [
    108, 176, 33, 186, 146, 229, 1, 197,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlaceLimitOrderIxArgs {
    pub params: PlaceLimitOrderParams,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceLimitOrderIxData(pub PlaceLimitOrderIxArgs);
impl From<PlaceLimitOrderIxArgs> for PlaceLimitOrderIxData {
    fn from(args: PlaceLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <PlaceLimitOrderParams>::deserialize(&mut reader)?
        };
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(PlaceLimitOrderIxArgs {
                params,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_LIMIT_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceLimitOrderKeys,
    args: PlaceLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: PlaceLimitOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_limit_order_ix(
    keys: PlaceLimitOrderKeys,
    args: PlaceLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    place_limit_order_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn place_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    args: PlaceLimitOrderIxArgs,
) -> ProgramResult {
    let keys: PlaceLimitOrderKeys = accounts.into();
    let ix = place_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_limit_order_invoke(
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    args: PlaceLimitOrderIxArgs,
) -> ProgramResult {
    place_limit_order_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn place_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    args: PlaceLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceLimitOrderKeys = accounts.into();
    let ix = place_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_limit_order_invoke_signed(
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    args: PlaceLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_limit_order_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn place_limit_order_verify_account_keys(
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    keys: PlaceLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.limit_order.key, keys.limit_order),
        (*accounts.payer.key, keys.payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.sender.key, keys.sender),
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
pub fn place_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: PlaceLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve,
        accounts.limit_order,
        accounts.payer,
        accounts.user_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: PlaceLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.limit_order, accounts.payer, accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_limit_order_verify_account_privileges<'me, 'info>(
    accounts: PlaceLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_limit_order_verify_writable_privileges(accounts)?;
    place_limit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct RebalanceLiquidityAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub rent_payer: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RebalanceLiquidityKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub owner: Pubkey,
    pub rent_payer: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RebalanceLiquidityAccounts<'_, '_>> for RebalanceLiquidityKeys {
    fn from(accounts: RebalanceLiquidityAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            owner: *accounts.owner.key,
            rent_payer: *accounts.rent_payer.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RebalanceLiquidityKeys>
for [AccountMeta; REBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RebalanceLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
impl From<[Pubkey; REBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RebalanceLiquidityKeys {
    fn from(pubkeys: [Pubkey; REBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            owner: pubkeys[9],
            rent_payer: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            memo_program: pubkeys[13],
            system_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<RebalanceLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RebalanceLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.owner.clone(),
            accounts.rent_payer.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RebalanceLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            owner: &arr[9],
            rent_payer: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            memo_program: &arr[13],
            system_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const REBALANCE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    92, 4, 176, 193, 119, 185, 83, 9,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RebalanceLiquidityIxArgs {
    pub params: RebalanceLiquidityParams,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RebalanceLiquidityIxData(pub RebalanceLiquidityIxArgs);
impl From<RebalanceLiquidityIxArgs> for RebalanceLiquidityIxData {
    fn from(args: RebalanceLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RebalanceLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceLiquidityParams>::deserialize(&mut reader)?
        };
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(RebalanceLiquidityIxArgs {
                params,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn rebalance_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RebalanceLiquidityKeys,
    args: RebalanceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RebalanceLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn rebalance_liquidity_ix(
    keys: RebalanceLiquidityKeys,
    args: RebalanceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    rebalance_liquidity_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn rebalance_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceLiquidityAccounts<'_, '_>,
    args: RebalanceLiquidityIxArgs,
) -> ProgramResult {
    let keys: RebalanceLiquidityKeys = accounts.into();
    let ix = rebalance_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn rebalance_liquidity_invoke(
    accounts: RebalanceLiquidityAccounts<'_, '_>,
    args: RebalanceLiquidityIxArgs,
) -> ProgramResult {
    rebalance_liquidity_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn rebalance_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceLiquidityAccounts<'_, '_>,
    args: RebalanceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RebalanceLiquidityKeys = accounts.into();
    let ix = rebalance_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn rebalance_liquidity_invoke_signed(
    accounts: RebalanceLiquidityAccounts<'_, '_>,
    args: RebalanceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    rebalance_liquidity_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn rebalance_liquidity_verify_account_keys(
    accounts: RebalanceLiquidityAccounts<'_, '_>,
    keys: RebalanceLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.owner.key, keys.owner),
        (*accounts.rent_payer.key, keys.rent_payer),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
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
pub fn rebalance_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RebalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.rent_payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn rebalance_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RebalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.rent_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn rebalance_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RebalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    rebalance_liquidity_verify_writable_privileges(accounts)?;
    rebalance_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct RemoveAllLiquidityAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAllLiquidityKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveAllLiquidityAccounts<'_, '_>> for RemoveAllLiquidityKeys {
    fn from(accounts: RemoveAllLiquidityAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveAllLiquidityKeys>
for [AccountMeta; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAllLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveAllLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<RemoveAllLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAllLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveAllLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const REMOVE_ALL_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    10, 51, 61, 35, 112, 105, 24, 85,
];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAllLiquidityIxData;
impl RemoveAllLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ALL_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ALL_LIQUIDITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_all_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveAllLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveAllLiquidityIxData.try_to_vec()?,
    })
}
pub fn remove_all_liquidity_ix(
    keys: RemoveAllLiquidityKeys,
) -> std::io::Result<Instruction> {
    remove_all_liquidity_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn remove_all_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveAllLiquidityKeys = accounts.into();
    let ix = remove_all_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_all_liquidity_invoke(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    remove_all_liquidity_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn remove_all_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAllLiquidityKeys = accounts.into();
    let ix = remove_all_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_all_liquidity_invoke_signed(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_all_liquidity_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn remove_all_liquidity_verify_account_keys(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    keys: RemoveAllLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_all_liquidity_verify_writable_privileges(accounts)?;
    remove_all_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: RemoveLiquidityAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<RemoveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [80, 85, 209, 72, 24, 206, 177, 108];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityIxArgs {
    pub bin_liquidity_removal: Vec<BinLiquidityReduction>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityIxData(pub RemoveLiquidityIxArgs);
impl From<RemoveLiquidityIxArgs> for RemoveLiquidityIxData {
    fn from(args: RemoveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bin_liquidity_removal: Vec<BinLiquidityReduction> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(RemoveLiquidityIxArgs {
                bin_liquidity_removal,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bin_liquidity_removal, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_ix(
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn remove_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_invoke(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    remove_liquidity_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn remove_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_invoke_signed(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity_verify_account_keys(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    keys: RemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_verify_writable_privileges(accounts)?;
    remove_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidity2Accounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidity2Keys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveLiquidity2Accounts<'_, '_>> for RemoveLiquidity2Keys {
    fn from(accounts: RemoveLiquidity2Accounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveLiquidity2Keys> for [AccountMeta; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidity2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            sender: pubkeys[9],
            token_x_program: pubkeys[10],
            token_y_program: pubkeys[11],
            memo_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<RemoveLiquidity2Accounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidity2Accounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN]>
for RemoveLiquidity2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            sender: &arr[9],
            token_x_program: &arr[10],
            token_y_program: &arr[11],
            memo_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const REMOVE_LIQUIDITY2_IX_DISCM: [u8; 8usize] = [
    230, 215, 82, 127, 241, 101, 227, 146,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidity2IxArgs {
    pub bin_liquidity_removal: Vec<BinLiquidityReduction>,
    pub remaining_accounts_info: RemainingAccountsInfo,
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
        let bin_liquidity_removal: Vec<BinLiquidityReduction> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(RemoveLiquidity2IxArgs {
                bin_liquidity_removal,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bin_liquidity_removal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
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
    remove_liquidity2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
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
    remove_liquidity2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
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
        LB_CLMM_PROGRAM_ID,
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
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
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
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
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
    for should_be_signer in [accounts.sender] {
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
pub const REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityByRangeAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityByRangeKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveLiquidityByRangeAccounts<'_, '_>> for RemoveLiquidityByRangeKeys {
    fn from(accounts: RemoveLiquidityByRangeAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveLiquidityByRangeKeys>
for [AccountMeta; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityByRangeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN]>
for RemoveLiquidityByRangeKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<RemoveLiquidityByRangeAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityByRangeAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN]>
for RemoveLiquidityByRangeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM: [u8; 8usize] = [
    26, 82, 102, 152, 240, 74, 105, 26,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityByRangeIxArgs {
    pub from_bin_id: i32,
    pub to_bin_id: i32,
    pub bps_to_remove: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityByRangeIxData(pub RemoveLiquidityByRangeIxArgs);
impl From<RemoveLiquidityByRangeIxArgs> for RemoveLiquidityByRangeIxData {
    fn from(args: RemoveLiquidityByRangeIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidityByRangeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let from_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let to_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bps_to_remove: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveLiquidityByRangeIxArgs {
                from_bin_id,
                to_bin_id,
                bps_to_remove,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.from_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.to_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bps_to_remove, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_by_range_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidityByRangeKeys,
    args: RemoveLiquidityByRangeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidityByRangeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_by_range_ix(
    keys: RemoveLiquidityByRangeKeys,
    args: RemoveLiquidityByRangeIxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_by_range_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn remove_liquidity_by_range_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    args: RemoveLiquidityByRangeIxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidityByRangeKeys = accounts.into();
    let ix = remove_liquidity_by_range_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_by_range_invoke(
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    args: RemoveLiquidityByRangeIxArgs,
) -> ProgramResult {
    remove_liquidity_by_range_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn remove_liquidity_by_range_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    args: RemoveLiquidityByRangeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidityByRangeKeys = accounts.into();
    let ix = remove_liquidity_by_range_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_by_range_invoke_signed(
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    args: RemoveLiquidityByRangeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_by_range_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity_by_range_verify_account_keys(
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    keys: RemoveLiquidityByRangeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_by_range_verify_writable_privileges(accounts)?;
    remove_liquidity_by_range_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY_BY_RANGE2_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityByRange2Accounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityByRange2Keys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveLiquidityByRange2Accounts<'_, '_>> for RemoveLiquidityByRange2Keys {
    fn from(accounts: RemoveLiquidityByRange2Accounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveLiquidityByRange2Keys>
for [AccountMeta; REMOVE_LIQUIDITY_BY_RANGE2_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityByRange2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
impl From<[Pubkey; REMOVE_LIQUIDITY_BY_RANGE2_IX_ACCOUNTS_LEN]>
for RemoveLiquidityByRange2Keys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_BY_RANGE2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            sender: pubkeys[9],
            token_x_program: pubkeys[10],
            token_y_program: pubkeys[11],
            memo_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<RemoveLiquidityByRange2Accounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE2_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityByRange2Accounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE2_IX_ACCOUNTS_LEN]>
for RemoveLiquidityByRange2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            sender: &arr[9],
            token_x_program: &arr[10],
            token_y_program: &arr[11],
            memo_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const REMOVE_LIQUIDITY_BY_RANGE2_IX_DISCM: [u8; 8usize] = [
    204, 2, 195, 145, 53, 145, 145, 205,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityByRange2IxArgs {
    pub from_bin_id: i32,
    pub to_bin_id: i32,
    pub bps_to_remove: u16,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityByRange2IxData(pub RemoveLiquidityByRange2IxArgs);
impl From<RemoveLiquidityByRange2IxArgs> for RemoveLiquidityByRange2IxData {
    fn from(args: RemoveLiquidityByRange2IxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidityByRange2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_BY_RANGE2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let from_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let to_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bps_to_remove: u16 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(RemoveLiquidityByRange2IxArgs {
                from_bin_id,
                to_bin_id,
                bps_to_remove,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_BY_RANGE2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.from_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.to_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bps_to_remove, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_by_range2_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidityByRange2Keys,
    args: RemoveLiquidityByRange2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_BY_RANGE2_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidityByRange2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_by_range2_ix(
    keys: RemoveLiquidityByRange2Keys,
    args: RemoveLiquidityByRange2IxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_by_range2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn remove_liquidity_by_range2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityByRange2Accounts<'_, '_>,
    args: RemoveLiquidityByRange2IxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidityByRange2Keys = accounts.into();
    let ix = remove_liquidity_by_range2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_by_range2_invoke(
    accounts: RemoveLiquidityByRange2Accounts<'_, '_>,
    args: RemoveLiquidityByRange2IxArgs,
) -> ProgramResult {
    remove_liquidity_by_range2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn remove_liquidity_by_range2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityByRange2Accounts<'_, '_>,
    args: RemoveLiquidityByRange2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidityByRange2Keys = accounts.into();
    let ix = remove_liquidity_by_range2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_by_range2_invoke_signed(
    accounts: RemoveLiquidityByRange2Accounts<'_, '_>,
    args: RemoveLiquidityByRange2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_by_range2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity_by_range2_verify_account_keys(
    accounts: RemoveLiquidityByRange2Accounts<'_, '_>,
    keys: RemoveLiquidityByRange2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range2_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRange2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range2_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRange2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range2_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRange2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_by_range2_verify_writable_privileges(accounts)?;
    remove_liquidity_by_range2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetActivationPointAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetActivationPointKeys {
    pub lb_pair: Pubkey,
    pub signer: Pubkey,
}
impl From<SetActivationPointAccounts<'_, '_>> for SetActivationPointKeys {
    fn from(accounts: SetActivationPointAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<SetActivationPointKeys>
for [AccountMeta; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetActivationPointKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
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
impl From<[Pubkey; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN]> for SetActivationPointKeys {
    fn from(pubkeys: [Pubkey; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            signer: pubkeys[1],
        }
    }
}
impl<'info> From<SetActivationPointAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetActivationPointAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.signer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN]>
for SetActivationPointAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            signer: &arr[1],
        }
    }
}
pub const SET_ACTIVATION_POINT_IX_DISCM: [u8; 8usize] = [
    91, 249, 15, 165, 26, 129, 254, 125,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetActivationPointIxArgs {
    pub activation_point: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetActivationPointIxData(pub SetActivationPointIxArgs);
impl From<SetActivationPointIxArgs> for SetActivationPointIxData {
    fn from(args: SetActivationPointIxArgs) -> Self {
        Self(args)
    }
}
impl SetActivationPointIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ACTIVATION_POINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetActivationPointIxArgs {
                activation_point,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ACTIVATION_POINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.activation_point, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_activation_point_ix_with_program_id(
    program_id: Pubkey,
    keys: SetActivationPointKeys,
    args: SetActivationPointIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetActivationPointIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_activation_point_ix(
    keys: SetActivationPointKeys,
    args: SetActivationPointIxArgs,
) -> std::io::Result<Instruction> {
    set_activation_point_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn set_activation_point_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetActivationPointAccounts<'_, '_>,
    args: SetActivationPointIxArgs,
) -> ProgramResult {
    let keys: SetActivationPointKeys = accounts.into();
    let ix = set_activation_point_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_activation_point_invoke(
    accounts: SetActivationPointAccounts<'_, '_>,
    args: SetActivationPointIxArgs,
) -> ProgramResult {
    set_activation_point_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn set_activation_point_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetActivationPointAccounts<'_, '_>,
    args: SetActivationPointIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetActivationPointKeys = accounts.into();
    let ix = set_activation_point_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_activation_point_invoke_signed(
    accounts: SetActivationPointAccounts<'_, '_>,
    args: SetActivationPointIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_activation_point_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_activation_point_verify_account_keys(
    accounts: SetActivationPointAccounts<'_, '_>,
    keys: SetActivationPointKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_activation_point_verify_writable_privileges<'me, 'info>(
    accounts: SetActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_activation_point_verify_signer_privileges<'me, 'info>(
    accounts: SetActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_activation_point_verify_account_privileges<'me, 'info>(
    accounts: SetActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_activation_point_verify_writable_privileges(accounts)?;
    set_activation_point_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PAIR_STATUS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetPairStatusAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPairStatusKeys {
    pub lb_pair: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
}
impl From<SetPairStatusAccounts<'_, '_>> for SetPairStatusKeys {
    fn from(accounts: SetPairStatusAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<SetPairStatusKeys> for [AccountMeta; SET_PAIR_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPairStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
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
impl From<[Pubkey; SET_PAIR_STATUS_IX_ACCOUNTS_LEN]> for SetPairStatusKeys {
    fn from(pubkeys: [Pubkey; SET_PAIR_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            operator: pubkeys[1],
            signer: pubkeys[2],
        }
    }
}
impl<'info> From<SetPairStatusAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PAIR_STATUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPairStatusAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.operator.clone(), accounts.signer.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PAIR_STATUS_IX_ACCOUNTS_LEN]>
for SetPairStatusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PAIR_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            operator: &arr[1],
            signer: &arr[2],
        }
    }
}
pub const SET_PAIR_STATUS_IX_DISCM: [u8; 8usize] = [
    67, 248, 231, 137, 154, 149, 217, 174,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPairStatusIxArgs {
    pub status: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPairStatusIxData(pub SetPairStatusIxArgs);
impl From<SetPairStatusIxArgs> for SetPairStatusIxData {
    fn from(args: SetPairStatusIxArgs) -> Self {
        Self(args)
    }
}
impl SetPairStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PAIR_STATUS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetPairStatusIxArgs { status }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PAIR_STATUS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.status, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pair_status_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPairStatusKeys,
    args: SetPairStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PAIR_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPairStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pair_status_ix(
    keys: SetPairStatusKeys,
    args: SetPairStatusIxArgs,
) -> std::io::Result<Instruction> {
    set_pair_status_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn set_pair_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPairStatusAccounts<'_, '_>,
    args: SetPairStatusIxArgs,
) -> ProgramResult {
    let keys: SetPairStatusKeys = accounts.into();
    let ix = set_pair_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pair_status_invoke(
    accounts: SetPairStatusAccounts<'_, '_>,
    args: SetPairStatusIxArgs,
) -> ProgramResult {
    set_pair_status_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn set_pair_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPairStatusAccounts<'_, '_>,
    args: SetPairStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPairStatusKeys = accounts.into();
    let ix = set_pair_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pair_status_invoke_signed(
    accounts: SetPairStatusAccounts<'_, '_>,
    args: SetPairStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pair_status_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pair_status_verify_account_keys(
    accounts: SetPairStatusAccounts<'_, '_>,
    keys: SetPairStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pair_status_verify_writable_privileges<'me, 'info>(
    accounts: SetPairStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pair_status_verify_signer_privileges<'me, 'info>(
    accounts: SetPairStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pair_status_verify_account_privileges<'me, 'info>(
    accounts: SetPairStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pair_status_verify_writable_privileges(accounts)?;
    set_pair_status_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PAIR_STATUS_PERMISSIONLESS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPairStatusPermissionlessAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPairStatusPermissionlessKeys {
    pub lb_pair: Pubkey,
    pub signer: Pubkey,
}
impl From<SetPairStatusPermissionlessAccounts<'_, '_>>
for SetPairStatusPermissionlessKeys {
    fn from(accounts: SetPairStatusPermissionlessAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<SetPairStatusPermissionlessKeys>
for [AccountMeta; SET_PAIR_STATUS_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPairStatusPermissionlessKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
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
impl From<[Pubkey; SET_PAIR_STATUS_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for SetPairStatusPermissionlessKeys {
    fn from(pubkeys: [Pubkey; SET_PAIR_STATUS_PERMISSIONLESS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            signer: pubkeys[1],
        }
    }
}
impl<'info> From<SetPairStatusPermissionlessAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PAIR_STATUS_PERMISSIONLESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPairStatusPermissionlessAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.signer.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_PAIR_STATUS_PERMISSIONLESS_IX_ACCOUNTS_LEN]>
for SetPairStatusPermissionlessAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PAIR_STATUS_PERMISSIONLESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            signer: &arr[1],
        }
    }
}
pub const SET_PAIR_STATUS_PERMISSIONLESS_IX_DISCM: [u8; 8usize] = [
    78, 59, 152, 211, 70, 183, 46, 208,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPairStatusPermissionlessIxArgs {
    pub status: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPairStatusPermissionlessIxData(pub SetPairStatusPermissionlessIxArgs);
impl From<SetPairStatusPermissionlessIxArgs> for SetPairStatusPermissionlessIxData {
    fn from(args: SetPairStatusPermissionlessIxArgs) -> Self {
        Self(args)
    }
}
impl SetPairStatusPermissionlessIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PAIR_STATUS_PERMISSIONLESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPairStatusPermissionlessIxArgs {
                status,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PAIR_STATUS_PERMISSIONLESS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.status, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pair_status_permissionless_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPairStatusPermissionlessKeys,
    args: SetPairStatusPermissionlessIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PAIR_STATUS_PERMISSIONLESS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetPairStatusPermissionlessIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pair_status_permissionless_ix(
    keys: SetPairStatusPermissionlessKeys,
    args: SetPairStatusPermissionlessIxArgs,
) -> std::io::Result<Instruction> {
    set_pair_status_permissionless_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn set_pair_status_permissionless_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPairStatusPermissionlessAccounts<'_, '_>,
    args: SetPairStatusPermissionlessIxArgs,
) -> ProgramResult {
    let keys: SetPairStatusPermissionlessKeys = accounts.into();
    let ix = set_pair_status_permissionless_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pair_status_permissionless_invoke(
    accounts: SetPairStatusPermissionlessAccounts<'_, '_>,
    args: SetPairStatusPermissionlessIxArgs,
) -> ProgramResult {
    set_pair_status_permissionless_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_pair_status_permissionless_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPairStatusPermissionlessAccounts<'_, '_>,
    args: SetPairStatusPermissionlessIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPairStatusPermissionlessKeys = accounts.into();
    let ix = set_pair_status_permissionless_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pair_status_permissionless_invoke_signed(
    accounts: SetPairStatusPermissionlessAccounts<'_, '_>,
    args: SetPairStatusPermissionlessIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pair_status_permissionless_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pair_status_permissionless_verify_account_keys(
    accounts: SetPairStatusPermissionlessAccounts<'_, '_>,
    keys: SetPairStatusPermissionlessKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pair_status_permissionless_verify_writable_privileges<'me, 'info>(
    accounts: SetPairStatusPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pair_status_permissionless_verify_signer_privileges<'me, 'info>(
    accounts: SetPairStatusPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pair_status_permissionless_verify_account_privileges<'me, 'info>(
    accounts: SetPairStatusPermissionlessAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pair_status_permissionless_verify_writable_privileges(accounts)?;
    set_pair_status_permissionless_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PERMISSIONLESS_OPERATION_BITS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetPermissionlessOperationBitsAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPermissionlessOperationBitsKeys {
    pub position: Pubkey,
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetPermissionlessOperationBitsAccounts<'_, '_>>
for SetPermissionlessOperationBitsKeys {
    fn from(accounts: SetPermissionlessOperationBitsAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            owner: *accounts.owner.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetPermissionlessOperationBitsKeys>
for [AccountMeta; SET_PERMISSIONLESS_OPERATION_BITS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPermissionlessOperationBitsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; SET_PERMISSIONLESS_OPERATION_BITS_IX_ACCOUNTS_LEN]>
for SetPermissionlessOperationBitsKeys {
    fn from(
        pubkeys: [Pubkey; SET_PERMISSIONLESS_OPERATION_BITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: pubkeys[0],
            owner: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<SetPermissionlessOperationBitsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PERMISSIONLESS_OPERATION_BITS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPermissionlessOperationBitsAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.owner.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_PERMISSIONLESS_OPERATION_BITS_IX_ACCOUNTS_LEN]>
for SetPermissionlessOperationBitsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PERMISSIONLESS_OPERATION_BITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            owner: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const SET_PERMISSIONLESS_OPERATION_BITS_IX_DISCM: [u8; 8usize] = [
    84, 58, 203, 139, 163, 81, 190, 186,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPermissionlessOperationBitsIxArgs {
    pub bits: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPermissionlessOperationBitsIxData(
    pub SetPermissionlessOperationBitsIxArgs,
);
impl From<SetPermissionlessOperationBitsIxArgs>
for SetPermissionlessOperationBitsIxData {
    fn from(args: SetPermissionlessOperationBitsIxArgs) -> Self {
        Self(args)
    }
}
impl SetPermissionlessOperationBitsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PERMISSIONLESS_OPERATION_BITS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bits: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPermissionlessOperationBitsIxArgs {
                bits,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PERMISSIONLESS_OPERATION_BITS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bits, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_permissionless_operation_bits_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPermissionlessOperationBitsKeys,
    args: SetPermissionlessOperationBitsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PERMISSIONLESS_OPERATION_BITS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetPermissionlessOperationBitsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_permissionless_operation_bits_ix(
    keys: SetPermissionlessOperationBitsKeys,
    args: SetPermissionlessOperationBitsIxArgs,
) -> std::io::Result<Instruction> {
    set_permissionless_operation_bits_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn set_permissionless_operation_bits_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPermissionlessOperationBitsAccounts<'_, '_>,
    args: SetPermissionlessOperationBitsIxArgs,
) -> ProgramResult {
    let keys: SetPermissionlessOperationBitsKeys = accounts.into();
    let ix = set_permissionless_operation_bits_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn set_permissionless_operation_bits_invoke(
    accounts: SetPermissionlessOperationBitsAccounts<'_, '_>,
    args: SetPermissionlessOperationBitsIxArgs,
) -> ProgramResult {
    set_permissionless_operation_bits_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_permissionless_operation_bits_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPermissionlessOperationBitsAccounts<'_, '_>,
    args: SetPermissionlessOperationBitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPermissionlessOperationBitsKeys = accounts.into();
    let ix = set_permissionless_operation_bits_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_permissionless_operation_bits_invoke_signed(
    accounts: SetPermissionlessOperationBitsAccounts<'_, '_>,
    args: SetPermissionlessOperationBitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_permissionless_operation_bits_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_permissionless_operation_bits_verify_account_keys(
    accounts: SetPermissionlessOperationBitsAccounts<'_, '_>,
    keys: SetPermissionlessOperationBitsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_permissionless_operation_bits_verify_writable_privileges<'me, 'info>(
    accounts: SetPermissionlessOperationBitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_permissionless_operation_bits_verify_signer_privileges<'me, 'info>(
    accounts: SetPermissionlessOperationBitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_permissionless_operation_bits_verify_account_privileges<'me, 'info>(
    accounts: SetPermissionlessOperationBitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_permissionless_operation_bits_verify_writable_privileges(accounts)?;
    set_permissionless_operation_bits_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPreActivationDurationAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPreActivationDurationKeys {
    pub lb_pair: Pubkey,
    pub signer: Pubkey,
}
impl From<SetPreActivationDurationAccounts<'_, '_>> for SetPreActivationDurationKeys {
    fn from(accounts: SetPreActivationDurationAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<SetPreActivationDurationKeys>
for [AccountMeta; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPreActivationDurationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
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
impl From<[Pubkey; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN]>
for SetPreActivationDurationKeys {
    fn from(pubkeys: [Pubkey; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            signer: pubkeys[1],
        }
    }
}
impl<'info> From<SetPreActivationDurationAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPreActivationDurationAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.signer.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN]>
for SetPreActivationDurationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            signer: &arr[1],
        }
    }
}
pub const SET_PRE_ACTIVATION_DURATION_IX_DISCM: [u8; 8usize] = [
    165, 61, 201, 244, 130, 159, 22, 100,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPreActivationDurationIxArgs {
    pub pre_activation_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPreActivationDurationIxData(pub SetPreActivationDurationIxArgs);
impl From<SetPreActivationDurationIxArgs> for SetPreActivationDurationIxData {
    fn from(args: SetPreActivationDurationIxArgs) -> Self {
        Self(args)
    }
}
impl SetPreActivationDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PRE_ACTIVATION_DURATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pre_activation_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPreActivationDurationIxArgs {
                pre_activation_duration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PRE_ACTIVATION_DURATION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pre_activation_duration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pre_activation_duration_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPreActivationDurationKeys,
    args: SetPreActivationDurationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPreActivationDurationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pre_activation_duration_ix(
    keys: SetPreActivationDurationKeys,
    args: SetPreActivationDurationIxArgs,
) -> std::io::Result<Instruction> {
    set_pre_activation_duration_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn set_pre_activation_duration_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    args: SetPreActivationDurationIxArgs,
) -> ProgramResult {
    let keys: SetPreActivationDurationKeys = accounts.into();
    let ix = set_pre_activation_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pre_activation_duration_invoke(
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    args: SetPreActivationDurationIxArgs,
) -> ProgramResult {
    set_pre_activation_duration_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_pre_activation_duration_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    args: SetPreActivationDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPreActivationDurationKeys = accounts.into();
    let ix = set_pre_activation_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pre_activation_duration_invoke_signed(
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    args: SetPreActivationDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pre_activation_duration_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pre_activation_duration_verify_account_keys(
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    keys: SetPreActivationDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pre_activation_duration_verify_writable_privileges<'me, 'info>(
    accounts: SetPreActivationDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pre_activation_duration_verify_signer_privileges<'me, 'info>(
    accounts: SetPreActivationDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pre_activation_duration_verify_account_privileges<'me, 'info>(
    accounts: SetPreActivationDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pre_activation_duration_verify_writable_privileges(accounts)?;
    set_pre_activation_duration_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPreActivationSwapAddressAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPreActivationSwapAddressKeys {
    pub lb_pair: Pubkey,
    pub signer: Pubkey,
}
impl From<SetPreActivationSwapAddressAccounts<'_, '_>>
for SetPreActivationSwapAddressKeys {
    fn from(accounts: SetPreActivationSwapAddressAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            signer: *accounts.signer.key,
        }
    }
}
impl From<SetPreActivationSwapAddressKeys>
for [AccountMeta; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPreActivationSwapAddressKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
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
impl From<[Pubkey; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN]>
for SetPreActivationSwapAddressKeys {
    fn from(pubkeys: [Pubkey; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            signer: pubkeys[1],
        }
    }
}
impl<'info> From<SetPreActivationSwapAddressAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPreActivationSwapAddressAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.signer.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN]>
for SetPreActivationSwapAddressAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            signer: &arr[1],
        }
    }
}
pub const SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM: [u8; 8usize] = [
    57, 139, 47, 123, 216, 80, 223, 10,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPreActivationSwapAddressIxArgs {
    pub pre_activation_swap_address: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPreActivationSwapAddressIxData(pub SetPreActivationSwapAddressIxArgs);
impl From<SetPreActivationSwapAddressIxArgs> for SetPreActivationSwapAddressIxData {
    fn from(args: SetPreActivationSwapAddressIxArgs) -> Self {
        Self(args)
    }
}
impl SetPreActivationSwapAddressIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pre_activation_swap_address: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SetPreActivationSwapAddressIxArgs {
                pre_activation_swap_address,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.pre_activation_swap_address,
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
pub fn set_pre_activation_swap_address_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPreActivationSwapAddressKeys,
    args: SetPreActivationSwapAddressIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetPreActivationSwapAddressIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pre_activation_swap_address_ix(
    keys: SetPreActivationSwapAddressKeys,
    args: SetPreActivationSwapAddressIxArgs,
) -> std::io::Result<Instruction> {
    set_pre_activation_swap_address_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn set_pre_activation_swap_address_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    args: SetPreActivationSwapAddressIxArgs,
) -> ProgramResult {
    let keys: SetPreActivationSwapAddressKeys = accounts.into();
    let ix = set_pre_activation_swap_address_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pre_activation_swap_address_invoke(
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    args: SetPreActivationSwapAddressIxArgs,
) -> ProgramResult {
    set_pre_activation_swap_address_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_pre_activation_swap_address_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    args: SetPreActivationSwapAddressIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPreActivationSwapAddressKeys = accounts.into();
    let ix = set_pre_activation_swap_address_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pre_activation_swap_address_invoke_signed(
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    args: SetPreActivationSwapAddressIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pre_activation_swap_address_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pre_activation_swap_address_verify_account_keys(
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    keys: SetPreActivationSwapAddressKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.signer.key, keys.signer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pre_activation_swap_address_verify_writable_privileges<'me, 'info>(
    accounts: SetPreActivationSwapAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pre_activation_swap_address_verify_signer_privileges<'me, 'info>(
    accounts: SetPreActivationSwapAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pre_activation_swap_address_verify_account_privileges<'me, 'info>(
    accounts: SetPreActivationSwapAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pre_activation_swap_address_verify_writable_privileges(accounts)?;
    set_pre_activation_swap_address_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapIxData(pub SwapIxArgs);
impl From<SwapIxArgs> for SwapIxData {
    fn from(args: SwapIxArgs) -> Self {
        Self(args)
    }
}
impl SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapKeys,
    args: SwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_ix(keys: SwapKeys, args: SwapIxArgs) -> std::io::Result<Instruction> {
    swap_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_invoke(accounts: SwapAccounts<'_, '_>, args: SwapIxArgs) -> ProgramResult {
    swap_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_invoke_signed(
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_verify_signer_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_verify_writable_privileges(accounts)?;
    swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct Swap2Accounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Swap2Keys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<Swap2Accounts<'_, '_>> for Swap2Keys {
    fn from(accounts: Swap2Accounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<Swap2Keys> for [AccountMeta; SWAP2_IX_ACCOUNTS_LEN] {
    fn from(keys: Swap2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            memo_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<Swap2Accounts<'_, 'info>>
for [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN] {
    fn from(accounts: Swap2Accounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN]>
for Swap2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            memo_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const SWAP2_IX_DISCM: [u8; 8usize] = [65, 75, 63, 76, 235, 91, 91, 136];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Swap2IxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
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
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(Swap2IxArgs {
                amount_in,
                min_amount_out,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
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
    swap2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
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
    swap2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
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
    swap2_invoke_signed_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap2_verify_account_keys(
    accounts: Swap2Accounts<'_, '_>,
    keys: Swap2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
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
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
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
    for should_be_signer in [accounts.user] {
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
pub const SWAP_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactOutAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactOutKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapExactOutAccounts<'_, '_>> for SwapExactOutKeys {
    fn from(accounts: SwapExactOutAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapExactOutKeys> for [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]> for SwapExactOutKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]>
for SwapExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SWAP_EXACT_OUT_IX_DISCM: [u8; 8usize] = [250, 73, 101, 33, 38, 207, 75, 184];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactOutIxArgs {
    pub max_in_amount: u64,
    pub out_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactOutIxData(pub SwapExactOutIxArgs);
impl From<SwapExactOutIxArgs> for SwapExactOutIxData {
    fn from(args: SwapExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapExactOutIxArgs {
                max_in_amount,
                out_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.out_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactOutKeys,
    args: SwapExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_out_ix(
    keys: SwapExactOutKeys,
    args: SwapExactOutIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_out_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn swap_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
) -> ProgramResult {
    let keys: SwapExactOutKeys = accounts.into();
    let ix = swap_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_out_invoke(
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
) -> ProgramResult {
    swap_exact_out_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn swap_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactOutKeys = accounts.into();
    let ix = swap_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_out_invoke_signed(
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_out_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exact_out_verify_account_keys(
    accounts: SwapExactOutAccounts<'_, '_>,
    keys: SwapExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_account_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_out_verify_writable_privileges(accounts)?;
    swap_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXACT_OUT2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactOut2Accounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactOut2Keys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapExactOut2Accounts<'_, '_>> for SwapExactOut2Keys {
    fn from(accounts: SwapExactOut2Accounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapExactOut2Keys> for [AccountMeta; SWAP_EXACT_OUT2_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactOut2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
impl From<[Pubkey; SWAP_EXACT_OUT2_IX_ACCOUNTS_LEN]> for SwapExactOut2Keys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_OUT2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            memo_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<SwapExactOut2Accounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_OUT2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactOut2Accounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_OUT2_IX_ACCOUNTS_LEN]>
for SwapExactOut2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXACT_OUT2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            memo_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const SWAP_EXACT_OUT2_IX_DISCM: [u8; 8usize] = [43, 215, 247, 132, 137, 60, 243, 81];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactOut2IxArgs {
    pub max_in_amount: u64,
    pub out_amount: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactOut2IxData(pub SwapExactOut2IxArgs);
impl From<SwapExactOut2IxArgs> for SwapExactOut2IxData {
    fn from(args: SwapExactOut2IxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactOut2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_OUT2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(SwapExactOut2IxArgs {
                max_in_amount,
                out_amount,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_OUT2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_out2_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactOut2Keys,
    args: SwapExactOut2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_OUT2_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactOut2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_out2_ix(
    keys: SwapExactOut2Keys,
    args: SwapExactOut2IxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_out2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn swap_exact_out2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOut2Accounts<'_, '_>,
    args: SwapExactOut2IxArgs,
) -> ProgramResult {
    let keys: SwapExactOut2Keys = accounts.into();
    let ix = swap_exact_out2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_out2_invoke(
    accounts: SwapExactOut2Accounts<'_, '_>,
    args: SwapExactOut2IxArgs,
) -> ProgramResult {
    swap_exact_out2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn swap_exact_out2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOut2Accounts<'_, '_>,
    args: SwapExactOut2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactOut2Keys = accounts.into();
    let ix = swap_exact_out2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_out2_invoke_signed(
    accounts: SwapExactOut2Accounts<'_, '_>,
    args: SwapExactOut2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_out2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exact_out2_verify_account_keys(
    accounts: SwapExactOut2Accounts<'_, '_>,
    keys: SwapExactOut2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_out2_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactOut2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_out2_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactOut2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_out2_verify_account_privileges<'me, 'info>(
    accounts: SwapExactOut2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_out2_verify_writable_privileges(accounts)?;
    swap_exact_out2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapWithPriceImpactAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapWithPriceImpactKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapWithPriceImpactAccounts<'_, '_>> for SwapWithPriceImpactKeys {
    fn from(accounts: SwapWithPriceImpactAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapWithPriceImpactKeys>
for [AccountMeta; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapWithPriceImpactKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
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
impl From<[Pubkey; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN]> for SwapWithPriceImpactKeys {
    fn from(pubkeys: [Pubkey; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapWithPriceImpactAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapWithPriceImpactAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN]>
for SwapWithPriceImpactAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SWAP_WITH_PRICE_IMPACT_IX_DISCM: [u8; 8usize] = [
    56, 173, 230, 208, 173, 228, 156, 205,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapWithPriceImpactIxArgs {
    pub amount_in: u64,
    pub active_id: Option<i32>,
    pub max_price_impact_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithPriceImpactIxData(pub SwapWithPriceImpactIxArgs);
impl From<SwapWithPriceImpactIxArgs> for SwapWithPriceImpactIxData {
    fn from(args: SwapWithPriceImpactIxArgs) -> Self {
        Self(args)
    }
}
impl SwapWithPriceImpactIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_WITH_PRICE_IMPACT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_id: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
        let max_price_impact_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapWithPriceImpactIxArgs {
                amount_in,
                active_id,
                max_price_impact_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_WITH_PRICE_IMPACT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_price_impact_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_with_price_impact_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapWithPriceImpactKeys,
    args: SwapWithPriceImpactIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapWithPriceImpactIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_with_price_impact_ix(
    keys: SwapWithPriceImpactKeys,
    args: SwapWithPriceImpactIxArgs,
) -> std::io::Result<Instruction> {
    swap_with_price_impact_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn swap_with_price_impact_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    args: SwapWithPriceImpactIxArgs,
) -> ProgramResult {
    let keys: SwapWithPriceImpactKeys = accounts.into();
    let ix = swap_with_price_impact_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_with_price_impact_invoke(
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    args: SwapWithPriceImpactIxArgs,
) -> ProgramResult {
    swap_with_price_impact_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn swap_with_price_impact_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    args: SwapWithPriceImpactIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapWithPriceImpactKeys = accounts.into();
    let ix = swap_with_price_impact_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_with_price_impact_invoke_signed(
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    args: SwapWithPriceImpactIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_with_price_impact_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_with_price_impact_verify_account_keys(
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    keys: SwapWithPriceImpactKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact_verify_writable_privileges<'me, 'info>(
    accounts: SwapWithPriceImpactAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact_verify_signer_privileges<'me, 'info>(
    accounts: SwapWithPriceImpactAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact_verify_account_privileges<'me, 'info>(
    accounts: SwapWithPriceImpactAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_with_price_impact_verify_writable_privileges(accounts)?;
    swap_with_price_impact_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_WITH_PRICE_IMPACT2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct SwapWithPriceImpact2Accounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapWithPriceImpact2Keys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapWithPriceImpact2Accounts<'_, '_>> for SwapWithPriceImpact2Keys {
    fn from(accounts: SwapWithPriceImpact2Accounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapWithPriceImpact2Keys>
for [AccountMeta; SWAP_WITH_PRICE_IMPACT2_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapWithPriceImpact2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
impl From<[Pubkey; SWAP_WITH_PRICE_IMPACT2_IX_ACCOUNTS_LEN]>
for SwapWithPriceImpact2Keys {
    fn from(pubkeys: [Pubkey; SWAP_WITH_PRICE_IMPACT2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            memo_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<SwapWithPriceImpact2Accounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapWithPriceImpact2Accounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT2_IX_ACCOUNTS_LEN]>
for SwapWithPriceImpact2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            memo_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const SWAP_WITH_PRICE_IMPACT2_IX_DISCM: [u8; 8usize] = [
    74, 98, 192, 214, 177, 51, 75, 51,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapWithPriceImpact2IxArgs {
    pub amount_in: u64,
    pub active_id: Option<i32>,
    pub max_price_impact_bps: u16,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithPriceImpact2IxData(pub SwapWithPriceImpact2IxArgs);
impl From<SwapWithPriceImpact2IxArgs> for SwapWithPriceImpact2IxData {
    fn from(args: SwapWithPriceImpact2IxArgs) -> Self {
        Self(args)
    }
}
impl SwapWithPriceImpact2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_WITH_PRICE_IMPACT2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_id: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
        let max_price_impact_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(SwapWithPriceImpact2IxArgs {
                amount_in,
                active_id,
                max_price_impact_bps,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_WITH_PRICE_IMPACT2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_price_impact_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_with_price_impact2_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapWithPriceImpact2Keys,
    args: SwapWithPriceImpact2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_WITH_PRICE_IMPACT2_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapWithPriceImpact2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_with_price_impact2_ix(
    keys: SwapWithPriceImpact2Keys,
    args: SwapWithPriceImpact2IxArgs,
) -> std::io::Result<Instruction> {
    swap_with_price_impact2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn swap_with_price_impact2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPriceImpact2Accounts<'_, '_>,
    args: SwapWithPriceImpact2IxArgs,
) -> ProgramResult {
    let keys: SwapWithPriceImpact2Keys = accounts.into();
    let ix = swap_with_price_impact2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_with_price_impact2_invoke(
    accounts: SwapWithPriceImpact2Accounts<'_, '_>,
    args: SwapWithPriceImpact2IxArgs,
) -> ProgramResult {
    swap_with_price_impact2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn swap_with_price_impact2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPriceImpact2Accounts<'_, '_>,
    args: SwapWithPriceImpact2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapWithPriceImpact2Keys = accounts.into();
    let ix = swap_with_price_impact2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_with_price_impact2_invoke_signed(
    accounts: SwapWithPriceImpact2Accounts<'_, '_>,
    args: SwapWithPriceImpact2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_with_price_impact2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_with_price_impact2_verify_account_keys(
    accounts: SwapWithPriceImpact2Accounts<'_, '_>,
    keys: SwapWithPriceImpact2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact2_verify_writable_privileges<'me, 'info>(
    accounts: SwapWithPriceImpact2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact2_verify_signer_privileges<'me, 'info>(
    accounts: SwapWithPriceImpact2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact2_verify_account_privileges<'me, 'info>(
    accounts: SwapWithPriceImpact2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_with_price_impact2_verify_writable_privileges(accounts)?;
    swap_with_price_impact2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_BASE_FEE_PARAMETERS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateBaseFeeParametersAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateBaseFeeParametersKeys {
    pub lb_pair: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateBaseFeeParametersAccounts<'_, '_>> for UpdateBaseFeeParametersKeys {
    fn from(accounts: UpdateBaseFeeParametersAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateBaseFeeParametersKeys>
for [AccountMeta; UPDATE_BASE_FEE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateBaseFeeParametersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_BASE_FEE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateBaseFeeParametersKeys {
    fn from(pubkeys: [Pubkey; UPDATE_BASE_FEE_PARAMETERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            operator: pubkeys[1],
            signer: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateBaseFeeParametersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_BASE_FEE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateBaseFeeParametersAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_BASE_FEE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateBaseFeeParametersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_BASE_FEE_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            operator: &arr[1],
            signer: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_BASE_FEE_PARAMETERS_IX_DISCM: [u8; 8usize] = [
    75, 168, 223, 161, 16, 195, 3, 47,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateBaseFeeParametersIxArgs {
    pub fee_parameter: BaseFeeParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateBaseFeeParametersIxData(pub UpdateBaseFeeParametersIxArgs);
impl From<UpdateBaseFeeParametersIxArgs> for UpdateBaseFeeParametersIxData {
    fn from(args: UpdateBaseFeeParametersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateBaseFeeParametersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_BASE_FEE_PARAMETERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_parameter = if reader.is_empty() {
            Default::default()
        } else {
            <BaseFeeParameter>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateBaseFeeParametersIxArgs {
                fee_parameter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_BASE_FEE_PARAMETERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_parameter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_base_fee_parameters_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateBaseFeeParametersKeys,
    args: UpdateBaseFeeParametersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_BASE_FEE_PARAMETERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateBaseFeeParametersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_base_fee_parameters_ix(
    keys: UpdateBaseFeeParametersKeys,
    args: UpdateBaseFeeParametersIxArgs,
) -> std::io::Result<Instruction> {
    update_base_fee_parameters_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn update_base_fee_parameters_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateBaseFeeParametersAccounts<'_, '_>,
    args: UpdateBaseFeeParametersIxArgs,
) -> ProgramResult {
    let keys: UpdateBaseFeeParametersKeys = accounts.into();
    let ix = update_base_fee_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_base_fee_parameters_invoke(
    accounts: UpdateBaseFeeParametersAccounts<'_, '_>,
    args: UpdateBaseFeeParametersIxArgs,
) -> ProgramResult {
    update_base_fee_parameters_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn update_base_fee_parameters_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateBaseFeeParametersAccounts<'_, '_>,
    args: UpdateBaseFeeParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateBaseFeeParametersKeys = accounts.into();
    let ix = update_base_fee_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_base_fee_parameters_invoke_signed(
    accounts: UpdateBaseFeeParametersAccounts<'_, '_>,
    args: UpdateBaseFeeParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_base_fee_parameters_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_base_fee_parameters_verify_account_keys(
    accounts: UpdateBaseFeeParametersAccounts<'_, '_>,
    keys: UpdateBaseFeeParametersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_base_fee_parameters_verify_writable_privileges<'me, 'info>(
    accounts: UpdateBaseFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_base_fee_parameters_verify_signer_privileges<'me, 'info>(
    accounts: UpdateBaseFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_base_fee_parameters_verify_account_privileges<'me, 'info>(
    accounts: UpdateBaseFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_base_fee_parameters_verify_writable_privileges(accounts)?;
    update_base_fee_parameters_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_DYNAMIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDynamicFeeParametersAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDynamicFeeParametersKeys {
    pub lb_pair: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateDynamicFeeParametersAccounts<'_, '_>>
for UpdateDynamicFeeParametersKeys {
    fn from(accounts: UpdateDynamicFeeParametersAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateDynamicFeeParametersKeys>
for [AccountMeta; UPDATE_DYNAMIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateDynamicFeeParametersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_DYNAMIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateDynamicFeeParametersKeys {
    fn from(pubkeys: [Pubkey; UPDATE_DYNAMIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            operator: pubkeys[1],
            signer: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateDynamicFeeParametersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_DYNAMIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateDynamicFeeParametersAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_DYNAMIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateDynamicFeeParametersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_DYNAMIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            operator: &arr[1],
            signer: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_DYNAMIC_FEE_PARAMETERS_IX_DISCM: [u8; 8usize] = [
    92, 161, 46, 246, 255, 189, 22, 22,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDynamicFeeParametersIxArgs {
    pub fee_parameter: DynamicFeeParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDynamicFeeParametersIxData(pub UpdateDynamicFeeParametersIxArgs);
impl From<UpdateDynamicFeeParametersIxArgs> for UpdateDynamicFeeParametersIxData {
    fn from(args: UpdateDynamicFeeParametersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateDynamicFeeParametersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DYNAMIC_FEE_PARAMETERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_parameter = if reader.is_empty() {
            Default::default()
        } else {
            <DynamicFeeParameter>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateDynamicFeeParametersIxArgs {
                fee_parameter,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DYNAMIC_FEE_PARAMETERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_parameter, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_dynamic_fee_parameters_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateDynamicFeeParametersKeys,
    args: UpdateDynamicFeeParametersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_DYNAMIC_FEE_PARAMETERS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateDynamicFeeParametersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_dynamic_fee_parameters_ix(
    keys: UpdateDynamicFeeParametersKeys,
    args: UpdateDynamicFeeParametersIxArgs,
) -> std::io::Result<Instruction> {
    update_dynamic_fee_parameters_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn update_dynamic_fee_parameters_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDynamicFeeParametersAccounts<'_, '_>,
    args: UpdateDynamicFeeParametersIxArgs,
) -> ProgramResult {
    let keys: UpdateDynamicFeeParametersKeys = accounts.into();
    let ix = update_dynamic_fee_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_dynamic_fee_parameters_invoke(
    accounts: UpdateDynamicFeeParametersAccounts<'_, '_>,
    args: UpdateDynamicFeeParametersIxArgs,
) -> ProgramResult {
    update_dynamic_fee_parameters_invoke_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_dynamic_fee_parameters_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDynamicFeeParametersAccounts<'_, '_>,
    args: UpdateDynamicFeeParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateDynamicFeeParametersKeys = accounts.into();
    let ix = update_dynamic_fee_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_dynamic_fee_parameters_invoke_signed(
    accounts: UpdateDynamicFeeParametersAccounts<'_, '_>,
    args: UpdateDynamicFeeParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_dynamic_fee_parameters_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_dynamic_fee_parameters_verify_account_keys(
    accounts: UpdateDynamicFeeParametersAccounts<'_, '_>,
    keys: UpdateDynamicFeeParametersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_dynamic_fee_parameters_verify_writable_privileges<'me, 'info>(
    accounts: UpdateDynamicFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_dynamic_fee_parameters_verify_signer_privileges<'me, 'info>(
    accounts: UpdateDynamicFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_dynamic_fee_parameters_verify_account_privileges<'me, 'info>(
    accounts: UpdateDynamicFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_dynamic_fee_parameters_verify_writable_privileges(accounts)?;
    update_dynamic_fee_parameters_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEES_AND_REWARD2_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeesAndReward2Accounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeesAndReward2Keys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
}
impl From<UpdateFeesAndReward2Accounts<'_, '_>> for UpdateFeesAndReward2Keys {
    fn from(accounts: UpdateFeesAndReward2Accounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<UpdateFeesAndReward2Keys>
for [AccountMeta; UPDATE_FEES_AND_REWARD2_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeesAndReward2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEES_AND_REWARD2_IX_ACCOUNTS_LEN]>
for UpdateFeesAndReward2Keys {
    fn from(pubkeys: [Pubkey; UPDATE_FEES_AND_REWARD2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            owner: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateFeesAndReward2Accounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEES_AND_REWARD2_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeesAndReward2Accounts<'_, 'info>) -> Self {
        [accounts.position.clone(), accounts.lb_pair.clone(), accounts.owner.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEES_AND_REWARD2_IX_ACCOUNTS_LEN]>
for UpdateFeesAndReward2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FEES_AND_REWARD2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            owner: &arr[2],
        }
    }
}
pub const UPDATE_FEES_AND_REWARD2_IX_DISCM: [u8; 8usize] = [
    32, 142, 184, 154, 103, 65, 184, 88,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeesAndReward2IxArgs {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeesAndReward2IxData(pub UpdateFeesAndReward2IxArgs);
impl From<UpdateFeesAndReward2IxArgs> for UpdateFeesAndReward2IxData {
    fn from(args: UpdateFeesAndReward2IxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFeesAndReward2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEES_AND_REWARD2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateFeesAndReward2IxArgs {
                min_bin_id,
                max_bin_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEES_AND_REWARD2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.min_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_bin_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fees_and_reward2_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeesAndReward2Keys,
    args: UpdateFeesAndReward2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEES_AND_REWARD2_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFeesAndReward2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fees_and_reward2_ix(
    keys: UpdateFeesAndReward2Keys,
    args: UpdateFeesAndReward2IxArgs,
) -> std::io::Result<Instruction> {
    update_fees_and_reward2_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn update_fees_and_reward2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAndReward2Accounts<'_, '_>,
    args: UpdateFeesAndReward2IxArgs,
) -> ProgramResult {
    let keys: UpdateFeesAndReward2Keys = accounts.into();
    let ix = update_fees_and_reward2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fees_and_reward2_invoke(
    accounts: UpdateFeesAndReward2Accounts<'_, '_>,
    args: UpdateFeesAndReward2IxArgs,
) -> ProgramResult {
    update_fees_and_reward2_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn update_fees_and_reward2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAndReward2Accounts<'_, '_>,
    args: UpdateFeesAndReward2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeesAndReward2Keys = accounts.into();
    let ix = update_fees_and_reward2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fees_and_reward2_invoke_signed(
    accounts: UpdateFeesAndReward2Accounts<'_, '_>,
    args: UpdateFeesAndReward2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fees_and_reward2_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_fees_and_reward2_verify_account_keys(
    accounts: UpdateFeesAndReward2Accounts<'_, '_>,
    keys: UpdateFeesAndReward2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fees_and_reward2_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeesAndReward2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position, accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fees_and_reward2_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeesAndReward2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fees_and_reward2_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeesAndReward2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fees_and_reward2_verify_writable_privileges(accounts)?;
    update_fees_and_reward2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeesAndRewardsAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeesAndRewardsKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub owner: Pubkey,
}
impl From<UpdateFeesAndRewardsAccounts<'_, '_>> for UpdateFeesAndRewardsKeys {
    fn from(accounts: UpdateFeesAndRewardsAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<UpdateFeesAndRewardsKeys>
for [AccountMeta; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeesAndRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]>
for UpdateFeesAndRewardsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_lower: pubkeys[2],
            bin_array_upper: pubkeys[3],
            owner: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateFeesAndRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeesAndRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]>
for UpdateFeesAndRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_lower: &arr[2],
            bin_array_upper: &arr[3],
            owner: &arr[4],
        }
    }
}
pub const UPDATE_FEES_AND_REWARDS_IX_DISCM: [u8; 8usize] = [
    154, 230, 250, 13, 236, 209, 75, 223,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeesAndRewardsIxData;
impl UpdateFeesAndRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEES_AND_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEES_AND_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fees_and_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeesAndRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateFeesAndRewardsIxData.try_to_vec()?,
    })
}
pub fn update_fees_and_rewards_ix(
    keys: UpdateFeesAndRewardsKeys,
) -> std::io::Result<Instruction> {
    update_fees_and_rewards_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys)
}
pub fn update_fees_and_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateFeesAndRewardsKeys = accounts.into();
    let ix = update_fees_and_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fees_and_rewards_invoke(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
) -> ProgramResult {
    update_fees_and_rewards_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts)
}
pub fn update_fees_and_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeesAndRewardsKeys = accounts.into();
    let ix = update_fees_and_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fees_and_rewards_invoke_signed(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fees_and_rewards_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_fees_and_rewards_verify_account_keys(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    keys: UpdateFeesAndRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fees_and_rewards_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fees_and_rewards_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fees_and_rewards_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fees_and_rewards_verify_writable_privileges(accounts)?;
    update_fees_and_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePositionOperatorAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePositionOperatorKeys {
    pub position: Pubkey,
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdatePositionOperatorAccounts<'_, '_>> for UpdatePositionOperatorKeys {
    fn from(accounts: UpdatePositionOperatorAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            owner: *accounts.owner.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdatePositionOperatorKeys>
for [AccountMeta; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePositionOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN]>
for UpdatePositionOperatorKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            owner: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdatePositionOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePositionOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.owner.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN]>
for UpdatePositionOperatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            owner: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_POSITION_OPERATOR_IX_DISCM: [u8; 8usize] = [
    202, 184, 103, 143, 180, 191, 116, 217,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePositionOperatorIxArgs {
    pub operator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePositionOperatorIxData(pub UpdatePositionOperatorIxArgs);
impl From<UpdatePositionOperatorIxArgs> for UpdatePositionOperatorIxData {
    fn from(args: UpdatePositionOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePositionOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POSITION_OPERATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdatePositionOperatorIxArgs {
                operator,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POSITION_OPERATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.operator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_position_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePositionOperatorKeys,
    args: UpdatePositionOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePositionOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_position_operator_ix(
    keys: UpdatePositionOperatorKeys,
    args: UpdatePositionOperatorIxArgs,
) -> std::io::Result<Instruction> {
    update_position_operator_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn update_position_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    args: UpdatePositionOperatorIxArgs,
) -> ProgramResult {
    let keys: UpdatePositionOperatorKeys = accounts.into();
    let ix = update_position_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_position_operator_invoke(
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    args: UpdatePositionOperatorIxArgs,
) -> ProgramResult {
    update_position_operator_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn update_position_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    args: UpdatePositionOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePositionOperatorKeys = accounts.into();
    let ix = update_position_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_position_operator_invoke_signed(
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    args: UpdatePositionOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_position_operator_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_position_operator_verify_account_keys(
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    keys: UpdatePositionOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_position_operator_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePositionOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_position_operator_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePositionOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_position_operator_verify_account_privileges<'me, 'info>(
    accounts: UpdatePositionOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_position_operator_verify_writable_privileges(accounts)?;
    update_position_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardDurationAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardDurationKeys {
    pub lb_pair: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub bin_array: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateRewardDurationAccounts<'_, '_>> for UpdateRewardDurationKeys {
    fn from(accounts: UpdateRewardDurationAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            bin_array: *accounts.bin_array.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateRewardDurationKeys>
for [AccountMeta; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardDurationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
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
impl From<[Pubkey; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]>
for UpdateRewardDurationKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            operator: pubkeys[1],
            signer: pubkeys[2],
            bin_array: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateRewardDurationAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardDurationAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.bin_array.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]>
for UpdateRewardDurationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            operator: &arr[1],
            signer: &arr[2],
            bin_array: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_REWARD_DURATION_IX_DISCM: [u8; 8usize] = [
    138, 174, 196, 169, 213, 235, 254, 107,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardDurationIxArgs {
    pub reward_index: u64,
    pub new_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardDurationIxData(pub UpdateRewardDurationIxArgs);
impl From<UpdateRewardDurationIxArgs> for UpdateRewardDurationIxData {
    fn from(args: UpdateRewardDurationIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRewardDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_DURATION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRewardDurationIxArgs {
                reward_index,
                new_duration,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_DURATION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_duration, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reward_duration_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewardDurationKeys,
    args: UpdateRewardDurationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRewardDurationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_reward_duration_ix(
    keys: UpdateRewardDurationKeys,
    args: UpdateRewardDurationIxArgs,
) -> std::io::Result<Instruction> {
    update_reward_duration_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn update_reward_duration_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    args: UpdateRewardDurationIxArgs,
) -> ProgramResult {
    let keys: UpdateRewardDurationKeys = accounts.into();
    let ix = update_reward_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reward_duration_invoke(
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    args: UpdateRewardDurationIxArgs,
) -> ProgramResult {
    update_reward_duration_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn update_reward_duration_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    args: UpdateRewardDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewardDurationKeys = accounts.into();
    let ix = update_reward_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reward_duration_invoke_signed(
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    args: UpdateRewardDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reward_duration_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_reward_duration_verify_account_keys(
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    keys: UpdateRewardDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reward_duration_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair, accounts.bin_array] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reward_duration_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRewardDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_reward_duration_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewardDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reward_duration_verify_writable_privileges(accounts)?;
    update_reward_duration_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardFunderAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardFunderKeys {
    pub lb_pair: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateRewardFunderAccounts<'_, '_>> for UpdateRewardFunderKeys {
    fn from(accounts: UpdateRewardFunderAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateRewardFunderKeys>
for [AccountMeta; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardFunderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN]> for UpdateRewardFunderKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            operator: pubkeys[1],
            signer: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateRewardFunderAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardFunderAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN]>
for UpdateRewardFunderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            operator: &arr[1],
            signer: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_REWARD_FUNDER_IX_DISCM: [u8; 8usize] = [
    211, 28, 48, 32, 215, 160, 35, 23,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardFunderIxArgs {
    pub reward_index: u64,
    pub new_funder: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardFunderIxData(pub UpdateRewardFunderIxArgs);
impl From<UpdateRewardFunderIxArgs> for UpdateRewardFunderIxData {
    fn from(args: UpdateRewardFunderIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRewardFunderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_FUNDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRewardFunderIxArgs {
                reward_index,
                new_funder,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_FUNDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_funder, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reward_funder_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewardFunderKeys,
    args: UpdateRewardFunderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRewardFunderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_reward_funder_ix(
    keys: UpdateRewardFunderKeys,
    args: UpdateRewardFunderIxArgs,
) -> std::io::Result<Instruction> {
    update_reward_funder_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn update_reward_funder_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    args: UpdateRewardFunderIxArgs,
) -> ProgramResult {
    let keys: UpdateRewardFunderKeys = accounts.into();
    let ix = update_reward_funder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reward_funder_invoke(
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    args: UpdateRewardFunderIxArgs,
) -> ProgramResult {
    update_reward_funder_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn update_reward_funder_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    args: UpdateRewardFunderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewardFunderKeys = accounts.into();
    let ix = update_reward_funder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reward_funder_invoke_signed(
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    args: UpdateRewardFunderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reward_funder_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_reward_funder_verify_account_keys(
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    keys: UpdateRewardFunderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reward_funder_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardFunderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reward_funder_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRewardFunderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_reward_funder_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewardFunderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reward_funder_verify_writable_privileges(accounts)?;
    update_reward_funder_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawIneligibleRewardAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawIneligibleRewardKeys {
    pub lb_pair: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub funder_token_account: Pubkey,
    pub funder: Pubkey,
    pub bin_array: Pubkey,
    pub token_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawIneligibleRewardAccounts<'_, '_>> for WithdrawIneligibleRewardKeys {
    fn from(accounts: WithdrawIneligibleRewardAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            funder_token_account: *accounts.funder_token_account.key,
            funder: *accounts.funder.key,
            bin_array: *accounts.bin_array.key,
            token_program: *accounts.token_program.key,
            memo_program: *accounts.memo_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawIneligibleRewardKeys>
for [AccountMeta; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawIneligibleRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
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
impl From<[Pubkey; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]>
for WithdrawIneligibleRewardKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reward_vault: pubkeys[1],
            reward_mint: pubkeys[2],
            funder_token_account: pubkeys[3],
            funder: pubkeys[4],
            bin_array: pubkeys[5],
            token_program: pubkeys[6],
            memo_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<WithdrawIneligibleRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawIneligibleRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.funder_token_account.clone(),
            accounts.funder.clone(),
            accounts.bin_array.clone(),
            accounts.token_program.clone(),
            accounts.memo_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]>
for WithdrawIneligibleRewardAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            reward_vault: &arr[1],
            reward_mint: &arr[2],
            funder_token_account: &arr[3],
            funder: &arr[4],
            bin_array: &arr[5],
            token_program: &arr[6],
            memo_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const WITHDRAW_INELIGIBLE_REWARD_IX_DISCM: [u8; 8usize] = [
    148, 206, 42, 195, 247, 49, 103, 8,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIneligibleRewardIxArgs {
    pub reward_index: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIneligibleRewardIxData(pub WithdrawIneligibleRewardIxArgs);
impl From<WithdrawIneligibleRewardIxArgs> for WithdrawIneligibleRewardIxData {
    fn from(args: WithdrawIneligibleRewardIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawIneligibleRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_INELIGIBLE_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(WithdrawIneligibleRewardIxArgs {
                reward_index,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_INELIGIBLE_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_ineligible_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawIneligibleRewardKeys,
    args: WithdrawIneligibleRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawIneligibleRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_ineligible_reward_ix(
    keys: WithdrawIneligibleRewardKeys,
    args: WithdrawIneligibleRewardIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_ineligible_reward_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn withdraw_ineligible_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    args: WithdrawIneligibleRewardIxArgs,
) -> ProgramResult {
    let keys: WithdrawIneligibleRewardKeys = accounts.into();
    let ix = withdraw_ineligible_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_ineligible_reward_invoke(
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    args: WithdrawIneligibleRewardIxArgs,
) -> ProgramResult {
    withdraw_ineligible_reward_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn withdraw_ineligible_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    args: WithdrawIneligibleRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawIneligibleRewardKeys = accounts.into();
    let ix = withdraw_ineligible_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_ineligible_reward_invoke_signed(
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    args: WithdrawIneligibleRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_ineligible_reward_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_ineligible_reward_verify_account_keys(
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    keys: WithdrawIneligibleRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.funder.key, keys.funder),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_ineligible_reward_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawIneligibleRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reward_vault,
        accounts.funder_token_account,
        accounts.bin_array,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_ineligible_reward_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawIneligibleRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_ineligible_reward_verify_account_privileges<'me, 'info>(
    accounts: WithdrawIneligibleRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_ineligible_reward_verify_writable_privileges(accounts)?;
    withdraw_ineligible_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeeAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub receiver_token_x: &'me AccountInfo<'info>,
    pub receiver_token_y: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeeKeys {
    pub lb_pair: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub receiver_token_x: Pubkey,
    pub receiver_token_y: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
}
impl From<WithdrawProtocolFeeAccounts<'_, '_>> for WithdrawProtocolFeeKeys {
    fn from(accounts: WithdrawProtocolFeeAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            receiver_token_x: *accounts.receiver_token_x.key,
            receiver_token_y: *accounts.receiver_token_y.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
        }
    }
}
impl From<WithdrawProtocolFeeKeys>
for [AccountMeta; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for WithdrawProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reserve_x: pubkeys[1],
            reserve_y: pubkeys[2],
            token_x_mint: pubkeys[3],
            token_y_mint: pubkeys[4],
            receiver_token_x: pubkeys[5],
            receiver_token_y: pubkeys[6],
            operator: pubkeys[7],
            signer: pubkeys[8],
            token_x_program: pubkeys[9],
            token_y_program: pubkeys[10],
        }
    }
}
impl<'info> From<WithdrawProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.receiver_token_x.clone(),
            accounts.receiver_token_y.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            reserve_x: &arr[1],
            reserve_y: &arr[2],
            token_x_mint: &arr[3],
            token_y_mint: &arr[4],
            receiver_token_x: &arr[5],
            receiver_token_y: &arr[6],
            operator: &arr[7],
            signer: &arr[8],
            token_x_program: &arr[9],
            token_y_program: &arr[10],
        }
    }
}
pub const WITHDRAW_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    158, 201, 158, 189, 33, 93, 162, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawProtocolFeeIxArgs {
    pub max_amount_x: u64,
    pub max_amount_y: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeeIxData(pub WithdrawProtocolFeeIxArgs);
impl From<WithdrawProtocolFeeIxArgs> for WithdrawProtocolFeeIxData {
    fn from(args: WithdrawProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(WithdrawProtocolFeeIxArgs {
                max_amount_x,
                max_amount_y,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PROTOCOL_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProtocolFeeKeys,
    args: WithdrawProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fee_ix(
    keys: WithdrawProtocolFeeKeys,
    args: WithdrawProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_protocol_fee_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn withdraw_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: WithdrawProtocolFeeKeys = accounts.into();
    let ix = withdraw_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_protocol_fee_invoke(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
) -> ProgramResult {
    withdraw_protocol_fee_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn withdraw_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProtocolFeeKeys = accounts.into();
    let ix = withdraw_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_protocol_fee_invoke_signed(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_protocol_fee_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_protocol_fee_verify_account_keys(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    keys: WithdrawProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.receiver_token_x.key, keys.receiver_token_x),
        (*accounts.receiver_token_y.key, keys.receiver_token_y),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.receiver_token_x,
        accounts.receiver_token_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_protocol_fee_verify_writable_privileges(accounts)?;
    withdraw_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ZAP_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ZapProtocolFeeAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub receiver_token: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub sysvar_instructions: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ZapProtocolFeeKeys {
    pub lb_pair: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub receiver_token: Pubkey,
    pub operator: Pubkey,
    pub signer: Pubkey,
    pub token_program: Pubkey,
    pub sysvar_instructions: Pubkey,
}
impl From<ZapProtocolFeeAccounts<'_, '_>> for ZapProtocolFeeKeys {
    fn from(accounts: ZapProtocolFeeAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            receiver_token: *accounts.receiver_token.key,
            operator: *accounts.operator.key,
            signer: *accounts.signer.key,
            token_program: *accounts.token_program.key,
            sysvar_instructions: *accounts.sysvar_instructions.key,
        }
    }
}
impl From<ZapProtocolFeeKeys> for [AccountMeta; ZAP_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ZapProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sysvar_instructions,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ZAP_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for ZapProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; ZAP_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reserve: pubkeys[1],
            token_mint: pubkeys[2],
            receiver_token: pubkeys[3],
            operator: pubkeys[4],
            signer: pubkeys[5],
            token_program: pubkeys[6],
            sysvar_instructions: pubkeys[7],
        }
    }
}
impl<'info> From<ZapProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; ZAP_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ZapProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.receiver_token.clone(),
            accounts.operator.clone(),
            accounts.signer.clone(),
            accounts.token_program.clone(),
            accounts.sysvar_instructions.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ZAP_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for ZapProtocolFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ZAP_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            reserve: &arr[1],
            token_mint: &arr[2],
            receiver_token: &arr[3],
            operator: &arr[4],
            signer: &arr[5],
            token_program: &arr[6],
            sysvar_instructions: &arr[7],
        }
    }
}
pub const ZAP_PROTOCOL_FEE_IX_DISCM: [u8; 8usize] = [
    213, 155, 187, 34, 56, 182, 91, 240,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ZapProtocolFeeIxArgs {
    pub max_amount: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ZapProtocolFeeIxData(pub ZapProtocolFeeIxArgs);
impl From<ZapProtocolFeeIxArgs> for ZapProtocolFeeIxData {
    fn from(args: ZapProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ZapProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ZAP_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info = if reader.is_empty() {
            Default::default()
        } else {
            <RemainingAccountsInfo>::deserialize(&mut reader)?
        };
        Ok(
            Self(ZapProtocolFeeIxArgs {
                max_amount,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ZAP_PROTOCOL_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn zap_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ZapProtocolFeeKeys,
    args: ZapProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ZAP_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ZapProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn zap_protocol_fee_ix(
    keys: ZapProtocolFeeKeys,
    args: ZapProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    zap_protocol_fee_ix_with_program_id(LB_CLMM_PROGRAM_ID, keys, args)
}
pub fn zap_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ZapProtocolFeeAccounts<'_, '_>,
    args: ZapProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: ZapProtocolFeeKeys = accounts.into();
    let ix = zap_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn zap_protocol_fee_invoke(
    accounts: ZapProtocolFeeAccounts<'_, '_>,
    args: ZapProtocolFeeIxArgs,
) -> ProgramResult {
    zap_protocol_fee_invoke_with_program_id(LB_CLMM_PROGRAM_ID, accounts, args)
}
pub fn zap_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ZapProtocolFeeAccounts<'_, '_>,
    args: ZapProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ZapProtocolFeeKeys = accounts.into();
    let ix = zap_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn zap_protocol_fee_invoke_signed(
    accounts: ZapProtocolFeeAccounts<'_, '_>,
    args: ZapProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    zap_protocol_fee_invoke_signed_with_program_id(
        LB_CLMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn zap_protocol_fee_verify_account_keys(
    accounts: ZapProtocolFeeAccounts<'_, '_>,
    keys: ZapProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.receiver_token.key, keys.receiver_token),
        (*accounts.operator.key, keys.operator),
        (*accounts.signer.key, keys.signer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.sysvar_instructions.key, keys.sysvar_instructions),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn zap_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: ZapProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reserve,
        accounts.receiver_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn zap_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: ZapProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn zap_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: ZapProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    zap_protocol_fee_verify_writable_privileges(accounts)?;
    zap_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
