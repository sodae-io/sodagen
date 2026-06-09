use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum CloneProgramIx {
    InitializeClone(InitializeCloneIxArgs),
    InitializePools,
    InitializeOracles,
    UpdateCloneParameters(UpdateCloneParametersIxArgs),
    UpdatePoolParameters(UpdatePoolParametersIxArgs),
    UpdateOracles(UpdateOraclesIxArgs),
    InitializeUser(InitializeUserIxArgs),
    AddPool(AddPoolIxArgs),
    UpdatePrices(UpdatePricesIxArgs),
    InitializeBorrowPosition(InitializeBorrowPositionIxArgs),
    AddCollateralToBorrow(AddCollateralToBorrowIxArgs),
    WithdrawCollateralFromBorrow(WithdrawCollateralFromBorrowIxArgs),
    PayBorrowDebt(PayBorrowDebtIxArgs),
    BorrowMore(BorrowMoreIxArgs),
    AddCollateralToComet(AddCollateralToCometIxArgs),
    WithdrawCollateralFromComet(WithdrawCollateralFromCometIxArgs),
    AddLiquidityToComet(AddLiquidityToCometIxArgs),
    WithdrawLiquidityFromComet(WithdrawLiquidityFromCometIxArgs),
    LiquidateCometCollateralIld(LiquidateCometCollateralIldIxArgs),
    LiquidateCometOnassetIld(LiquidateCometOnassetIldIxArgs),
    LiquidateBorrowPosition(LiquidateBorrowPositionIxArgs),
    CollectLpRewards(CollectLpRewardsIxArgs),
    PayImpermanentLossDebt(PayImpermanentLossDebtIxArgs),
    CloseUserAccount,
    WrapAsset(WrapAssetIxArgs),
    UnwrapOnasset(UnwrapOnassetIxArgs),
    RemoveCometPosition(RemoveCometPositionIxArgs),
    Swap(SwapIxArgs),
    CreateTokenMetadata(CreateTokenMetadataIxArgs),
    RemovePool(RemovePoolIxArgs),
}
impl CloneProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_CLONE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_CLONE_IX_DISCM.len()..];
            let comet_collateral_ild_liquidator_fee_bps: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let comet_onasset_ild_liquidator_fee_bps: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let borrow_liquidator_fee_bps: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let treasury_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let collateral_oracle_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let collateralization_ratio: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeClone(InitializeCloneIxArgs {
                    comet_collateral_ild_liquidator_fee_bps,
                    comet_onasset_ild_liquidator_fee_bps,
                    borrow_liquidator_fee_bps,
                    treasury_address,
                    collateral_oracle_index,
                    collateralization_ratio,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POOLS_IX_DISCM) {
            return Ok(Self::InitializePools);
        }
        if buf.starts_with(&INITIALIZE_ORACLES_IX_DISCM) {
            return Ok(Self::InitializeOracles);
        }
        if buf.starts_with(&UPDATE_CLONE_PARAMETERS_IX_DISCM) {
            let mut reader = &buf[UPDATE_CLONE_PARAMETERS_IX_DISCM.len()..];
            let params = <CloneParameters as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateCloneParameters(UpdateCloneParametersIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&UPDATE_POOL_PARAMETERS_IX_DISCM) {
            let mut reader = &buf[UPDATE_POOL_PARAMETERS_IX_DISCM.len()..];
            let index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let params = <PoolParameters as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::UpdatePoolParameters(UpdatePoolParametersIxArgs {
                    index,
                    params,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ORACLES_IX_DISCM) {
            let mut reader = &buf[UPDATE_ORACLES_IX_DISCM.len()..];
            let params = <UpdateOracleParameters as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(Self::UpdateOracles(UpdateOraclesIxArgs { params }));
        }
        if buf.starts_with(&INITIALIZE_USER_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_USER_IX_DISCM.len()..];
            let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::InitializeUser(InitializeUserIxArgs { authority }));
        }
        if buf.starts_with(&ADD_POOL_IX_DISCM) {
            let mut reader = &buf[ADD_POOL_IX_DISCM.len()..];
            let min_overcollateral_ratio: u16 = crate::borsh_de_or_default(&mut reader)?;
            let max_liquidation_overcollateral_ratio: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let liquidity_trading_fee_bps: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let treasury_trading_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
            let il_health_score_coefficient: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let position_health_score_coefficient: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let oracle_info_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddPool(AddPoolIxArgs {
                    min_overcollateral_ratio,
                    max_liquidation_overcollateral_ratio,
                    liquidity_trading_fee_bps,
                    treasury_trading_fee_bps,
                    il_health_score_coefficient,
                    position_health_score_coefficient,
                    oracle_info_index,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PRICES_IX_DISCM) {
            let mut reader = &buf[UPDATE_PRICES_IX_DISCM.len()..];
            let oracle_indices: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdatePrices(UpdatePricesIxArgs {
                    oracle_indices,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_BORROW_POSITION_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_BORROW_POSITION_IX_DISCM.len()..];
            let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let onasset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeBorrowPosition(InitializeBorrowPositionIxArgs {
                    pool_index,
                    onasset_amount,
                    collateral_amount,
                }),
            );
        }
        if buf.starts_with(&ADD_COLLATERAL_TO_BORROW_IX_DISCM) {
            let mut reader = &buf[ADD_COLLATERAL_TO_BORROW_IX_DISCM.len()..];
            let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddCollateralToBorrow(AddCollateralToBorrowIxArgs {
                    borrow_index,
                    amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_COLLATERAL_FROM_BORROW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_COLLATERAL_FROM_BORROW_IX_DISCM.len()..];
            let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawCollateralFromBorrow(WithdrawCollateralFromBorrowIxArgs {
                    borrow_index,
                    amount,
                }),
            );
        }
        if buf.starts_with(&PAY_BORROW_DEBT_IX_DISCM) {
            let mut reader = &buf[PAY_BORROW_DEBT_IX_DISCM.len()..];
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PayBorrowDebt(PayBorrowDebtIxArgs {
                    user,
                    borrow_index,
                    amount,
                }),
            );
        }
        if buf.starts_with(&BORROW_MORE_IX_DISCM) {
            let mut reader = &buf[BORROW_MORE_IX_DISCM.len()..];
            let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BorrowMore(BorrowMoreIxArgs {
                    borrow_index,
                    amount,
                }),
            );
        }
        if buf.starts_with(&ADD_COLLATERAL_TO_COMET_IX_DISCM) {
            let mut reader = &buf[ADD_COLLATERAL_TO_COMET_IX_DISCM.len()..];
            let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddCollateralToComet(AddCollateralToCometIxArgs {
                    collateral_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_COLLATERAL_FROM_COMET_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_COLLATERAL_FROM_COMET_IX_DISCM.len()..];
            let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawCollateralFromComet(WithdrawCollateralFromCometIxArgs {
                    collateral_amount,
                }),
            );
        }
        if buf.starts_with(&ADD_LIQUIDITY_TO_COMET_IX_DISCM) {
            let mut reader = &buf[ADD_LIQUIDITY_TO_COMET_IX_DISCM.len()..];
            let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddLiquidityToComet(AddLiquidityToCometIxArgs {
                    pool_index,
                    collateral_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_LIQUIDITY_FROM_COMET_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_LIQUIDITY_FROM_COMET_IX_DISCM.len()..];
            let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawLiquidityFromComet(WithdrawLiquidityFromCometIxArgs {
                    comet_position_index,
                    amount,
                }),
            );
        }
        if buf.starts_with(&LIQUIDATE_COMET_COLLATERAL_ILD_IX_DISCM) {
            let mut reader = &buf[LIQUIDATE_COMET_COLLATERAL_ILD_IX_DISCM.len()..];
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidateCometCollateralIld(LiquidateCometCollateralIldIxArgs {
                    user,
                    comet_position_index,
                }),
            );
        }
        if buf.starts_with(&LIQUIDATE_COMET_ONASSET_ILD_IX_DISCM) {
            let mut reader = &buf[LIQUIDATE_COMET_ONASSET_ILD_IX_DISCM.len()..];
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidateCometOnassetIld(LiquidateCometOnassetIldIxArgs {
                    user,
                    comet_position_index,
                    amount,
                }),
            );
        }
        if buf.starts_with(&LIQUIDATE_BORROW_POSITION_IX_DISCM) {
            let mut reader = &buf[LIQUIDATE_BORROW_POSITION_IX_DISCM.len()..];
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidateBorrowPosition(LiquidateBorrowPositionIxArgs {
                    user,
                    borrow_index,
                    amount,
                }),
            );
        }
        if buf.starts_with(&COLLECT_LP_REWARDS_IX_DISCM) {
            let mut reader = &buf[COLLECT_LP_REWARDS_IX_DISCM.len()..];
            let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CollectLpRewards(CollectLpRewardsIxArgs {
                    comet_position_index,
                }),
            );
        }
        if buf.starts_with(&PAY_IMPERMANENT_LOSS_DEBT_IX_DISCM) {
            let mut reader = &buf[PAY_IMPERMANENT_LOSS_DEBT_IX_DISCM.len()..];
            let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let payment_type: PaymentType = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PayImpermanentLossDebt(PayImpermanentLossDebtIxArgs {
                    user,
                    comet_position_index,
                    amount,
                    payment_type,
                }),
            );
        }
        if buf.starts_with(&CLOSE_USER_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseUserAccount);
        }
        if buf.starts_with(&WRAP_ASSET_IX_DISCM) {
            let mut reader = &buf[WRAP_ASSET_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WrapAsset(WrapAssetIxArgs {
                    amount,
                    pool_index,
                }),
            );
        }
        if buf.starts_with(&UNWRAP_ONASSET_IX_DISCM) {
            let mut reader = &buf[UNWRAP_ONASSET_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UnwrapOnasset(UnwrapOnassetIxArgs {
                    amount,
                    pool_index,
                }),
            );
        }
        if buf.starts_with(&REMOVE_COMET_POSITION_IX_DISCM) {
            let mut reader = &buf[REMOVE_COMET_POSITION_IX_DISCM.len()..];
            let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveCometPosition(RemoveCometPositionIxArgs {
                    comet_position_index,
                }),
            );
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let quantity: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quantity_is_input: bool = crate::borsh_de_or_default(&mut reader)?;
            let quantity_is_collateral: bool = crate::borsh_de_or_default(&mut reader)?;
            let result_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    pool_index,
                    quantity,
                    quantity_is_input,
                    quantity_is_collateral,
                    result_threshold,
                }),
            );
        }
        if buf.starts_with(&CREATE_TOKEN_METADATA_IX_DISCM) {
            let mut reader = &buf[CREATE_TOKEN_METADATA_IX_DISCM.len()..];
            let metadata_args = if reader.is_empty() {
                Default::default()
            } else {
                <MetadataArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateTokenMetadata(CreateTokenMetadataIxArgs {
                    metadata_args,
                }),
            );
        }
        if buf.starts_with(&REMOVE_POOL_IX_DISCM) {
            let mut reader = &buf[REMOVE_POOL_IX_DISCM.len()..];
            let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::RemovePool(RemovePoolIxArgs { pool_index }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeClone(args) => {
                writer.write_all(&INITIALIZE_CLONE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.comet_collateral_ild_liquidator_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.comet_onasset_ild_liquidator_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.borrow_liquidator_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.treasury_address, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.collateral_oracle_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.collateralization_ratio,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializePools => writer.write_all(&INITIALIZE_POOLS_IX_DISCM),
            Self::InitializeOracles => writer.write_all(&INITIALIZE_ORACLES_IX_DISCM),
            Self::UpdateCloneParameters(args) => {
                writer.write_all(&UPDATE_CLONE_PARAMETERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::UpdatePoolParameters(args) => {
                writer.write_all(&UPDATE_POOL_PARAMETERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::UpdateOracles(args) => {
                writer.write_all(&UPDATE_ORACLES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeUser(args) => {
                writer.write_all(&INITIALIZE_USER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.authority, &mut writer)?;
                Ok(())
            }
            Self::AddPool(args) => {
                writer.write_all(&ADD_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.min_overcollateral_ratio,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.max_liquidation_overcollateral_ratio,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.liquidity_trading_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.treasury_trading_fee_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.il_health_score_coefficient,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.position_health_score_coefficient,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.oracle_info_index, &mut writer)?;
                Ok(())
            }
            Self::UpdatePrices(args) => {
                writer.write_all(&UPDATE_PRICES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.oracle_indices, &mut writer)?;
                Ok(())
            }
            Self::InitializeBorrowPosition(args) => {
                writer.write_all(&INITIALIZE_BORROW_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pool_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.onasset_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.collateral_amount, &mut writer)?;
                Ok(())
            }
            Self::AddCollateralToBorrow(args) => {
                writer.write_all(&ADD_COLLATERAL_TO_BORROW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.borrow_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawCollateralFromBorrow(args) => {
                writer.write_all(&WITHDRAW_COLLATERAL_FROM_BORROW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.borrow_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::PayBorrowDebt(args) => {
                writer.write_all(&PAY_BORROW_DEBT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.borrow_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::BorrowMore(args) => {
                writer.write_all(&BORROW_MORE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.borrow_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::AddCollateralToComet(args) => {
                writer.write_all(&ADD_COLLATERAL_TO_COMET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.collateral_amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawCollateralFromComet(args) => {
                writer.write_all(&WITHDRAW_COLLATERAL_FROM_COMET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.collateral_amount, &mut writer)?;
                Ok(())
            }
            Self::AddLiquidityToComet(args) => {
                writer.write_all(&ADD_LIQUIDITY_TO_COMET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pool_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.collateral_amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawLiquidityFromComet(args) => {
                writer.write_all(&WITHDRAW_LIQUIDITY_FROM_COMET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.comet_position_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::LiquidateCometCollateralIld(args) => {
                writer.write_all(&LIQUIDATE_COMET_COLLATERAL_ILD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.comet_position_index,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::LiquidateCometOnassetIld(args) => {
                writer.write_all(&LIQUIDATE_COMET_ONASSET_ILD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.comet_position_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::LiquidateBorrowPosition(args) => {
                writer.write_all(&LIQUIDATE_BORROW_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.borrow_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::CollectLpRewards(args) => {
                writer.write_all(&COLLECT_LP_REWARDS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.comet_position_index,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::PayImpermanentLossDebt(args) => {
                writer.write_all(&PAY_IMPERMANENT_LOSS_DEBT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.user, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.comet_position_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.payment_type, &mut writer)?;
                Ok(())
            }
            Self::CloseUserAccount => writer.write_all(&CLOSE_USER_ACCOUNT_IX_DISCM),
            Self::WrapAsset(args) => {
                writer.write_all(&WRAP_ASSET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pool_index, &mut writer)?;
                Ok(())
            }
            Self::UnwrapOnasset(args) => {
                writer.write_all(&UNWRAP_ONASSET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pool_index, &mut writer)?;
                Ok(())
            }
            Self::RemoveCometPosition(args) => {
                writer.write_all(&REMOVE_COMET_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.comet_position_index,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pool_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quantity, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quantity_is_input, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.quantity_is_collateral,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.result_threshold, &mut writer)?;
                Ok(())
            }
            Self::CreateTokenMetadata(args) => {
                writer.write_all(&CREATE_TOKEN_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.metadata_args, &mut writer)?;
                Ok(())
            }
            Self::RemovePool(args) => {
                writer.write_all(&REMOVE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pool_index, &mut writer)?;
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
pub const INITIALIZE_CLONE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeCloneAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeCloneKeys {
    pub admin: Pubkey,
    pub clone: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeCloneAccounts<'_, '_>> for InitializeCloneKeys {
    fn from(accounts: InitializeCloneAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            clone: *accounts.clone.key,
            collateral_mint: *accounts.collateral_mint.key,
            collateral_vault: *accounts.collateral_vault.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeCloneKeys> for [AccountMeta; INITIALIZE_CLONE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCloneKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; INITIALIZE_CLONE_IX_ACCOUNTS_LEN]> for InitializeCloneKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CLONE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            clone: pubkeys[1],
            collateral_mint: pubkeys[2],
            collateral_vault: pubkeys[3],
            rent: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeCloneAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CLONE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeCloneAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.clone.clone(),
            accounts.collateral_mint.clone(),
            accounts.collateral_vault.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CLONE_IX_ACCOUNTS_LEN]>
for InitializeCloneAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_CLONE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            clone: &arr[1],
            collateral_mint: &arr[2],
            collateral_vault: &arr[3],
            rent: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INITIALIZE_CLONE_IX_DISCM: [u8; 8usize] = [
    80, 108, 183, 240, 26, 141, 202, 138,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeCloneIxArgs {
    pub comet_collateral_ild_liquidator_fee_bps: u16,
    pub comet_onasset_ild_liquidator_fee_bps: u16,
    pub borrow_liquidator_fee_bps: u16,
    pub treasury_address: Pubkey,
    pub collateral_oracle_index: u8,
    pub collateralization_ratio: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeCloneIxData(pub InitializeCloneIxArgs);
impl From<InitializeCloneIxArgs> for InitializeCloneIxData {
    fn from(args: InitializeCloneIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeCloneIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CLONE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let comet_collateral_ild_liquidator_fee_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let comet_onasset_ild_liquidator_fee_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_liquidator_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let treasury_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_oracle_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collateralization_ratio: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeCloneIxArgs {
                comet_collateral_ild_liquidator_fee_bps,
                comet_onasset_ild_liquidator_fee_bps,
                borrow_liquidator_fee_bps,
                treasury_address,
                collateral_oracle_index,
                collateralization_ratio,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CLONE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.comet_collateral_ild_liquidator_fee_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.comet_onasset_ild_liquidator_fee_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.borrow_liquidator_fee_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.treasury_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.collateral_oracle_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.collateralization_ratio, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_clone_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCloneKeys,
    args: InitializeCloneIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CLONE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeCloneIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_clone_ix(
    keys: InitializeCloneKeys,
    args: InitializeCloneIxArgs,
) -> std::io::Result<Instruction> {
    initialize_clone_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn initialize_clone_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCloneAccounts<'_, '_>,
    args: InitializeCloneIxArgs,
) -> ProgramResult {
    let keys: InitializeCloneKeys = accounts.into();
    let ix = initialize_clone_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_clone_invoke(
    accounts: InitializeCloneAccounts<'_, '_>,
    args: InitializeCloneIxArgs,
) -> ProgramResult {
    initialize_clone_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn initialize_clone_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCloneAccounts<'_, '_>,
    args: InitializeCloneIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCloneKeys = accounts.into();
    let ix = initialize_clone_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_clone_invoke_signed(
    accounts: InitializeCloneAccounts<'_, '_>,
    args: InitializeCloneIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_clone_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_clone_verify_account_keys(
    accounts: InitializeCloneAccounts<'_, '_>,
    keys: InitializeCloneKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.clone.key, keys.clone),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_clone_verify_writable_privileges<'me, 'info>(
    accounts: InitializeCloneAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.clone] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_clone_verify_signer_privileges<'me, 'info>(
    accounts: InitializeCloneAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_clone_verify_account_privileges<'me, 'info>(
    accounts: InitializeCloneAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_clone_verify_writable_privileges(accounts)?;
    initialize_clone_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOLS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolsAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolsKeys {
    pub admin: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePoolsAccounts<'_, '_>> for InitializePoolsKeys {
    fn from(accounts: InitializePoolsAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePoolsKeys> for [AccountMeta; INITIALIZE_POOLS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pools,
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
impl From<[Pubkey; INITIALIZE_POOLS_IX_ACCOUNTS_LEN]> for InitializePoolsKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOLS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            clone: pubkeys[1],
            pools: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializePoolsAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOLS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolsAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POOLS_IX_ACCOUNTS_LEN]>
for InitializePoolsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_POOLS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            clone: &arr[1],
            pools: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_POOLS_IX_DISCM: [u8; 8usize] = [
    48, 75, 127, 251, 17, 111, 153, 216,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolsIxData;
impl InitializePoolsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOLS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOLS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pools_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOLS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializePoolsIxData.try_to_vec()?,
    })
}
pub fn initialize_pools_ix(keys: InitializePoolsKeys) -> std::io::Result<Instruction> {
    initialize_pools_ix_with_program_id(CLONE_PROGRAM_ID, keys)
}
pub fn initialize_pools_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializePoolsKeys = accounts.into();
    let ix = initialize_pools_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pools_invoke(
    accounts: InitializePoolsAccounts<'_, '_>,
) -> ProgramResult {
    initialize_pools_invoke_with_program_id(CLONE_PROGRAM_ID, accounts)
}
pub fn initialize_pools_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolsKeys = accounts.into();
    let ix = initialize_pools_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pools_invoke_signed(
    accounts: InitializePoolsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pools_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_pools_verify_account_keys(
    accounts: InitializePoolsAccounts<'_, '_>,
    keys: InitializePoolsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_pools_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.pools] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pools_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pools_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pools_verify_writable_privileges(accounts)?;
    initialize_pools_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_ORACLES_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeOraclesAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeOraclesKeys {
    pub admin: Pubkey,
    pub clone: Pubkey,
    pub oracles: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeOraclesAccounts<'_, '_>> for InitializeOraclesKeys {
    fn from(accounts: InitializeOraclesAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            clone: *accounts.clone.key,
            oracles: *accounts.oracles.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeOraclesKeys> for [AccountMeta; INITIALIZE_ORACLES_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeOraclesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracles,
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
impl From<[Pubkey; INITIALIZE_ORACLES_IX_ACCOUNTS_LEN]> for InitializeOraclesKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_ORACLES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            clone: pubkeys[1],
            oracles: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeOraclesAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_ORACLES_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeOraclesAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.clone.clone(),
            accounts.oracles.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_ORACLES_IX_ACCOUNTS_LEN]>
for InitializeOraclesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_ORACLES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            clone: &arr[1],
            oracles: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_ORACLES_IX_DISCM: [u8; 8usize] = [
    187, 13, 76, 77, 183, 146, 201, 56,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeOraclesIxData;
impl InitializeOraclesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_ORACLES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_ORACLES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_oracles_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeOraclesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_ORACLES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeOraclesIxData.try_to_vec()?,
    })
}
pub fn initialize_oracles_ix(
    keys: InitializeOraclesKeys,
) -> std::io::Result<Instruction> {
    initialize_oracles_ix_with_program_id(CLONE_PROGRAM_ID, keys)
}
pub fn initialize_oracles_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOraclesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeOraclesKeys = accounts.into();
    let ix = initialize_oracles_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_oracles_invoke(
    accounts: InitializeOraclesAccounts<'_, '_>,
) -> ProgramResult {
    initialize_oracles_invoke_with_program_id(CLONE_PROGRAM_ID, accounts)
}
pub fn initialize_oracles_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeOraclesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeOraclesKeys = accounts.into();
    let ix = initialize_oracles_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_oracles_invoke_signed(
    accounts: InitializeOraclesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_oracles_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_oracles_verify_account_keys(
    accounts: InitializeOraclesAccounts<'_, '_>,
    keys: InitializeOraclesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.clone.key, keys.clone),
        (*accounts.oracles.key, keys.oracles),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_oracles_verify_writable_privileges<'me, 'info>(
    accounts: InitializeOraclesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.oracles] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_oracles_verify_signer_privileges<'me, 'info>(
    accounts: InitializeOraclesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_oracles_verify_account_privileges<'me, 'info>(
    accounts: InitializeOraclesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_oracles_verify_writable_privileges(accounts)?;
    initialize_oracles_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CLONE_PARAMETERS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCloneParametersAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCloneParametersKeys {
    pub admin: Pubkey,
    pub clone: Pubkey,
}
impl From<UpdateCloneParametersAccounts<'_, '_>> for UpdateCloneParametersKeys {
    fn from(accounts: UpdateCloneParametersAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            clone: *accounts.clone.key,
        }
    }
}
impl From<UpdateCloneParametersKeys>
for [AccountMeta; UPDATE_CLONE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCloneParametersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CLONE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateCloneParametersKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CLONE_PARAMETERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            clone: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateCloneParametersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CLONE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateCloneParametersAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.clone.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CLONE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateCloneParametersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CLONE_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            clone: &arr[1],
        }
    }
}
pub const UPDATE_CLONE_PARAMETERS_IX_DISCM: [u8; 8usize] = [
    65, 102, 56, 210, 52, 197, 35, 30,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateCloneParametersIxArgs {
    pub params: CloneParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCloneParametersIxData(pub UpdateCloneParametersIxArgs);
impl From<UpdateCloneParametersIxArgs> for UpdateCloneParametersIxData {
    fn from(args: UpdateCloneParametersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateCloneParametersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CLONE_PARAMETERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = <CloneParameters as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(UpdateCloneParametersIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CLONE_PARAMETERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_clone_parameters_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCloneParametersKeys,
    args: UpdateCloneParametersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CLONE_PARAMETERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateCloneParametersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_clone_parameters_ix(
    keys: UpdateCloneParametersKeys,
    args: UpdateCloneParametersIxArgs,
) -> std::io::Result<Instruction> {
    update_clone_parameters_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn update_clone_parameters_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCloneParametersAccounts<'_, '_>,
    args: UpdateCloneParametersIxArgs,
) -> ProgramResult {
    let keys: UpdateCloneParametersKeys = accounts.into();
    let ix = update_clone_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_clone_parameters_invoke(
    accounts: UpdateCloneParametersAccounts<'_, '_>,
    args: UpdateCloneParametersIxArgs,
) -> ProgramResult {
    update_clone_parameters_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn update_clone_parameters_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCloneParametersAccounts<'_, '_>,
    args: UpdateCloneParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCloneParametersKeys = accounts.into();
    let ix = update_clone_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_clone_parameters_invoke_signed(
    accounts: UpdateCloneParametersAccounts<'_, '_>,
    args: UpdateCloneParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_clone_parameters_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_clone_parameters_verify_account_keys(
    accounts: UpdateCloneParametersAccounts<'_, '_>,
    keys: UpdateCloneParametersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.clone.key, keys.clone),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_clone_parameters_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCloneParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.clone] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_clone_parameters_verify_signer_privileges<'me, 'info>(
    accounts: UpdateCloneParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_clone_parameters_verify_account_privileges<'me, 'info>(
    accounts: UpdateCloneParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_clone_parameters_verify_writable_privileges(accounts)?;
    update_clone_parameters_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_POOL_PARAMETERS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePoolParametersAccounts<'me, 'info> {
    pub auth: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePoolParametersKeys {
    pub auth: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
}
impl From<UpdatePoolParametersAccounts<'_, '_>> for UpdatePoolParametersKeys {
    fn from(accounts: UpdatePoolParametersAccounts) -> Self {
        Self {
            auth: *accounts.auth.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
        }
    }
}
impl From<UpdatePoolParametersKeys>
for [AccountMeta; UPDATE_POOL_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePoolParametersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.auth,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_POOL_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdatePoolParametersKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POOL_PARAMETERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            auth: pubkeys[0],
            clone: pubkeys[1],
            pools: pubkeys[2],
        }
    }
}
impl<'info> From<UpdatePoolParametersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POOL_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePoolParametersAccounts<'_, 'info>) -> Self {
        [accounts.auth.clone(), accounts.clone.clone(), accounts.pools.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_POOL_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdatePoolParametersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_POOL_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            auth: &arr[0],
            clone: &arr[1],
            pools: &arr[2],
        }
    }
}
pub const UPDATE_POOL_PARAMETERS_IX_DISCM: [u8; 8usize] = [
    141, 98, 249, 196, 230, 84, 52, 127,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePoolParametersIxArgs {
    pub index: u8,
    pub params: PoolParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePoolParametersIxData(pub UpdatePoolParametersIxArgs);
impl From<UpdatePoolParametersIxArgs> for UpdatePoolParametersIxData {
    fn from(args: UpdatePoolParametersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePoolParametersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POOL_PARAMETERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let params = <PoolParameters as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(UpdatePoolParametersIxArgs {
                index,
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POOL_PARAMETERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_pool_parameters_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePoolParametersKeys,
    args: UpdatePoolParametersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POOL_PARAMETERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePoolParametersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_pool_parameters_ix(
    keys: UpdatePoolParametersKeys,
    args: UpdatePoolParametersIxArgs,
) -> std::io::Result<Instruction> {
    update_pool_parameters_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn update_pool_parameters_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolParametersAccounts<'_, '_>,
    args: UpdatePoolParametersIxArgs,
) -> ProgramResult {
    let keys: UpdatePoolParametersKeys = accounts.into();
    let ix = update_pool_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pool_parameters_invoke(
    accounts: UpdatePoolParametersAccounts<'_, '_>,
    args: UpdatePoolParametersIxArgs,
) -> ProgramResult {
    update_pool_parameters_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn update_pool_parameters_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolParametersAccounts<'_, '_>,
    args: UpdatePoolParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePoolParametersKeys = accounts.into();
    let ix = update_pool_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pool_parameters_invoke_signed(
    accounts: UpdatePoolParametersAccounts<'_, '_>,
    args: UpdatePoolParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pool_parameters_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_pool_parameters_verify_account_keys(
    accounts: UpdatePoolParametersAccounts<'_, '_>,
    keys: UpdatePoolParametersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.auth.key, keys.auth),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_pool_parameters_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePoolParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pools] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pool_parameters_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePoolParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.auth] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pool_parameters_verify_account_privileges<'me, 'info>(
    accounts: UpdatePoolParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pool_parameters_verify_writable_privileges(accounts)?;
    update_pool_parameters_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ORACLES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOraclesAccounts<'me, 'info> {
    pub auth: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOraclesKeys {
    pub auth: Pubkey,
    pub clone: Pubkey,
    pub oracles: Pubkey,
}
impl From<UpdateOraclesAccounts<'_, '_>> for UpdateOraclesKeys {
    fn from(accounts: UpdateOraclesAccounts) -> Self {
        Self {
            auth: *accounts.auth.key,
            clone: *accounts.clone.key,
            oracles: *accounts.oracles.key,
        }
    }
}
impl From<UpdateOraclesKeys> for [AccountMeta; UPDATE_ORACLES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOraclesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.auth,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ORACLES_IX_ACCOUNTS_LEN]> for UpdateOraclesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ORACLES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            auth: pubkeys[0],
            clone: pubkeys[1],
            oracles: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateOraclesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ORACLES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOraclesAccounts<'_, 'info>) -> Self {
        [accounts.auth.clone(), accounts.clone.clone(), accounts.oracles.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ORACLES_IX_ACCOUNTS_LEN]>
for UpdateOraclesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ORACLES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            auth: &arr[0],
            clone: &arr[1],
            oracles: &arr[2],
        }
    }
}
pub const UPDATE_ORACLES_IX_DISCM: [u8; 8usize] = [209, 115, 103, 72, 108, 69, 218, 189];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOraclesIxArgs {
    pub params: UpdateOracleParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOraclesIxData(pub UpdateOraclesIxArgs);
impl From<UpdateOraclesIxArgs> for UpdateOraclesIxData {
    fn from(args: UpdateOraclesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOraclesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = <UpdateOracleParameters as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(Self(UpdateOraclesIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_oracles_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOraclesKeys,
    args: UpdateOraclesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ORACLES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOraclesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_oracles_ix(
    keys: UpdateOraclesKeys,
    args: UpdateOraclesIxArgs,
) -> std::io::Result<Instruction> {
    update_oracles_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn update_oracles_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOraclesAccounts<'_, '_>,
    args: UpdateOraclesIxArgs,
) -> ProgramResult {
    let keys: UpdateOraclesKeys = accounts.into();
    let ix = update_oracles_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_oracles_invoke(
    accounts: UpdateOraclesAccounts<'_, '_>,
    args: UpdateOraclesIxArgs,
) -> ProgramResult {
    update_oracles_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn update_oracles_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOraclesAccounts<'_, '_>,
    args: UpdateOraclesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOraclesKeys = accounts.into();
    let ix = update_oracles_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_oracles_invoke_signed(
    accounts: UpdateOraclesAccounts<'_, '_>,
    args: UpdateOraclesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_oracles_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_oracles_verify_account_keys(
    accounts: UpdateOraclesAccounts<'_, '_>,
    keys: UpdateOraclesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.auth.key, keys.auth),
        (*accounts.clone.key, keys.clone),
        (*accounts.oracles.key, keys.oracles),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_oracles_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOraclesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.oracles] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_oracles_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOraclesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.auth] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_oracles_verify_account_privileges<'me, 'info>(
    accounts: UpdateOraclesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_oracles_verify_writable_privileges(accounts)?;
    update_oracles_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_USER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeUserKeys {
    pub payer: Pubkey,
    pub user_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeUserAccounts<'_, '_>> for InitializeUserKeys {
    fn from(accounts: InitializeUserAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            user_account: *accounts.user_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeUserKeys> for [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeUserKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_account,
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
impl From<[Pubkey; INITIALIZE_USER_IX_ACCOUNTS_LEN]> for InitializeUserKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            user_account: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeUserAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeUserAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.user_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]>
for InitializeUserAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            user_account: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_USER_IX_DISCM: [u8; 8usize] = [111, 17, 185, 250, 60, 122, 38, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeUserIxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeUserIxData(pub InitializeUserIxArgs);
impl From<InitializeUserIxArgs> for InitializeUserIxData {
    fn from(args: InitializeUserIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitializeUserIxArgs { authority }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.authority, &mut writer)?;
        Ok(())
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
    args: InitializeUserIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeUserIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_user_ix(
    keys: InitializeUserKeys,
    args: InitializeUserIxArgs,
) -> std::io::Result<Instruction> {
    initialize_user_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn initialize_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUserAccounts<'_, '_>,
    args: InitializeUserIxArgs,
) -> ProgramResult {
    let keys: InitializeUserKeys = accounts.into();
    let ix = initialize_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_user_invoke(
    accounts: InitializeUserAccounts<'_, '_>,
    args: InitializeUserIxArgs,
) -> ProgramResult {
    initialize_user_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn initialize_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUserAccounts<'_, '_>,
    args: InitializeUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeUserKeys = accounts.into();
    let ix = initialize_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_user_invoke_signed(
    accounts: InitializeUserAccounts<'_, '_>,
    args: InitializeUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_user_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_user_verify_account_keys(
    accounts: InitializeUserAccounts<'_, '_>,
    keys: InitializeUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.system_program.key, keys.system_program),
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
    for should_be_writable in [accounts.payer, accounts.user_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_user_verify_signer_privileges<'me, 'info>(
    accounts: InitializeUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
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
pub const ADD_POOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct AddPoolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub onasset_token_account: &'me AccountInfo<'info>,
    pub underlying_asset_mint: &'me AccountInfo<'info>,
    pub underlying_asset_token_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddPoolKeys {
    pub admin: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub onasset_mint: Pubkey,
    pub onasset_token_account: Pubkey,
    pub underlying_asset_mint: Pubkey,
    pub underlying_asset_token_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddPoolAccounts<'_, '_>> for AddPoolKeys {
    fn from(accounts: AddPoolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            onasset_mint: *accounts.onasset_mint.key,
            onasset_token_account: *accounts.onasset_token_account.key,
            underlying_asset_mint: *accounts.underlying_asset_mint.key,
            underlying_asset_token_account: *accounts.underlying_asset_token_account.key,
            system_program: *accounts.system_program.key,
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
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onasset_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_asset_token_account,
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
impl From<[Pubkey; ADD_POOL_IX_ACCOUNTS_LEN]> for AddPoolKeys {
    fn from(pubkeys: [Pubkey; ADD_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            clone: pubkeys[1],
            pools: pubkeys[2],
            onasset_mint: pubkeys[3],
            onasset_token_account: pubkeys[4],
            underlying_asset_mint: pubkeys[5],
            underlying_asset_token_account: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<AddPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.onasset_mint.clone(),
            accounts.onasset_token_account.clone(),
            accounts.underlying_asset_mint.clone(),
            accounts.underlying_asset_token_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_POOL_IX_ACCOUNTS_LEN]>
for AddPoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            clone: &arr[1],
            pools: &arr[2],
            onasset_mint: &arr[3],
            onasset_token_account: &arr[4],
            underlying_asset_mint: &arr[5],
            underlying_asset_token_account: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const ADD_POOL_IX_DISCM: [u8; 8usize] = [115, 230, 212, 211, 175, 49, 39, 169];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddPoolIxArgs {
    pub min_overcollateral_ratio: u16,
    pub max_liquidation_overcollateral_ratio: u16,
    pub liquidity_trading_fee_bps: u16,
    pub treasury_trading_fee_bps: u16,
    pub il_health_score_coefficient: u16,
    pub position_health_score_coefficient: u16,
    pub oracle_info_index: u8,
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
        let min_overcollateral_ratio: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_liquidation_overcollateral_ratio: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liquidity_trading_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let treasury_trading_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let il_health_score_coefficient: u16 = crate::borsh_de_or_default(&mut reader)?;
        let position_health_score_coefficient: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_info_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddPoolIxArgs {
                min_overcollateral_ratio,
                max_liquidation_overcollateral_ratio,
                liquidity_trading_fee_bps,
                treasury_trading_fee_bps,
                il_health_score_coefficient,
                position_health_score_coefficient,
                oracle_info_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.min_overcollateral_ratio, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.max_liquidation_overcollateral_ratio,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.liquidity_trading_fee_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.treasury_trading_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.il_health_score_coefficient,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.position_health_score_coefficient,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.oracle_info_index, &mut writer)?;
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
    add_pool_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
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
    add_pool_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
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
    add_pool_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_pool_verify_account_keys(
    accounts: AddPoolAccounts<'_, '_>,
    keys: AddPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.onasset_token_account.key, keys.onasset_token_account),
        (*accounts.underlying_asset_mint.key, keys.underlying_asset_mint),
        (
            *accounts.underlying_asset_token_account.key,
            keys.underlying_asset_token_account,
        ),
        (*accounts.system_program.key, keys.system_program),
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
    for should_be_writable in [accounts.admin, accounts.pools] {
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
pub const UPDATE_PRICES_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePricesAccounts<'me, 'info> {
    pub oracles: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePricesKeys {
    pub oracles: Pubkey,
}
impl From<UpdatePricesAccounts<'_, '_>> for UpdatePricesKeys {
    fn from(accounts: UpdatePricesAccounts) -> Self {
        Self {
            oracles: *accounts.oracles.key,
        }
    }
}
impl From<UpdatePricesKeys> for [AccountMeta; UPDATE_PRICES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePricesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PRICES_IX_ACCOUNTS_LEN]> for UpdatePricesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PRICES_IX_ACCOUNTS_LEN]) -> Self {
        Self { oracles: pubkeys[0] }
    }
}
impl<'info> From<UpdatePricesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PRICES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePricesAccounts<'_, 'info>) -> Self {
        [accounts.oracles.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PRICES_IX_ACCOUNTS_LEN]>
for UpdatePricesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PRICES_IX_ACCOUNTS_LEN]) -> Self {
        Self { oracles: &arr[0] }
    }
}
pub const UPDATE_PRICES_IX_DISCM: [u8; 8usize] = [62, 161, 234, 136, 106, 26, 18, 160];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePricesIxArgs {
    pub oracle_indices: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePricesIxData(pub UpdatePricesIxArgs);
impl From<UpdatePricesIxArgs> for UpdatePricesIxData {
    fn from(args: UpdatePricesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePricesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PRICES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let oracle_indices: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdatePricesIxArgs {
                oracle_indices,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PRICES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_indices, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_prices_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePricesKeys,
    args: UpdatePricesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PRICES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePricesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_prices_ix(
    keys: UpdatePricesKeys,
    args: UpdatePricesIxArgs,
) -> std::io::Result<Instruction> {
    update_prices_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn update_prices_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePricesAccounts<'_, '_>,
    args: UpdatePricesIxArgs,
) -> ProgramResult {
    let keys: UpdatePricesKeys = accounts.into();
    let ix = update_prices_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_prices_invoke(
    accounts: UpdatePricesAccounts<'_, '_>,
    args: UpdatePricesIxArgs,
) -> ProgramResult {
    update_prices_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn update_prices_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePricesAccounts<'_, '_>,
    args: UpdatePricesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePricesKeys = accounts.into();
    let ix = update_prices_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_prices_invoke_signed(
    accounts: UpdatePricesAccounts<'_, '_>,
    args: UpdatePricesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_prices_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_prices_verify_account_keys(
    accounts: UpdatePricesAccounts<'_, '_>,
    keys: UpdatePricesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.oracles.key, keys.oracles)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_prices_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePricesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.oracles] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_prices_verify_account_privileges<'me, 'info>(
    accounts: UpdatePricesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_prices_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_BORROW_POSITION_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeBorrowPositionAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub user_onasset_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeBorrowPositionKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub vault: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub onasset_mint: Pubkey,
    pub user_onasset_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitializeBorrowPositionAccounts<'_, '_>> for InitializeBorrowPositionKeys {
    fn from(accounts: InitializeBorrowPositionAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            vault: *accounts.vault.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            onasset_mint: *accounts.onasset_mint.key,
            user_onasset_token_account: *accounts.user_onasset_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitializeBorrowPositionKeys>
for [AccountMeta; INITIALIZE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeBorrowPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_onasset_token_account,
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
impl From<[Pubkey; INITIALIZE_BORROW_POSITION_IX_ACCOUNTS_LEN]>
for InitializeBorrowPositionKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_BORROW_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
            vault: pubkeys[5],
            user_collateral_token_account: pubkeys[6],
            onasset_mint: pubkeys[7],
            user_onasset_token_account: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializeBorrowPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeBorrowPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.vault.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.onasset_mint.clone(),
            accounts.user_onasset_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_BORROW_POSITION_IX_ACCOUNTS_LEN]>
for InitializeBorrowPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_BORROW_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
            vault: &arr[5],
            user_collateral_token_account: &arr[6],
            onasset_mint: &arr[7],
            user_onasset_token_account: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const INITIALIZE_BORROW_POSITION_IX_DISCM: [u8; 8usize] = [
    101, 93, 229, 188, 115, 151, 231, 124,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeBorrowPositionIxArgs {
    pub pool_index: u8,
    pub onasset_amount: u64,
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeBorrowPositionIxData(pub InitializeBorrowPositionIxArgs);
impl From<InitializeBorrowPositionIxArgs> for InitializeBorrowPositionIxData {
    fn from(args: InitializeBorrowPositionIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeBorrowPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_BORROW_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let onasset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeBorrowPositionIxArgs {
                pool_index,
                onasset_amount,
                collateral_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_BORROW_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pool_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.onasset_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.collateral_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_borrow_position_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeBorrowPositionKeys,
    args: InitializeBorrowPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_BORROW_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeBorrowPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_borrow_position_ix(
    keys: InitializeBorrowPositionKeys,
    args: InitializeBorrowPositionIxArgs,
) -> std::io::Result<Instruction> {
    initialize_borrow_position_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn initialize_borrow_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBorrowPositionAccounts<'_, '_>,
    args: InitializeBorrowPositionIxArgs,
) -> ProgramResult {
    let keys: InitializeBorrowPositionKeys = accounts.into();
    let ix = initialize_borrow_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_borrow_position_invoke(
    accounts: InitializeBorrowPositionAccounts<'_, '_>,
    args: InitializeBorrowPositionIxArgs,
) -> ProgramResult {
    initialize_borrow_position_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn initialize_borrow_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBorrowPositionAccounts<'_, '_>,
    args: InitializeBorrowPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeBorrowPositionKeys = accounts.into();
    let ix = initialize_borrow_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_borrow_position_invoke_signed(
    accounts: InitializeBorrowPositionAccounts<'_, '_>,
    args: InitializeBorrowPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_borrow_position_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_borrow_position_verify_account_keys(
    accounts: InitializeBorrowPositionAccounts<'_, '_>,
    keys: InitializeBorrowPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (*accounts.vault.key, keys.vault),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.user_onasset_token_account.key, keys.user_onasset_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_borrow_position_verify_writable_privileges<'me, 'info>(
    accounts: InitializeBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.pools,
        accounts.oracles,
        accounts.vault,
        accounts.user_collateral_token_account,
        accounts.onasset_mint,
        accounts.user_onasset_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_borrow_position_verify_signer_privileges<'me, 'info>(
    accounts: InitializeBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_borrow_position_verify_account_privileges<'me, 'info>(
    accounts: InitializeBorrowPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_borrow_position_verify_writable_privileges(accounts)?;
    initialize_borrow_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_COLLATERAL_TO_BORROW_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddCollateralToBorrowAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddCollateralToBorrowKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub vault: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddCollateralToBorrowAccounts<'_, '_>> for AddCollateralToBorrowKeys {
    fn from(accounts: AddCollateralToBorrowAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            vault: *accounts.vault.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddCollateralToBorrowKeys>
for [AccountMeta; ADD_COLLATERAL_TO_BORROW_IX_ACCOUNTS_LEN] {
    fn from(keys: AddCollateralToBorrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
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
impl From<[Pubkey; ADD_COLLATERAL_TO_BORROW_IX_ACCOUNTS_LEN]>
for AddCollateralToBorrowKeys {
    fn from(pubkeys: [Pubkey; ADD_COLLATERAL_TO_BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            vault: pubkeys[3],
            user_collateral_token_account: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<AddCollateralToBorrowAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_COLLATERAL_TO_BORROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddCollateralToBorrowAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.vault.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_COLLATERAL_TO_BORROW_IX_ACCOUNTS_LEN]>
for AddCollateralToBorrowAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_COLLATERAL_TO_BORROW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            vault: &arr[3],
            user_collateral_token_account: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const ADD_COLLATERAL_TO_BORROW_IX_DISCM: [u8; 8usize] = [
    205, 12, 181, 19, 249, 95, 13, 197,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddCollateralToBorrowIxArgs {
    pub borrow_index: u8,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddCollateralToBorrowIxData(pub AddCollateralToBorrowIxArgs);
impl From<AddCollateralToBorrowIxArgs> for AddCollateralToBorrowIxData {
    fn from(args: AddCollateralToBorrowIxArgs) -> Self {
        Self(args)
    }
}
impl AddCollateralToBorrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_COLLATERAL_TO_BORROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddCollateralToBorrowIxArgs {
                borrow_index,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_COLLATERAL_TO_BORROW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.borrow_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_collateral_to_borrow_ix_with_program_id(
    program_id: Pubkey,
    keys: AddCollateralToBorrowKeys,
    args: AddCollateralToBorrowIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_COLLATERAL_TO_BORROW_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddCollateralToBorrowIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_collateral_to_borrow_ix(
    keys: AddCollateralToBorrowKeys,
    args: AddCollateralToBorrowIxArgs,
) -> std::io::Result<Instruction> {
    add_collateral_to_borrow_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn add_collateral_to_borrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddCollateralToBorrowAccounts<'_, '_>,
    args: AddCollateralToBorrowIxArgs,
) -> ProgramResult {
    let keys: AddCollateralToBorrowKeys = accounts.into();
    let ix = add_collateral_to_borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_collateral_to_borrow_invoke(
    accounts: AddCollateralToBorrowAccounts<'_, '_>,
    args: AddCollateralToBorrowIxArgs,
) -> ProgramResult {
    add_collateral_to_borrow_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn add_collateral_to_borrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddCollateralToBorrowAccounts<'_, '_>,
    args: AddCollateralToBorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddCollateralToBorrowKeys = accounts.into();
    let ix = add_collateral_to_borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_collateral_to_borrow_invoke_signed(
    accounts: AddCollateralToBorrowAccounts<'_, '_>,
    args: AddCollateralToBorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_collateral_to_borrow_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_collateral_to_borrow_verify_account_keys(
    accounts: AddCollateralToBorrowAccounts<'_, '_>,
    keys: AddCollateralToBorrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.vault.key, keys.vault),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_collateral_to_borrow_verify_writable_privileges<'me, 'info>(
    accounts: AddCollateralToBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.vault,
        accounts.user_collateral_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_collateral_to_borrow_verify_signer_privileges<'me, 'info>(
    accounts: AddCollateralToBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_collateral_to_borrow_verify_account_privileges<'me, 'info>(
    accounts: AddCollateralToBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_collateral_to_borrow_verify_writable_privileges(accounts)?;
    add_collateral_to_borrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_COLLATERAL_FROM_BORROW_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawCollateralFromBorrowAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawCollateralFromBorrowKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub vault: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawCollateralFromBorrowAccounts<'_, '_>>
for WithdrawCollateralFromBorrowKeys {
    fn from(accounts: WithdrawCollateralFromBorrowAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            vault: *accounts.vault.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawCollateralFromBorrowKeys>
for [AccountMeta; WITHDRAW_COLLATERAL_FROM_BORROW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawCollateralFromBorrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
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
impl From<[Pubkey; WITHDRAW_COLLATERAL_FROM_BORROW_IX_ACCOUNTS_LEN]>
for WithdrawCollateralFromBorrowKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_COLLATERAL_FROM_BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
            vault: pubkeys[5],
            user_collateral_token_account: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<WithdrawCollateralFromBorrowAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_COLLATERAL_FROM_BORROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawCollateralFromBorrowAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.vault.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_COLLATERAL_FROM_BORROW_IX_ACCOUNTS_LEN]>
for WithdrawCollateralFromBorrowAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_COLLATERAL_FROM_BORROW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
            vault: &arr[5],
            user_collateral_token_account: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const WITHDRAW_COLLATERAL_FROM_BORROW_IX_DISCM: [u8; 8usize] = [
    192, 177, 60, 105, 57, 92, 160, 221,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawCollateralFromBorrowIxArgs {
    pub borrow_index: u8,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawCollateralFromBorrowIxData(pub WithdrawCollateralFromBorrowIxArgs);
impl From<WithdrawCollateralFromBorrowIxArgs> for WithdrawCollateralFromBorrowIxData {
    fn from(args: WithdrawCollateralFromBorrowIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawCollateralFromBorrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_COLLATERAL_FROM_BORROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawCollateralFromBorrowIxArgs {
                borrow_index,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_COLLATERAL_FROM_BORROW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.borrow_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_collateral_from_borrow_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawCollateralFromBorrowKeys,
    args: WithdrawCollateralFromBorrowIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_COLLATERAL_FROM_BORROW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawCollateralFromBorrowIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_collateral_from_borrow_ix(
    keys: WithdrawCollateralFromBorrowKeys,
    args: WithdrawCollateralFromBorrowIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_collateral_from_borrow_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn withdraw_collateral_from_borrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawCollateralFromBorrowAccounts<'_, '_>,
    args: WithdrawCollateralFromBorrowIxArgs,
) -> ProgramResult {
    let keys: WithdrawCollateralFromBorrowKeys = accounts.into();
    let ix = withdraw_collateral_from_borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_collateral_from_borrow_invoke(
    accounts: WithdrawCollateralFromBorrowAccounts<'_, '_>,
    args: WithdrawCollateralFromBorrowIxArgs,
) -> ProgramResult {
    withdraw_collateral_from_borrow_invoke_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_collateral_from_borrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawCollateralFromBorrowAccounts<'_, '_>,
    args: WithdrawCollateralFromBorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawCollateralFromBorrowKeys = accounts.into();
    let ix = withdraw_collateral_from_borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_collateral_from_borrow_invoke_signed(
    accounts: WithdrawCollateralFromBorrowAccounts<'_, '_>,
    args: WithdrawCollateralFromBorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_collateral_from_borrow_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_collateral_from_borrow_verify_account_keys(
    accounts: WithdrawCollateralFromBorrowAccounts<'_, '_>,
    keys: WithdrawCollateralFromBorrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (*accounts.vault.key, keys.vault),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_from_borrow_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawCollateralFromBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.pools,
        accounts.oracles,
        accounts.vault,
        accounts.user_collateral_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_from_borrow_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawCollateralFromBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_from_borrow_verify_account_privileges<'me, 'info>(
    accounts: WithdrawCollateralFromBorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_collateral_from_borrow_verify_writable_privileges(accounts)?;
    withdraw_collateral_from_borrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAY_BORROW_DEBT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct PayBorrowDebtAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub payer_onasset_token_account: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PayBorrowDebtKeys {
    pub payer: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub payer_onasset_token_account: Pubkey,
    pub onasset_mint: Pubkey,
    pub token_program: Pubkey,
}
impl From<PayBorrowDebtAccounts<'_, '_>> for PayBorrowDebtKeys {
    fn from(accounts: PayBorrowDebtAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            payer_onasset_token_account: *accounts.payer_onasset_token_account.key,
            onasset_mint: *accounts.onasset_mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PayBorrowDebtKeys> for [AccountMeta; PAY_BORROW_DEBT_IX_ACCOUNTS_LEN] {
    fn from(keys: PayBorrowDebtKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer_onasset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
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
impl From<[Pubkey; PAY_BORROW_DEBT_IX_ACCOUNTS_LEN]> for PayBorrowDebtKeys {
    fn from(pubkeys: [Pubkey; PAY_BORROW_DEBT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            payer_onasset_token_account: pubkeys[4],
            onasset_mint: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<PayBorrowDebtAccounts<'_, 'info>>
for [AccountInfo<'info>; PAY_BORROW_DEBT_IX_ACCOUNTS_LEN] {
    fn from(accounts: PayBorrowDebtAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.payer_onasset_token_account.clone(),
            accounts.onasset_mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAY_BORROW_DEBT_IX_ACCOUNTS_LEN]>
for PayBorrowDebtAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAY_BORROW_DEBT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            payer_onasset_token_account: &arr[4],
            onasset_mint: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const PAY_BORROW_DEBT_IX_DISCM: [u8; 8usize] = [182, 215, 36, 35, 119, 58, 60, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayBorrowDebtIxArgs {
    pub user: Pubkey,
    pub borrow_index: u8,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PayBorrowDebtIxData(pub PayBorrowDebtIxArgs);
impl From<PayBorrowDebtIxArgs> for PayBorrowDebtIxData {
    fn from(args: PayBorrowDebtIxArgs) -> Self {
        Self(args)
    }
}
impl PayBorrowDebtIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAY_BORROW_DEBT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PayBorrowDebtIxArgs {
                user,
                borrow_index,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAY_BORROW_DEBT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.borrow_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pay_borrow_debt_ix_with_program_id(
    program_id: Pubkey,
    keys: PayBorrowDebtKeys,
    args: PayBorrowDebtIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAY_BORROW_DEBT_IX_ACCOUNTS_LEN] = keys.into();
    let data: PayBorrowDebtIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pay_borrow_debt_ix(
    keys: PayBorrowDebtKeys,
    args: PayBorrowDebtIxArgs,
) -> std::io::Result<Instruction> {
    pay_borrow_debt_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn pay_borrow_debt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PayBorrowDebtAccounts<'_, '_>,
    args: PayBorrowDebtIxArgs,
) -> ProgramResult {
    let keys: PayBorrowDebtKeys = accounts.into();
    let ix = pay_borrow_debt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pay_borrow_debt_invoke(
    accounts: PayBorrowDebtAccounts<'_, '_>,
    args: PayBorrowDebtIxArgs,
) -> ProgramResult {
    pay_borrow_debt_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn pay_borrow_debt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PayBorrowDebtAccounts<'_, '_>,
    args: PayBorrowDebtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PayBorrowDebtKeys = accounts.into();
    let ix = pay_borrow_debt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pay_borrow_debt_invoke_signed(
    accounts: PayBorrowDebtAccounts<'_, '_>,
    args: PayBorrowDebtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pay_borrow_debt_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pay_borrow_debt_verify_account_keys(
    accounts: PayBorrowDebtAccounts<'_, '_>,
    keys: PayBorrowDebtKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.payer_onasset_token_account.key, keys.payer_onasset_token_account),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pay_borrow_debt_verify_writable_privileges<'me, 'info>(
    accounts: PayBorrowDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.payer_onasset_token_account,
        accounts.onasset_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pay_borrow_debt_verify_signer_privileges<'me, 'info>(
    accounts: PayBorrowDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pay_borrow_debt_verify_account_privileges<'me, 'info>(
    accounts: PayBorrowDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pay_borrow_debt_verify_writable_privileges(accounts)?;
    pay_borrow_debt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BORROW_MORE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct BorrowMoreAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub user_onasset_token_account: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowMoreKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub user_onasset_token_account: Pubkey,
    pub onasset_mint: Pubkey,
    pub token_program: Pubkey,
}
impl From<BorrowMoreAccounts<'_, '_>> for BorrowMoreKeys {
    fn from(accounts: BorrowMoreAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            user_onasset_token_account: *accounts.user_onasset_token_account.key,
            onasset_mint: *accounts.onasset_mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<BorrowMoreKeys> for [AccountMeta; BORROW_MORE_IX_ACCOUNTS_LEN] {
    fn from(keys: BorrowMoreKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_onasset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
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
impl From<[Pubkey; BORROW_MORE_IX_ACCOUNTS_LEN]> for BorrowMoreKeys {
    fn from(pubkeys: [Pubkey; BORROW_MORE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
            user_onasset_token_account: pubkeys[5],
            onasset_mint: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<BorrowMoreAccounts<'_, 'info>>
for [AccountInfo<'info>; BORROW_MORE_IX_ACCOUNTS_LEN] {
    fn from(accounts: BorrowMoreAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.user_onasset_token_account.clone(),
            accounts.onasset_mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BORROW_MORE_IX_ACCOUNTS_LEN]>
for BorrowMoreAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BORROW_MORE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
            user_onasset_token_account: &arr[5],
            onasset_mint: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const BORROW_MORE_IX_DISCM: [u8; 8usize] = [99, 191, 157, 15, 122, 190, 25, 86];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowMoreIxArgs {
    pub borrow_index: u8,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowMoreIxData(pub BorrowMoreIxArgs);
impl From<BorrowMoreIxArgs> for BorrowMoreIxData {
    fn from(args: BorrowMoreIxArgs) -> Self {
        Self(args)
    }
}
impl BorrowMoreIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_MORE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BorrowMoreIxArgs {
                borrow_index,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_MORE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.borrow_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn borrow_more_ix_with_program_id(
    program_id: Pubkey,
    keys: BorrowMoreKeys,
    args: BorrowMoreIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BORROW_MORE_IX_ACCOUNTS_LEN] = keys.into();
    let data: BorrowMoreIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn borrow_more_ix(
    keys: BorrowMoreKeys,
    args: BorrowMoreIxArgs,
) -> std::io::Result<Instruction> {
    borrow_more_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn borrow_more_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BorrowMoreAccounts<'_, '_>,
    args: BorrowMoreIxArgs,
) -> ProgramResult {
    let keys: BorrowMoreKeys = accounts.into();
    let ix = borrow_more_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn borrow_more_invoke(
    accounts: BorrowMoreAccounts<'_, '_>,
    args: BorrowMoreIxArgs,
) -> ProgramResult {
    borrow_more_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn borrow_more_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BorrowMoreAccounts<'_, '_>,
    args: BorrowMoreIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BorrowMoreKeys = accounts.into();
    let ix = borrow_more_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn borrow_more_invoke_signed(
    accounts: BorrowMoreAccounts<'_, '_>,
    args: BorrowMoreIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    borrow_more_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, args, seeds)
}
pub fn borrow_more_verify_account_keys(
    accounts: BorrowMoreAccounts<'_, '_>,
    keys: BorrowMoreKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (*accounts.user_onasset_token_account.key, keys.user_onasset_token_account),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn borrow_more_verify_writable_privileges<'me, 'info>(
    accounts: BorrowMoreAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.user_onasset_token_account,
        accounts.onasset_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn borrow_more_verify_signer_privileges<'me, 'info>(
    accounts: BorrowMoreAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn borrow_more_verify_account_privileges<'me, 'info>(
    accounts: BorrowMoreAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    borrow_more_verify_writable_privileges(accounts)?;
    borrow_more_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_COLLATERAL_TO_COMET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddCollateralToCometAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddCollateralToCometKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub vault: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddCollateralToCometAccounts<'_, '_>> for AddCollateralToCometKeys {
    fn from(accounts: AddCollateralToCometAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            vault: *accounts.vault.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddCollateralToCometKeys>
for [AccountMeta; ADD_COLLATERAL_TO_COMET_IX_ACCOUNTS_LEN] {
    fn from(keys: AddCollateralToCometKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
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
impl From<[Pubkey; ADD_COLLATERAL_TO_COMET_IX_ACCOUNTS_LEN]>
for AddCollateralToCometKeys {
    fn from(pubkeys: [Pubkey; ADD_COLLATERAL_TO_COMET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            vault: pubkeys[3],
            user_collateral_token_account: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<AddCollateralToCometAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_COLLATERAL_TO_COMET_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddCollateralToCometAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.vault.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_COLLATERAL_TO_COMET_IX_ACCOUNTS_LEN]>
for AddCollateralToCometAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_COLLATERAL_TO_COMET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            vault: &arr[3],
            user_collateral_token_account: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const ADD_COLLATERAL_TO_COMET_IX_DISCM: [u8; 8usize] = [
    209, 211, 225, 123, 219, 71, 154, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddCollateralToCometIxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddCollateralToCometIxData(pub AddCollateralToCometIxArgs);
impl From<AddCollateralToCometIxArgs> for AddCollateralToCometIxData {
    fn from(args: AddCollateralToCometIxArgs) -> Self {
        Self(args)
    }
}
impl AddCollateralToCometIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_COLLATERAL_TO_COMET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddCollateralToCometIxArgs {
                collateral_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_COLLATERAL_TO_COMET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.collateral_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_collateral_to_comet_ix_with_program_id(
    program_id: Pubkey,
    keys: AddCollateralToCometKeys,
    args: AddCollateralToCometIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_COLLATERAL_TO_COMET_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddCollateralToCometIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_collateral_to_comet_ix(
    keys: AddCollateralToCometKeys,
    args: AddCollateralToCometIxArgs,
) -> std::io::Result<Instruction> {
    add_collateral_to_comet_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn add_collateral_to_comet_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddCollateralToCometAccounts<'_, '_>,
    args: AddCollateralToCometIxArgs,
) -> ProgramResult {
    let keys: AddCollateralToCometKeys = accounts.into();
    let ix = add_collateral_to_comet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_collateral_to_comet_invoke(
    accounts: AddCollateralToCometAccounts<'_, '_>,
    args: AddCollateralToCometIxArgs,
) -> ProgramResult {
    add_collateral_to_comet_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn add_collateral_to_comet_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddCollateralToCometAccounts<'_, '_>,
    args: AddCollateralToCometIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddCollateralToCometKeys = accounts.into();
    let ix = add_collateral_to_comet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_collateral_to_comet_invoke_signed(
    accounts: AddCollateralToCometAccounts<'_, '_>,
    args: AddCollateralToCometIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_collateral_to_comet_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_collateral_to_comet_verify_account_keys(
    accounts: AddCollateralToCometAccounts<'_, '_>,
    keys: AddCollateralToCometKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.vault.key, keys.vault),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_collateral_to_comet_verify_writable_privileges<'me, 'info>(
    accounts: AddCollateralToCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.vault,
        accounts.user_collateral_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_collateral_to_comet_verify_signer_privileges<'me, 'info>(
    accounts: AddCollateralToCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_collateral_to_comet_verify_account_privileges<'me, 'info>(
    accounts: AddCollateralToCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_collateral_to_comet_verify_writable_privileges(accounts)?;
    add_collateral_to_comet_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_COLLATERAL_FROM_COMET_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawCollateralFromCometAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawCollateralFromCometKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub vault: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawCollateralFromCometAccounts<'_, '_>>
for WithdrawCollateralFromCometKeys {
    fn from(accounts: WithdrawCollateralFromCometAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            vault: *accounts.vault.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawCollateralFromCometKeys>
for [AccountMeta; WITHDRAW_COLLATERAL_FROM_COMET_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawCollateralFromCometKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
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
impl From<[Pubkey; WITHDRAW_COLLATERAL_FROM_COMET_IX_ACCOUNTS_LEN]>
for WithdrawCollateralFromCometKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_COLLATERAL_FROM_COMET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
            vault: pubkeys[5],
            user_collateral_token_account: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<WithdrawCollateralFromCometAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_COLLATERAL_FROM_COMET_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawCollateralFromCometAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.vault.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_COLLATERAL_FROM_COMET_IX_ACCOUNTS_LEN]>
for WithdrawCollateralFromCometAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_COLLATERAL_FROM_COMET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
            vault: &arr[5],
            user_collateral_token_account: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const WITHDRAW_COLLATERAL_FROM_COMET_IX_DISCM: [u8; 8usize] = [
    208, 162, 137, 187, 186, 161, 87, 136,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawCollateralFromCometIxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawCollateralFromCometIxData(pub WithdrawCollateralFromCometIxArgs);
impl From<WithdrawCollateralFromCometIxArgs> for WithdrawCollateralFromCometIxData {
    fn from(args: WithdrawCollateralFromCometIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawCollateralFromCometIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_COLLATERAL_FROM_COMET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawCollateralFromCometIxArgs {
                collateral_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_COLLATERAL_FROM_COMET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.collateral_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_collateral_from_comet_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawCollateralFromCometKeys,
    args: WithdrawCollateralFromCometIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_COLLATERAL_FROM_COMET_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawCollateralFromCometIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_collateral_from_comet_ix(
    keys: WithdrawCollateralFromCometKeys,
    args: WithdrawCollateralFromCometIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_collateral_from_comet_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn withdraw_collateral_from_comet_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawCollateralFromCometAccounts<'_, '_>,
    args: WithdrawCollateralFromCometIxArgs,
) -> ProgramResult {
    let keys: WithdrawCollateralFromCometKeys = accounts.into();
    let ix = withdraw_collateral_from_comet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_collateral_from_comet_invoke(
    accounts: WithdrawCollateralFromCometAccounts<'_, '_>,
    args: WithdrawCollateralFromCometIxArgs,
) -> ProgramResult {
    withdraw_collateral_from_comet_invoke_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_collateral_from_comet_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawCollateralFromCometAccounts<'_, '_>,
    args: WithdrawCollateralFromCometIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawCollateralFromCometKeys = accounts.into();
    let ix = withdraw_collateral_from_comet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_collateral_from_comet_invoke_signed(
    accounts: WithdrawCollateralFromCometAccounts<'_, '_>,
    args: WithdrawCollateralFromCometIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_collateral_from_comet_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_collateral_from_comet_verify_account_keys(
    accounts: WithdrawCollateralFromCometAccounts<'_, '_>,
    keys: WithdrawCollateralFromCometKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (*accounts.vault.key, keys.vault),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_from_comet_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawCollateralFromCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.pools,
        accounts.oracles,
        accounts.vault,
        accounts.user_collateral_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_from_comet_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawCollateralFromCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_collateral_from_comet_verify_account_privileges<'me, 'info>(
    accounts: WithdrawCollateralFromCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_collateral_from_comet_verify_writable_privileges(accounts)?;
    withdraw_collateral_from_comet_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_TO_COMET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityToCometAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityToCometKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
}
impl From<AddLiquidityToCometAccounts<'_, '_>> for AddLiquidityToCometKeys {
    fn from(accounts: AddLiquidityToCometAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
        }
    }
}
impl From<AddLiquidityToCometKeys>
for [AccountMeta; ADD_LIQUIDITY_TO_COMET_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityToCometKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_TO_COMET_IX_ACCOUNTS_LEN]> for AddLiquidityToCometKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_TO_COMET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
        }
    }
}
impl<'info> From<AddLiquidityToCometAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_TO_COMET_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityToCometAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_TO_COMET_IX_ACCOUNTS_LEN]>
for AddLiquidityToCometAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_TO_COMET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
        }
    }
}
pub const ADD_LIQUIDITY_TO_COMET_IX_DISCM: [u8; 8usize] = [
    25, 218, 193, 157, 185, 81, 42, 173,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityToCometIxArgs {
    pub pool_index: u8,
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityToCometIxData(pub AddLiquidityToCometIxArgs);
impl From<AddLiquidityToCometIxArgs> for AddLiquidityToCometIxData {
    fn from(args: AddLiquidityToCometIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityToCometIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_TO_COMET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddLiquidityToCometIxArgs {
                pool_index,
                collateral_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_TO_COMET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pool_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.collateral_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_to_comet_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityToCometKeys,
    args: AddLiquidityToCometIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_TO_COMET_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityToCometIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_to_comet_ix(
    keys: AddLiquidityToCometKeys,
    args: AddLiquidityToCometIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_to_comet_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn add_liquidity_to_comet_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityToCometAccounts<'_, '_>,
    args: AddLiquidityToCometIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityToCometKeys = accounts.into();
    let ix = add_liquidity_to_comet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_to_comet_invoke(
    accounts: AddLiquidityToCometAccounts<'_, '_>,
    args: AddLiquidityToCometIxArgs,
) -> ProgramResult {
    add_liquidity_to_comet_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn add_liquidity_to_comet_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityToCometAccounts<'_, '_>,
    args: AddLiquidityToCometIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityToCometKeys = accounts.into();
    let ix = add_liquidity_to_comet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_to_comet_invoke_signed(
    accounts: AddLiquidityToCometAccounts<'_, '_>,
    args: AddLiquidityToCometIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_to_comet_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_to_comet_verify_account_keys(
    accounts: AddLiquidityToCometAccounts<'_, '_>,
    keys: AddLiquidityToCometKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_to_comet_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityToCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.pools,
        accounts.oracles,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_to_comet_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityToCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_to_comet_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityToCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_to_comet_verify_writable_privileges(accounts)?;
    add_liquidity_to_comet_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_LIQUIDITY_FROM_COMET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawLiquidityFromCometAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawLiquidityFromCometKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
}
impl From<WithdrawLiquidityFromCometAccounts<'_, '_>>
for WithdrawLiquidityFromCometKeys {
    fn from(accounts: WithdrawLiquidityFromCometAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
        }
    }
}
impl From<WithdrawLiquidityFromCometKeys>
for [AccountMeta; WITHDRAW_LIQUIDITY_FROM_COMET_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawLiquidityFromCometKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_LIQUIDITY_FROM_COMET_IX_ACCOUNTS_LEN]>
for WithdrawLiquidityFromCometKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_LIQUIDITY_FROM_COMET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
        }
    }
}
impl<'info> From<WithdrawLiquidityFromCometAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_LIQUIDITY_FROM_COMET_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawLiquidityFromCometAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_LIQUIDITY_FROM_COMET_IX_ACCOUNTS_LEN]>
for WithdrawLiquidityFromCometAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_LIQUIDITY_FROM_COMET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
        }
    }
}
pub const WITHDRAW_LIQUIDITY_FROM_COMET_IX_DISCM: [u8; 8usize] = [
    173, 148, 139, 45, 140, 127, 26, 32,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawLiquidityFromCometIxArgs {
    pub comet_position_index: u8,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawLiquidityFromCometIxData(pub WithdrawLiquidityFromCometIxArgs);
impl From<WithdrawLiquidityFromCometIxArgs> for WithdrawLiquidityFromCometIxData {
    fn from(args: WithdrawLiquidityFromCometIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawLiquidityFromCometIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_LIQUIDITY_FROM_COMET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawLiquidityFromCometIxArgs {
                comet_position_index,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_LIQUIDITY_FROM_COMET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.comet_position_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_liquidity_from_comet_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawLiquidityFromCometKeys,
    args: WithdrawLiquidityFromCometIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_LIQUIDITY_FROM_COMET_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawLiquidityFromCometIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_liquidity_from_comet_ix(
    keys: WithdrawLiquidityFromCometKeys,
    args: WithdrawLiquidityFromCometIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_liquidity_from_comet_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn withdraw_liquidity_from_comet_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLiquidityFromCometAccounts<'_, '_>,
    args: WithdrawLiquidityFromCometIxArgs,
) -> ProgramResult {
    let keys: WithdrawLiquidityFromCometKeys = accounts.into();
    let ix = withdraw_liquidity_from_comet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_liquidity_from_comet_invoke(
    accounts: WithdrawLiquidityFromCometAccounts<'_, '_>,
    args: WithdrawLiquidityFromCometIxArgs,
) -> ProgramResult {
    withdraw_liquidity_from_comet_invoke_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_liquidity_from_comet_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLiquidityFromCometAccounts<'_, '_>,
    args: WithdrawLiquidityFromCometIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawLiquidityFromCometKeys = accounts.into();
    let ix = withdraw_liquidity_from_comet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_liquidity_from_comet_invoke_signed(
    accounts: WithdrawLiquidityFromCometAccounts<'_, '_>,
    args: WithdrawLiquidityFromCometIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_liquidity_from_comet_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_liquidity_from_comet_verify_account_keys(
    accounts: WithdrawLiquidityFromCometAccounts<'_, '_>,
    keys: WithdrawLiquidityFromCometKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_liquidity_from_comet_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawLiquidityFromCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.pools,
        accounts.oracles,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_liquidity_from_comet_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawLiquidityFromCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_liquidity_from_comet_verify_account_privileges<'me, 'info>(
    accounts: WithdrawLiquidityFromCometAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_liquidity_from_comet_verify_writable_privileges(accounts)?;
    withdraw_liquidity_from_comet_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUIDATE_COMET_COLLATERAL_ILD_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateCometCollateralIldAccounts<'me, 'info> {
    pub liquidator: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub liquidator_collateral_token_account: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateCometCollateralIldKeys {
    pub liquidator: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub collateral_mint: Pubkey,
    pub liquidator_collateral_token_account: Pubkey,
    pub vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<LiquidateCometCollateralIldAccounts<'_, '_>>
for LiquidateCometCollateralIldKeys {
    fn from(accounts: LiquidateCometCollateralIldAccounts) -> Self {
        Self {
            liquidator: *accounts.liquidator.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            collateral_mint: *accounts.collateral_mint.key,
            liquidator_collateral_token_account: *accounts
                .liquidator_collateral_token_account
                .key,
            vault: *accounts.vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LiquidateCometCollateralIldKeys>
for [AccountMeta; LIQUIDATE_COMET_COLLATERAL_ILD_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidateCometCollateralIldKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidator_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
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
impl From<[Pubkey; LIQUIDATE_COMET_COLLATERAL_ILD_IX_ACCOUNTS_LEN]>
for LiquidateCometCollateralIldKeys {
    fn from(pubkeys: [Pubkey; LIQUIDATE_COMET_COLLATERAL_ILD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidator: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
            collateral_mint: pubkeys[5],
            liquidator_collateral_token_account: pubkeys[6],
            vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<LiquidateCometCollateralIldAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUIDATE_COMET_COLLATERAL_ILD_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidateCometCollateralIldAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidator.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.collateral_mint.clone(),
            accounts.liquidator_collateral_token_account.clone(),
            accounts.vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LIQUIDATE_COMET_COLLATERAL_ILD_IX_ACCOUNTS_LEN]>
for LiquidateCometCollateralIldAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUIDATE_COMET_COLLATERAL_ILD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidator: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
            collateral_mint: &arr[5],
            liquidator_collateral_token_account: &arr[6],
            vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const LIQUIDATE_COMET_COLLATERAL_ILD_IX_DISCM: [u8; 8usize] = [
    173, 42, 162, 112, 216, 18, 147, 150,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidateCometCollateralIldIxArgs {
    pub user: Pubkey,
    pub comet_position_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateCometCollateralIldIxData(pub LiquidateCometCollateralIldIxArgs);
impl From<LiquidateCometCollateralIldIxArgs> for LiquidateCometCollateralIldIxData {
    fn from(args: LiquidateCometCollateralIldIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidateCometCollateralIldIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_COMET_COLLATERAL_ILD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidateCometCollateralIldIxArgs {
                user,
                comet_position_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_COMET_COLLATERAL_ILD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.comet_position_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_comet_collateral_ild_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidateCometCollateralIldKeys,
    args: LiquidateCometCollateralIldIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUIDATE_COMET_COLLATERAL_ILD_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LiquidateCometCollateralIldIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_comet_collateral_ild_ix(
    keys: LiquidateCometCollateralIldKeys,
    args: LiquidateCometCollateralIldIxArgs,
) -> std::io::Result<Instruction> {
    liquidate_comet_collateral_ild_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn liquidate_comet_collateral_ild_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateCometCollateralIldAccounts<'_, '_>,
    args: LiquidateCometCollateralIldIxArgs,
) -> ProgramResult {
    let keys: LiquidateCometCollateralIldKeys = accounts.into();
    let ix = liquidate_comet_collateral_ild_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquidate_comet_collateral_ild_invoke(
    accounts: LiquidateCometCollateralIldAccounts<'_, '_>,
    args: LiquidateCometCollateralIldIxArgs,
) -> ProgramResult {
    liquidate_comet_collateral_ild_invoke_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn liquidate_comet_collateral_ild_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateCometCollateralIldAccounts<'_, '_>,
    args: LiquidateCometCollateralIldIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidateCometCollateralIldKeys = accounts.into();
    let ix = liquidate_comet_collateral_ild_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquidate_comet_collateral_ild_invoke_signed(
    accounts: LiquidateCometCollateralIldAccounts<'_, '_>,
    args: LiquidateCometCollateralIldIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquidate_comet_collateral_ild_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquidate_comet_collateral_ild_verify_account_keys(
    accounts: LiquidateCometCollateralIldAccounts<'_, '_>,
    keys: LiquidateCometCollateralIldKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidator.key, keys.liquidator),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (
            *accounts.liquidator_collateral_token_account.key,
            keys.liquidator_collateral_token_account,
        ),
        (*accounts.vault.key, keys.vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquidate_comet_collateral_ild_verify_writable_privileges<'me, 'info>(
    accounts: LiquidateCometCollateralIldAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.collateral_mint,
        accounts.liquidator_collateral_token_account,
        accounts.vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquidate_comet_collateral_ild_verify_signer_privileges<'me, 'info>(
    accounts: LiquidateCometCollateralIldAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.liquidator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquidate_comet_collateral_ild_verify_account_privileges<'me, 'info>(
    accounts: LiquidateCometCollateralIldAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquidate_comet_collateral_ild_verify_writable_privileges(accounts)?;
    liquidate_comet_collateral_ild_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUIDATE_COMET_ONASSET_ILD_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateCometOnassetIldAccounts<'me, 'info> {
    pub liquidator: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub liquidator_onasset_token_account: &'me AccountInfo<'info>,
    pub liquidator_collateral_token_account: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateCometOnassetIldKeys {
    pub liquidator: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub onasset_mint: Pubkey,
    pub liquidator_onasset_token_account: Pubkey,
    pub liquidator_collateral_token_account: Pubkey,
    pub vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<LiquidateCometOnassetIldAccounts<'_, '_>> for LiquidateCometOnassetIldKeys {
    fn from(accounts: LiquidateCometOnassetIldAccounts) -> Self {
        Self {
            liquidator: *accounts.liquidator.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            onasset_mint: *accounts.onasset_mint.key,
            liquidator_onasset_token_account: *accounts
                .liquidator_onasset_token_account
                .key,
            liquidator_collateral_token_account: *accounts
                .liquidator_collateral_token_account
                .key,
            vault: *accounts.vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LiquidateCometOnassetIldKeys>
for [AccountMeta; LIQUIDATE_COMET_ONASSET_ILD_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidateCometOnassetIldKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidator_onasset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidator_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
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
impl From<[Pubkey; LIQUIDATE_COMET_ONASSET_ILD_IX_ACCOUNTS_LEN]>
for LiquidateCometOnassetIldKeys {
    fn from(pubkeys: [Pubkey; LIQUIDATE_COMET_ONASSET_ILD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidator: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
            onasset_mint: pubkeys[5],
            liquidator_onasset_token_account: pubkeys[6],
            liquidator_collateral_token_account: pubkeys[7],
            vault: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<LiquidateCometOnassetIldAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUIDATE_COMET_ONASSET_ILD_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidateCometOnassetIldAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidator.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.onasset_mint.clone(),
            accounts.liquidator_onasset_token_account.clone(),
            accounts.liquidator_collateral_token_account.clone(),
            accounts.vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; LIQUIDATE_COMET_ONASSET_ILD_IX_ACCOUNTS_LEN]>
for LiquidateCometOnassetIldAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUIDATE_COMET_ONASSET_ILD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidator: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
            onasset_mint: &arr[5],
            liquidator_onasset_token_account: &arr[6],
            liquidator_collateral_token_account: &arr[7],
            vault: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const LIQUIDATE_COMET_ONASSET_ILD_IX_DISCM: [u8; 8usize] = [
    203, 186, 24, 213, 251, 103, 57, 65,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidateCometOnassetIldIxArgs {
    pub user: Pubkey,
    pub comet_position_index: u8,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateCometOnassetIldIxData(pub LiquidateCometOnassetIldIxArgs);
impl From<LiquidateCometOnassetIldIxArgs> for LiquidateCometOnassetIldIxData {
    fn from(args: LiquidateCometOnassetIldIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidateCometOnassetIldIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_COMET_ONASSET_ILD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidateCometOnassetIldIxArgs {
                user,
                comet_position_index,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_COMET_ONASSET_ILD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.comet_position_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_comet_onasset_ild_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidateCometOnassetIldKeys,
    args: LiquidateCometOnassetIldIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUIDATE_COMET_ONASSET_ILD_IX_ACCOUNTS_LEN] = keys.into();
    let data: LiquidateCometOnassetIldIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_comet_onasset_ild_ix(
    keys: LiquidateCometOnassetIldKeys,
    args: LiquidateCometOnassetIldIxArgs,
) -> std::io::Result<Instruction> {
    liquidate_comet_onasset_ild_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn liquidate_comet_onasset_ild_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateCometOnassetIldAccounts<'_, '_>,
    args: LiquidateCometOnassetIldIxArgs,
) -> ProgramResult {
    let keys: LiquidateCometOnassetIldKeys = accounts.into();
    let ix = liquidate_comet_onasset_ild_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquidate_comet_onasset_ild_invoke(
    accounts: LiquidateCometOnassetIldAccounts<'_, '_>,
    args: LiquidateCometOnassetIldIxArgs,
) -> ProgramResult {
    liquidate_comet_onasset_ild_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn liquidate_comet_onasset_ild_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateCometOnassetIldAccounts<'_, '_>,
    args: LiquidateCometOnassetIldIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidateCometOnassetIldKeys = accounts.into();
    let ix = liquidate_comet_onasset_ild_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquidate_comet_onasset_ild_invoke_signed(
    accounts: LiquidateCometOnassetIldAccounts<'_, '_>,
    args: LiquidateCometOnassetIldIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquidate_comet_onasset_ild_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquidate_comet_onasset_ild_verify_account_keys(
    accounts: LiquidateCometOnassetIldAccounts<'_, '_>,
    keys: LiquidateCometOnassetIldKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidator.key, keys.liquidator),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (
            *accounts.liquidator_onasset_token_account.key,
            keys.liquidator_onasset_token_account,
        ),
        (
            *accounts.liquidator_collateral_token_account.key,
            keys.liquidator_collateral_token_account,
        ),
        (*accounts.vault.key, keys.vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquidate_comet_onasset_ild_verify_writable_privileges<'me, 'info>(
    accounts: LiquidateCometOnassetIldAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.onasset_mint,
        accounts.liquidator_onasset_token_account,
        accounts.liquidator_collateral_token_account,
        accounts.vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquidate_comet_onasset_ild_verify_signer_privileges<'me, 'info>(
    accounts: LiquidateCometOnassetIldAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.liquidator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquidate_comet_onasset_ild_verify_account_privileges<'me, 'info>(
    accounts: LiquidateCometOnassetIldAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquidate_comet_onasset_ild_verify_writable_privileges(accounts)?;
    liquidate_comet_onasset_ild_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateBorrowPositionAccounts<'me, 'info> {
    pub liquidator: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub liquidator_collateral_token_account: &'me AccountInfo<'info>,
    pub liquidator_onasset_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateBorrowPositionKeys {
    pub liquidator: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub user_account: Pubkey,
    pub onasset_mint: Pubkey,
    pub vault: Pubkey,
    pub liquidator_collateral_token_account: Pubkey,
    pub liquidator_onasset_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<LiquidateBorrowPositionAccounts<'_, '_>> for LiquidateBorrowPositionKeys {
    fn from(accounts: LiquidateBorrowPositionAccounts) -> Self {
        Self {
            liquidator: *accounts.liquidator.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            user_account: *accounts.user_account.key,
            onasset_mint: *accounts.onasset_mint.key,
            vault: *accounts.vault.key,
            liquidator_collateral_token_account: *accounts
                .liquidator_collateral_token_account
                .key,
            liquidator_onasset_token_account: *accounts
                .liquidator_onasset_token_account
                .key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LiquidateBorrowPositionKeys>
for [AccountMeta; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidateBorrowPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidator_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidator_onasset_token_account,
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
impl From<[Pubkey; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN]>
for LiquidateBorrowPositionKeys {
    fn from(pubkeys: [Pubkey; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            liquidator: pubkeys[0],
            clone: pubkeys[1],
            pools: pubkeys[2],
            oracles: pubkeys[3],
            user_account: pubkeys[4],
            onasset_mint: pubkeys[5],
            vault: pubkeys[6],
            liquidator_collateral_token_account: pubkeys[7],
            liquidator_onasset_token_account: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<LiquidateBorrowPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUIDATE_BORROW_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidateBorrowPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.liquidator.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.user_account.clone(),
            accounts.onasset_mint.clone(),
            accounts.vault.clone(),
            accounts.liquidator_collateral_token_account.clone(),
            accounts.liquidator_onasset_token_account.clone(),
            accounts.token_program.clone(),
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
            liquidator: &arr[0],
            clone: &arr[1],
            pools: &arr[2],
            oracles: &arr[3],
            user_account: &arr[4],
            onasset_mint: &arr[5],
            vault: &arr[6],
            liquidator_collateral_token_account: &arr[7],
            liquidator_onasset_token_account: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const LIQUIDATE_BORROW_POSITION_IX_DISCM: [u8; 8usize] = [
    235, 201, 17, 133, 234, 72, 84, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidateBorrowPositionIxArgs {
    pub user: Pubkey,
    pub borrow_index: u8,
    pub amount: u64,
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
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let borrow_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidateBorrowPositionIxArgs {
                user,
                borrow_index,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_BORROW_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.borrow_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
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
    liquidate_borrow_position_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
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
    liquidate_borrow_position_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
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
        CLONE_PROGRAM_ID,
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
        (*accounts.liquidator.key, keys.liquidator),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.vault.key, keys.vault),
        (
            *accounts.liquidator_collateral_token_account.key,
            keys.liquidator_collateral_token_account,
        ),
        (
            *accounts.liquidator_onasset_token_account.key,
            keys.liquidator_onasset_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.clone,
        accounts.pools,
        accounts.oracles,
        accounts.user_account,
        accounts.onasset_mint,
        accounts.vault,
        accounts.liquidator_collateral_token_account,
        accounts.liquidator_onasset_token_account,
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
    for should_be_signer in [accounts.liquidator] {
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
pub const COLLECT_LP_REWARDS_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CollectLpRewardsAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub user_onasset_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectLpRewardsKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub collateral_vault: Pubkey,
    pub onasset_mint: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub user_onasset_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectLpRewardsAccounts<'_, '_>> for CollectLpRewardsKeys {
    fn from(accounts: CollectLpRewardsAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            collateral_vault: *accounts.collateral_vault.key,
            onasset_mint: *accounts.onasset_mint.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            user_onasset_token_account: *accounts.user_onasset_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectLpRewardsKeys> for [AccountMeta; COLLECT_LP_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectLpRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_onasset_token_account,
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
impl From<[Pubkey; COLLECT_LP_REWARDS_IX_ACCOUNTS_LEN]> for CollectLpRewardsKeys {
    fn from(pubkeys: [Pubkey; COLLECT_LP_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            collateral_vault: pubkeys[4],
            onasset_mint: pubkeys[5],
            user_collateral_token_account: pubkeys[6],
            user_onasset_token_account: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<CollectLpRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_LP_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectLpRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.collateral_vault.clone(),
            accounts.onasset_mint.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.user_onasset_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_LP_REWARDS_IX_ACCOUNTS_LEN]>
for CollectLpRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_LP_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            collateral_vault: &arr[4],
            onasset_mint: &arr[5],
            user_collateral_token_account: &arr[6],
            user_onasset_token_account: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const COLLECT_LP_REWARDS_IX_DISCM: [u8; 8usize] = [
    141, 134, 109, 237, 96, 31, 249, 148,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectLpRewardsIxArgs {
    pub comet_position_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectLpRewardsIxData(pub CollectLpRewardsIxArgs);
impl From<CollectLpRewardsIxArgs> for CollectLpRewardsIxData {
    fn from(args: CollectLpRewardsIxArgs) -> Self {
        Self(args)
    }
}
impl CollectLpRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_LP_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CollectLpRewardsIxArgs {
                comet_position_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_LP_REWARDS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.comet_position_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_lp_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectLpRewardsKeys,
    args: CollectLpRewardsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_LP_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectLpRewardsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_lp_rewards_ix(
    keys: CollectLpRewardsKeys,
    args: CollectLpRewardsIxArgs,
) -> std::io::Result<Instruction> {
    collect_lp_rewards_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn collect_lp_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectLpRewardsAccounts<'_, '_>,
    args: CollectLpRewardsIxArgs,
) -> ProgramResult {
    let keys: CollectLpRewardsKeys = accounts.into();
    let ix = collect_lp_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_lp_rewards_invoke(
    accounts: CollectLpRewardsAccounts<'_, '_>,
    args: CollectLpRewardsIxArgs,
) -> ProgramResult {
    collect_lp_rewards_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn collect_lp_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectLpRewardsAccounts<'_, '_>,
    args: CollectLpRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectLpRewardsKeys = accounts.into();
    let ix = collect_lp_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_lp_rewards_invoke_signed(
    accounts: CollectLpRewardsAccounts<'_, '_>,
    args: CollectLpRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_lp_rewards_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_lp_rewards_verify_account_keys(
    accounts: CollectLpRewardsAccounts<'_, '_>,
    keys: CollectLpRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.user_onasset_token_account.key, keys.user_onasset_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_lp_rewards_verify_writable_privileges<'me, 'info>(
    accounts: CollectLpRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.collateral_vault,
        accounts.onasset_mint,
        accounts.user_collateral_token_account,
        accounts.user_onasset_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_lp_rewards_verify_signer_privileges<'me, 'info>(
    accounts: CollectLpRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_lp_rewards_verify_account_privileges<'me, 'info>(
    accounts: CollectLpRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_lp_rewards_verify_writable_privileges(accounts)?;
    collect_lp_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAY_IMPERMANENT_LOSS_DEBT_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct PayImpermanentLossDebtAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub payer_collateral_token_account: &'me AccountInfo<'info>,
    pub payer_onasset_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PayImpermanentLossDebtKeys {
    pub payer: Pubkey,
    pub user_account: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub onasset_mint: Pubkey,
    pub payer_collateral_token_account: Pubkey,
    pub payer_onasset_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<PayImpermanentLossDebtAccounts<'_, '_>> for PayImpermanentLossDebtKeys {
    fn from(accounts: PayImpermanentLossDebtAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            user_account: *accounts.user_account.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            collateral_mint: *accounts.collateral_mint.key,
            collateral_vault: *accounts.collateral_vault.key,
            onasset_mint: *accounts.onasset_mint.key,
            payer_collateral_token_account: *accounts.payer_collateral_token_account.key,
            payer_onasset_token_account: *accounts.payer_onasset_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PayImpermanentLossDebtKeys>
for [AccountMeta; PAY_IMPERMANENT_LOSS_DEBT_IX_ACCOUNTS_LEN] {
    fn from(keys: PayImpermanentLossDebtKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_onasset_token_account,
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
impl From<[Pubkey; PAY_IMPERMANENT_LOSS_DEBT_IX_ACCOUNTS_LEN]>
for PayImpermanentLossDebtKeys {
    fn from(pubkeys: [Pubkey; PAY_IMPERMANENT_LOSS_DEBT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            user_account: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            collateral_mint: pubkeys[4],
            collateral_vault: pubkeys[5],
            onasset_mint: pubkeys[6],
            payer_collateral_token_account: pubkeys[7],
            payer_onasset_token_account: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<PayImpermanentLossDebtAccounts<'_, 'info>>
for [AccountInfo<'info>; PAY_IMPERMANENT_LOSS_DEBT_IX_ACCOUNTS_LEN] {
    fn from(accounts: PayImpermanentLossDebtAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.user_account.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.collateral_mint.clone(),
            accounts.collateral_vault.clone(),
            accounts.onasset_mint.clone(),
            accounts.payer_collateral_token_account.clone(),
            accounts.payer_onasset_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAY_IMPERMANENT_LOSS_DEBT_IX_ACCOUNTS_LEN]>
for PayImpermanentLossDebtAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAY_IMPERMANENT_LOSS_DEBT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            user_account: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            collateral_mint: &arr[4],
            collateral_vault: &arr[5],
            onasset_mint: &arr[6],
            payer_collateral_token_account: &arr[7],
            payer_onasset_token_account: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const PAY_IMPERMANENT_LOSS_DEBT_IX_DISCM: [u8; 8usize] = [
    80, 181, 183, 177, 8, 170, 1, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayImpermanentLossDebtIxArgs {
    pub user: Pubkey,
    pub comet_position_index: u8,
    pub amount: u64,
    pub payment_type: PaymentType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PayImpermanentLossDebtIxData(pub PayImpermanentLossDebtIxArgs);
impl From<PayImpermanentLossDebtIxArgs> for PayImpermanentLossDebtIxData {
    fn from(args: PayImpermanentLossDebtIxArgs) -> Self {
        Self(args)
    }
}
impl PayImpermanentLossDebtIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAY_IMPERMANENT_LOSS_DEBT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let payment_type: PaymentType = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PayImpermanentLossDebtIxArgs {
                user,
                comet_position_index,
                amount,
                payment_type,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAY_IMPERMANENT_LOSS_DEBT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.comet_position_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.payment_type, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pay_impermanent_loss_debt_ix_with_program_id(
    program_id: Pubkey,
    keys: PayImpermanentLossDebtKeys,
    args: PayImpermanentLossDebtIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAY_IMPERMANENT_LOSS_DEBT_IX_ACCOUNTS_LEN] = keys.into();
    let data: PayImpermanentLossDebtIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pay_impermanent_loss_debt_ix(
    keys: PayImpermanentLossDebtKeys,
    args: PayImpermanentLossDebtIxArgs,
) -> std::io::Result<Instruction> {
    pay_impermanent_loss_debt_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn pay_impermanent_loss_debt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PayImpermanentLossDebtAccounts<'_, '_>,
    args: PayImpermanentLossDebtIxArgs,
) -> ProgramResult {
    let keys: PayImpermanentLossDebtKeys = accounts.into();
    let ix = pay_impermanent_loss_debt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pay_impermanent_loss_debt_invoke(
    accounts: PayImpermanentLossDebtAccounts<'_, '_>,
    args: PayImpermanentLossDebtIxArgs,
) -> ProgramResult {
    pay_impermanent_loss_debt_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn pay_impermanent_loss_debt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PayImpermanentLossDebtAccounts<'_, '_>,
    args: PayImpermanentLossDebtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PayImpermanentLossDebtKeys = accounts.into();
    let ix = pay_impermanent_loss_debt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pay_impermanent_loss_debt_invoke_signed(
    accounts: PayImpermanentLossDebtAccounts<'_, '_>,
    args: PayImpermanentLossDebtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pay_impermanent_loss_debt_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pay_impermanent_loss_debt_verify_account_keys(
    accounts: PayImpermanentLossDebtAccounts<'_, '_>,
    keys: PayImpermanentLossDebtKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (
            *accounts.payer_collateral_token_account.key,
            keys.payer_collateral_token_account,
        ),
        (*accounts.payer_onasset_token_account.key, keys.payer_onasset_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pay_impermanent_loss_debt_verify_writable_privileges<'me, 'info>(
    accounts: PayImpermanentLossDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_account,
        accounts.clone,
        accounts.collateral_vault,
        accounts.onasset_mint,
        accounts.payer_collateral_token_account,
        accounts.payer_onasset_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pay_impermanent_loss_debt_verify_signer_privileges<'me, 'info>(
    accounts: PayImpermanentLossDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pay_impermanent_loss_debt_verify_account_privileges<'me, 'info>(
    accounts: PayImpermanentLossDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pay_impermanent_loss_debt_verify_writable_privileges(accounts)?;
    pay_impermanent_loss_debt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_USER_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseUserAccountAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseUserAccountKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub destination: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseUserAccountAccounts<'_, '_>> for CloseUserAccountKeys {
    fn from(accounts: CloseUserAccountAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            destination: *accounts.destination.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseUserAccountKeys> for [AccountMeta; CLOSE_USER_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseUserAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
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
impl From<[Pubkey; CLOSE_USER_ACCOUNT_IX_ACCOUNTS_LEN]> for CloseUserAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_USER_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            destination: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CloseUserAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_USER_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseUserAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_account.clone(),
            accounts.destination.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_USER_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseUserAccountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_USER_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            destination: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CLOSE_USER_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    236, 181, 3, 71, 194, 18, 151, 191,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseUserAccountIxData;
impl CloseUserAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_USER_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_USER_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_user_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseUserAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_USER_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseUserAccountIxData.try_to_vec()?,
    })
}
pub fn close_user_account_ix(
    keys: CloseUserAccountKeys,
) -> std::io::Result<Instruction> {
    close_user_account_ix_with_program_id(CLONE_PROGRAM_ID, keys)
}
pub fn close_user_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseUserAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseUserAccountKeys = accounts.into();
    let ix = close_user_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_user_account_invoke(
    accounts: CloseUserAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_user_account_invoke_with_program_id(CLONE_PROGRAM_ID, accounts)
}
pub fn close_user_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseUserAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseUserAccountKeys = accounts.into();
    let ix = close_user_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_user_account_invoke_signed(
    accounts: CloseUserAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_user_account_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, seeds)
}
pub fn close_user_account_verify_account_keys(
    accounts: CloseUserAccountAccounts<'_, '_>,
    keys: CloseUserAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.destination.key, keys.destination),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_user_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseUserAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_account, accounts.destination] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_user_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseUserAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_user_account_verify_account_privileges<'me, 'info>(
    accounts: CloseUserAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_user_account_verify_writable_privileges(accounts)?;
    close_user_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WRAP_ASSET_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WrapAssetAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub underlying_asset_token_account: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub user_asset_token_account: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub user_onasset_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WrapAssetKeys {
    pub user: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub underlying_asset_token_account: Pubkey,
    pub asset_mint: Pubkey,
    pub user_asset_token_account: Pubkey,
    pub onasset_mint: Pubkey,
    pub user_onasset_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<WrapAssetAccounts<'_, '_>> for WrapAssetKeys {
    fn from(accounts: WrapAssetAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            underlying_asset_token_account: *accounts.underlying_asset_token_account.key,
            asset_mint: *accounts.asset_mint.key,
            user_asset_token_account: *accounts.user_asset_token_account.key,
            onasset_mint: *accounts.onasset_mint.key,
            user_onasset_token_account: *accounts.user_onasset_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WrapAssetKeys> for [AccountMeta; WRAP_ASSET_IX_ACCOUNTS_LEN] {
    fn from(keys: WrapAssetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_asset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_asset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_onasset_token_account,
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
impl From<[Pubkey; WRAP_ASSET_IX_ACCOUNTS_LEN]> for WrapAssetKeys {
    fn from(pubkeys: [Pubkey; WRAP_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            clone: pubkeys[1],
            pools: pubkeys[2],
            underlying_asset_token_account: pubkeys[3],
            asset_mint: pubkeys[4],
            user_asset_token_account: pubkeys[5],
            onasset_mint: pubkeys[6],
            user_onasset_token_account: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<WrapAssetAccounts<'_, 'info>>
for [AccountInfo<'info>; WRAP_ASSET_IX_ACCOUNTS_LEN] {
    fn from(accounts: WrapAssetAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.underlying_asset_token_account.clone(),
            accounts.asset_mint.clone(),
            accounts.user_asset_token_account.clone(),
            accounts.onasset_mint.clone(),
            accounts.user_onasset_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WRAP_ASSET_IX_ACCOUNTS_LEN]>
for WrapAssetAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WRAP_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            clone: &arr[1],
            pools: &arr[2],
            underlying_asset_token_account: &arr[3],
            asset_mint: &arr[4],
            user_asset_token_account: &arr[5],
            onasset_mint: &arr[6],
            user_onasset_token_account: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const WRAP_ASSET_IX_DISCM: [u8; 8usize] = [75, 63, 96, 57, 92, 198, 158, 199];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WrapAssetIxArgs {
    pub amount: u64,
    pub pool_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WrapAssetIxData(pub WrapAssetIxArgs);
impl From<WrapAssetIxArgs> for WrapAssetIxData {
    fn from(args: WrapAssetIxArgs) -> Self {
        Self(args)
    }
}
impl WrapAssetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WRAP_ASSET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WrapAssetIxArgs {
                amount,
                pool_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WRAP_ASSET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pool_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn wrap_asset_ix_with_program_id(
    program_id: Pubkey,
    keys: WrapAssetKeys,
    args: WrapAssetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WRAP_ASSET_IX_ACCOUNTS_LEN] = keys.into();
    let data: WrapAssetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn wrap_asset_ix(
    keys: WrapAssetKeys,
    args: WrapAssetIxArgs,
) -> std::io::Result<Instruction> {
    wrap_asset_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn wrap_asset_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WrapAssetAccounts<'_, '_>,
    args: WrapAssetIxArgs,
) -> ProgramResult {
    let keys: WrapAssetKeys = accounts.into();
    let ix = wrap_asset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn wrap_asset_invoke(
    accounts: WrapAssetAccounts<'_, '_>,
    args: WrapAssetIxArgs,
) -> ProgramResult {
    wrap_asset_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn wrap_asset_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WrapAssetAccounts<'_, '_>,
    args: WrapAssetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WrapAssetKeys = accounts.into();
    let ix = wrap_asset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn wrap_asset_invoke_signed(
    accounts: WrapAssetAccounts<'_, '_>,
    args: WrapAssetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    wrap_asset_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, args, seeds)
}
pub fn wrap_asset_verify_account_keys(
    accounts: WrapAssetAccounts<'_, '_>,
    keys: WrapAssetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (
            *accounts.underlying_asset_token_account.key,
            keys.underlying_asset_token_account,
        ),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.user_asset_token_account.key, keys.user_asset_token_account),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.user_onasset_token_account.key, keys.user_onasset_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn wrap_asset_verify_writable_privileges<'me, 'info>(
    accounts: WrapAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.underlying_asset_token_account,
        accounts.user_asset_token_account,
        accounts.onasset_mint,
        accounts.user_onasset_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn wrap_asset_verify_signer_privileges<'me, 'info>(
    accounts: WrapAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn wrap_asset_verify_account_privileges<'me, 'info>(
    accounts: WrapAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    wrap_asset_verify_writable_privileges(accounts)?;
    wrap_asset_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNWRAP_ONASSET_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct UnwrapOnassetAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub underlying_asset_token_account: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub user_asset_token_account: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub user_onasset_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnwrapOnassetKeys {
    pub user: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub underlying_asset_token_account: Pubkey,
    pub asset_mint: Pubkey,
    pub user_asset_token_account: Pubkey,
    pub onasset_mint: Pubkey,
    pub user_onasset_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<UnwrapOnassetAccounts<'_, '_>> for UnwrapOnassetKeys {
    fn from(accounts: UnwrapOnassetAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            underlying_asset_token_account: *accounts.underlying_asset_token_account.key,
            asset_mint: *accounts.asset_mint.key,
            user_asset_token_account: *accounts.user_asset_token_account.key,
            onasset_mint: *accounts.onasset_mint.key,
            user_onasset_token_account: *accounts.user_onasset_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<UnwrapOnassetKeys> for [AccountMeta; UNWRAP_ONASSET_IX_ACCOUNTS_LEN] {
    fn from(keys: UnwrapOnassetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_asset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_asset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_onasset_token_account,
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
impl From<[Pubkey; UNWRAP_ONASSET_IX_ACCOUNTS_LEN]> for UnwrapOnassetKeys {
    fn from(pubkeys: [Pubkey; UNWRAP_ONASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            clone: pubkeys[1],
            pools: pubkeys[2],
            underlying_asset_token_account: pubkeys[3],
            asset_mint: pubkeys[4],
            user_asset_token_account: pubkeys[5],
            onasset_mint: pubkeys[6],
            user_onasset_token_account: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<UnwrapOnassetAccounts<'_, 'info>>
for [AccountInfo<'info>; UNWRAP_ONASSET_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnwrapOnassetAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.underlying_asset_token_account.clone(),
            accounts.asset_mint.clone(),
            accounts.user_asset_token_account.clone(),
            accounts.onasset_mint.clone(),
            accounts.user_onasset_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNWRAP_ONASSET_IX_ACCOUNTS_LEN]>
for UnwrapOnassetAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNWRAP_ONASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            clone: &arr[1],
            pools: &arr[2],
            underlying_asset_token_account: &arr[3],
            asset_mint: &arr[4],
            user_asset_token_account: &arr[5],
            onasset_mint: &arr[6],
            user_onasset_token_account: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const UNWRAP_ONASSET_IX_DISCM: [u8; 8usize] = [92, 116, 66, 13, 122, 101, 99, 33];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnwrapOnassetIxArgs {
    pub amount: u64,
    pub pool_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnwrapOnassetIxData(pub UnwrapOnassetIxArgs);
impl From<UnwrapOnassetIxArgs> for UnwrapOnassetIxData {
    fn from(args: UnwrapOnassetIxArgs) -> Self {
        Self(args)
    }
}
impl UnwrapOnassetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNWRAP_ONASSET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UnwrapOnassetIxArgs {
                amount,
                pool_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNWRAP_ONASSET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pool_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unwrap_onasset_ix_with_program_id(
    program_id: Pubkey,
    keys: UnwrapOnassetKeys,
    args: UnwrapOnassetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNWRAP_ONASSET_IX_ACCOUNTS_LEN] = keys.into();
    let data: UnwrapOnassetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unwrap_onasset_ix(
    keys: UnwrapOnassetKeys,
    args: UnwrapOnassetIxArgs,
) -> std::io::Result<Instruction> {
    unwrap_onasset_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn unwrap_onasset_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnwrapOnassetAccounts<'_, '_>,
    args: UnwrapOnassetIxArgs,
) -> ProgramResult {
    let keys: UnwrapOnassetKeys = accounts.into();
    let ix = unwrap_onasset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn unwrap_onasset_invoke(
    accounts: UnwrapOnassetAccounts<'_, '_>,
    args: UnwrapOnassetIxArgs,
) -> ProgramResult {
    unwrap_onasset_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn unwrap_onasset_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnwrapOnassetAccounts<'_, '_>,
    args: UnwrapOnassetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnwrapOnassetKeys = accounts.into();
    let ix = unwrap_onasset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unwrap_onasset_invoke_signed(
    accounts: UnwrapOnassetAccounts<'_, '_>,
    args: UnwrapOnassetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unwrap_onasset_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, args, seeds)
}
pub fn unwrap_onasset_verify_account_keys(
    accounts: UnwrapOnassetAccounts<'_, '_>,
    keys: UnwrapOnassetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (
            *accounts.underlying_asset_token_account.key,
            keys.underlying_asset_token_account,
        ),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.user_asset_token_account.key, keys.user_asset_token_account),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.user_onasset_token_account.key, keys.user_onasset_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unwrap_onasset_verify_writable_privileges<'me, 'info>(
    accounts: UnwrapOnassetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.underlying_asset_token_account,
        accounts.user_asset_token_account,
        accounts.onasset_mint,
        accounts.user_onasset_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unwrap_onasset_verify_signer_privileges<'me, 'info>(
    accounts: UnwrapOnassetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unwrap_onasset_verify_account_privileges<'me, 'info>(
    accounts: UnwrapOnassetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unwrap_onasset_verify_writable_privileges(accounts)?;
    unwrap_onasset_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_COMET_POSITION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RemoveCometPositionAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_account: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveCometPositionKeys {
    pub user: Pubkey,
    pub user_account: Pubkey,
    pub pools: Pubkey,
}
impl From<RemoveCometPositionAccounts<'_, '_>> for RemoveCometPositionKeys {
    fn from(accounts: RemoveCometPositionAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_account: *accounts.user_account.key,
            pools: *accounts.pools.key,
        }
    }
}
impl From<RemoveCometPositionKeys>
for [AccountMeta; REMOVE_COMET_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveCometPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_COMET_POSITION_IX_ACCOUNTS_LEN]> for RemoveCometPositionKeys {
    fn from(pubkeys: [Pubkey; REMOVE_COMET_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_account: pubkeys[1],
            pools: pubkeys[2],
        }
    }
}
impl<'info> From<RemoveCometPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_COMET_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveCometPositionAccounts<'_, 'info>) -> Self {
        [accounts.user.clone(), accounts.user_account.clone(), accounts.pools.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_COMET_POSITION_IX_ACCOUNTS_LEN]>
for RemoveCometPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_COMET_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_account: &arr[1],
            pools: &arr[2],
        }
    }
}
pub const REMOVE_COMET_POSITION_IX_DISCM: [u8; 8usize] = [
    168, 73, 75, 213, 103, 99, 152, 104,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveCometPositionIxArgs {
    pub comet_position_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveCometPositionIxData(pub RemoveCometPositionIxArgs);
impl From<RemoveCometPositionIxArgs> for RemoveCometPositionIxData {
    fn from(args: RemoveCometPositionIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveCometPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_COMET_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let comet_position_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveCometPositionIxArgs {
                comet_position_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_COMET_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.comet_position_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_comet_position_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveCometPositionKeys,
    args: RemoveCometPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_COMET_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveCometPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_comet_position_ix(
    keys: RemoveCometPositionKeys,
    args: RemoveCometPositionIxArgs,
) -> std::io::Result<Instruction> {
    remove_comet_position_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn remove_comet_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveCometPositionAccounts<'_, '_>,
    args: RemoveCometPositionIxArgs,
) -> ProgramResult {
    let keys: RemoveCometPositionKeys = accounts.into();
    let ix = remove_comet_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_comet_position_invoke(
    accounts: RemoveCometPositionAccounts<'_, '_>,
    args: RemoveCometPositionIxArgs,
) -> ProgramResult {
    remove_comet_position_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn remove_comet_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveCometPositionAccounts<'_, '_>,
    args: RemoveCometPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveCometPositionKeys = accounts.into();
    let ix = remove_comet_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_comet_position_invoke_signed(
    accounts: RemoveCometPositionAccounts<'_, '_>,
    args: RemoveCometPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_comet_position_invoke_signed_with_program_id(
        CLONE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_comet_position_verify_account_keys(
    accounts: RemoveCometPositionAccounts<'_, '_>,
    keys: RemoveCometPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_account.key, keys.user_account),
        (*accounts.pools.key, keys.pools),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_comet_position_verify_writable_privileges<'me, 'info>(
    accounts: RemoveCometPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_account, accounts.pools] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_comet_position_verify_signer_privileges<'me, 'info>(
    accounts: RemoveCometPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_comet_position_verify_account_privileges<'me, 'info>(
    accounts: RemoveCometPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_comet_position_verify_writable_privileges(accounts)?;
    remove_comet_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub user_onasset_token_account: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub treasury_onasset_token_account: &'me AccountInfo<'info>,
    pub treasury_collateral_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub clone_staking: &'me AccountInfo<'info>,
    pub user_staking_account: &'me AccountInfo<'info>,
    pub clone_staking_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub user: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub user_onasset_token_account: Pubkey,
    pub onasset_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub treasury_onasset_token_account: Pubkey,
    pub treasury_collateral_token_account: Pubkey,
    pub token_program: Pubkey,
    pub clone_staking: Pubkey,
    pub user_staking_account: Pubkey,
    pub clone_staking_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            user_onasset_token_account: *accounts.user_onasset_token_account.key,
            onasset_mint: *accounts.onasset_mint.key,
            collateral_mint: *accounts.collateral_mint.key,
            collateral_vault: *accounts.collateral_vault.key,
            treasury_onasset_token_account: *accounts.treasury_onasset_token_account.key,
            treasury_collateral_token_account: *accounts
                .treasury_collateral_token_account
                .key,
            token_program: *accounts.token_program.key,
            clone_staking: *accounts.clone_staking.key,
            user_staking_account: *accounts.user_staking_account.key,
            clone_staking_program: *accounts.clone_staking_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_onasset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_onasset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone_staking,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_staking_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone_staking_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            clone: pubkeys[1],
            pools: pubkeys[2],
            oracles: pubkeys[3],
            user_collateral_token_account: pubkeys[4],
            user_onasset_token_account: pubkeys[5],
            onasset_mint: pubkeys[6],
            collateral_mint: pubkeys[7],
            collateral_vault: pubkeys[8],
            treasury_onasset_token_account: pubkeys[9],
            treasury_collateral_token_account: pubkeys[10],
            token_program: pubkeys[11],
            clone_staking: pubkeys[12],
            user_staking_account: pubkeys[13],
            clone_staking_program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.user_onasset_token_account.clone(),
            accounts.onasset_mint.clone(),
            accounts.collateral_mint.clone(),
            accounts.collateral_vault.clone(),
            accounts.treasury_onasset_token_account.clone(),
            accounts.treasury_collateral_token_account.clone(),
            accounts.token_program.clone(),
            accounts.clone_staking.clone(),
            accounts.user_staking_account.clone(),
            accounts.clone_staking_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            clone: &arr[1],
            pools: &arr[2],
            oracles: &arr[3],
            user_collateral_token_account: &arr[4],
            user_onasset_token_account: &arr[5],
            onasset_mint: &arr[6],
            collateral_mint: &arr[7],
            collateral_vault: &arr[8],
            treasury_onasset_token_account: &arr[9],
            treasury_collateral_token_account: &arr[10],
            token_program: &arr[11],
            clone_staking: &arr[12],
            user_staking_account: &arr[13],
            clone_staking_program: &arr[14],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub pool_index: u8,
    pub quantity: u64,
    pub quantity_is_input: bool,
    pub quantity_is_collateral: bool,
    pub result_threshold: u64,
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
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quantity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quantity_is_input: bool = crate::borsh_de_or_default(&mut reader)?;
        let quantity_is_collateral: bool = crate::borsh_de_or_default(&mut reader)?;
        let result_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapIxArgs {
                pool_index,
                quantity,
                quantity_is_input,
                quantity_is_collateral,
                result_threshold,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pool_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quantity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quantity_is_input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quantity_is_collateral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.result_threshold, &mut writer)?;
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
    swap_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (*accounts.user_onasset_token_account.key, keys.user_onasset_token_account),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (
            *accounts.treasury_onasset_token_account.key,
            keys.treasury_onasset_token_account,
        ),
        (
            *accounts.treasury_collateral_token_account.key,
            keys.treasury_collateral_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.clone_staking.key, keys.clone_staking),
        (*accounts.user_staking_account.key, keys.user_staking_account),
        (*accounts.clone_staking_program.key, keys.clone_staking_program),
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
        accounts.clone,
        accounts.pools,
        accounts.oracles,
        accounts.user_collateral_token_account,
        accounts.user_onasset_token_account,
        accounts.onasset_mint,
        accounts.collateral_mint,
        accounts.collateral_vault,
        accounts.treasury_onasset_token_account,
        accounts.treasury_collateral_token_account,
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
pub const CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenMetadataAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub metaplex_program: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenMetadataKeys {
    pub admin: Pubkey,
    pub clone: Pubkey,
    pub mint: Pubkey,
    pub metaplex_program: Pubkey,
    pub metadata: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTokenMetadataAccounts<'_, '_>> for CreateTokenMetadataKeys {
    fn from(accounts: CreateTokenMetadataAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            clone: *accounts.clone.key,
            mint: *accounts.mint.key,
            metaplex_program: *accounts.metaplex_program.key,
            metadata: *accounts.metadata.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
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
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metaplex_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
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
impl From<[Pubkey; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]> for CreateTokenMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            clone: pubkeys[1],
            mint: pubkeys[2],
            metaplex_program: pubkeys[3],
            metadata: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateTokenMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TOKEN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.clone.clone(),
            accounts.mint.clone(),
            accounts.metaplex_program.clone(),
            accounts.metadata.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
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
            clone: &arr[1],
            mint: &arr[2],
            metaplex_program: &arr[3],
            metadata: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CREATE_TOKEN_METADATA_IX_DISCM: [u8; 8usize] = [
    221, 80, 176, 37, 153, 188, 160, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTokenMetadataIxArgs {
    pub metadata_args: MetadataArgs,
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
        let metadata_args = if reader.is_empty() {
            Default::default()
        } else {
            <MetadataArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateTokenMetadataIxArgs {
                metadata_args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.metadata_args, &mut writer)?;
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
    create_token_metadata_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
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
    create_token_metadata_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
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
        CLONE_PROGRAM_ID,
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
        (*accounts.clone.key, keys.clone),
        (*accounts.mint.key, keys.mint),
        (*accounts.metaplex_program.key, keys.metaplex_program),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
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
pub const REMOVE_POOL_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RemovePoolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub underlying_asset_mint: &'me AccountInfo<'info>,
    pub underlying_asset_token_account: &'me AccountInfo<'info>,
    pub treasury_asset_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePoolKeys {
    pub admin: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub underlying_asset_mint: Pubkey,
    pub underlying_asset_token_account: Pubkey,
    pub treasury_asset_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<RemovePoolAccounts<'_, '_>> for RemovePoolKeys {
    fn from(accounts: RemovePoolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            underlying_asset_mint: *accounts.underlying_asset_mint.key,
            underlying_asset_token_account: *accounts.underlying_asset_token_account.key,
            treasury_asset_token_account: *accounts.treasury_asset_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RemovePoolKeys> for [AccountMeta; REMOVE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: RemovePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.underlying_asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.underlying_asset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_asset_token_account,
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
impl From<[Pubkey; REMOVE_POOL_IX_ACCOUNTS_LEN]> for RemovePoolKeys {
    fn from(pubkeys: [Pubkey; REMOVE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            clone: pubkeys[1],
            pools: pubkeys[2],
            underlying_asset_mint: pubkeys[3],
            underlying_asset_token_account: pubkeys[4],
            treasury_asset_token_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<RemovePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemovePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.underlying_asset_mint.clone(),
            accounts.underlying_asset_token_account.clone(),
            accounts.treasury_asset_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_POOL_IX_ACCOUNTS_LEN]>
for RemovePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            clone: &arr[1],
            pools: &arr[2],
            underlying_asset_mint: &arr[3],
            underlying_asset_token_account: &arr[4],
            treasury_asset_token_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const REMOVE_POOL_IX_DISCM: [u8; 8usize] = [132, 42, 53, 138, 28, 220, 170, 55];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemovePoolIxArgs {
    pub pool_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePoolIxData(pub RemovePoolIxArgs);
impl From<RemovePoolIxArgs> for RemovePoolIxData {
    fn from(args: RemovePoolIxArgs) -> Self {
        Self(args)
    }
}
impl RemovePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(RemovePoolIxArgs { pool_index }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pool_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: RemovePoolKeys,
    args: RemovePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemovePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_pool_ix(
    keys: RemovePoolKeys,
    args: RemovePoolIxArgs,
) -> std::io::Result<Instruction> {
    remove_pool_ix_with_program_id(CLONE_PROGRAM_ID, keys, args)
}
pub fn remove_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemovePoolAccounts<'_, '_>,
    args: RemovePoolIxArgs,
) -> ProgramResult {
    let keys: RemovePoolKeys = accounts.into();
    let ix = remove_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_pool_invoke(
    accounts: RemovePoolAccounts<'_, '_>,
    args: RemovePoolIxArgs,
) -> ProgramResult {
    remove_pool_invoke_with_program_id(CLONE_PROGRAM_ID, accounts, args)
}
pub fn remove_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemovePoolAccounts<'_, '_>,
    args: RemovePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemovePoolKeys = accounts.into();
    let ix = remove_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_pool_invoke_signed(
    accounts: RemovePoolAccounts<'_, '_>,
    args: RemovePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_pool_invoke_signed_with_program_id(CLONE_PROGRAM_ID, accounts, args, seeds)
}
pub fn remove_pool_verify_account_keys(
    accounts: RemovePoolAccounts<'_, '_>,
    keys: RemovePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.underlying_asset_mint.key, keys.underlying_asset_mint),
        (
            *accounts.underlying_asset_token_account.key,
            keys.underlying_asset_token_account,
        ),
        (*accounts.treasury_asset_token_account.key, keys.treasury_asset_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_pool_verify_writable_privileges<'me, 'info>(
    accounts: RemovePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.pools,
        accounts.underlying_asset_token_account,
        accounts.treasury_asset_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_pool_verify_signer_privileges<'me, 'info>(
    accounts: RemovePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_pool_verify_account_privileges<'me, 'info>(
    accounts: RemovePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_pool_verify_writable_privileges(accounts)?;
    remove_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
